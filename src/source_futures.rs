// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use futures_channel::{mpsc, oneshot};
use futures_core::future::Future;
use futures_core::stream::Stream;
use futures_core::task;
use futures_core::task::Poll;
use futures_util::future::FutureExt;
use futures_util::stream::StreamExt;
use std::marker::Unpin;
use std::pin;
use std::pin::Pin;

use Continue;
use MainContext;
use Priority;
use Source;

/// Represents a `Future` around a `glib::Source`. The future will
/// be resolved once the source has provided a value
pub struct SourceFuture<F, T> {
    create_source: Option<F>,
    source: Option<(Source, oneshot::Receiver<T>)>,
}

impl<F, T: 'static> SourceFuture<F, T>
where
    F: FnOnce(oneshot::Sender<T>) -> Source + 'static,
{
    /// Create a new `SourceFuture`
    ///
    /// The provided closure should return a newly created `glib::Source` when called
    /// and pass the value provided by the source to the oneshot sender that is passed
    /// to the closure.
    pub fn new(create_source: F) -> SourceFuture<F, T> {
        SourceFuture {
            create_source: Some(create_source),
            source: None,
        }
    }
}

impl<F, T> Unpin for SourceFuture<F, T> {}

impl<F, T> Future for SourceFuture<F, T>
where
    F: FnOnce(oneshot::Sender<T>) -> Source + 'static,
{
    type Output = T;

    fn poll(mut self: pin::Pin<&mut Self>, ctx: &mut task::Context) -> Poll<T> {
        let SourceFuture {
            ref mut create_source,
            ref mut source,
            ..
        } = *self;

        if let Some(create_source) = create_source.take() {
            let main_context = MainContext::ref_thread_default();
            assert!(
                main_context.is_owner(),
                "Spawning futures only allowed if the thread is owning the MainContext"
            );

            // Channel for sending back the Source result to our future here.
            //
            // In theory we could directly continue polling the
            // corresponding task from the Source callback,
            // however this would break at the very least
            // the g_main_current_source() API.
            let (send, recv) = oneshot::channel();

            let s = create_source(send);

            s.attach(Some(&main_context));
            *source = Some((s, recv));
        }

        // At this point we must have a receiver
        let res = {
            let &mut (_, ref mut receiver) = source.as_mut().unwrap();
            receiver.poll_unpin(ctx)
        };
        #[allow(clippy::match_wild_err_arm)]
        match res {
            Poll::Ready(Err(_)) => panic!("Source sender was unexpectedly closed"),
            Poll::Ready(Ok(v)) => {
                // Get rid of the reference to the source, it triggered
                let _ = source.take();
                Poll::Ready(v)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<T, F> Drop for SourceFuture<T, F> {
    fn drop(&mut self) {
        // Get rid of the source, we don't care anymore if it still triggers
        if let Some((source, _)) = self.source.take() {
            source.destroy();
        }
    }
}

/// Create a `Future` that will resolve after the given number of milliseconds.
///
/// The `Future` must be spawned on an `Executor` backed by a `glib::MainContext`.
pub fn timeout_future(value: u32) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
    timeout_future_with_priority(::PRIORITY_DEFAULT, value)
}

/// Create a `Future` that will resolve after the given number of milliseconds.
///
/// The `Future` must be spawned on an `Executor` backed by a `glib::MainContext`.
pub fn timeout_future_with_priority(
    priority: Priority,
    value: u32,
) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
    Box::pin(SourceFuture::new(move |send| {
        let mut send = Some(send);
        ::timeout_source_new(value, None, priority, move || {
            let _ = send.take().unwrap().send(());
            Continue(false)
        })
    }))
}

/// Create a `Future` that will resolve after the given number of seconds.
///
/// The `Future` must be spawned on an `Executor` backed by a `glib::MainContext`.
pub fn timeout_future_seconds(value: u32) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
    timeout_future_seconds_with_priority(::PRIORITY_DEFAULT, value)
}

/// Create a `Future` that will resolve after the given number of seconds.
///
/// The `Future` must be spawned on an `Executor` backed by a `glib::MainContext`.
pub fn timeout_future_seconds_with_priority(
    priority: Priority,
    value: u32,
) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
    Box::pin(SourceFuture::new(move |send| {
        let mut send = Some(send);
        ::timeout_source_new_seconds(value, None, priority, move || {
            let _ = send.take().unwrap().send(());
            Continue(false)
        })
    }))
}

/// Create a `Future` that will resolve once the child process with the given pid exits
///
/// The `Future` will resolve to the pid of the child process and the exit code.
///
/// The `Future` must be spawned on an `Executor` backed by a `glib::MainContext`.
pub fn child_watch_future(
    pid: ::Pid,
) -> Pin<Box<dyn Future<Output = (::Pid, i32)> + Send + 'static>> {
    child_watch_future_with_priority(::PRIORITY_DEFAULT, pid)
}

/// Create a `Future` that will resolve once the child process with the given pid exits
///
/// The `Future` will resolve to the pid of the child process and the exit code.
///
/// The `Future` must be spawned on an `Executor` backed by a `glib::MainContext`.
pub fn child_watch_future_with_priority(
    priority: Priority,
    pid: ::Pid,
) -> Pin<Box<dyn Future<Output = (::Pid, i32)> + Send + 'static>> {
    Box::pin(SourceFuture::new(move |send| {
        let mut send = Some(send);
        ::child_watch_source_new(pid, None, priority, move |pid, code| {
            let _ = send.take().unwrap().send((pid, code));
        })
    }))
}

#[cfg(any(unix, feature = "dox"))]
/// Create a `Future` that will resolve once the given UNIX signal is raised
///
/// The `Future` must be spawned on an `Executor` backed by a `glib::MainContext`.
pub fn unix_signal_future(signum: i32) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
    unix_signal_future_with_priority(::PRIORITY_DEFAULT, signum)
}

#[cfg(any(unix, feature = "dox"))]
/// Create a `Future` that will resolve once the given UNIX signal is raised
///
/// The `Future` must be spawned on an `Executor` backed by a `glib::MainContext`.
pub fn unix_signal_future_with_priority(
    priority: Priority,
    signum: i32,
) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
    Box::pin(SourceFuture::new(move |send| {
        let mut send = Some(send);
        ::unix_signal_source_new(signum, None, priority, move || {
            let _ = send.take().unwrap().send(());
            Continue(false)
        })
    }))
}

/// Represents a `Stream` around a `glib::Source`. The stream will
/// be provide all values that are provided by the source
pub struct SourceStream<F, T> {
    create_source: Option<F>,
    source: Option<(Source, mpsc::UnboundedReceiver<T>)>,
}

impl<F, T> Unpin for SourceStream<F, T> {}

impl<F, T: 'static> SourceStream<F, T>
where
    F: FnOnce(mpsc::UnboundedSender<T>) -> Source + 'static,
{
    /// Create a new `SourceStream`
    ///
    /// The provided closure should return a newly created `glib::Source` when called
    /// and pass the values provided by the source to the sender that is passed
    /// to the closure.
    pub fn new(create_source: F) -> SourceStream<F, T> {
        SourceStream {
            create_source: Some(create_source),
            source: None,
        }
    }
}

impl<F, T> Stream for SourceStream<F, T>
where
    F: FnOnce(mpsc::UnboundedSender<T>) -> Source + 'static,
{
    type Item = T;

    fn poll_next(mut self: pin::Pin<&mut Self>, ctx: &mut task::Context) -> Poll<Option<T>> {
        let SourceStream {
            ref mut create_source,
            ref mut source,
            ..
        } = *self;

        if let Some(create_source) = create_source.take() {
            let main_context = MainContext::ref_thread_default();
            assert!(
                main_context.is_owner(),
                "Spawning futures only allowed if the thread is owning the MainContext"
            );

            // Channel for sending back the Source result to our future here.
            //
            // In theory we could directly continue polling the
            // corresponding task from the Source callback,
            // however this would break at the very least
            // the g_main_current_source() API.
            let (send, recv) = mpsc::unbounded();

            let s = create_source(send);

            s.attach(Some(&main_context));
            *source = Some((s, recv));
        }

        // At this point we must have a receiver
        let res = {
            let &mut (_, ref mut receiver) = source.as_mut().unwrap();
            receiver.poll_next_unpin(ctx)
        };
        #[allow(clippy::match_wild_err_arm)]
        match res {
            Poll::Ready(v) => {
                if v.is_none() {
                    // Get rid of the reference to the source, it triggered
                    let _ = source.take();
                }
                Poll::Ready(v)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<T, F> Drop for SourceStream<T, F> {
    fn drop(&mut self) {
        // Get rid of the source, we don't care anymore if it still triggers
        if let Some((source, _)) = self.source.take() {
            source.destroy();
        }
    }
}

/// Create a `Stream` that will provide a value every given number of milliseconds.
///
/// The `Future` must be spawned on an `Executor` backed by a `glib::MainContext`.
pub fn interval_stream(value: u32) -> Pin<Box<dyn Stream<Item = ()> + Send + 'static>> {
    interval_stream_with_priority(::PRIORITY_DEFAULT, value)
}

/// Create a `Stream` that will provide a value every given number of milliseconds.
///
/// The `Future` must be spawned on an `Executor` backed by a `glib::MainContext`.
pub fn interval_stream_with_priority(
    priority: Priority,
    value: u32,
) -> Pin<Box<dyn Stream<Item = ()> + Send + 'static>> {
    Box::pin(SourceStream::new(move |send| {
        ::timeout_source_new(value, None, priority, move || {
            if send.unbounded_send(()).is_err() {
                Continue(false)
            } else {
                Continue(true)
            }
        })
    }))
}

/// Create a `Stream` that will provide a value every given number of seconds.
///
/// The `Stream` must be spawned on an `Executor` backed by a `glib::MainContext`.
pub fn interval_stream_seconds(value: u32) -> Pin<Box<dyn Stream<Item = ()> + Send + 'static>> {
    interval_stream_seconds_with_priority(::PRIORITY_DEFAULT, value)
}

/// Create a `Stream` that will provide a value every given number of seconds.
///
/// The `Stream` must be spawned on an `Executor` backed by a `glib::MainContext`.
pub fn interval_stream_seconds_with_priority(
    priority: Priority,
    value: u32,
) -> Pin<Box<dyn Stream<Item = ()> + Send + 'static>> {
    Box::pin(SourceStream::new(move |send| {
        ::timeout_source_new_seconds(value, None, priority, move || {
            if send.unbounded_send(()).is_err() {
                Continue(false)
            } else {
                Continue(true)
            }
        })
    }))
}

#[cfg(any(unix, feature = "dox"))]
/// Create a `Stream` that will provide a value whenever the given UNIX signal is raised
///
/// The `Stream` must be spawned on an `Executor` backed by a `glib::MainContext`.
pub fn unix_signal_stream(signum: i32) -> Pin<Box<dyn Stream<Item = ()> + Send + 'static>> {
    unix_signal_stream_with_priority(::PRIORITY_DEFAULT, signum)
}

#[cfg(any(unix, feature = "dox"))]
/// Create a `Stream` that will provide a value whenever the given UNIX signal is raised
///
/// The `Stream` must be spawned on an `Executor` backed by a `glib::MainContext`.
pub fn unix_signal_stream_with_priority(
    priority: Priority,
    signum: i32,
) -> Pin<Box<dyn Stream<Item = ()> + Send + 'static>> {
    Box::pin(SourceStream::new(move |send| {
        ::unix_signal_source_new(signum, None, priority, move || {
            if send.unbounded_send(()).is_err() {
                Continue(false)
            } else {
                Continue(true)
            }
        })
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_timeout() {
        let c = MainContext::new();

        let res = c.block_on(timeout_future(20));

        assert_eq!(res, ());
    }

    #[test]
    fn test_timeout_send() {
        let c = MainContext::new();
        let l = ::MainLoop::new(Some(&c), false);

        let l_clone = l.clone();
        c.spawn(timeout_future(20).then(move |()| {
            l_clone.quit();
            futures_util::future::ready(())
        }));

        l.run();
    }

    #[test]
    fn test_interval() {
        let c = MainContext::new();

        let mut count = 0;

        {
            let count = &mut count;
            let res = c.block_on(
                interval_stream(20)
                    .take(2)
                    .for_each(|()| {
                        *count = *count + 1;

                        futures_util::future::ready(())
                    })
                    .map(|_| ()),
            );

            assert_eq!(res, ());
        }

        assert_eq!(count, 2);
    }

    #[test]
    fn test_timeout_and_channel() {
        let c = MainContext::default();

        let res = c.block_on(timeout_future(20).then(|()| {
            let (sender, receiver) = oneshot::channel();

            thread::spawn(move || {
                sender.send(1).unwrap();
            });

            receiver.then(|i| futures_util::future::ready(i.unwrap()))
        }));

        assert_eq!(res, 1);
    }
}

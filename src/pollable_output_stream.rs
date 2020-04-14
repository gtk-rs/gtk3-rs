// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use error::to_std_io_result;
use futures_channel::oneshot;
use futures_core::task::{Context, Poll};
use futures_core::Future;
use futures_io::AsyncWrite;
use gio_sys;
use glib;
use glib::object::{Cast, IsA};
use glib::translate::*;
use glib_sys;
use std::cell::RefCell;
use std::io;
use std::mem::transmute;
use std::pin::Pin;
use Cancellable;
use OutputStreamExt;
use PollableOutputStream;
use PollableOutputStreamExt;

use futures_core::stream::Stream;

pub trait PollableOutputStreamExtManual {
    fn create_source<F, C>(
        &self,
        cancellable: Option<&C>,
        name: Option<&str>,
        priority: glib::Priority,
        func: F,
    ) -> glib::Source
    where
        F: FnMut(&Self) -> glib::Continue + 'static,
        C: IsA<Cancellable>;

    fn create_source_future<C: IsA<Cancellable>>(
        &self,
        cancellable: Option<&C>,
        priority: glib::Priority,
    ) -> Pin<Box<dyn std::future::Future<Output = ()> + 'static>>;

    fn create_source_stream<C: IsA<Cancellable>>(
        &self,
        cancellable: Option<&C>,
        priority: glib::Priority,
    ) -> Pin<Box<dyn Stream<Item = ()> + 'static>>;

    fn into_async_write(self) -> Result<OutputStreamAsyncWrite<Self>, Self>
    where
        Self: IsA<PollableOutputStream>,
    {
        if self.can_poll() {
            Ok(OutputStreamAsyncWrite(self, None))
        } else {
            Err(self)
        }
    }
}

impl<O: IsA<PollableOutputStream>> PollableOutputStreamExtManual for O {
    fn create_source<F, C>(
        &self,
        cancellable: Option<&C>,
        name: Option<&str>,
        priority: glib::Priority,
        func: F,
    ) -> glib::Source
    where
        F: FnMut(&Self) -> glib::Continue + 'static,
        C: IsA<Cancellable>,
    {
        unsafe extern "C" fn trampoline<
            O: IsA<PollableOutputStream>,
            F: FnMut(&O) -> glib::Continue + 'static,
        >(
            stream: *mut gio_sys::GPollableOutputStream,
            func: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let func: &RefCell<F> = &*(func as *const RefCell<F>);
            let mut func = func.borrow_mut();
            (&mut *func)(&PollableOutputStream::from_glib_borrow(stream).unsafe_cast_ref())
                .to_glib()
        }
        unsafe extern "C" fn destroy_closure<O, F>(ptr: glib_sys::gpointer) {
            Box::<RefCell<F>>::from_raw(ptr as *mut _);
        }
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        unsafe {
            let source = gio_sys::g_pollable_output_stream_create_source(
                self.as_ref().to_glib_none().0,
                gcancellable.0,
            );

            let trampoline = trampoline::<Self, F> as glib_sys::gpointer;
            glib_sys::g_source_set_callback(
                source,
                Some(transmute::<
                    _,
                    unsafe extern "C" fn(glib_sys::gpointer) -> glib_sys::gboolean,
                >(trampoline)),
                Box::into_raw(Box::new(RefCell::new(func))) as glib_sys::gpointer,
                Some(destroy_closure::<Self, F>),
            );
            glib_sys::g_source_set_priority(source, priority.to_glib());

            if let Some(name) = name {
                glib_sys::g_source_set_name(source, name.to_glib_none().0);
            }

            from_glib_full(source)
        }
    }

    fn create_source_future<C: IsA<Cancellable>>(
        &self,
        cancellable: Option<&C>,
        priority: glib::Priority,
    ) -> Pin<Box<dyn std::future::Future<Output = ()> + 'static>> {
        let cancellable: Option<Cancellable> = cancellable.map(|c| c.as_ref()).cloned();

        let obj = self.clone();
        Box::pin(glib::SourceFuture::new(move |send| {
            let mut send = Some(send);
            obj.create_source(cancellable.as_ref(), None, priority, move |_| {
                let _ = send.take().unwrap().send(());
                glib::Continue(false)
            })
        }))
    }

    fn create_source_stream<C: IsA<Cancellable>>(
        &self,
        cancellable: Option<&C>,
        priority: glib::Priority,
    ) -> Pin<Box<dyn Stream<Item = ()> + 'static>> {
        let cancellable: Option<Cancellable> = cancellable.map(|c| c.as_ref()).cloned();

        let obj = self.clone();
        Box::pin(glib::SourceStream::new(move |send| {
            let send = Some(send);
            obj.create_source(cancellable.as_ref(), None, priority, move |_| {
                if send.as_ref().unwrap().unbounded_send(()).is_err() {
                    glib::Continue(false)
                } else {
                    glib::Continue(true)
                }
            })
        }))
    }
}

#[derive(Debug)]
pub struct OutputStreamAsyncWrite<T: IsA<PollableOutputStream>>(
    T,
    Option<oneshot::Receiver<Result<(), glib::Error>>>,
);

impl<T: IsA<PollableOutputStream>> OutputStreamAsyncWrite<T> {
    pub fn into_output_stream(self) -> T {
        self.0
    }

    pub fn output_stream(&self) -> &T {
        &self.0
    }
}

impl<T: IsA<PollableOutputStream>> AsyncWrite for OutputStreamAsyncWrite<T> {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<io::Result<usize>> {
        let stream = Pin::get_ref(self.as_ref());
        let gio_result = stream.0.as_ref().write_nonblocking(buf, ::NONE_CANCELLABLE);

        match gio_result {
            Ok(size) => Poll::Ready(Ok(size as usize)),
            Err(err) => {
                let kind = err.kind::<crate::IOErrorEnum>().unwrap();
                if kind == crate::IOErrorEnum::WouldBlock {
                    let mut waker = Some(cx.waker().clone());
                    let source = stream.0.as_ref().create_source(
                        ::NONE_CANCELLABLE,
                        None,
                        glib::PRIORITY_DEFAULT,
                        move |_| {
                            if let Some(waker) = waker.take() {
                                waker.wake();
                            }
                            glib::Continue(false)
                        },
                    );
                    let main_context = glib::MainContext::ref_thread_default();
                    source.attach(Some(&main_context));

                    Poll::Pending
                } else {
                    Poll::Ready(Err(io::Error::new(io::ErrorKind::from(kind), err)))
                }
            }
        }
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        let stream = unsafe { Pin::get_unchecked_mut(self) };

        let rx = if let Some(ref mut rx) = stream.1 {
            rx
        } else {
            let (tx, rx) = oneshot::channel();
            stream
                .0
                .as_ref()
                .close_async(glib::PRIORITY_DEFAULT, ::NONE_CANCELLABLE, move |res| {
                    let _ = tx.send(res);
                });

            stream.1 = Some(rx);
            stream.1.as_mut().unwrap()
        };

        match Pin::new(rx).poll(cx) {
            Poll::Ready(Ok(res)) => {
                let _ = stream.1.take();
                Poll::Ready(to_std_io_result(res))
            }
            Poll::Ready(Err(_)) => {
                let _ = stream.1.take();
                Poll::Ready(Ok(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        let stream = unsafe { Pin::get_unchecked_mut(self) };

        let rx = if let Some(ref mut rx) = stream.1 {
            rx
        } else {
            let (tx, rx) = oneshot::channel();
            stream
                .0
                .as_ref()
                .flush_async(glib::PRIORITY_DEFAULT, ::NONE_CANCELLABLE, move |res| {
                    let _ = tx.send(res);
                });

            stream.1 = Some(rx);
            stream.1.as_mut().unwrap()
        };

        match Pin::new(rx).poll(cx) {
            Poll::Ready(Ok(res)) => {
                let _ = stream.1.take();
                Poll::Ready(to_std_io_result(res))
            }
            Poll::Ready(Err(_)) => {
                let _ = stream.1.take();
                Poll::Ready(Ok(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

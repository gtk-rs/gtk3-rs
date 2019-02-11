// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::cell::RefCell;
use std::marker::PhantomData;
use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;

use Continue;
use MainContext;
use Priority;
use Source;
use SourceId;

use get_thread_id;

use ffi as glib_ffi;
use translate::{from_glib_full, mut_override, ToGlib, ToGlibPtr};

#[repr(C)]
struct ChannelSource<T> {
    source: glib_ffi::GSource,
    receiver: Option<mpsc::Receiver<T>>,
    callback: Option<RefCell<Box<FnMut(T) -> Continue + 'static>>>,
    source_funcs: Option<Box<glib_ffi::GSourceFuncs>>,
    thread_id: Option<usize>,
    ready: AtomicBool,
}

unsafe extern "C" fn prepare<T>(
    source: *mut glib_ffi::GSource,
    timeout: *mut i32,
) -> glib_ffi::gboolean {
    let source = &*(source as *const ChannelSource<T>);

    *timeout = -1;

    // Check if we have at least one item available in the receiver
    if source.ready.load(Ordering::SeqCst) {
        glib_ffi::GTRUE
    } else {
        glib_ffi::GFALSE
    }
}

unsafe extern "C" fn check<T>(source: *mut glib_ffi::GSource) -> glib_ffi::gboolean {
    let source = &*(source as *const ChannelSource<T>);

    // Check if we have at least one item available in the receiver
    if source.ready.load(Ordering::SeqCst) {
        glib_ffi::GTRUE
    } else {
        glib_ffi::GFALSE
    }
}

unsafe extern "C" fn dispatch<T>(
    source: *mut glib_ffi::GSource,
    callback: glib_ffi::GSourceFunc,
    _user_data: glib_ffi::gpointer,
) -> glib_ffi::gboolean {
    let source = &mut *(source as *mut ChannelSource<T>);
    assert!(callback.is_none());

    glib_ffi::g_source_set_ready_time(&mut source.source, -1);
    source.ready.store(false, Ordering::SeqCst);

    // Check the thread to ensure we're only ever called from the same thread
    assert_eq!(
        get_thread_id(),
        source
            .thread_id
            .expect("ChannelSource called before Receiver was attached"),
        "Source dispatched on a different thread than before"
    );

    // Now iterate over all items that we currently have in the receiver until it is
    // empty again. If all senders are disconnected at some point we remove the GSource
    // from the main context it was attached to as it will never ever be called again.
    let receiver = source
        .receiver
        .as_ref()
        .expect("ChannelSource without Receiver");
    loop {
        match receiver.try_recv() {
            Err(mpsc::TryRecvError::Empty) => break,
            Err(mpsc::TryRecvError::Disconnected) => return glib_ffi::G_SOURCE_REMOVE,
            Ok(item) => {
                let callback = source
                    .callback
                    .as_mut()
                    .expect("ChannelSource called before Receiver was attached");
                if (&mut *callback.borrow_mut())(item) == Continue(false) {
                    return glib_ffi::G_SOURCE_REMOVE;
                }
            }
        }
    }

    glib_ffi::G_SOURCE_CONTINUE
}

unsafe extern "C" fn finalize<T>(source: *mut glib_ffi::GSource) {
    let source = &mut *(source as *mut ChannelSource<T>);

    // Drop all memory we own by taking it out of the Options
    let _ = source.receiver.take();
    let _ = source.callback.take();
    let _ = source.source_funcs.take();
}

impl<T> ChannelSource<T> {
    fn new(receiver: mpsc::Receiver<T>, priority: Priority) -> Source {
        unsafe {
            let source_funcs = Box::new(glib_ffi::GSourceFuncs {
                check: Some(check::<T>),
                prepare: Some(prepare::<T>),
                dispatch: Some(dispatch::<T>),
                finalize: Some(finalize::<T>),
                closure_callback: None,
                closure_marshal: None,
            });

            let source = glib_ffi::g_source_new(
                mut_override(&*source_funcs),
                mem::size_of::<ChannelSource<T>>() as u32,
            ) as *mut ChannelSource<T>;
            assert!(!source.is_null());

            {
                let source = &mut *source;
                ptr::write(&mut source.receiver, Some(receiver));
                ptr::write(&mut source.callback, None);
                ptr::write(&mut source.source_funcs, Some(source_funcs));
                source.thread_id = None;
                source.ready = AtomicBool::new(false);
            }

            glib_ffi::g_source_set_priority(mut_override(&(*source).source), priority.to_glib());

            from_glib_full(source as *mut glib_ffi::GSource)
        }
    }

    fn mark_ready(&self) {
        self.ready.store(true, Ordering::SeqCst);
        unsafe {
            glib_ffi::g_source_set_ready_time(mut_override(&self.source), 0);
        }
    }
}

/// A `Sender` that can be used to send items to the corresponding main context receiver.
///
/// This `Sender` behaves the same as `std::sync::mpsc::Sender`.
///
/// See [`MainContext::channel()`] for how to create such a `Sender`.
///
/// [`MainContext::channel()`]: struct.MainContext.html#method.channel
#[derive(Clone, Debug)]
pub struct Sender<T>(Option<mpsc::Sender<T>>, Source);

impl<T> Sender<T> {
    /// Sends a value to the channel.
    pub fn send(&self, t: T) -> Result<(), mpsc::SendError<T>> {
        // If the source is destroyed the receiver end is disconnected
        if self.1.is_destroyed() {
            return Err(mpsc::SendError(t));
        }

        let sender = self.0.as_ref().expect("No Sender anymore");
        sender.send(t)?;

        // Once sending succeeded, wake up the receiver
        unsafe {
            let source = &*(self.1.to_glib_none().0 as *const ChannelSource<T>);
            source.mark_ready();
        }
        Ok(())
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        // Wake up the Source so that it can be removed if this was the last sender.
        // We have to drop the Sender first because of that
        drop(self.0.take().expect("No Sender anymore"));

        unsafe {
            let source = &*(self.1.to_glib_none().0 as *const ChannelSource<T>);
            source.mark_ready();
        }
    }
}

/// A `SyncSender` that can be used to send items to the corresponding main context receiver.
///
/// This `SyncSender` behaves the same as `std::sync::mpsc::SyncSender`.
///
/// See [`MainContext::sync_channel()`] for how to create such a `SyncSender`.
///
/// [`MainContext::sync_channel()`]: struct.MainContext.html#method.sync_channel
#[derive(Clone, Debug)]
pub struct SyncSender<T>(Option<mpsc::SyncSender<T>>, Source);

impl<T> SyncSender<T> {
    /// Sends a value to the channel and blocks if the channel is full.
    pub fn send(&self, t: T) -> Result<(), mpsc::SendError<T>> {
        // If the source is destroyed the receiver end is disconnected
        if self.1.is_destroyed() {
            return Err(mpsc::SendError(t));
        }

        let sender = self.0.as_ref().expect("No Sender anymore");
        sender.send(t)?;

        // Once sending succeeded, wake up the receiver
        unsafe {
            let source = &*(self.1.to_glib_none().0 as *const ChannelSource<T>);
            source.mark_ready();
        }
        Ok(())
    }

    /// Sends a value to the channel.
    pub fn try_send(&self, t: T) -> Result<(), mpsc::TrySendError<T>> {
        // If the source is destroyed the receiver end is disconnected
        if self.1.is_destroyed() {
            return Err(mpsc::TrySendError::Disconnected(t));
        }

        let sender = self.0.as_ref().expect("No Sender anymore");
        sender.try_send(t)?;

        // Once sending succeeded, wake up the receiver
        unsafe {
            let source = &*(self.1.to_glib_none().0 as *const ChannelSource<T>);
            source.mark_ready();
        }
        Ok(())
    }
}

impl<T> Drop for SyncSender<T> {
    fn drop(&mut self) {
        // Wake up the Source so that it can be removed if this was the last sender.
        // We have to drop the Sender first because of that
        drop(self.0.take().expect("No Sender anymore"));

        unsafe {
            let source = &*(self.1.to_glib_none().0 as *const ChannelSource<T>);
            source.mark_ready();
        }
    }
}

/// A `Receiver` that can be attached to a main context to receive items from its corresponding
/// `Sender` or `SyncSender`.
///
/// See [`MainContext::channel()`] or [`MainContext::sync_channel()`] for how to create
/// such a `Receiver`.
///
/// [`MainContext::channel()`]: struct.MainContext.html#method.channel
/// [`MainContext::sync_channel()`]: struct.MainContext.html#method.sync_channel
#[derive(Debug)]
pub struct Receiver<T>(Option<Source>, PhantomData<*const T>);

// It's safe to send the Receiver to other threads for attaching it as
// long as the items to be sent can also be sent between threads.
unsafe impl<T: Send> Send for Receiver<T> {}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        // If the receiver was never attached to a main context
        // we need to destroy the underlying source
        if let Some(source) = self.0.take() {
            source.destroy();
        }
    }
}

impl<T> Receiver<T> {
    /// Attaches the receiver to the given `context` and calls `func` whenever an item is
    /// available on the channel.
    ///
    /// Passing `None` for the context will attach it to the thread default main context.
    ///
    /// # Panics
    ///
    /// This function panics if called from a thread that is not the owner of the provided
    /// `context`, or if `None` is provided of the thread default main context.
    pub fn attach<'a, P: Into<Option<&'a MainContext>>, F: FnMut(T) -> Continue + 'static>(
        mut self,
        context: P,
        func: F,
    ) -> SourceId {
        let context = context.into();
        unsafe {
            let source = self.0.take().expect("No Source anymore");

            {
                let source = &mut *(source.to_glib_none().0 as *mut ChannelSource<T>);
                source.callback = Some(RefCell::new(Box::new(func)));
                source.thread_id = Some(get_thread_id());
            }

            let id = if let Some(context) = context {
                assert!(context.is_owner());
                source.attach(context)
            } else {
                let context = MainContext::ref_thread_default();
                assert!(context.is_owner());
                source.attach(&context)
            };

            id
        }
    }
}

impl MainContext {
    /// Creates a channel for a main context.
    ///
    /// The `Receiver` has to be attached to a main context at a later time, together with a
    /// closure that will be called for every item sent to a `Sender`.
    ///
    /// The `Sender` can be cloned and both the `Sender` and `Receiver` can be sent to different
    /// threads as long as the item type implements the `Send` trait.
    ///
    /// When the last `Sender` is dropped the channel is removed from the main context. If the
    /// `Receiver` is dropped and not attached to a main context all sending to the `Sender`
    /// will fail.
    ///
    /// The returned `Sender` behaves the same as `std::sync::mpsc::Sender`.
    pub fn channel<T>(priority: Priority) -> (Sender<T>, Receiver<T>) {
        let (sender, receiver) = mpsc::channel();

        let source = ChannelSource::new(receiver, priority);

        let receiver = Receiver(Some(source.clone()), PhantomData);
        let sender = Sender(Some(sender), source);

        (sender, receiver)
    }

    /// Creates a synchronous channel for a main context with a given bound on the capacity of the
    /// channel.
    ///
    /// The `Receiver` has to be attached to a main context at a later time, together with a
    /// closure that will be called for every item sent to a `SyncSender`.
    ///
    /// The `SyncSender` can be cloned and both the `SyncSender` and `Receiver` can be sent to different
    /// threads as long as the item type implements the `Send` trait.
    ///
    /// When the last `SyncSender` is dropped the channel is removed from the main context. If the
    /// `Receiver` is dropped and not attached to a main context all sending to the `SyncSender`
    /// will fail.
    ///
    /// The returned `SyncSender` behaves the same as `std::sync::mpsc::SyncSender`.
    ///
    /// # Panics
    ///
    /// This function will panic if the current thread is not the owner of the main context.
    pub fn sync_channel<T>(priority: Priority, bound: usize) -> (SyncSender<T>, Receiver<T>) {
        let (sender, receiver) = mpsc::sync_channel(bound);

        let source = ChannelSource::new(receiver, priority);

        let receiver = Receiver(Some(source.clone()), PhantomData);
        let sender = SyncSender(Some(sender), source);

        (sender, receiver)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use MainLoop;

    #[test]
    fn test_channel() {
        let c = MainContext::new();
        let l = MainLoop::new(Some(&c), false);

        c.acquire();

        let (sender, receiver) = MainContext::channel(Priority::default());

        let sum = Rc::new(RefCell::new(0));
        let sum_clone = sum.clone();
        let l_clone = l.clone();
        receiver.attach(&c, move |item| {
            *sum_clone.borrow_mut() += item;
            if *sum_clone.borrow() == 6 {
                l_clone.quit();
                Continue(false)
            } else {
                Continue(true)
            }
        });

        sender.send(1).unwrap();
        sender.send(2).unwrap();
        sender.send(3).unwrap();

        l.run();

        assert_eq!(*sum.borrow(), 6);
    }

    #[test]
    fn test_drop_sender() {
        let c = MainContext::new();
        let l = MainLoop::new(Some(&c), false);

        c.acquire();

        let (sender, receiver) = MainContext::channel::<i32>(Priority::default());

        struct Helper(MainLoop);
        impl Drop for Helper {
            fn drop(&mut self) {
                self.0.quit();
            }
        }

        let helper = Helper(l.clone());
        receiver.attach(&c, move |_| {
            let _ = helper;

            Continue(true)
        });

        drop(sender);

        l.run();
    }

    #[test]
    fn test_drop_receiver() {
        let (sender, receiver) = MainContext::channel::<i32>(Priority::default());

        drop(receiver);
        assert_eq!(sender.send(1), Err(mpsc::SendError(1)));
    }
}

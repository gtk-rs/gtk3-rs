// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use futures_core::task::{Context, Poll};
use futures_io::AsyncRead;
use gio_sys;
use glib;
use glib::object::{Cast, IsA};
use glib::translate::*;
use glib_sys;
use std::cell::RefCell;
use std::io;
use std::mem::transmute;
use std::ptr;
use Cancellable;
use PollableInputStream;
use PollableInputStreamExt;

use futures_core::stream::Stream;
use std::pin::Pin;

pub trait PollableInputStreamExtManual: Sized {
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

    fn read_nonblocking<C: IsA<Cancellable>>(
        &self,
        buffer: &mut [u8],
        cancellable: Option<&C>,
    ) -> Result<isize, glib::Error>;

    fn into_async_read(self) -> Result<InputStreamAsyncRead<Self>, Self>
    where
        Self: IsA<PollableInputStream>,
    {
        if self.can_poll() {
            Ok(InputStreamAsyncRead(self))
        } else {
            Err(self)
        }
    }
}

impl<O: IsA<PollableInputStream>> PollableInputStreamExtManual for O {
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
            O: IsA<PollableInputStream>,
            F: FnMut(&O) -> glib::Continue + 'static,
        >(
            stream: *mut gio_sys::GPollableInputStream,
            func: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let func: &RefCell<F> = &*(func as *const RefCell<F>);
            let mut func = func.borrow_mut();
            (&mut *func)(&PollableInputStream::from_glib_borrow(stream).unsafe_cast()).to_glib()
        }
        unsafe extern "C" fn destroy_closure<O, F>(ptr: glib_sys::gpointer) {
            Box::<RefCell<F>>::from_raw(ptr as *mut _);
        }
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        unsafe {
            let source = gio_sys::g_pollable_input_stream_create_source(
                self.as_ref().to_glib_none().0,
                gcancellable.0,
            );

            let trampoline = trampoline::<Self, F> as glib_sys::gpointer;
            glib_sys::g_source_set_callback(
                source,
                Some(transmute(trampoline)),
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

    fn read_nonblocking<C: IsA<Cancellable>>(
        &self,
        buffer: &mut [u8],
        cancellable: Option<&C>,
    ) -> Result<isize, glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let count = buffer.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_pollable_input_stream_read_nonblocking(
                self.as_ref().to_glib_none().0,
                buffer.to_glib_none().0,
                count,
                gcancellable.0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
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
            obj.create_source(cancellable.as_ref(), None, priority, move |_| {
                if send.unbounded_send(()).is_err() {
                    glib::Continue(false)
                } else {
                    glib::Continue(true)
                }
            })
        }))
    }
}

#[derive(Debug)]
pub struct InputStreamAsyncRead<T: IsA<PollableInputStream>>(T);

impl<T: IsA<PollableInputStream>> InputStreamAsyncRead<T> {
    pub fn into_input_stream(self) -> T {
        self.0
    }

    pub fn input_stream(&self) -> &T {
        &self.0
    }
}

impl<T: IsA<PollableInputStream>> AsyncRead for InputStreamAsyncRead<T> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let stream = Pin::get_ref(self.as_ref());
        let gio_result = stream.0.as_ref().read_nonblocking(buf, ::NONE_CANCELLABLE);

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
}

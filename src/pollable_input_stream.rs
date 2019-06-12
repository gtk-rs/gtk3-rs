// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use fragile::Fragile;
use gio_sys;
use glib;
use glib::object::{Cast, IsA};
use glib::translate::*;
use glib_sys;
use std::cell::RefCell;
use std::mem::transmute;
use std::ptr;
use Cancellable;
use Error;
use PollableInputStream;

#[cfg(feature = "futures")]
use futures::future::Future;
#[cfg(feature = "futures")]
use futures::stream::Stream;

pub trait PollableInputStreamExtManual: Sized {
    fn create_source<F>(
        &self,
        cancellable: Option<&Cancellable>,
        name: Option<&str>,
        priority: glib::Priority,
        func: F,
    ) -> glib::Source
    where
        F: FnMut(&Self) -> glib::Continue + 'static;

    #[cfg(feature = "futures")]
    fn create_source_future(
        &self,
        cancellable: Option<&Cancellable>,
        priority: glib::Priority,
    ) -> Box<dyn Future<Output = ()> + std::marker::Unpin>;

    #[cfg(feature = "futures")]
    fn create_source_stream(
        &self,
        cancellable: Option<&Cancellable>,
        priority: glib::Priority,
    ) -> Box<dyn Stream<Item = ()> + std::marker::Unpin>;

    fn read_nonblocking(
        &self,
        buffer: &mut [u8],
        cancellable: Option<&Cancellable>,
    ) -> Result<isize, Error>;
}

impl<O: IsA<PollableInputStream>> PollableInputStreamExtManual for O {
    fn create_source<F>(
        &self,
        cancellable: Option<&Cancellable>,
        name: Option<&str>,
        priority: glib::Priority,
        func: F,
    ) -> glib::Source
    where
        F: FnMut(&Self) -> glib::Continue + 'static,
    {
        #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
        unsafe extern "C" fn trampoline<O: IsA<PollableInputStream>>(
            stream: *mut gio_sys::GPollableInputStream,
            func: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let func: &Fragile<RefCell<Box<dyn FnMut(&O) -> glib::Continue + 'static>>> =
                transmute(func);
            let func = func.get();
            let mut func = func.borrow_mut();
            (&mut *func)(&PollableInputStream::from_glib_borrow(stream).unsafe_cast()).to_glib()
        }
        unsafe extern "C" fn destroy_closure<O>(ptr: glib_sys::gpointer) {
            Box::<Fragile<RefCell<Box<dyn FnMut(&O) -> glib::Continue + 'static>>>>::from_raw(
                ptr as *mut _,
            );
        }
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let source = gio_sys::g_pollable_input_stream_create_source(
                self.as_ref().to_glib_none().0,
                cancellable.0,
            );

            let trampoline = trampoline::<Self> as glib_sys::gpointer;
            glib_sys::g_source_set_callback(
                source,
                Some(transmute(trampoline)),
                into_raw(func),
                Some(destroy_closure::<Self>),
            );
            glib_sys::g_source_set_priority(source, priority.to_glib());

            if let Some(name) = name {
                glib_sys::g_source_set_name(source, name.to_glib_none().0);
            }

            from_glib_full(source)
        }
    }

    fn read_nonblocking(
        &self,
        buffer: &mut [u8],
        cancellable: Option<&Cancellable>,
    ) -> Result<isize, Error> {
        let cancellable = cancellable.to_glib_none();
        let count = buffer.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_pollable_input_stream_read_nonblocking(
                self.as_ref().to_glib_none().0,
                buffer.to_glib_none().0,
                count,
                cancellable.0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(feature = "futures")]
    fn create_source_future(
        &self,
        cancellable: Option<&Cancellable>,
        priority: glib::Priority,
    ) -> Box<dyn Future<Output = ()> + std::marker::Unpin> {
        let cancellable: Option<Cancellable> = cancellable.cloned();

        let obj = Fragile::new(self.clone());
        Box::new(glib::SourceFuture::new(move |send| {
            let mut send = Some(send);
            obj.get()
                .create_source(cancellable.as_ref(), None, priority, move |_| {
                    let _ = send.take().unwrap().send(());
                    glib::Continue(false)
                })
        }))
    }

    #[cfg(feature = "futures")]
    fn create_source_stream(
        &self,
        cancellable: Option<&Cancellable>,
        priority: glib::Priority,
    ) -> Box<dyn Stream<Item = ()> + std::marker::Unpin> {
        let cancellable: Option<Cancellable> = cancellable.cloned();

        let obj = Fragile::new(self.clone());
        Box::new(glib::SourceStream::new(move |send| {
            obj.get()
                .create_source(cancellable.as_ref(), None, priority, move |_| {
                    if send.unbounded_send(()).is_err() {
                        glib::Continue(false)
                    } else {
                        glib::Continue(true)
                    }
                })
        }))
    }
}

fn into_raw<O, F: FnMut(&O) -> glib::Continue + 'static>(func: F) -> glib_sys::gpointer {
    let func: Box<Fragile<RefCell<Box<dyn FnMut(&O) -> glib::Continue + 'static>>>> =
        Box::new(Fragile::new(RefCell::new(Box::new(func))));
    Box::into_raw(func) as glib_sys::gpointer
}

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
use std::pin::Pin;
use Cancellable;
use PollableOutputStream;

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
        #[allow(clippy::transmute_ptr_to_ref)]
        unsafe extern "C" fn trampoline<O: IsA<PollableOutputStream>>(
            stream: *mut gio_sys::GPollableOutputStream,
            func: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let func: &Fragile<RefCell<Box<dyn FnMut(&O) -> glib::Continue + 'static>>> =
                transmute(func);
            let func = func.get();
            let mut func = func.borrow_mut();
            (&mut *func)(&PollableOutputStream::from_glib_borrow(stream).unsafe_cast()).to_glib()
        }
        unsafe extern "C" fn destroy_closure<O>(ptr: glib_sys::gpointer) {
            Box::<Fragile<RefCell<Box<dyn FnMut(&O) -> glib::Continue + 'static>>>>::from_raw(
                ptr as *mut _,
            );
        }
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        unsafe {
            let source = gio_sys::g_pollable_output_stream_create_source(
                self.as_ref().to_glib_none().0,
                gcancellable.0,
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

    fn create_source_future<C: IsA<Cancellable>>(
        &self,
        cancellable: Option<&C>,
        priority: glib::Priority,
    ) -> Pin<Box<dyn std::future::Future<Output = ()> + 'static>> {
        let cancellable: Option<Cancellable> = cancellable.map(|c| c.as_ref()).cloned();

        let obj = Fragile::new(self.clone());
        Box::pin(glib::SourceFuture::new(move |send| {
            let mut send = Some(send);
            obj.get()
                .create_source(cancellable.as_ref(), None, priority, move |_| {
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

        let obj = Fragile::new(self.clone());
        Box::pin(glib::SourceStream::new(move |send| {
            let send = Some(send);
            obj.get()
                .create_source(cancellable.as_ref(), None, priority, move |_| {
                    if send.as_ref().unwrap().unbounded_send(()).is_err() {
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

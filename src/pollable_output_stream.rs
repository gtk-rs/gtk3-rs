// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use PollableOutputStream;
use Cancellable;
use ffi;
use glib_ffi;
use glib;
use glib::object::{IsA, Downcast};
use glib::translate::*;
use std::cell::RefCell;
use std::mem::transmute;
use send_cell::SendCell;

#[cfg(feature="futures")]
use futures_core::{Future, Never};
#[cfg(feature="futures")]
use futures_core::stream::Stream;

pub trait PollableOutputStreamExtManual {
    fn create_source<'a, 'b, N: Into<Option<&'b str>>, P: Into<Option<&'a Cancellable>>, F>(&self, cancellable: P, name: N, priority: glib::Priority, func: F) -> glib::Source
    where F: FnMut(&Self) -> glib::Continue + 'static;

    #[cfg(feature="futures")]
    fn create_source_future<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P, priority: glib::Priority) -> Box<Future<Item = Self, Error = Never>>;

    #[cfg(feature="futures")]
    fn create_source_stream<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P, priority: glib::Priority) -> Box<Stream<Item = Self, Error = Never>>;
}

impl<O: IsA<PollableOutputStream> + Clone + 'static> PollableOutputStreamExtManual for O {
    fn create_source<'a, 'b, N: Into<Option<&'b str>>, P: Into<Option<&'a Cancellable>>, F>(&self, cancellable: P, name: N, priority: glib::Priority, func: F) -> glib::Source
    where F: FnMut(&Self) -> glib::Continue + 'static {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let source = ffi::g_pollable_output_stream_create_source(self.to_glib_none().0, cancellable.0);

            let trampoline = trampoline::<Self> as glib_ffi::gpointer;
            glib_ffi::g_source_set_callback(source, Some(transmute(trampoline)), into_raw(func), Some(destroy_closure::<Self>));
            glib_ffi::g_source_set_priority(source, priority.to_glib());

            let name = name.into();
            if let Some(name) = name {
                glib_ffi::g_source_set_name(source, name.to_glib_none().0);
            }

            from_glib_full(source)
        }
    }

    #[cfg(feature="futures")]
    fn create_source_future<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P, priority: glib::Priority) -> Box<Future<Item = Self, Error = Never>> {
        use send_cell::SendCell;

        let cancellable = cancellable.into();
        let cancellable: Option<Cancellable> = cancellable.cloned();

        let obj = SendCell::new(self.clone());
        Box::new(glib::SourceFuture::new(move |send| {
            let mut send = Some(SendCell::new(send));
            obj.borrow().create_source(cancellable.as_ref(), None, priority, move |obj| {
                let _ = send.take().unwrap().into_inner().send(obj.clone());
                glib::Continue(false)
            })
        }))
    }

    #[cfg(feature="futures")]
    fn create_source_stream<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P, priority: glib::Priority) -> Box<Stream<Item = Self, Error = Never>> {
        use send_cell::SendCell;

        let cancellable = cancellable.into();
        let cancellable: Option<Cancellable> = cancellable.cloned();

        let obj = SendCell::new(self.clone());
        Box::new(glib::SourceStream::new(move |send| {
            let send = Some(SendCell::new(send));
            obj.borrow().create_source(cancellable.as_ref(), None, priority, move |obj| {
                if send.as_ref().unwrap().borrow().unbounded_send(obj.clone()).is_err() {
                    glib::Continue(false)
                } else {
                    glib::Continue(true)
                }
            })
        }))
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
unsafe extern "C" fn trampoline<O: IsA<PollableOutputStream>>(stream: *mut ffi::GPollableOutputStream, func: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let func: &SendCell<RefCell<Box<FnMut(&O) -> glib::Continue + 'static>>> = transmute(func);
    let func = func.borrow();
    let mut func = func.borrow_mut();
    (&mut *func)(&PollableOutputStream::from_glib_borrow(stream).downcast_unchecked()).to_glib()
}

unsafe extern "C" fn destroy_closure<O>(ptr: glib_ffi::gpointer) {
    callback_guard!();
    Box::<SendCell<RefCell<Box<FnMut(&O) -> glib::Continue + 'static>>>>::from_raw(ptr as *mut _);
}

fn into_raw<O, F: FnMut(&O) -> glib::Continue + 'static>(func: F) -> glib_ffi::gpointer {
    let func: Box<SendCell<RefCell<Box<FnMut(&O) -> glib::Continue + 'static>>>> =
        Box::new(SendCell::new(RefCell::new(Box::new(func))));
    Box::into_raw(func) as glib_ffi::gpointer
}


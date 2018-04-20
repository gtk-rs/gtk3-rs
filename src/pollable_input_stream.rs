// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use PollableInputStream;
use Cancellable;
use Error;
use ffi;
use glib_ffi;
use glib;
use glib::object::{IsA, Downcast};
use glib::translate::*;
use std::ptr;
use std::cell::RefCell;
use std::mem::transmute;
use send_cell::SendCell;

pub trait PollableInputStreamExtManual {
    fn create_source<'a, 'b, N: Into<Option<&'b str>>, P: Into<Option<&'a Cancellable>>, F>(&self, cancellable: P, name: N, priority: glib::Priority, func: F) -> glib::Source
    where F: FnMut(&Self) -> glib::Continue + 'static;

    fn read_nonblocking<'a, P: Into<Option<&'a Cancellable>>>(&self, buffer: &mut [u8], cancellable: P) -> Result<isize, Error>;
}

impl<O: IsA<PollableInputStream>> PollableInputStreamExtManual for O {
    fn create_source<'a, 'b, N: Into<Option<&'b str>>, P: Into<Option<&'a Cancellable>>, F>(&self, cancellable: P, name: N, priority: glib::Priority, func: F) -> glib::Source
    where F: FnMut(&Self) -> glib::Continue + 'static {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let source = ffi::g_pollable_input_stream_create_source(self.to_glib_none().0, cancellable.0);

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

    fn read_nonblocking<'a, P: Into<Option<&'a Cancellable>>>(&self, buffer: &mut [u8], cancellable: P) -> Result<isize, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let count = buffer.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_pollable_input_stream_read_nonblocking(self.to_glib_none().0, buffer.to_glib_none().0, count, cancellable.0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
unsafe extern "C" fn trampoline<O: IsA<PollableInputStream>>(stream: *mut ffi::GPollableInputStream, func: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let func: &SendCell<RefCell<Box<FnMut(&O) -> glib::Continue + 'static>>> = transmute(func);
    let func = func.borrow();
    let mut func = func.borrow_mut();
    (&mut *func)(&PollableInputStream::from_glib_borrow(stream).downcast_unchecked()).to_glib()
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


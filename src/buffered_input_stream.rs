// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::mem::transmute;
use std::ptr;
use glib_ffi;
use gobject_ffi;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use BufferedInputStream;
use Cancellable;
use Error;

pub trait BufferedInputStreamExtManual {
    fn fill_async<'a, P: Into<Option<&'a Cancellable>>, Q: Fn(Result<isize, Error>) + Send + Sync + 'static>(&self, count: isize, io_priority: i32, cancellable: P, callback: Q);

    fn peek(&self, buffer: &mut [u8], offset: usize) -> usize;
}

impl<O: IsA<BufferedInputStream>> BufferedInputStreamExtManual for O {
    fn fill_async<'a, P: Into<Option<&'a Cancellable>>, Q: Fn(Result<isize, Error>) + Send + Sync + 'static>(&self, count: isize, io_priority: i32, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Fn(Result<isize, Error>) + Send + Sync + 'static>> = Box::new(Box::new(callback));
        extern "C" fn fill_async_trampoline(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            unsafe {
                let mut error = ptr::null_mut();
                let size = ffi::g_buffered_input_stream_fill_finish(_source_object as *mut _, res, &mut error);
                let result = if error.is_null() { Ok(size) } else { Err(from_glib_full(error)) };
                let callback: &&(Fn(Result<isize, Error>) + Send + Sync + 'static) = transmute(user_data);
                callback(result);
            }
        }
        let callback = fill_async_trampoline;
        unsafe {
            ffi::g_buffered_input_stream_fill_async(self.to_glib_none().0, count, io_priority, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn peek(&self, buffer: &mut [u8], offset: usize) -> usize {
        let count = buffer.len() as usize;
        unsafe {
            ffi::g_buffered_input_stream_peek(self.to_glib_none().0, buffer.to_glib_none().0, offset, count)
        }
    }
}

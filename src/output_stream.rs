// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Cancellable;
use Error;
use InputStream;
use OutputStreamSpliceFlags;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::ptr;
use OutputStream;

pub trait OutputStreamExtManual {
    fn splice_async<'a, P: IsA<InputStream>, Q: Into<Option<&'a Cancellable>>, R: Fn(Result<isize, Error>) + Send + 'static>(&self, source: &P, flags: OutputStreamSpliceFlags, io_priority: i32, cancellable: Q, callback: R);

    fn write_async<'a, B: AsRef<[u8]> + Send + 'static, P: Into<Option<&'a Cancellable>>, Q: Fn(Result<isize, Error>) + Send + 'static>(&self, buffer: B, io_priority: i32, cancellable: P, callback: Q);

    fn write_bytes_async<'a, P: Into<Option<&'a Cancellable>>, Q: Fn(Result<isize, Error>) + Send + 'static>(&self, bytes: &glib::Bytes, io_priority: i32, cancellable: P, callback: Q);
}

impl<O: IsA<OutputStream>> OutputStreamExtManual for O {
    fn splice_async<'a, P: IsA<InputStream>, Q: Into<Option<&'a Cancellable>>, R: Fn(Result<isize, Error>) + Send + 'static>(&self, source: &P, flags: OutputStreamSpliceFlags, io_priority: i32, cancellable: Q, callback: R) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Fn(Result<isize, Error>) + Send + 'static>> = Box::new(Box::new(callback));
        extern "C" fn splice_async_trampoline(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            unsafe {
                let mut error = ptr::null_mut();
                let ret = ffi::g_output_stream_splice_finish(_source_object as *mut _, res, &mut error);
                let result = if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) };
                let callback: Box<Box<Fn(Result<isize, Error>) + Send + 'static>> = Box::from_raw(user_data as *mut _);
                callback(result);
            }
        }
        let callback = splice_async_trampoline;
        unsafe {
            ffi::g_output_stream_splice_async(self.to_glib_none().0, source.to_glib_none().0, flags.to_glib(), io_priority, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn write_async<'a, B: AsRef<[u8]> + Send + 'static, P: Into<Option<&'a Cancellable>>, Q: Fn(Result<isize, Error>) + Send + 'static>(&self, buffer: B, io_priority: i32, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let buffer: Box<AsRef<[u8]> + Send + 'static> = Box::new(buffer);
        let (count, buffer_ptr) = {
            let slice = (*buffer).as_ref();
            (slice.len(), slice.as_ptr())
        };
        let user_data: Box<(Box<Fn(Result<isize, Error>) + Send + 'static>, Box<_>)> = Box::new((Box::new(callback), buffer));
        extern "C" fn write_async_trampoline(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            unsafe {
                let user_data: Box<(Box<Fn(Result<isize, Error>) + Send + 'static>, Box<AsRef<[u8]> + Send + 'static>)> = Box::from_raw(user_data as *mut _);
                let mut error = ptr::null_mut();
                let ret = ffi::g_output_stream_write_finish(_source_object as *mut _, res, &mut error);
                let result = if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) };
                user_data.0(result);
            }
        }
        let callback = write_async_trampoline;
        unsafe {
            ffi::g_output_stream_write_async(self.to_glib_none().0, mut_override(buffer_ptr), count, io_priority, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn write_bytes_async<'a, P: Into<Option<&'a Cancellable>>, Q: Fn(Result<isize, Error>) + Send + 'static>(&self, bytes: &glib::Bytes, io_priority: i32, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Fn(Result<isize, Error>) + Send + 'static>> = Box::new(Box::new(callback));
        extern "C" fn write_bytes_async_trampoline(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            unsafe {
                let mut error = ptr::null_mut();
                let ret = ffi::g_output_stream_write_bytes_finish(_source_object as *mut _, res, &mut error);
                let result = if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) };
                let callback: Box<Box<Fn(Result<isize, Error>) + Send + 'static>> = Box::from_raw(user_data as *mut _);
                callback(result);
            }
        }
        let callback = write_bytes_async_trampoline;
        unsafe {
            ffi::g_output_stream_write_bytes_async(self.to_glib_none().0, bytes.to_glib_none().0, io_priority, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }
}

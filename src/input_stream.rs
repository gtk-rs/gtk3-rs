// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>


use Cancellable;
use Error;
use ffi;
#[cfg(any(feature = "v2_34", feature = "dox"))]
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;
use InputStream;

pub trait InputStreamExtManual {
    fn read<'a, B: AsMut<[u8]>, P: Into<Option<&'a Cancellable>>>(&self, buffer: B, cancellable: P) -> Result<isize, Error>;

    fn read_all<'a, P: Into<Option<&'a Cancellable>>>(&self, buffer: Vec<u8>, cancellable: P) -> Result<(Vec<u8>, Option<Error>), (Vec<u8>, Error)>;

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn read_all_async<'a, P: Into<Option<&'a Cancellable>>, Q: Fn(Result<Vec<u8>, Error>) + Send + 'static>(&self, buffer: Vec<u8>, io_priority: i32, cancellable: P, callback: Q);

    fn read_async<'a, P: Into<Option<&'a Cancellable>>, Q: Fn(Result<Vec<u8>, Error>) + Send + 'static>(&self, buffer: Vec<u8>, io_priority: i32, cancellable: P, callback: Q);

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn read_bytes_async<'a, P: Into<Option<&'a Cancellable>>, Q: Fn(Result<glib::Bytes, Error>) + Send + 'static>(&self, count: usize, io_priority: i32, cancellable: P, callback: Q);

    fn skip_async<'a, P: Into<Option<&'a Cancellable>>, Q: Fn(Result<isize, Error>) + Send + 'static>(&self, count: usize, io_priority: i32, cancellable: P, callback: Q);
}

impl<O: IsA<InputStream>> InputStreamExtManual for O {
    fn read<'a, B: AsMut<[u8]>, P: Into<Option<&'a Cancellable>>>(&self, mut buffer: B, cancellable: P) -> Result<isize, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let buffer = buffer.as_mut();
        let count = buffer.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_input_stream_read(self.to_glib_none().0, buffer.to_glib_none().0, count, cancellable.0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_all<'a, P: Into<Option<&'a Cancellable>>>(&self, mut buffer: Vec<u8>, cancellable: P) -> Result<(Vec<u8>, Option<Error>), (Vec<u8>, Error)> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let buffer_ptr = buffer.as_mut_ptr();
        let capacity = buffer.capacity();
        unsafe {
            let mut bytes_read = mem::uninitialized();
            let mut error = ptr::null_mut();
            let _ = ffi::g_input_stream_read_all(self.to_glib_none().0, buffer_ptr, capacity, &mut bytes_read, cancellable.0, &mut error);

            assert!(buffer.capacity() >= bytes_read);
            buffer.set_len(bytes_read);

            if error.is_null() {
                Ok((buffer, None))
            } else if bytes_read != 0 {
                Ok((buffer, Some(from_glib_full(error))))
            } else {
                Err((buffer, from_glib_full(error)))
            }
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn read_all_async<'a, P: Into<Option<&'a Cancellable>>, Q: Fn(Result<Vec<u8>, Error>) + Send + 'static>(&self, mut buffer: Vec<u8>, io_priority: i32, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let buffer_ptr = buffer.as_mut_ptr();
        let capacity = buffer.capacity();
        let user_data: Box<Option<(Box<Fn(Result<Vec<u8>, Error>) + Send + 'static>, Vec<u8>)>> = Box::new(Some((Box::new(callback), buffer)));
        extern "C" fn read_all_async_trampoline(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            unsafe {
                let mut user_data: Box<Option<(Box<Fn(Result<Vec<u8>, Error>) + Send + 'static>, Vec<u8>)>> = Box::from_raw(user_data as *mut _);
                let (callback, mut vec) = user_data.take().unwrap();

                let mut error = ptr::null_mut();
                let mut bytes_read = mem::uninitialized();
                let _ = ffi::g_input_stream_read_all_finish(_source_object as *mut _, res, &mut bytes_read, &mut error);
                let result = if error.is_null() {
                    assert!(vec.capacity() >= bytes_read);
                    vec.set_len(bytes_read);
                    Ok(vec)
                } else {
                    Err(from_glib_full(error))
                };

                callback(result);
            }
        }
        let callback = read_all_async_trampoline;
        unsafe {
            ffi::g_input_stream_read_all_async(self.to_glib_none().0, buffer_ptr, capacity, io_priority, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn read_async<'a, P: Into<Option<&'a Cancellable>>, Q: Fn(Result<Vec<u8>, Error>) + Send + 'static>(&self, mut buffer: Vec<u8>, io_priority: i32, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let buffer_ptr = buffer.as_mut_ptr();
        let capacity = buffer.capacity() as usize;
        let user_data: Box<Option<(Box<Fn(Result<Vec<u8>, Error>) + Send + 'static>, Vec<u8>)>> = Box::new(Some((Box::new(callback), buffer)));
        extern "C" fn read_async_trampoline(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            unsafe {
                let mut user_data: Box<Option<(Box<Fn(Result<Vec<u8>, Error>) + Send + 'static>, Vec<u8>)>> = Box::from_raw(user_data as *mut _);
                let (callback, mut vec) = user_data.take().unwrap();

                let mut error = ptr::null_mut();
                let ret = ffi::g_input_stream_read_finish(_source_object as *mut _, res, &mut error);
                let result = if error.is_null() {
                    assert!(vec.capacity() >= ret as usize);
                    vec.set_len(ret as usize);
                    Ok(vec)
                } else {
                    Err(from_glib_full(error))
                };

                callback(result);
            }
        }
        let callback = read_async_trampoline;
        unsafe {
            ffi::g_input_stream_read_async(self.to_glib_none().0, buffer_ptr, capacity, io_priority, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn read_bytes_async<'a, P: Into<Option<&'a Cancellable>>, Q: Fn(Result<glib::Bytes, Error>) + Send + 'static>(&self, count: usize, io_priority: i32, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Fn(Result<glib::Bytes, Error>) + Send + 'static>> = Box::new(Box::new(callback));
        extern "C" fn read_bytes_async_trampoline(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            unsafe {
                let mut error = ptr::null_mut();
                let ret = ffi::g_input_stream_read_bytes_finish(_source_object as *mut _, res, &mut error);
                let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
                let callback: Box<Box<Fn(Result<glib::Bytes, Error>) + Send + 'static>> = Box::from_raw(user_data as *mut _);
                callback(result);
            }
        }
        let callback = read_bytes_async_trampoline;
        unsafe {
            ffi::g_input_stream_read_bytes_async(self.to_glib_none().0, count, io_priority, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }


    fn skip_async<'a, P: Into<Option<&'a Cancellable>>, Q: Fn(Result<isize, Error>) + Send + 'static>(&self, count: usize, io_priority: i32, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Fn(Result<isize, Error>) + Send + 'static>> = Box::new(Box::new(callback));
        extern "C" fn skip_async_trampoline(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            unsafe {
                let mut error = ptr::null_mut();
                let ret = ffi::g_input_stream_skip_finish(_source_object as *mut _, res, &mut error);
                let result = if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) };
                let callback: Box<Box<Fn(Result<isize, Error>) + Send + 'static>> = Box::from_raw(user_data as *mut _);
                callback(result);
            }
        }
        let callback = skip_async_trampoline;
        unsafe {
            ffi::g_input_stream_skip_async(self.to_glib_none().0, count, io_priority, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }
}

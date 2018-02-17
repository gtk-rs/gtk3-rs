// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Cancellable;
use Error;
use FileCreateFlags;
use FileIOStream;
use FileInfo;
use FileInputStream;
use FileOutputStream;
use FileQueryInfoFlags;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib::Priority;
use glib_ffi;
use gobject_ffi;
use std::ptr;
use File;

pub trait FileExtManual {
    fn append_to_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<FileOutputStream, Error>) + Send + 'static>(&self, flags: FileCreateFlags, io_priority: Priority, cancellable: P, callback: Q);

    fn create_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<FileOutputStream, Error>) + Send + 'static>(&self, flags: FileCreateFlags, io_priority: Priority, cancellable: P, callback: Q);

    fn create_readwrite_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<FileIOStream, Error>) + Send + 'static>(&self, flags: FileCreateFlags, io_priority: Priority, cancellable: P, callback: Q);

    fn open_readwrite_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<FileIOStream, Error>) + Send + 'static>(&self, io_priority: Priority, cancellable: P, callback: Q);

    fn query_filesystem_info_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<FileInfo, Error>) + Send + 'static>(&self, attributes: &str, io_priority: Priority, cancellable: P, callback: Q);

    fn query_info_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<FileInfo, Error>) + Send + 'static>(&self, attributes: &str, flags: FileQueryInfoFlags, io_priority: Priority, cancellable: P, callback: Q);

    fn read_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<FileInputStream, Error>) + Send + 'static>(&self, io_priority: Priority, cancellable: P, callback: Q);

    fn replace_async<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b Cancellable>>, R: FnOnce(Result<FileOutputStream, Error>) + Send + 'static>(&self, etag: P, make_backup: bool, flags: FileCreateFlags, io_priority: Priority, cancellable: Q, callback: R);

    fn replace_readwrite_async<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b Cancellable>>, R: FnOnce(Result<FileIOStream, Error>) + Send + 'static>(&self, etag: P, make_backup: bool, flags: FileCreateFlags, io_priority: Priority, cancellable: Q, callback: R);

    fn set_display_name_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<File, Error>) + Send + 'static>(&self, display_name: &str, io_priority: Priority, cancellable: P, callback: Q);
}

impl<O: IsA<File>> FileExtManual for O {
    fn append_to_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<FileOutputStream, Error>) + Send + 'static>(&self, flags: FileCreateFlags, io_priority: Priority, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn append_to_async_trampoline<Q: FnOnce(Result<FileOutputStream, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_file_append_to_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = append_to_async_trampoline::<Q>;
        unsafe {
            ffi::g_file_append_to_async(self.to_glib_none().0, flags.to_glib(), io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn create_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<FileOutputStream, Error>) + Send + 'static>(&self, flags: FileCreateFlags, io_priority: Priority, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn create_async_trampoline<Q: FnOnce(Result<FileOutputStream, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_file_create_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = create_async_trampoline::<Q>;
        unsafe {
            ffi::g_file_create_async(self.to_glib_none().0, flags.to_glib(), io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn create_readwrite_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<FileIOStream, Error>) + Send + 'static>(&self, flags: FileCreateFlags, io_priority: Priority, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn create_readwrite_async_trampoline<Q: FnOnce(Result<FileIOStream, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_file_create_readwrite_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = create_readwrite_async_trampoline::<Q>;
        unsafe {
            ffi::g_file_create_readwrite_async(self.to_glib_none().0, flags.to_glib(), io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn open_readwrite_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<FileIOStream, Error>) + Send + 'static>(&self, io_priority: Priority, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn open_readwrite_async_trampoline<Q: FnOnce(Result<FileIOStream, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_file_open_readwrite_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = open_readwrite_async_trampoline::<Q>;
        unsafe {
            ffi::g_file_open_readwrite_async(self.to_glib_none().0, io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn query_filesystem_info_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<FileInfo, Error>) + Send + 'static>(&self, attributes: &str, io_priority: Priority, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn query_filesystem_info_async_trampoline<Q: FnOnce(Result<FileInfo, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_file_query_filesystem_info_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = query_filesystem_info_async_trampoline::<Q>;
        unsafe {
            ffi::g_file_query_filesystem_info_async(self.to_glib_none().0, attributes.to_glib_none().0, io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn query_info_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<FileInfo, Error>) + Send + 'static>(&self, attributes: &str, flags: FileQueryInfoFlags, io_priority: Priority, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn query_info_async_trampoline<Q: FnOnce(Result<FileInfo, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_file_query_info_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = query_info_async_trampoline::<Q>;
        unsafe {
            ffi::g_file_query_info_async(self.to_glib_none().0, attributes.to_glib_none().0, flags.to_glib(), io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn read_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<FileInputStream, Error>) + Send + 'static>(&self, io_priority: Priority, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn read_async_trampoline<Q: FnOnce(Result<FileInputStream, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_file_read_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_async_trampoline::<Q>;
        unsafe {
            ffi::g_file_read_async(self.to_glib_none().0, io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn replace_async<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b Cancellable>>, R: FnOnce(Result<FileOutputStream, Error>) + Send + 'static>(&self, etag: P, make_backup: bool, flags: FileCreateFlags, io_priority: Priority, cancellable: Q, callback: R) {
        let etag = etag.into();
        let etag = etag.to_glib_none();
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<R>> = Box::new(Box::new(callback));
        unsafe extern "C" fn replace_async_trampoline<R: FnOnce(Result<FileOutputStream, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_file_replace_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<R>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = replace_async_trampoline::<R>;
        unsafe {
            ffi::g_file_replace_async(self.to_glib_none().0, etag.0, make_backup.to_glib(), flags.to_glib(), io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn replace_readwrite_async<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b Cancellable>>, R: FnOnce(Result<FileIOStream, Error>) + Send + 'static>(&self, etag: P, make_backup: bool, flags: FileCreateFlags, io_priority: Priority, cancellable: Q, callback: R) {
        let etag = etag.into();
        let etag = etag.to_glib_none();
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<R>> = Box::new(Box::new(callback));
        unsafe extern "C" fn replace_readwrite_async_trampoline<R: FnOnce(Result<FileIOStream, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_file_replace_readwrite_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<R>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = replace_readwrite_async_trampoline::<R>;
        unsafe {
            ffi::g_file_replace_readwrite_async(self.to_glib_none().0, etag.0, make_backup.to_glib(), flags.to_glib(), io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn set_display_name_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<File, Error>) + Send + 'static>(&self, display_name: &str, io_priority: Priority, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn set_display_name_async_trampoline<Q: FnOnce(Result<File, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_file_set_display_name_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = set_display_name_async_trampoline::<Q>;
        unsafe {
            ffi::g_file_set_display_name_async(self.to_glib_none().0, display_name.to_glib_none().0, io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }
}

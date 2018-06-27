// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Cancellable;
use Error;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::ptr;
use File;
use FileCreateFlags;

#[cfg(feature = "futures")]
use futures_core;

pub trait FileExtManual: Sized {
    fn replace_contents_async<'a, 'b, B: AsRef<[u8]> + Send + 'static, P: Into<Option<&'a str>>, Q: Into<Option<&'b Cancellable>>, R: FnOnce(Result<(B, String), (B, Error)>) + Send + 'static>(&self, contents: B, etag: P, make_backup: bool, flags: FileCreateFlags, cancellable: Q, callback: R);

    #[cfg(feature = "futures")]
    fn replace_contents_async_future<'a, B: AsRef<[u8]> + Send + 'static, P: Into<Option<&'a str>>>(&self, contents: B, etag: P, make_backup: bool, flags: FileCreateFlags) -> Box<futures_core::Future<Item = (Self, (B, String)), Error = (Self, (B, Error))>>;
}

impl<O: IsA<File> + IsA<glib::Object> + Clone + 'static> FileExtManual for O {
    fn replace_contents_async<'a, 'b, B: AsRef<[u8]> + Send + 'static, P: Into<Option<&'a str>>, Q: Into<Option<&'b Cancellable>>, R: FnOnce(Result<(B, String), (B, Error)>) + Send + 'static>(&self, contents: B, etag: P, make_backup: bool, flags: FileCreateFlags, cancellable: Q, callback: R) {
        let etag = etag.into();
        let etag = etag.to_glib_none();
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let contents: Box<B> = Box::new(contents);
        let (count, contents_ptr) = {
            let slice = (*contents).as_ref();
            (slice.len(), slice.as_ptr())
        };
        let user_data: Box<Option<(Box<R>, Box<B>)>> = Box::new(Some((Box::new(callback), contents)));
        unsafe extern "C" fn replace_contents_async_trampoline<B: AsRef<[u8]> + Send + 'static, R: FnOnce(Result<(B, String), (B, Error)>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut user_data: Box<Option<(Box<R>, Box<B>)>> = Box::from_raw(user_data as *mut _);
            let (callback, contents) = user_data.take().unwrap();
            let contents = *contents;

            let mut error = ptr::null_mut();
            let mut new_etag = ptr::null_mut();
            let _ = ffi::g_file_replace_contents_finish(_source_object as *mut _, res, &mut new_etag, &mut error);
            let result = if error.is_null() { Ok((contents, from_glib_full(new_etag))) } else { Err((contents, from_glib_full(error))) };
            callback(result);
        }
        let callback = replace_contents_async_trampoline::<B, R>;
        unsafe {
            ffi::g_file_replace_contents_async(self.to_glib_none().0, mut_override(contents_ptr), count, etag.0, make_backup.to_glib(), flags.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn replace_contents_async_future<'a, B: AsRef<[u8]> + Send + 'static, P: Into<Option<&'a str>>>(&self, contents: B, etag: P, make_backup: bool, flags: FileCreateFlags) -> Box<futures_core::Future<Item = (Self, (B, String)), Error = (Self, (B, Error))>> {
        use GioFuture;
        use send_cell::SendCell;

        let etag = etag.into();
        let etag = etag.map(String::from);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = SendCell::new(send);
            let obj_clone = SendCell::new(obj.clone());
            obj.replace_contents_async(
                 contents,
                 etag.as_ref().map(|s| s.as_str()),
                 make_backup,
                 flags,
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }
}


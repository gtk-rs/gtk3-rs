// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gio_sys;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use std::ptr;
use Cancellable;
use Error;
use File;
use FileCreateFlags;

#[cfg(feature = "futures")]
use futures_core;

pub trait FileExtManual: Sized {
    fn replace_contents_async<B: AsRef<[u8]> + Send + 'static, R: FnOnce(Result<(B, glib::GString), (B, Error)>) + Send + 'static>(&self, contents: B, etag: Option<&str>, make_backup: bool, flags: FileCreateFlags, cancellable: Option<&Cancellable>, callback: R);

    #[cfg(feature = "futures")]
    fn replace_contents_async_future<'a, B: AsRef<[u8]> + Send + 'static>(&self, contents: B, etag: Option<&str>, make_backup: bool, flags: FileCreateFlags) -> Box<futures_core::Future<Item = (Self, (B, glib::GString)), Error = (Self, (B, Error))>> where Self: Clone;
}

impl<O: IsA<File>> FileExtManual for O {
    fn replace_contents_async<B: AsRef<[u8]> + Send + 'static, R: FnOnce(Result<(B, glib::GString), (B, Error)>) + Send + 'static>(&self, contents: B, etag: Option<&str>, make_backup: bool, flags: FileCreateFlags, cancellable: Option<&Cancellable>, callback: R) {
        let etag = etag.to_glib_none();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Option<(R, B)>> = Box::new(Some((callback, contents)));
        // Need to do this after boxing as the contents pointer might change by moving into the box
        let (count, contents_ptr) = {
            let contents = &(*user_data).as_ref().unwrap().1;
            let slice = contents.as_ref();
            (slice.len(), slice.as_ptr())
        };
        unsafe extern "C" fn replace_contents_async_trampoline<B: AsRef<[u8]> + Send + 'static, R: FnOnce(Result<(B, glib::GString), (B, Error)>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer)
        {
            let mut user_data: Box<Option<(R, B)>> = Box::from_raw(user_data as *mut _);
            let (callback, contents) = user_data.take().unwrap();

            let mut error = ptr::null_mut();
            let mut new_etag = ptr::null_mut();
            let _ = gio_sys::g_file_replace_contents_finish(_source_object as *mut _, res, &mut new_etag, &mut error);
            let result = if error.is_null() { Ok((contents, from_glib_full(new_etag))) } else { Err((contents, from_glib_full(error))) };
            callback(result);
        }
        let callback = replace_contents_async_trampoline::<B, R>;
        unsafe {
            gio_sys::g_file_replace_contents_async(self.as_ref().to_glib_none().0, mut_override(contents_ptr), count, etag.0, make_backup.to_glib(), flags.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn replace_contents_async_future<B: AsRef<[u8]> + Send + 'static>(&self, contents: B, etag: Option<&str>, make_backup: bool, flags: FileCreateFlags) -> Box<futures_core::Future<Item = (Self, (B, glib::GString)), Error = (Self, (B, Error))>> where Self: Clone {
        use GioFuture;
        use fragile::Fragile;

        let etag = etag.map(glib::GString::from);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
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

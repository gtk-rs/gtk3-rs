// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Cancellable;
use Error;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;
use glib_ffi;
use gobject_ffi;
use SocketListener;
use Socket;

pub trait SocketListenerExtManual {
    fn accept_socket_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(Socket, Option<glib::Object>), Error>) + Send + 'static>(&self, cancellable: P, callback: Q);
}

impl<O: IsA<SocketListener>> SocketListenerExtManual for O {
    fn accept_socket_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(Socket, Option<glib::Object>), Error>) + Send + 'static>(&self, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn accept_socket_async_trampoline<Q: FnOnce(Result<(Socket, Option<glib::Object>), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let mut source_object = ptr::null_mut();
            let res = ffi::g_socket_listener_accept_socket_finish(_source_object as *mut _, res, &mut source_object, &mut error);
            let result = if error.is_null() { Ok((from_glib_full(res), from_glib_none(source_object))) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = accept_socket_async_trampoline::<Q>;
        unsafe {
            ffi::g_socket_listener_accept_socket_async(self.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }
}

// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Cancellable;
use Error;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;
use glib_ffi;
use gobject_ffi;
use SocketClient;
use SocketConnection;
use SocketConnectable;

pub trait SocketClientExtManual {
    fn connect_async<'a, P: IsA<SocketConnectable>, Q: Into<Option<&'a Cancellable>>, R: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, connectable: &P, cancellable: Q, callback: R);

    fn connect_to_host_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, host_and_port: &str, default_port: u16, cancellable: P, callback: Q);

    fn connect_to_service_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, domain: &str, service: &str, cancellable: P, callback: Q);

    fn connect_to_uri_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, uri: &str, default_port: u16, cancellable: P, callback: Q);
}

impl<O: IsA<SocketClient>> SocketClientExtManual for O {
    fn connect_async<'a, P: IsA<SocketConnectable>, Q: Into<Option<&'a Cancellable>>, R: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, connectable: &P, cancellable: Q, callback: R) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<R>> = Box::new(Box::new(callback));
        unsafe extern "C" fn connect_async_trampoline<R: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let res = ffi::g_socket_client_connect_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(res)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<R>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_async_trampoline::<R>;
        unsafe {
            ffi::g_socket_client_connect_async(self.to_glib_none().0, connectable.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn connect_to_host_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, host_and_port: &str, default_port: u16, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn connect_to_host_async_trampoline<Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let res = ffi::g_socket_client_connect_to_host_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(res)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_to_host_async_trampoline::<Q>;
        unsafe {
            ffi::g_socket_client_connect_to_host_async(self.to_glib_none().0, host_and_port.to_glib_none().0, default_port, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn connect_to_service_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, domain: &str, service: &str, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn connect_to_service_async_trampoline<Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let res = ffi::g_socket_client_connect_to_service_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(res)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_to_service_async_trampoline::<Q>;
        unsafe {
            ffi::g_socket_client_connect_to_service_async(self.to_glib_none().0, domain.to_glib_none().0, service.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn connect_to_uri_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, uri: &str, default_port: u16, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn connect_to_uri_async_trampoline<Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let res = ffi::g_socket_client_connect_to_uri_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(res)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_to_uri_async_trampoline::<Q>;
        unsafe {
            ffi::g_socket_client_connect_to_uri_async(self.to_glib_none().0, uri.to_glib_none().0, default_port, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }
}

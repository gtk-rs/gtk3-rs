// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gio_sys;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::pin::Pin;
use std::ptr;
use Cancellable;
use Socket;
use SocketConnection;
use SocketListener;

pub trait SocketListenerExtManual: Sized {
    fn accept_socket_async<
        Q: FnOnce(Result<(Socket, Option<glib::Object>), glib::Error>) + Send + 'static,
        C: IsA<Cancellable>,
    >(
        &self,
        cancellable: Option<&C>,
        callback: Q,
    );

    fn accept_socket_async_future(
        &self,
    ) -> Pin<
        Box<
            dyn std::future::Future<Output = Result<(Socket, Option<glib::Object>), glib::Error>>
                + 'static,
        >,
    >;

    fn accept_async<
        C: IsA<Cancellable>,
        Q: FnOnce(Result<(SocketConnection, Option<glib::Object>), glib::Error>) + Send + 'static,
    >(
        &self,
        cancellable: Option<&C>,
        callback: Q,
    );

    fn accept_async_future(
        &self,
    ) -> Pin<
        Box_<
            dyn std::future::Future<
                    Output = Result<(SocketConnection, Option<glib::Object>), glib::Error>,
                > + 'static,
        >,
    >;
}

impl<O: IsA<SocketListener>> SocketListenerExtManual for O {
    fn accept_socket_async<
        Q: FnOnce(Result<(Socket, Option<glib::Object>), glib::Error>) + Send + 'static,
        C: IsA<Cancellable>,
    >(
        &self,
        cancellable: Option<&C>,
        callback: Q,
    ) {
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn accept_socket_async_trampoline<
            Q: FnOnce(Result<(Socket, Option<glib::Object>), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut source_object = ptr::null_mut();
            let res = gio_sys::g_socket_listener_accept_socket_finish(
                _source_object as *mut _,
                res,
                &mut source_object,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((from_glib_full(res), from_glib_none(source_object)))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = accept_socket_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_socket_listener_accept_socket_async(
                self.as_ref().to_glib_none().0,
                gcancellable.0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    fn accept_socket_async_future(
        &self,
    ) -> Pin<
        Box<
            dyn std::future::Future<Output = Result<(Socket, Option<glib::Object>), glib::Error>>
                + 'static,
        >,
    > {
        Box::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.accept_socket_async(Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn accept_async<
        C: IsA<Cancellable>,
        Q: FnOnce(Result<(SocketConnection, Option<glib::Object>), glib::Error>) + Send + 'static,
    >(
        &self,
        cancellable: Option<&C>,
        callback: Q,
    ) {
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn accept_async_trampoline<
            Q: FnOnce(Result<(SocketConnection, Option<glib::Object>), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut source_object = ptr::null_mut();
            let ret = gio_sys::g_socket_listener_accept_finish(
                _source_object as *mut _,
                res,
                &mut source_object,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((from_glib_full(ret), from_glib_none(source_object)))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = accept_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_socket_listener_accept_async(
                self.as_ref().to_glib_none().0,
                gcancellable.0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    fn accept_async_future(
        &self,
    ) -> Pin<
        Box_<
            dyn std::future::Future<
                    Output = Result<(SocketConnection, Option<glib::Object>), glib::Error>,
                > + 'static,
        >,
    > {
        Box::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.accept_async(Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }
}

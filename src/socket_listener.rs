// Copyright 2013-2018, The Gtk-rs Project Developers.
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
use Socket;
use SocketListener;

#[cfg(feature = "futures")]
use futures::future;

pub trait SocketListenerExtManual: Sized {
    fn accept_socket_async<Q: FnOnce(Result<(Socket, Option<glib::Object>), Error>) + Send + 'static>(&self, cancellable: Option<&Cancellable>, callback: Q);

    #[cfg(feature = "futures")]
    fn accept_socket_async_future(
        &self,
    ) -> Box<dyn future::Future<Output = Result<(Socket, Option<glib::Object>), Error>> + std::marker::Unpin>;
}

impl<O: IsA<SocketListener>> SocketListenerExtManual for O {
    fn accept_socket_async<Q: FnOnce(Result<(Socket, Option<glib::Object>), Error>) + Send + 'static>(&self, cancellable: Option<&Cancellable>, callback: Q) {
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn accept_socket_async_trampoline<Q: FnOnce(Result<(Socket, Option<glib::Object>), Error>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer)
        {
            let mut error = ptr::null_mut();
            let mut source_object = ptr::null_mut();
            let res = gio_sys::g_socket_listener_accept_socket_finish(_source_object as *mut _, res, &mut source_object, &mut error);
            let result = if error.is_null() { Ok((from_glib_full(res), from_glib_none(source_object))) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = accept_socket_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_socket_listener_accept_socket_async(self.as_ref().to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn accept_socket_async_future(
        &self,
    ) -> Box<dyn future::Future<Output = Result<(Socket, Option<glib::Object>), Error>> + std::marker::Unpin> {
        use GioFuture;

        GioFuture::new(self, move |obj, send| {
            use fragile::Fragile;

            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.accept_socket_async(
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }
}

// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "futures", feature = "dox"))]
use futures::future;
use gio_sys;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::ptr;
use Cancellable;
use SocketAddress;

glib_wrapper! {
    pub struct SocketAddressEnumerator(Object<gio_sys::GSocketAddressEnumerator, gio_sys::GSocketAddressEnumeratorClass, SocketAddressEnumeratorClass>);

    match fn {
        get_type => || gio_sys::g_socket_address_enumerator_get_type(),
    }
}

pub const NONE_SOCKET_ADDRESS_ENUMERATOR: Option<&SocketAddressEnumerator> = None;

pub trait SocketAddressEnumeratorExt: 'static {
    fn next<P: IsA<Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<SocketAddress, glib::Error>;

    fn next_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<SocketAddress, glib::Error>) + Send + 'static,
    >(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    );

    #[cfg(any(feature = "futures", feature = "dox"))]
    fn next_async_future(
        &self,
    ) -> Box_<dyn future::Future<Output = Result<SocketAddress, glib::Error>> + std::marker::Unpin>;
}

impl<O: IsA<SocketAddressEnumerator>> SocketAddressEnumeratorExt for O {
    fn next<P: IsA<Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<SocketAddress, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_socket_address_enumerator_next(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn next_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<SocketAddress, glib::Error>) + Send + 'static,
    >(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn next_async_trampoline<
            Q: FnOnce(Result<SocketAddress, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_socket_address_enumerator_next_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = next_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_socket_address_enumerator_next_async(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(any(feature = "futures", feature = "dox"))]
    fn next_async_future(
        &self,
    ) -> Box_<dyn future::Future<Output = Result<SocketAddress, glib::Error>> + std::marker::Unpin>
    {
        use fragile::Fragile;
        use GioFuture;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.next_async(Some(&cancellable), move |res| {
                let _ = send.into_inner().send(res);
            });

            cancellable
        })
    }
}

impl fmt::Display for SocketAddressEnumerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SocketAddressEnumerator")
    }
}

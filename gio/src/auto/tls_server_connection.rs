// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::IOStream;
use crate::TlsAuthenticationMode;
use crate::TlsCertificate;
use crate::TlsConnection;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::glib_wrapper! {
    pub struct TlsServerConnection(Interface<ffi::GTlsServerConnection>) @requires TlsConnection, IOStream;

    match fn {
        get_type => || ffi::g_tls_server_connection_get_type(),
    }
}

impl TlsServerConnection {
    pub fn new<P: IsA<IOStream>, Q: IsA<TlsCertificate>>(
        base_io_stream: &P,
        certificate: Option<&Q>,
    ) -> Result<TlsServerConnection, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_server_connection_new(
                base_io_stream.as_ref().to_glib_none().0,
                certificate.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

pub const NONE_TLS_SERVER_CONNECTION: Option<&TlsServerConnection> = None;

pub trait TlsServerConnectionExt: 'static {
    fn get_property_authentication_mode(&self) -> TlsAuthenticationMode;

    fn set_property_authentication_mode(&self, authentication_mode: TlsAuthenticationMode);

    fn connect_property_authentication_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<TlsServerConnection>> TlsServerConnectionExt for O {
    fn get_property_authentication_mode(&self) -> TlsAuthenticationMode {
        unsafe {
            let mut value = Value::from_type(<TlsAuthenticationMode as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"authentication-mode\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `authentication-mode` getter")
                .unwrap()
        }
    }

    fn set_property_authentication_mode(&self, authentication_mode: TlsAuthenticationMode) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"authentication-mode\0".as_ptr() as *const _,
                Value::from(&authentication_mode).to_glib_none().0,
            );
        }
    }

    fn connect_property_authentication_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_authentication_mode_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GTlsServerConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TlsServerConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&TlsServerConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::authentication-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_authentication_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for TlsServerConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TlsServerConnection")
    }
}

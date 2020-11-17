// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::Credentials;
use crate::IOStream;
use glib;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct DBusAuthObserver(Object<ffi::GDBusAuthObserver>);

    match fn {
        get_type => || ffi::g_dbus_auth_observer_get_type(),
    }
}

impl DBusAuthObserver {
    pub fn new() -> DBusAuthObserver {
        unsafe { from_glib_full(ffi::g_dbus_auth_observer_new()) }
    }

    pub fn allow_mechanism(&self, mechanism: &str) -> bool {
        unsafe {
            from_glib(ffi::g_dbus_auth_observer_allow_mechanism(
                self.to_glib_none().0,
                mechanism.to_glib_none().0,
            ))
        }
    }

    pub fn authorize_authenticated_peer<P: IsA<IOStream>>(
        &self,
        stream: &P,
        credentials: Option<&Credentials>,
    ) -> bool {
        unsafe {
            from_glib(ffi::g_dbus_auth_observer_authorize_authenticated_peer(
                self.to_glib_none().0,
                stream.as_ref().to_glib_none().0,
                credentials.to_glib_none().0,
            ))
        }
    }

    pub fn connect_allow_mechanism<F: Fn(&DBusAuthObserver, &str) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn allow_mechanism_trampoline<
            F: Fn(&DBusAuthObserver, &str) -> bool + 'static,
        >(
            this: *mut ffi::GDBusAuthObserver,
            mechanism: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &glib::GString::from_glib_borrow(mechanism),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"allow-mechanism\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    allow_mechanism_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_authorize_authenticated_peer<
        F: Fn(&DBusAuthObserver, &IOStream, Option<&Credentials>) -> bool + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn authorize_authenticated_peer_trampoline<
            F: Fn(&DBusAuthObserver, &IOStream, Option<&Credentials>) -> bool + 'static,
        >(
            this: *mut ffi::GDBusAuthObserver,
            stream: *mut ffi::GIOStream,
            credentials: *mut ffi::GCredentials,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &from_glib_borrow(stream),
                Option::<Credentials>::from_glib_borrow(credentials)
                    .as_ref()
                    .as_ref(),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"authorize-authenticated-peer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    authorize_authenticated_peer_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for DBusAuthObserver {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DBusAuthObserver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DBusAuthObserver")
    }
}

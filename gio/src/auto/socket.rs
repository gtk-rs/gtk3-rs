// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::Cancellable;
use crate::Credentials;
use crate::InetAddress;
use crate::SocketAddress;
use crate::SocketConnection;
use crate::SocketFamily;
use crate::SocketProtocol;
use crate::SocketType;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::ObjectExt;
use glib::StaticType;
use glib::Value;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib::glib_wrapper! {
    pub struct Socket(Object<ffi::GSocket, ffi::GSocketClass>);

    match fn {
        get_type => || ffi::g_socket_get_type(),
    }
}

impl Socket {
    pub fn new(
        family: SocketFamily,
        type_: SocketType,
        protocol: SocketProtocol,
    ) -> Result<Socket, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_new(
                family.to_glib(),
                type_.to_glib(),
                protocol.to_glib(),
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

unsafe impl glib::SendUnique for Socket {
    fn is_unique(&self) -> bool {
        self.ref_count() == 1
    }
}

pub const NONE_SOCKET: Option<&Socket> = None;

pub trait SocketExt: 'static {
    fn accept<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<Socket, glib::Error>;

    fn bind<P: IsA<SocketAddress>>(
        &self,
        address: &P,
        allow_reuse: bool,
    ) -> Result<(), glib::Error>;

    fn check_connect_result(&self) -> Result<(), glib::Error>;

    fn close(&self) -> Result<(), glib::Error>;

    fn condition_check(&self, condition: glib::IOCondition) -> glib::IOCondition;

    fn condition_timed_wait<P: IsA<Cancellable>>(
        &self,
        condition: glib::IOCondition,
        timeout_us: i64,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error>;

    fn condition_wait<P: IsA<Cancellable>>(
        &self,
        condition: glib::IOCondition,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error>;

    fn connect<P: IsA<SocketAddress>, Q: IsA<Cancellable>>(
        &self,
        address: &P,
        cancellable: Option<&Q>,
    ) -> Result<(), glib::Error>;

    fn connection_factory_create_connection(&self) -> Option<SocketConnection>;

    fn get_available_bytes(&self) -> isize;

    fn get_blocking(&self) -> bool;

    fn get_broadcast(&self) -> bool;

    fn get_credentials(&self) -> Result<Credentials, glib::Error>;

    fn get_family(&self) -> SocketFamily;

    fn get_keepalive(&self) -> bool;

    fn get_listen_backlog(&self) -> i32;

    fn get_local_address(&self) -> Result<SocketAddress, glib::Error>;

    fn get_multicast_loopback(&self) -> bool;

    fn get_multicast_ttl(&self) -> u32;

    fn get_option(&self, level: i32, optname: i32) -> Result<i32, glib::Error>;

    fn get_protocol(&self) -> SocketProtocol;

    fn get_remote_address(&self) -> Result<SocketAddress, glib::Error>;

    fn get_socket_type(&self) -> SocketType;

    fn get_timeout(&self) -> u32;

    fn get_ttl(&self) -> u32;

    fn is_closed(&self) -> bool;

    fn is_connected(&self) -> bool;

    fn join_multicast_group<P: IsA<InetAddress>>(
        &self,
        group: &P,
        source_specific: bool,
        iface: Option<&str>,
    ) -> Result<(), glib::Error>;

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    fn join_multicast_group_ssm<P: IsA<InetAddress>, Q: IsA<InetAddress>>(
        &self,
        group: &P,
        source_specific: Option<&Q>,
        iface: Option<&str>,
    ) -> Result<(), glib::Error>;

    fn leave_multicast_group<P: IsA<InetAddress>>(
        &self,
        group: &P,
        source_specific: bool,
        iface: Option<&str>,
    ) -> Result<(), glib::Error>;

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    fn leave_multicast_group_ssm<P: IsA<InetAddress>, Q: IsA<InetAddress>>(
        &self,
        group: &P,
        source_specific: Option<&Q>,
        iface: Option<&str>,
    ) -> Result<(), glib::Error>;

    fn listen(&self) -> Result<(), glib::Error>;

    fn set_blocking(&self, blocking: bool);

    fn set_broadcast(&self, broadcast: bool);

    fn set_keepalive(&self, keepalive: bool);

    fn set_listen_backlog(&self, backlog: i32);

    fn set_multicast_loopback(&self, loopback: bool);

    fn set_multicast_ttl(&self, ttl: u32);

    fn set_option(&self, level: i32, optname: i32, value: i32) -> Result<(), glib::Error>;

    fn set_timeout(&self, timeout: u32);

    fn set_ttl(&self, ttl: u32);

    fn shutdown(&self, shutdown_read: bool, shutdown_write: bool) -> Result<(), glib::Error>;

    fn speaks_ipv4(&self) -> bool;

    fn get_property_type(&self) -> SocketType;

    fn connect_property_blocking_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_broadcast_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_keepalive_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_listen_backlog_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_local_address_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_multicast_loopback_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_multicast_ttl_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_remote_address_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_timeout_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_ttl_notify<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Socket>> SocketExt for O {
    fn accept<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<Socket, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_accept(
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

    fn bind<P: IsA<SocketAddress>>(
        &self,
        address: &P,
        allow_reuse: bool,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_bind(
                self.as_ref().to_glib_none().0,
                address.as_ref().to_glib_none().0,
                allow_reuse.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn check_connect_result(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_check_connect_result(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn close(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_close(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn condition_check(&self, condition: glib::IOCondition) -> glib::IOCondition {
        unsafe {
            from_glib(ffi::g_socket_condition_check(
                self.as_ref().to_glib_none().0,
                condition.to_glib(),
            ))
        }
    }

    fn condition_timed_wait<P: IsA<Cancellable>>(
        &self,
        condition: glib::IOCondition,
        timeout_us: i64,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_condition_timed_wait(
                self.as_ref().to_glib_none().0,
                condition.to_glib(),
                timeout_us,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn condition_wait<P: IsA<Cancellable>>(
        &self,
        condition: glib::IOCondition,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_condition_wait(
                self.as_ref().to_glib_none().0,
                condition.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect<P: IsA<SocketAddress>, Q: IsA<Cancellable>>(
        &self,
        address: &P,
        cancellable: Option<&Q>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_connect(
                self.as_ref().to_glib_none().0,
                address.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connection_factory_create_connection(&self) -> Option<SocketConnection> {
        unsafe {
            from_glib_full(ffi::g_socket_connection_factory_create_connection(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_available_bytes(&self) -> isize {
        unsafe { ffi::g_socket_get_available_bytes(self.as_ref().to_glib_none().0) }
    }

    fn get_blocking(&self) -> bool {
        unsafe { from_glib(ffi::g_socket_get_blocking(self.as_ref().to_glib_none().0)) }
    }

    fn get_broadcast(&self) -> bool {
        unsafe { from_glib(ffi::g_socket_get_broadcast(self.as_ref().to_glib_none().0)) }
    }

    fn get_credentials(&self) -> Result<Credentials, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_get_credentials(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_family(&self) -> SocketFamily {
        unsafe { from_glib(ffi::g_socket_get_family(self.as_ref().to_glib_none().0)) }
    }

    fn get_keepalive(&self) -> bool {
        unsafe { from_glib(ffi::g_socket_get_keepalive(self.as_ref().to_glib_none().0)) }
    }

    fn get_listen_backlog(&self) -> i32 {
        unsafe { ffi::g_socket_get_listen_backlog(self.as_ref().to_glib_none().0) }
    }

    fn get_local_address(&self) -> Result<SocketAddress, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_get_local_address(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_multicast_loopback(&self) -> bool {
        unsafe {
            from_glib(ffi::g_socket_get_multicast_loopback(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_multicast_ttl(&self) -> u32 {
        unsafe { ffi::g_socket_get_multicast_ttl(self.as_ref().to_glib_none().0) }
    }

    fn get_option(&self, level: i32, optname: i32) -> Result<i32, glib::Error> {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_get_option(
                self.as_ref().to_glib_none().0,
                level,
                optname,
                value.as_mut_ptr(),
                &mut error,
            );
            let value = value.assume_init();
            if error.is_null() {
                Ok(value)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_protocol(&self) -> SocketProtocol {
        unsafe { from_glib(ffi::g_socket_get_protocol(self.as_ref().to_glib_none().0)) }
    }

    fn get_remote_address(&self) -> Result<SocketAddress, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_get_remote_address(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_socket_type(&self) -> SocketType {
        unsafe {
            from_glib(ffi::g_socket_get_socket_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_timeout(&self) -> u32 {
        unsafe { ffi::g_socket_get_timeout(self.as_ref().to_glib_none().0) }
    }

    fn get_ttl(&self) -> u32 {
        unsafe { ffi::g_socket_get_ttl(self.as_ref().to_glib_none().0) }
    }

    fn is_closed(&self) -> bool {
        unsafe { from_glib(ffi::g_socket_is_closed(self.as_ref().to_glib_none().0)) }
    }

    fn is_connected(&self) -> bool {
        unsafe { from_glib(ffi::g_socket_is_connected(self.as_ref().to_glib_none().0)) }
    }

    fn join_multicast_group<P: IsA<InetAddress>>(
        &self,
        group: &P,
        source_specific: bool,
        iface: Option<&str>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_join_multicast_group(
                self.as_ref().to_glib_none().0,
                group.as_ref().to_glib_none().0,
                source_specific.to_glib(),
                iface.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    fn join_multicast_group_ssm<P: IsA<InetAddress>, Q: IsA<InetAddress>>(
        &self,
        group: &P,
        source_specific: Option<&Q>,
        iface: Option<&str>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_join_multicast_group_ssm(
                self.as_ref().to_glib_none().0,
                group.as_ref().to_glib_none().0,
                source_specific.map(|p| p.as_ref()).to_glib_none().0,
                iface.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn leave_multicast_group<P: IsA<InetAddress>>(
        &self,
        group: &P,
        source_specific: bool,
        iface: Option<&str>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_leave_multicast_group(
                self.as_ref().to_glib_none().0,
                group.as_ref().to_glib_none().0,
                source_specific.to_glib(),
                iface.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    fn leave_multicast_group_ssm<P: IsA<InetAddress>, Q: IsA<InetAddress>>(
        &self,
        group: &P,
        source_specific: Option<&Q>,
        iface: Option<&str>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_leave_multicast_group_ssm(
                self.as_ref().to_glib_none().0,
                group.as_ref().to_glib_none().0,
                source_specific.map(|p| p.as_ref()).to_glib_none().0,
                iface.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn listen(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_listen(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_blocking(&self, blocking: bool) {
        unsafe {
            ffi::g_socket_set_blocking(self.as_ref().to_glib_none().0, blocking.to_glib());
        }
    }

    fn set_broadcast(&self, broadcast: bool) {
        unsafe {
            ffi::g_socket_set_broadcast(self.as_ref().to_glib_none().0, broadcast.to_glib());
        }
    }

    fn set_keepalive(&self, keepalive: bool) {
        unsafe {
            ffi::g_socket_set_keepalive(self.as_ref().to_glib_none().0, keepalive.to_glib());
        }
    }

    fn set_listen_backlog(&self, backlog: i32) {
        unsafe {
            ffi::g_socket_set_listen_backlog(self.as_ref().to_glib_none().0, backlog);
        }
    }

    fn set_multicast_loopback(&self, loopback: bool) {
        unsafe {
            ffi::g_socket_set_multicast_loopback(
                self.as_ref().to_glib_none().0,
                loopback.to_glib(),
            );
        }
    }

    fn set_multicast_ttl(&self, ttl: u32) {
        unsafe {
            ffi::g_socket_set_multicast_ttl(self.as_ref().to_glib_none().0, ttl);
        }
    }

    fn set_option(&self, level: i32, optname: i32, value: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_set_option(
                self.as_ref().to_glib_none().0,
                level,
                optname,
                value,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_timeout(&self, timeout: u32) {
        unsafe {
            ffi::g_socket_set_timeout(self.as_ref().to_glib_none().0, timeout);
        }
    }

    fn set_ttl(&self, ttl: u32) {
        unsafe {
            ffi::g_socket_set_ttl(self.as_ref().to_glib_none().0, ttl);
        }
    }

    fn shutdown(&self, shutdown_read: bool, shutdown_write: bool) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_shutdown(
                self.as_ref().to_glib_none().0,
                shutdown_read.to_glib(),
                shutdown_write.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn speaks_ipv4(&self) -> bool {
        unsafe { from_glib(ffi::g_socket_speaks_ipv4(self.as_ref().to_glib_none().0)) }
    }

    fn get_property_type(&self) -> SocketType {
        unsafe {
            let mut value = Value::from_type(<SocketType as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `type` getter")
                .unwrap()
        }
    }

    fn connect_property_blocking_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_blocking_trampoline<P, F: Fn(&P) + Send + 'static>(
            this: *mut ffi::GSocket,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Socket>,
        {
            let f: &F = &*(f as *const F);
            f(&Socket::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::blocking\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_blocking_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_broadcast_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_broadcast_trampoline<P, F: Fn(&P) + Send + 'static>(
            this: *mut ffi::GSocket,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Socket>,
        {
            let f: &F = &*(f as *const F);
            f(&Socket::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::broadcast\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_broadcast_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_keepalive_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_keepalive_trampoline<P, F: Fn(&P) + Send + 'static>(
            this: *mut ffi::GSocket,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Socket>,
        {
            let f: &F = &*(f as *const F);
            f(&Socket::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::keepalive\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_keepalive_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_listen_backlog_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_listen_backlog_trampoline<P, F: Fn(&P) + Send + 'static>(
            this: *mut ffi::GSocket,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Socket>,
        {
            let f: &F = &*(f as *const F);
            f(&Socket::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::listen-backlog\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_listen_backlog_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_local_address_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_local_address_trampoline<P, F: Fn(&P) + Send + 'static>(
            this: *mut ffi::GSocket,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Socket>,
        {
            let f: &F = &*(f as *const F);
            f(&Socket::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::local-address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_local_address_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_multicast_loopback_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_multicast_loopback_trampoline<P, F: Fn(&P) + Send + 'static>(
            this: *mut ffi::GSocket,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Socket>,
        {
            let f: &F = &*(f as *const F);
            f(&Socket::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::multicast-loopback\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_multicast_loopback_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_multicast_ttl_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_multicast_ttl_trampoline<P, F: Fn(&P) + Send + 'static>(
            this: *mut ffi::GSocket,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Socket>,
        {
            let f: &F = &*(f as *const F);
            f(&Socket::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::multicast-ttl\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_multicast_ttl_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_remote_address_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_remote_address_trampoline<P, F: Fn(&P) + Send + 'static>(
            this: *mut ffi::GSocket,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Socket>,
        {
            let f: &F = &*(f as *const F);
            f(&Socket::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::remote-address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_remote_address_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_timeout_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeout_trampoline<P, F: Fn(&P) + Send + 'static>(
            this: *mut ffi::GSocket,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Socket>,
        {
            let f: &F = &*(f as *const F);
            f(&Socket::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_timeout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_ttl_notify<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ttl_trampoline<P, F: Fn(&P) + Send + 'static>(
            this: *mut ffi::GSocket,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Socket>,
        {
            let f: &F = &*(f as *const F);
            f(&Socket::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ttl\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ttl_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Socket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Socket")
    }
}

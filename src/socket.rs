// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Cancellable;
use Error;
use ffi;
use glib_ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;
use std::cell::RefCell;
use std::mem::transmute;
use Socket;
use SocketAddress;
#[cfg(all(not(unix), feature = "dox"))]
use std::os::raw::c_int;

#[cfg(unix)]
use std::os::unix::io::{IntoRawFd, FromRawFd};

#[cfg(windows)]
use std::os::windows::io::{IntoRawSocket, FromRawSocket};

impl Socket {
    #[cfg(any(unix, feature = "dox"))]
    pub fn new_from_fd<T: IntoRawFd>(fd: T) -> Result<Socket, Error> {
        let fd = fd.into_raw_fd();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_new_from_fd(fd, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
    #[cfg(any(windows, feature = "dox"))]
    pub fn new_from_socket<T: IntoRawSocket>(socket: T) -> Result<Socket, Error> {
        let socket = socket.into_raw_socket();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_new_from_fd(socket as i32, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

pub trait SocketExtManual {
    fn receive<'a, B: AsMut<[u8]>, P: Into<Option<&'a Cancellable>>>(&self, buffer: B, cancellable: P) -> Result<usize, Error>;
    fn receive_from<'a, B: AsMut<[u8]>, P: Into<Option<&'a Cancellable>>>(&self, buffer: B, cancellable: P) -> Result<(usize, SocketAddress), Error>;
    fn receive_with_blocking<'a, B: AsMut<[u8]>, P: Into<Option<&'a Cancellable>>>(&self, buffer: B, blocking: bool, cancellable: P) -> Result<usize, Error>;

    fn send<'a, B: AsRef<[u8]>, P: Into<Option<&'a Cancellable>>>(&self, buffer: B, cancellable: P) -> Result<usize, Error>;
    fn send_to<'a, 'b, B: AsRef<[u8]>, P: IsA<SocketAddress> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b Cancellable>>>(&self, address: Q, buffer: B, cancellable: R) -> Result<usize, Error>;
    fn send_with_blocking<'a, B: AsRef<[u8]>, P: Into<Option<&'a Cancellable>>>(&self, buffer: B, blocking: bool, cancellable: P) -> Result<usize, Error>;

    #[cfg(any(unix, feature = "dox"))]
    fn get_fd<T: FromRawFd>(&self) -> T;

    #[cfg(any(windows, feature = "dox"))]
    fn get_socket<T: FromRawSocket>(&self) -> T;

    fn create_source<'a, 'b, N: Into<Option<&'b str>>, P: Into<Option<&'a Cancellable>>, F>(&self, condition: glib::IOCondition, cancellable: P, name: N, priority: glib::Priority, func: F) -> glib::Source
    where F: FnMut(&Socket, glib::IOCondition) -> glib::Continue + Send + 'static;
}

impl<O: IsA<Socket>> SocketExtManual for O {
    fn receive<'a, B: AsMut<[u8]>, P: Into<Option<&'a Cancellable>>>(&self, mut buffer: B, cancellable: P) -> Result<usize, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let buffer = buffer.as_mut();
        let buffer_ptr = buffer.as_mut_ptr();
        let count = buffer.len();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_receive(self.to_glib_none().0, buffer_ptr, count, cancellable.0, &mut error);
            if error.is_null() {
                Ok(ret as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn receive_from<'a, B: AsMut<[u8]>, P: Into<Option<&'a Cancellable>>>(&self, mut buffer: B, cancellable: P) -> Result<(usize, SocketAddress), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let buffer = buffer.as_mut();
        let buffer_ptr = buffer.as_mut_ptr();
        let count = buffer.len();
        unsafe {
            let mut error = ptr::null_mut();
            let mut addr_ptr = ptr::null_mut();

            let ret = ffi::g_socket_receive_from(self.to_glib_none().0, &mut addr_ptr, buffer_ptr, count, cancellable.0, &mut error);
            if error.is_null() {
                Ok((ret as usize, from_glib_full(addr_ptr)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn receive_with_blocking<'a, B: AsMut<[u8]>, P: Into<Option<&'a Cancellable>>>(&self, mut buffer: B, blocking: bool, cancellable: P) -> Result<usize, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let buffer = buffer.as_mut();
        let buffer_ptr = buffer.as_mut_ptr();
        let count = buffer.len();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_receive_with_blocking(self.to_glib_none().0, buffer_ptr, count, blocking.to_glib(), cancellable.0, &mut error);
            if error.is_null() {
                Ok(ret as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn send<'a, B: AsRef<[u8]>, P: Into<Option<&'a Cancellable>>>(&self, buffer: B, cancellable: P) -> Result<usize, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let (count, buffer_ptr) = {
            let slice = buffer.as_ref();
            (slice.len(), slice.as_ptr())
        };
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_send(self.to_glib_none().0, mut_override(buffer_ptr), count, cancellable.0, &mut error);
            if error.is_null() {
                Ok(ret as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn send_to<'a, 'b, B: AsRef<[u8]>, P: IsA<SocketAddress> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b Cancellable>>>(&self, address: Q, buffer: B, cancellable: R) -> Result<usize, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let (count, buffer_ptr) = {
            let slice = buffer.as_ref();
            (slice.len(), slice.as_ptr())
        };
        let address = address.into();
        unsafe {
            let mut error = ptr::null_mut();

            let ret = ffi::g_socket_send_to(self.to_glib_none().0, address.to_glib_none().0, mut_override(buffer_ptr), count, cancellable.0, &mut error);
            if error.is_null() {
                Ok(ret as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn send_with_blocking<'a, B: AsRef<[u8]>, P: Into<Option<&'a Cancellable>>>(&self, buffer: B, blocking: bool, cancellable: P) -> Result<usize, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let (count, buffer_ptr) = {
            let slice = buffer.as_ref();
            (slice.len(), slice.as_ptr())
        };
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_send_with_blocking(self.to_glib_none().0, mut_override(buffer_ptr), count, blocking.to_glib(), cancellable.0, &mut error);
            if error.is_null() {
                Ok(ret as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    fn get_fd<T: FromRawFd>(&self) -> T {
        unsafe {
            FromRawFd::from_raw_fd(ffi::g_socket_get_fd(self.to_glib_none().0))
        }
    }

    #[cfg(any(windows, feature = "dox"))]
    fn get_socket<T: FromRawSocket>(&self) -> T {
        unsafe {
            FromRawSocket::from_raw_socket(ffi::g_socket_get_fd(self.to_glib_none().0) as _)
        }
    }

    fn create_source<'a, 'b, N: Into<Option<&'b str>>, P: Into<Option<&'a Cancellable>>, F>(&self, condition: glib::IOCondition, cancellable: P, name: N, priority: glib::Priority, func: F) -> glib::Source
    where F: FnMut(&Socket, glib::IOCondition) -> glib::Continue + Send + 'static {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let source = ffi::g_socket_create_source(self.to_glib_none().0, condition.to_glib(), cancellable.0);
            let trampoline = trampoline as glib_ffi::gpointer;
            glib_ffi::g_source_set_callback(source, Some(transmute(trampoline)), into_raw(func), Some(destroy_closure));
            glib_ffi::g_source_set_priority(source, priority.to_glib());

            let name = name.into();
            if let Some(name) = name {
                glib_ffi::g_source_set_name(source, name.to_glib_none().0);
            }

            from_glib_full(source)
        }
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
unsafe extern "C" fn trampoline(socket: *mut ffi::GSocket, condition: glib_ffi::GIOCondition, func: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let func: &RefCell<Box<FnMut(&Socket, glib::IOCondition) -> glib::Continue + 'static>> = transmute(func);
    (&mut *func.borrow_mut())(&from_glib_borrow(socket), from_glib(condition)).to_glib()
}

unsafe extern "C" fn destroy_closure(ptr: glib_ffi::gpointer) {
    callback_guard!();
    Box::<RefCell<Box<FnMut(&Socket, glib::IOCondition) -> glib::Continue + 'static>>>::from_raw(ptr as *mut _);
}

fn into_raw<F: FnMut(&Socket, glib::IOCondition) -> glib::Continue + Send + 'static>(func: F) -> glib_ffi::gpointer {
    let func: Box<RefCell<Box<FnMut(&Socket, glib::IOCondition) -> glib::Continue + Send + 'static>>> =
        Box::new(RefCell::new(Box::new(func)));
    Box::into_raw(func) as glib_ffi::gpointer
}

#[cfg(all(not(unix), feature = "dox"))]
pub trait IntoRawFd {
    fn into_raw_fd(self) -> c_int;
}

#[cfg(all(not(unix), feature = "dox"))]
pub trait FromRawFd {
    unsafe fn from_raw_fd(fd: c_int) -> Self;
}

#[cfg(all(not(windows), feature = "dox"))]
pub trait IntoRawSocket {
    fn into_raw_socket(self) -> u64;
}

#[cfg(all(not(windows), feature = "dox"))]
pub trait FromRawSocket {
    unsafe fn from_raw_socket(sock: u64) -> Self;
}

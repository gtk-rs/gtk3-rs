// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Cancellable;
use Error;
use ffi;
use glib_ffi;
use glib;
use glib::object::{IsA, Downcast};
use glib::translate::*;
use std::ptr;
use std::cell::RefCell;
use std::mem::transmute;
use Socket;
use SocketAddress;
#[cfg(all(not(unix), feature = "dox"))]
use std::os::raw::c_int;
use send_cell::SendCell;

#[cfg(feature = "futures")]
use futures_core::{Future, Never};
#[cfg(feature = "futures")]
use futures_core::stream::Stream;

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

pub trait SocketExtManual: Sized {
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
    where F: FnMut(&Self, glib::IOCondition) -> glib::Continue + 'static;

    #[cfg(feature = "futures")]
    fn create_source_future<'a, P: Into<Option<&'a Cancellable>>>(&self, condition: glib::IOCondition, cancellable: P, priority: glib::Priority) -> Box<Future<Item = (Self, glib::IOCondition), Error = Never>>;

    #[cfg(feature = "futures")]
    fn create_source_stream<'a, P: Into<Option<&'a Cancellable>>>(&self, condition: glib::IOCondition, cancellable: P, priority: glib::Priority) -> Box<Stream<Item = (Self, glib::IOCondition), Error = Never>>;
}

impl<O: IsA<Socket> + Clone + 'static> SocketExtManual for O {
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
    where F: FnMut(&Self, glib::IOCondition) -> glib::Continue + 'static {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let source = ffi::g_socket_create_source(self.to_glib_none().0, condition.to_glib(), cancellable.0);
            let trampoline = trampoline::<O> as glib_ffi::gpointer;
            glib_ffi::g_source_set_callback(source, Some(transmute(trampoline)), into_raw(func), Some(destroy_closure::<O>));
            glib_ffi::g_source_set_priority(source, priority.to_glib());

            let name = name.into();
            if let Some(name) = name {
                glib_ffi::g_source_set_name(source, name.to_glib_none().0);
            }

            from_glib_full(source)
        }
    }

    #[cfg(feature = "futures")]
    fn create_source_future<'a, P: Into<Option<&'a Cancellable>>>(&self, condition: glib::IOCondition, cancellable: P, priority: glib::Priority) -> Box<Future<Item = (Self, glib::IOCondition), Error = Never>> {
        use send_cell::SendCell;

        let cancellable = cancellable.into();
        let cancellable: Option<Cancellable> = cancellable.cloned();

        let obj = SendCell::new(self.clone());
        Box::new(glib::SourceFuture::new(move |send| {
            let mut send = Some(SendCell::new(send));
            obj.borrow().create_source(condition, cancellable.as_ref(), None, priority, move |obj, condition| {
                let _ = send.take().unwrap().into_inner().send((obj.clone(), condition));
                glib::Continue(false)
            })
        }))
    }

    #[cfg(feature = "futures")]
    fn create_source_stream<'a, P: Into<Option<&'a Cancellable>>>(&self, condition: glib::IOCondition, cancellable: P, priority: glib::Priority) -> Box<Stream<Item = (Self, glib::IOCondition), Error = Never>> {
        use send_cell::SendCell;

        let cancellable = cancellable.into();
        let cancellable: Option<Cancellable> = cancellable.cloned();

        let obj = SendCell::new(self.clone());
        Box::new(glib::SourceStream::new(move |send| {
            let send = Some(SendCell::new(send));
            obj.borrow().create_source(condition, cancellable.as_ref(), None, priority, move |obj, condition| {
                if send.as_ref().unwrap().borrow().unbounded_send((obj.clone(), condition)).is_err() {
                    glib::Continue(false)
                } else {
                    glib::Continue(true)
                }
            })
        }))
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
unsafe extern "C" fn trampoline<O: IsA<Socket>>(socket: *mut ffi::GSocket, condition: glib_ffi::GIOCondition, func: glib_ffi::gpointer) -> glib_ffi::gboolean {
    let func: &SendCell<RefCell<Box<FnMut(&O, glib::IOCondition) -> glib::Continue + 'static>>> = transmute(func);
    let func = func.borrow();
    let mut func = func.borrow_mut();
    (&mut *func)(&Socket::from_glib_borrow(socket).downcast_unchecked(), from_glib(condition)).to_glib()
}

unsafe extern "C" fn destroy_closure<O>(ptr: glib_ffi::gpointer) {
    Box::<SendCell<RefCell<Box<FnMut(&O, glib::IOCondition) -> glib::Continue + 'static>>>>::from_raw(ptr as *mut _);
}

fn into_raw<O, F: FnMut(&O, glib::IOCondition) -> glib::Continue + 'static>(func: F) -> glib_ffi::gpointer {
    let func: Box<SendCell<RefCell<Box<FnMut(&O, glib::IOCondition) -> glib::Continue + 'static>>>> =
        Box::new(SendCell::new(RefCell::new(Box::new(func))));
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

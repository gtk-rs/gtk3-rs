// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gio_sys;
use glib;
use glib::object::{Cast, IsA};
use glib::translate::*;
use glib_sys;
use std::cell::RefCell;
use std::mem::transmute;
#[cfg(all(not(unix), feature = "dox"))]
use std::os::raw::c_int;
#[cfg(all(not(windows), feature = "dox"))]
use std::os::raw::c_void;
use std::pin::Pin;
use std::ptr;
use Cancellable;
use Socket;
use SocketAddress;

use futures_core::stream::Stream;

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

#[cfg(windows)]
use std::os::windows::io::{AsRawSocket, FromRawSocket, IntoRawSocket, RawSocket};

impl Socket {
    #[cfg(any(unix, feature = "dox"))]
    pub unsafe fn new_from_fd<T: IntoRawFd>(fd: T) -> Result<Socket, glib::Error> {
        let fd = fd.into_raw_fd();
        let mut error = ptr::null_mut();
        let ret = gio_sys::g_socket_new_from_fd(fd, &mut error);
        if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
    #[cfg(any(windows, feature = "dox"))]
    pub unsafe fn new_from_socket<T: IntoRawSocket>(socket: T) -> Result<Socket, glib::Error> {
        let socket = socket.into_raw_socket();
        let mut error = ptr::null_mut();
        let ret = gio_sys::g_socket_new_from_fd(socket as i32, &mut error);
        if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[cfg(any(unix, feature = "dox"))]
impl AsRawFd for Socket {
    fn as_raw_fd(&self) -> RawFd {
        unsafe { gio_sys::g_socket_get_fd(self.to_glib_none().0) as _ }
    }
}

#[cfg(any(windows, feature = "dox"))]
impl AsRawSocket for Socket {
    fn as_raw_socket(&self) -> RawSocket {
        unsafe { gio_sys::g_socket_get_fd(self.to_glib_none().0) as _ }
    }
}

pub trait SocketExtManual: Sized {
    fn receive<B: AsMut<[u8]>, C: IsA<Cancellable>>(
        &self,
        buffer: B,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error>;
    fn receive_from<B: AsMut<[u8]>, C: IsA<Cancellable>>(
        &self,
        buffer: B,
        cancellable: Option<&C>,
    ) -> Result<(usize, SocketAddress), glib::Error>;
    fn receive_with_blocking<B: AsMut<[u8]>, C: IsA<Cancellable>>(
        &self,
        buffer: B,
        blocking: bool,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error>;

    fn send<B: AsRef<[u8]>, C: IsA<Cancellable>>(
        &self,
        buffer: B,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error>;
    fn send_to<B: AsRef<[u8]>, P: IsA<SocketAddress>, C: IsA<Cancellable>>(
        &self,
        address: Option<&P>,
        buffer: B,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error>;
    fn send_with_blocking<B: AsRef<[u8]>, C: IsA<Cancellable>>(
        &self,
        buffer: B,
        blocking: bool,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error>;

    #[cfg(any(unix, feature = "dox"))]
    fn get_fd<T: FromRawFd>(&self) -> T;

    #[cfg(any(windows, feature = "dox"))]
    fn get_socket<T: FromRawSocket>(&self) -> T;

    fn create_source<F, C>(
        &self,
        condition: glib::IOCondition,
        cancellable: Option<&C>,
        name: Option<&str>,
        priority: glib::Priority,
        func: F,
    ) -> glib::Source
    where
        F: FnMut(&Self, glib::IOCondition) -> glib::Continue + 'static,
        C: IsA<Cancellable>;

    fn create_source_future<C: IsA<Cancellable>>(
        &self,
        condition: glib::IOCondition,
        cancellable: Option<&C>,
        priority: glib::Priority,
    ) -> Pin<Box<dyn std::future::Future<Output = glib::IOCondition> + 'static>>;

    fn create_source_stream<C: IsA<Cancellable>>(
        &self,
        condition: glib::IOCondition,
        cancellable: Option<&C>,
        priority: glib::Priority,
    ) -> Pin<Box<dyn Stream<Item = glib::IOCondition> + 'static>>;
}

impl<O: IsA<Socket>> SocketExtManual for O {
    fn receive<B: AsMut<[u8]>, C: IsA<Cancellable>>(
        &self,
        mut buffer: B,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let buffer = buffer.as_mut();
        let buffer_ptr = buffer.as_mut_ptr();
        let count = buffer.len();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_socket_receive(
                self.as_ref().to_glib_none().0,
                buffer_ptr,
                count,
                gcancellable.0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn receive_from<B: AsMut<[u8]>, C: IsA<Cancellable>>(
        &self,
        mut buffer: B,
        cancellable: Option<&C>,
    ) -> Result<(usize, SocketAddress), glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let buffer = buffer.as_mut();
        let buffer_ptr = buffer.as_mut_ptr();
        let count = buffer.len();
        unsafe {
            let mut error = ptr::null_mut();
            let mut addr_ptr = ptr::null_mut();

            let ret = gio_sys::g_socket_receive_from(
                self.as_ref().to_glib_none().0,
                &mut addr_ptr,
                buffer_ptr,
                count,
                gcancellable.0,
                &mut error,
            );
            if error.is_null() {
                Ok((ret as usize, from_glib_full(addr_ptr)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn receive_with_blocking<B: AsMut<[u8]>, C: IsA<Cancellable>>(
        &self,
        mut buffer: B,
        blocking: bool,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let buffer = buffer.as_mut();
        let buffer_ptr = buffer.as_mut_ptr();
        let count = buffer.len();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_socket_receive_with_blocking(
                self.as_ref().to_glib_none().0,
                buffer_ptr,
                count,
                blocking.to_glib(),
                gcancellable.0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn send<B: AsRef<[u8]>, C: IsA<Cancellable>>(
        &self,
        buffer: B,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let (count, buffer_ptr) = {
            let slice = buffer.as_ref();
            (slice.len(), slice.as_ptr())
        };
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_socket_send(
                self.as_ref().to_glib_none().0,
                mut_override(buffer_ptr),
                count,
                gcancellable.0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn send_to<B: AsRef<[u8]>, P: IsA<SocketAddress>, C: IsA<Cancellable>>(
        &self,
        address: Option<&P>,
        buffer: B,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let (count, buffer_ptr) = {
            let slice = buffer.as_ref();
            (slice.len(), slice.as_ptr())
        };
        unsafe {
            let mut error = ptr::null_mut();

            let ret = gio_sys::g_socket_send_to(
                self.as_ref().to_glib_none().0,
                address.map(|p| p.as_ref()).to_glib_none().0,
                mut_override(buffer_ptr),
                count,
                gcancellable.0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn send_with_blocking<B: AsRef<[u8]>, C: IsA<Cancellable>>(
        &self,
        buffer: B,
        blocking: bool,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let (count, buffer_ptr) = {
            let slice = buffer.as_ref();
            (slice.len(), slice.as_ptr())
        };
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_socket_send_with_blocking(
                self.as_ref().to_glib_none().0,
                mut_override(buffer_ptr),
                count,
                blocking.to_glib(),
                gcancellable.0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    fn get_fd<T: FromRawFd>(&self) -> T {
        unsafe { FromRawFd::from_raw_fd(gio_sys::g_socket_get_fd(self.as_ref().to_glib_none().0)) }
    }

    #[cfg(any(windows, feature = "dox"))]
    fn get_socket<T: FromRawSocket>(&self) -> T {
        unsafe {
            FromRawSocket::from_raw_socket(
                gio_sys::g_socket_get_fd(self.as_ref().to_glib_none().0) as _
            )
        }
    }

    fn create_source<F, C>(
        &self,
        condition: glib::IOCondition,
        cancellable: Option<&C>,
        name: Option<&str>,
        priority: glib::Priority,
        func: F,
    ) -> glib::Source
    where
        F: FnMut(&Self, glib::IOCondition) -> glib::Continue + 'static,
        C: IsA<Cancellable>,
    {
        unsafe extern "C" fn trampoline<
            O: IsA<Socket>,
            F: FnMut(&O, glib::IOCondition) -> glib::Continue + 'static,
        >(
            socket: *mut gio_sys::GSocket,
            condition: glib_sys::GIOCondition,
            func: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let func: &RefCell<F> = &*(func as *const RefCell<F>);
            let mut func = func.borrow_mut();
            (&mut *func)(
                &Socket::from_glib_borrow(socket).unsafe_cast(),
                from_glib(condition),
            )
            .to_glib()
        }
        unsafe extern "C" fn destroy_closure<O, F>(ptr: glib_sys::gpointer) {
            Box::<RefCell<F>>::from_raw(ptr as *mut _);
        }
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        unsafe {
            let source = gio_sys::g_socket_create_source(
                self.as_ref().to_glib_none().0,
                condition.to_glib(),
                gcancellable.0,
            );
            let trampoline = trampoline::<O, F> as glib_sys::gpointer;
            glib_sys::g_source_set_callback(
                source,
                Some(transmute(trampoline)),
                Box::into_raw(Box::new(RefCell::new(func))) as glib_sys::gpointer,
                Some(destroy_closure::<O, F>),
            );
            glib_sys::g_source_set_priority(source, priority.to_glib());

            if let Some(name) = name {
                glib_sys::g_source_set_name(source, name.to_glib_none().0);
            }

            from_glib_full(source)
        }
    }

    fn create_source_future<C: IsA<Cancellable>>(
        &self,
        condition: glib::IOCondition,
        cancellable: Option<&C>,
        priority: glib::Priority,
    ) -> Pin<Box<dyn std::future::Future<Output = glib::IOCondition> + 'static>> {
        let cancellable: Option<Cancellable> = cancellable.map(|c| c.as_ref()).cloned();

        let obj = self.clone();
        Box::pin(glib::SourceFuture::new(move |send| {
            let mut send = Some(send);
            obj.create_source(
                condition,
                cancellable.as_ref(),
                None,
                priority,
                move |_, condition| {
                    let _ = send.take().unwrap().send(condition);
                    glib::Continue(false)
                },
            )
        }))
    }

    fn create_source_stream<C: IsA<Cancellable>>(
        &self,
        condition: glib::IOCondition,
        cancellable: Option<&C>,
        priority: glib::Priority,
    ) -> Pin<Box<dyn Stream<Item = glib::IOCondition> + 'static>> {
        let cancellable: Option<Cancellable> = cancellable.map(|c| c.as_ref()).cloned();

        let obj = self.clone();
        Box::pin(glib::SourceStream::new(move |send| {
            let send = Some(send);
            obj.create_source(
                condition,
                cancellable.as_ref(),
                None,
                priority,
                move |_, condition| {
                    if send.as_ref().unwrap().unbounded_send(condition).is_err() {
                        glib::Continue(false)
                    } else {
                        glib::Continue(true)
                    }
                },
            )
        }))
    }
}

#[cfg(all(not(unix), feature = "dox"))]
pub trait IntoRawFd {
    fn into_raw_fd(self) -> c_int;
}

#[cfg(all(not(unix), feature = "dox"))]
pub trait FromRawFd {
    unsafe fn from_raw_fd(fd: c_int) -> Self;
}

#[cfg(all(not(unix), feature = "dox"))]
pub trait AsRawFd {
    fn as_raw_fd(&self) -> RawFd;
}

#[cfg(all(not(unix), feature = "dox"))]
pub type RawFd = c_int;

#[cfg(all(not(windows), feature = "dox"))]
pub trait IntoRawSocket {
    fn into_raw_socket(self) -> u64;
}

#[cfg(all(not(windows), feature = "dox"))]
pub trait FromRawSocket {
    unsafe fn from_raw_socket(sock: u64) -> Self;
}

#[cfg(all(not(windows), feature = "dox"))]
pub trait AsRawSocket {
    fn as_raw_socket(&self) -> RawSocket;
}

#[cfg(all(not(windows), feature = "dox"))]
pub type RawSocket = *mut c_void;

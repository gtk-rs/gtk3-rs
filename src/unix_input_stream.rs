// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::object::{Cast, IsA};
use glib::translate::*;
use InputStream;
use UnixInputStream;

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

#[cfg(all(not(unix), feature = "dox"))]
use socket::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

impl UnixInputStream {
    pub unsafe fn new<T: IntoRawFd>(fd: T) -> UnixInputStream {
        let fd = fd.into_raw_fd();
        let close_fd = true.to_glib();
        InputStream::from_glib_full(gio_sys::g_unix_input_stream_new(fd, close_fd)).unsafe_cast()
    }
}

impl AsRawFd for UnixInputStream {
    fn as_raw_fd(&self) -> RawFd {
        unsafe { gio_sys::g_unix_input_stream_get_fd(self.to_glib_none().0) as _ }
    }
}

pub trait UnixInputStreamExtManual: Sized {
    fn get_fd<T: FromRawFd>(&self) -> T;
    unsafe fn set_close_fd(&self, close_fd: bool);
}

impl<O: IsA<UnixInputStream>> UnixInputStreamExtManual for O {
    fn get_fd<T: FromRawFd>(&self) -> T {
        unsafe {
            T::from_raw_fd(gio_sys::g_unix_input_stream_get_fd(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    unsafe fn set_close_fd(&self, close_fd: bool) {
        gio_sys::g_unix_input_stream_set_close_fd(
            self.as_ref().to_glib_none().0,
            close_fd.to_glib(),
        );
    }
}

// Take a look at the license at the top of the repository in the LICENSE file.

use crate::InputStream;
use crate::UnixInputStream;
use glib::object::{Cast, IsA};
use glib::translate::*;

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

#[cfg(all(not(unix), feature = "dox"))]
use socket::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

impl UnixInputStream {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new<T: IntoRawFd>(fd: T) -> UnixInputStream {
        let fd = fd.into_raw_fd();
        let close_fd = true.to_glib();
        InputStream::from_glib_full(ffi::g_unix_input_stream_new(fd, close_fd)).unsafe_cast()
    }
}

impl AsRawFd for UnixInputStream {
    fn as_raw_fd(&self) -> RawFd {
        unsafe { ffi::g_unix_input_stream_get_fd(self.to_glib_none().0) as _ }
    }
}

pub trait UnixInputStreamExtManual: Sized {
    fn get_fd<T: FromRawFd>(&self) -> T;
    #[allow(clippy::missing_safety_doc)]
    unsafe fn set_close_fd(&self, close_fd: bool);
}

impl<O: IsA<UnixInputStream>> UnixInputStreamExtManual for O {
    fn get_fd<T: FromRawFd>(&self) -> T {
        unsafe {
            T::from_raw_fd(ffi::g_unix_input_stream_get_fd(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    unsafe fn set_close_fd(&self, close_fd: bool) {
        ffi::g_unix_input_stream_set_close_fd(self.as_ref().to_glib_none().0, close_fd.to_glib());
    }
}

// Take a look at the license at the top of the repository in the LICENSE file.

use crate::OutputStream;
use crate::UnixOutputStream;
use glib::object::{Cast, IsA};
use glib::translate::*;

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

#[cfg(all(not(unix), feature = "dox"))]
use socket::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

impl UnixOutputStream {
    #[doc(alias = "g_unix_output_stream_new")]
    pub unsafe fn new<T: IntoRawFd>(fd: T) -> UnixOutputStream {
        let fd = fd.into_raw_fd();
        let close_fd = true.to_glib();
        OutputStream::from_glib_full(ffi::g_unix_output_stream_new(fd, close_fd)).unsafe_cast()
    }
}

impl AsRawFd for UnixOutputStream {
    fn as_raw_fd(&self) -> RawFd {
        unsafe { ffi::g_unix_output_stream_get_fd(self.to_glib_none().0) as _ }
    }
}

pub trait UnixOutputStreamExtManual: Sized {
    #[doc(alias = "g_unix_output_stream_get_fd")]
    fn get_fd<T: FromRawFd>(&self) -> T;

    #[doc(alias = "g_unix_output_stream_set_close_fd")]
    unsafe fn set_close_fd(&self, close_fd: bool);
}

impl<O: IsA<UnixOutputStream>> UnixOutputStreamExtManual for O {
    fn get_fd<T: FromRawFd>(&self) -> T {
        unsafe {
            T::from_raw_fd(ffi::g_unix_output_stream_get_fd(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    unsafe fn set_close_fd(&self, close_fd: bool) {
        ffi::g_unix_output_stream_set_close_fd(self.as_ref().to_glib_none().0, close_fd.to_glib());
    }
}

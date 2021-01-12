// Take a look at the license at the top of the repository in the LICENSE file.

use crate::UnixFDList;
use glib::object::IsA;
use glib::translate::*;
use std::{mem, ptr};

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, IntoRawFd, RawFd};

#[cfg(all(not(unix), feature = "dox"))]
use socket::{AsRawFd, IntoRawFd, RawFd};

impl UnixFDList {
    #[doc(alias = "g_unix_fd_list_new_from_array")]
    pub fn from_array<T>(fds: T) -> UnixFDList
    where
        T: IntoIterator,
        T::Item: IntoRawFd,
    {
        let fds = fds.into_iter().map(|t| t.into_raw_fd()).collect::<Vec<_>>();
        unsafe {
            from_glib_full(ffi::g_unix_fd_list_new_from_array(
                fds.to_glib_none().0,
                fds.len() as i32,
            ))
        }
    }
}

pub trait UnixFDListExtManual: Sized {
    #[doc(alias = "g_unix_fd_list_append")]
    fn append<T: AsRawFd>(&self, fd: T) -> Result<i32, glib::Error>;

    #[doc(alias = "g_unix_fd_list_get")]
    fn get(&self, index_: i32) -> Result<RawFd, glib::Error>;

    #[doc(alias = "g_unix_fd_list_peek_fds")]
    fn peek_fds(&self) -> Vec<RawFd>;

    #[doc(alias = "g_unix_fd_list_steal_fds")]
    fn steal_fds(&self) -> Vec<RawFd>;
}

impl<O: IsA<UnixFDList>> UnixFDListExtManual for O {
    fn append<T: AsRawFd>(&self, fd: T) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_unix_fd_list_append(
                self.as_ref().to_glib_none().0,
                fd.as_raw_fd(),
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get(&self, index_: i32) -> Result<RawFd, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_unix_fd_list_get(self.as_ref().to_glib_none().0, index_, &mut error);
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn peek_fds(&self) -> Vec<RawFd> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                ffi::g_unix_fd_list_peek_fds(self.as_ref().to_glib_none().0, length.as_mut_ptr()),
                length.assume_init() as usize,
            );
            ret
        }
    }

    fn steal_fds(&self) -> Vec<RawFd> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(
                ffi::g_unix_fd_list_steal_fds(self.as_ref().to_glib_none().0, length.as_mut_ptr()),
                length.assume_init() as usize,
            );
            ret
        }
    }
}

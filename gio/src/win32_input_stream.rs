// Take a look at the license at the top of the repository in the LICENSE file.

use crate::InputStream;
use glib::object::{Cast, IsA};
use glib::translate::*;
use std::fmt;

use std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle};

glib::wrapper! {
    pub struct Win32InputStream(Object<ffi::GWin32InputStream, ffi::GWin32InputStreamClass>) @extends InputStream;

    match fn {
        get_type => || ffi::g_win32_input_stream_get_type(),
    }
}

pub const NONE_WIN32_INPUT_STREAM: Option<&Win32InputStream> = None;

pub trait Win32InputStreamExt: 'static {
    #[doc(alias = "g_win32_input_stream_get_close_handle")]
    fn get_close_handle(&self) -> bool;
}

impl<O: IsA<Win32InputStream>> Win32InputStreamExt for O {
    fn get_close_handle(&self) -> bool {
        unsafe {
            from_glib(ffi::g_win32_input_stream_get_close_handle(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for Win32InputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Win32InputStream")
    }
}

impl Win32InputStream {
    #[doc(alias = "g_win32_input_stream_new")]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn take_handle<T: IntoRawHandle>(handle: T) -> Win32InputStream {
        let handle = handle.into_raw_handle();
        let close_handle = true.to_glib();
        InputStream::from_glib_full(ffi::g_win32_input_stream_new(handle, close_handle))
            .unsafe_cast()
    }

    #[doc(alias = "g_win32_input_stream_new")]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn with_handle<T: AsRawHandle>(handle: T) -> Win32InputStream {
        let handle = handle.as_raw_handle();
        let close_handle = false.to_glib();
        InputStream::from_glib_full(ffi::g_win32_input_stream_new(handle, close_handle))
            .unsafe_cast()
    }
}

impl AsRawHandle for Win32InputStream {
    fn as_raw_handle(&self) -> RawHandle {
        unsafe { ffi::g_win32_input_stream_get_handle(self.to_glib_none().0) as _ }
    }
}

pub trait Win32InputStreamExtManual: Sized {
    #[doc(alias = "g_win32_input_stream_get_handle")]
    fn get_handle<T: FromRawHandle>(&self) -> T;

    #[doc(alias = "g_win32_input_stream_set_close_handle")]
    #[allow(clippy::missing_safety_doc)]
    unsafe fn set_close_handle(&self, close_handle: bool);
}

impl<O: IsA<Win32InputStream>> Win32InputStreamExtManual for O {
    fn get_handle<T: FromRawHandle>(&self) -> T {
        unsafe {
            T::from_raw_handle(ffi::g_win32_input_stream_get_handle(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    unsafe fn set_close_handle(&self, close_handle: bool) {
        ffi::g_win32_input_stream_set_close_handle(
            self.as_ref().to_glib_none().0,
            close_handle.to_glib(),
        );
    }
}

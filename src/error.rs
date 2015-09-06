// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::str;
use std::ffi::CStr;
use std::fmt::{self, Formatter, Debug, Display};
use std::error;
use glib_ffi::{self, GQuark};
use glib_container::GlibContainer;
use translate::ToGlibPtr;

/// The Error structure contains information about an error that has occurred.
pub struct Error {
    pointer: *mut glib_ffi::GError
}

impl Error {
    /// Creates a new Error; unlike Error::new(), message is not a printf()-style format string.
    /// Use this function if message contains text you don't have control over, that could include
    /// printf() escape sequences.
    pub fn new_literal(domain: GQuark, code: i32, message: &str) -> Option<Error> {
        let tmp_pointer = unsafe {
            glib_ffi::g_error_new_literal(domain, code, message.to_glib_none().0)
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(Error{pointer: tmp_pointer})
        }
    }

    /// Frees an Error struct and associated resources.
    pub fn release(&mut self) -> () {
        if !self.pointer.is_null() {
            unsafe { glib_ffi::g_error_free(self.pointer) };
            self.pointer = ::std::ptr::null_mut();
        }
    }

    /// Returns true if error matches domain and code , false otherwise. In particular, when error.pointer
    /// is NULL, false will be returned.
    ///
    /// If domain contains a FAILED (or otherwise generic) error code, you should generally not check
    /// for it explicitly, but should instead treat any not-explicitly-recognized error code as being
    /// equivalent to the FAILED code. This way, if the domain is extended in the future to provide a
    /// more specific error code for a certain case, your code will still work.
    pub fn matches(&self, domain: GQuark, code: i32) -> bool {
        match unsafe { glib_ffi::g_error_matches(self.pointer, domain, code) } {
            glib_ffi::GFALSE => false,
            _ => true
        }
    }

    /// Does nothing if self.pointer is NULL; if self.pointer is non-NULL, then *self.pointer must be NULL.
    /// A new GError is created and assigned to *self.pointer .
    pub fn set(&mut self, domain: GQuark, code: i32, message: &str) -> () {
        unsafe {
            glib_ffi::g_set_error_literal(&mut self.pointer, domain, code, message.to_glib_none().0)
        }
    }

    /// If other.pointer is NULL, free self ; otherwise, moves self into other. The error variable
    /// other.pointer points to must be NULL.
    ///
    /// Note that self is no longer valid after this call. If you want to keep using the same Error
    /// struct, you need to set it to NULL after calling this function on it.
    pub fn propagate(&mut self, other: &Error) -> () {
        unsafe { glib_ffi::g_propagate_error(&mut self.pointer, other.pointer) }
    }

    /// Returns the error message stored in the wrapped GError
    pub fn message(&self) -> &str {
        let c_str = unsafe { CStr::from_ptr((*self.pointer).message) };
        str::from_utf8(c_str.to_bytes()).unwrap()
    }
}

impl Clone for Error {
    fn clone(&self) -> Error {
        let tmp_pointer = unsafe { glib_ffi::g_error_copy(self.pointer) };

        if tmp_pointer.is_null() {
            Error {
                pointer: ::std::ptr::null_mut()
            }
        } else {
            GlibContainer::wrap(tmp_pointer)
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.message()
    }
}

impl Drop for Error {
    fn drop(&mut self) {
        self.release();
    }
}

impl GlibContainer<*mut glib_ffi::GError> for Error {
    fn wrap(pointer: *mut glib_ffi::GError) -> Error {
        Error {
            pointer: pointer
        }
    }

    fn unwrap(&self) -> *mut glib_ffi::GError {
        self.pointer
    }
}

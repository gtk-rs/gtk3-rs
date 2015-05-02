// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GdkPixbufLoader — Application-driven progressive image loading.

use std;
use ffi;
use glib::{to_bool, GlibContainer};
use glib::translate::ToGlibPtr;
use libc::{c_int, c_uint};

#[repr(C)]
pub struct PixbufLoader {
    pointer: *mut ffi::C_GdkPixbufLoader
}

impl PixbufLoader {
    pub fn new() -> Option<PixbufLoader> {
        let tmp = unsafe { ffi::gdk_pixbuf_loader_new() };

        if tmp.is_null() {
            None
        } else {
            Some(PixbufLoader::wrap_pointer(tmp))
        }
    }

    pub fn new_with_type(image_type: &str, error: &mut ::glib::Error) -> Option<PixbufLoader> {
        let tmp = unsafe { ffi::gdk_pixbuf_loader_new_with_type(image_type.borrow_to_glib().0, &mut error.unwrap()) };

        if tmp.is_null() {
            None
        } else {
            Some(PixbufLoader::wrap_pointer(tmp))
        }
    }

    pub fn new_with_mime_type(mime_type: &str, error: &mut ::glib::Error) -> Option<PixbufLoader> {
        let tmp = unsafe { ffi::gdk_pixbuf_loader_new_with_mime_type(mime_type.borrow_to_glib().0, &mut error.unwrap()) };

        if tmp.is_null() {
            None
        } else {
            Some(PixbufLoader::wrap_pointer(tmp))
        }
    }

    pub fn get_format(&self) -> Option<::PixbufFormat> {
        let tmp = unsafe { ffi::gdk_pixbuf_loader_get_format(self.unwrap_pointer()) };

        if tmp.is_null() {
            None
        } else {
            Some(::PixbufFormat::wrap_pointer(tmp))
        }
    }

    pub fn loader_write(&self, buf: &[u8]) -> Result<(), ::glib::Error> {
        unsafe {
            let mut error: *mut ::glib::ffi::C_GError = std::ptr::null_mut();
            match to_bool(ffi::gdk_pixbuf_loader_write(self.unwrap_pointer(), buf.as_ptr(), buf.len() as c_uint, &mut error)) {
                true => Ok(()),
                false => Err(::glib::Error::wrap(error))
            }
        }
    }

    /*pub fn loader_write_bytes(&self, buffer: &glib::Bytes, error: &mut ::glib::Error) -> bool {
        gdk_pixbuf_loader_write_bytes
    }*/

    pub fn set_size(&self, width: isize, height: isize) {
        unsafe { ffi::gdk_pixbuf_loader_set_size(self.unwrap_pointer(), width as c_int, height as c_int) }
    }

    pub fn get_pixbuf(&self) -> Option<::Pixbuf> {
        let tmp = unsafe { ffi::gdk_pixbuf_loader_get_pixbuf(self.unwrap_pointer()) };

        if tmp.is_null() {
            None
        } else {
            Some(::Pixbuf::wrap_pointer(tmp))
        }
    }

    /*pub fn get_animation(&self) -> Option<::PixbufAnimation> {
        let tmp = unsafe { ffi::gdk_pixbuf_loader_get_animation(self.unwrap_pointer()) };

        if tmp.is_null() {
            None
        } else {
            Some(::PixbufAnimation::wrap_pointer(tmp))
        }
    }*/

    pub fn close(&self) -> Result<(), ::glib::Error> {
        unsafe {
            let mut error: *mut ::glib::ffi::C_GError = std::ptr::null_mut();
            match to_bool(ffi::gdk_pixbuf_loader_close(self.unwrap_pointer(), &mut error)) {
                true => Ok(()),
                false => Err(::glib::Error::wrap(error))
            }
        }
    }
}

impl_GObjectFunctions!(PixbufLoader, C_GdkPixbufLoader);

// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ptr;
use libc::size_t;
use glib::Error;
use glib::translate::*;
use gdk_pixbuf_ffi as ffi;
use super::Pixbuf;
use super::animation::PixbufAnimation;
use super::format::PixbufFormat;

glib_wrapper! {
    pub struct PixbufLoader(Object<ffi::GdkPixbufLoader>);

    match fn {
        get_type => || ffi::gdk_pixbuf_loader_get_type(),
    }
}

impl PixbufLoader {
    pub fn new() -> PixbufLoader {
        unsafe { from_glib_full(ffi::gdk_pixbuf_loader_new()) }
    }

    pub fn new_with_type(image_type: &str) -> Result<PixbufLoader, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ptr = ffi::gdk_pixbuf_loader_new_with_type(image_type.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ptr))
            }
            else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn new_with_mime_type(mime_type: &str) -> Result<PixbufLoader, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ptr = ffi::gdk_pixbuf_loader_new_with_mime_type(mime_type.to_glib_none().0,
                                                                &mut error);
            if error.is_null() {
                Ok(from_glib_full(ptr))
            }
            else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn get_format(&self) -> PixbufFormat {
        unsafe {
            from_glib_none(ffi::gdk_pixbuf_loader_get_format(self.to_glib_none().0))
        }
    }

    pub fn loader_write(&self, buf: &[u8]) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            ffi::gdk_pixbuf_loader_write(self.to_glib_none().0, buf.as_ptr() as *mut u8,
                buf.len() as size_t, &mut error);
            if error.is_null() {
                Ok(())
            }
            else {
                Err(from_glib_full(error))
            }
        }
    }

    /*pub fn loader_write_bytes(&self, buffer: &glib::Bytes, error: &mut Error) -> bool {
      gdk_pixbuf_loader_write_bytes
      }*/

    pub fn set_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gdk_pixbuf_loader_set_size(self.to_glib_none().0, width, height)
        }
    }

    pub fn get_pixbuf(&self) -> Option<Pixbuf> {
        unsafe { from_glib_none(ffi::gdk_pixbuf_loader_get_pixbuf(self.to_glib_none().0)) }
    }

    pub fn get_animation(&self) -> Option<PixbufAnimation> {
        unsafe { from_glib_none(ffi::gdk_pixbuf_loader_get_animation(self.to_glib_none().0)) }
    }

    pub fn close(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            ffi::gdk_pixbuf_loader_close(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            }
            else {
                Err(from_glib_full(error))
            }
        }
    }
}

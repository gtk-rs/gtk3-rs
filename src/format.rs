// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_pixbuf_ffi as ffi;
use glib::translate::*;
use libc::c_char;

pub struct PixbufFormat(*mut ffi::GdkPixbufFormat);

impl PixbufFormat {
    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_format_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_description(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_format_get_description(self.to_glib_none().0))
        }
    }

    pub fn get_mime_types(&self) -> Vec<String> {
        unsafe {
            Vec::from_glib_full(
                ffi::gdk_pixbuf_format_get_mime_types(self.to_glib_none().0)
                    as *const *const c_char)
        }
    }

    pub fn get_extensions(&self) -> Vec<String> {
        unsafe {
            Vec::from_glib_full(
                ffi::gdk_pixbuf_format_get_extensions(self.to_glib_none().0)
                    as *const *const c_char)
        }
    }

    pub fn is_writable(&self) -> bool {
        unsafe { from_glib(ffi::gdk_pixbuf_format_is_writable(self.to_glib_none().0)) }
    }

    pub fn is_scalable(&self) -> bool {
        unsafe { from_glib(ffi::gdk_pixbuf_format_is_scalable(self.to_glib_none().0)) }
    }

    pub fn is_disabled(&self) -> bool {
        unsafe { from_glib(ffi::gdk_pixbuf_format_is_disabled(self.to_glib_none().0)) }
    }

    pub fn set_disabled(&self, disabled: bool) {
        unsafe { ffi::gdk_pixbuf_format_set_disabled(self.to_glib_none().0, disabled.to_glib()) }
    }

    pub fn get_license(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_format_get_license(self.to_glib_none().0))
        }
    }
}

// It's owned by the library and we never free it

impl<'a> ToGlibPtr<'a, *mut ffi::GdkPixbufFormat> for PixbufFormat {
    type Storage = ();

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut ffi::GdkPixbufFormat, PixbufFormat> {
        Stash(self.0, ())
    }
}

impl FromGlibPtrNone<*mut ffi::GdkPixbufFormat> for PixbufFormat {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GdkPixbufFormat) -> PixbufFormat {
        PixbufFormat(ptr)
    }
}

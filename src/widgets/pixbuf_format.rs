// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

//! GdkDeviceManager — Functions for handling input devices

use ffi;
use glib::{to_bool, to_gboolean};
use glib::translate::{FromGlibPtr, ToGlibPtr, FromGlibPtrContainer};

#[repr(C)]
pub struct PixbufFormat {
    pointer: *mut ffi::C_GdkPixbufFormat
}

impl PixbufFormat {
    pub fn copy(&self) -> Option<PixbufFormat> {
        let tmp = unsafe { ffi::gdk_pixbuf_format_copy() };

        if tmp.is_null() {
            None
        } else {
            Some(PixbufFormat::wrap_pointer(tmp))
        }
    }

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gdk_pixbuf_format_get_name(self.pointer))
        }
    }

    pub fn get_description(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gdk_pixbuf_format_get_description(self.pointer))
        }
    }

    pub fn get_mime_types(&self) -> Vec<String> {
        unsafe {
            let mut length = 0;
            let ptr = ffi::gdk_pixbuf_format_get_mime_types(self.unwrap_pointer())) as *const *const c_char;

            loop {
                if ptr.offset(length).is_null() {
                    break;
                }
                length += 1;
            }
            if length == 0 {
                Vec::new()
            } else {
                FromGlibPtrContainer::borrow_num(ptr, length)
            }
        }
    }

    pub fn get_extensions(&self) -> Vec<String> {
        unsafe {
            let mut length = 0;
            let ptr = ffi::gdk_pixbuf_format_get_extensions(self.unwrap_pointer())) as *const *const c_char;

            loop {
                if ptr.offset(length).is_null() {
                    break;
                }
                length += 1;
            }
            if length == 0 {
                Vec::new()
            } else {
                FromGlibPtrContainer::borrow_num(ptr, length)
            }
        }
    }

    pub fn is_writable(&self) -> bool {
        unsafe { to_bool(ffi::gdk_pixbuf_format_is_writable(self.unwrap_pointer())) }
    }

    pub fn is_scalable(&self) -> bool {
        unsafe { to_bool(ffi::gdk_pixbuf_format_is_scalable(self.unwrap_pointer())) }
    }

    pub fn is_disabled(&self) -> bool {
        unsafe { to_bool(ffi::gdk_pixbuf_format_is_disabled(self.unwrap_pointer())) }
    }

    pub fn set_disabled(&self, disabled: bool) -> bool {
        unsafe { ffi::gdk_pixbuf_format_set_disabled(self.unwrap_pointer(), to_gboolean(disabled)) }
    }

    pub fn get_license(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gdk_pixbuf_format_get_license(self.pointer))
        }
    }
}

impl Drop for PixbufFormat {
    fn drop(&mut self) {
        if !self.pointer.is_null() {
            unsafe { ffi::gdk_pixbuf_format_free(self.pointer) };
            self.pointer = ::std::ptr::null_mut();
        }
    }
}

impl_GObjectFunctions!(PixbufFormat, C_GdkPixbufFormat);
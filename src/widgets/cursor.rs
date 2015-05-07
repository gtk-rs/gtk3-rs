// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Cursors — Standard and pixmap cursors

use ffi;
use glib::translate::ToGlibPtr;
//use libc::{c_int};

#[repr(C)]
pub struct Cursor {
    pointer: *mut ffi::C_GdkCursor
}

impl Cursor {
    pub fn new(cursor_type: ::CursorType) -> Option<Cursor> {
        let tmp = unsafe { ffi::gdk_cursor_new(cursor_type) };

        if tmp.is_null() {
            None
        } else {
            Some(Cursor {
                pointer: tmp
            })
        }
    }

    /*pub fn new_from_pixbuf(display: &::Display, pixbuf: &::Pixbuf, x: i32, y: i32) -> Option<Cursor> {
        let tmp = unsafe { ffi::gdk_cursor_new_from_pixbuf(display.unwrap_pointer(), pixbuf.unwrap_pointer(), x as c_int, y as c_int) };

        if tmp.is_null() {
            None
        } else {
            Some(Cursor {
                pointer: tmp
            })
        }
    }*/

    pub fn new_from_name(display: &::Display, name: &str) -> Option<Cursor> {
        let tmp = unsafe {
            ffi::gdk_cursor_new_from_name(display.unwrap_pointer(), name.to_glib_none().0)
        };

        if tmp.is_null() {
            None
        } else {
            Some(Cursor {
                pointer: tmp
            })
        }
    }

    pub fn new_for_display(display: &::Display, cursor_type: ::CursorType) -> Option<Cursor> {
        let tmp = unsafe { ffi::gdk_cursor_new_for_display(display.unwrap_pointer(), cursor_type) };

        if tmp.is_null() {
            None
        } else {
            Some(Cursor {
                pointer: tmp
            })
        }
    }

    pub fn get_display(&self) -> Option<::Display> {
        let tmp = unsafe { ffi::gdk_cursor_get_display(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(::Display::wrap_pointer(tmp))
        }
    }

    /*pub fn get_image(&self) -> Option<::Pixbuf> {
        let tmp = unsafe { ffi::gdk_cursor_get_image(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(::Pixbuf::wrap_pointer(tmp))
        }
    }*/

    pub fn get_cursor_type(&self) -> ::CursorType {
        unsafe { ffi::gdk_cursor_get_cursor_type(self.pointer) }
    }
}

impl_GObjectFunctions!(Cursor, C_GdkCursor);
// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Cursors — Standard and pixmap cursors

use glib::translate::*;
use glib::types;
use display::Display;
use object::Object;
use pixbuf::Pixbuf;
use ffi;

pub type Type = ffi::enums::CursorType;

pub type Cursor = Object<ffi::C_GdkCursor>;

impl types::StaticType for Cursor {
    fn static_type() -> types::Type { unsafe { from_glib(ffi::gdk_cursor_get_type()) } }
}

impl Cursor {
    pub fn new(cursor_type: Type) -> Cursor {
        unsafe { from_glib_full(ffi::gdk_cursor_new(cursor_type)) }
    }

    pub fn new_from_pixbuf(display: &Display, pixbuf: &Pixbuf, x: i32, y: i32) -> Cursor {
        unsafe {
            from_glib_full(
                ffi::gdk_cursor_new_from_pixbuf(display.to_glib_none().0,
                    pixbuf.to_glib_none().0, x, y))
        }
    }

    pub fn new_from_name(display: &Display, name: &str) -> Option<Cursor> {
        unsafe {
            from_glib_full(ffi::gdk_cursor_new_from_name(display.to_glib_none().0,
                                                         name.to_glib_none().0))
        }
    }

    pub fn new_for_display(display: &Display, cursor_type: Type) -> Cursor {
        unsafe { 
            from_glib_full(ffi::gdk_cursor_new_for_display(display.to_glib_none().0, cursor_type))
        }
    }

    pub fn get_display(&self) -> Display {
        unsafe { from_glib_none(ffi::gdk_cursor_get_display(self.to_glib_none().0)) }
    }

    pub fn get_image(&self) -> Option<Pixbuf> {
        unsafe { from_glib_full(ffi::gdk_cursor_get_image(self.to_glib_none().0)) }
    }

    pub fn get_cursor_type(&self) -> Type {
        unsafe { ffi::gdk_cursor_get_cursor_type(self.to_glib_none().0) }
    }
}

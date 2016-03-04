// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use display::Display;
use gdk_pixbuf::Pixbuf;
use ffi;

pub type Type = ffi::GdkCursorType;

glib_wrapper! {
    pub struct Cursor(Object<ffi::GdkCursor>);

    match fn {
        get_type => || ffi::gdk_cursor_get_type(),
    }
}

impl Cursor {
    pub fn new(cursor_type: Type) -> Cursor {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gdk_cursor_new(cursor_type)) }
    }

    pub fn new_from_pixbuf(display: &Display, pixbuf: &Pixbuf, x: i32, y: i32) -> Cursor {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(
                ffi::gdk_cursor_new_from_pixbuf(display.to_glib_none().0,
                    pixbuf.to_glib_none().0, x, y))
        }
    }

    pub fn new_from_name(display: &Display, name: &str) -> Option<Cursor> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gdk_cursor_new_from_name(display.to_glib_none().0,
                                                         name.to_glib_none().0))
        }
    }

    pub fn new_for_display(display: &Display, cursor_type: Type) -> Cursor {
        skip_assert_initialized!();
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

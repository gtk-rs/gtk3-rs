// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Cursors — Standard and pixmap cursors

use glib::translate::*;
use glib::types;
use display::Display;
use object::Object;
use pixbuf::Pixbuf;
use ffi;

pub type Type = ffi::GdkCursorType;

pub type Cursor = Object<ffi::GdkCursor>;

impl types::StaticType for Cursor {
    fn static_type() -> types::Type { unsafe { from_glib(ffi::gdk_cursor_get_type()) } }
}

impl Cursor {
    /// Creates a new cursor from the set of builtin cursors for the default display. See
    /// Cursor::new_for_display().
    ///
    /// To make the cursor invisible, use Type::BlankCursor.
    ///
    /// Warning : Cursor::new() has been deprecated since version 3.16 and should not be used
    /// in newly-written code.
    ///
    /// Use Cursor::new_for_display() instead.
    pub fn new(cursor_type: Type) -> Cursor {
        unsafe { from_glib_full(ffi::gdk_cursor_new(cursor_type)) }
    }

    /// Creates a new cursor from a pixbuf.
    ///
    /// Not all GDK backends support RGBA cursors. If they are not supported, a monochrome
    /// approximation will be displayed. The functions Display::supports_cursor_alpha() and
    /// Display::supports_cursor_color() can be used to determine whether RGBA cursors are
    /// supported; Display::get_default_cursor_size() and Display::get_maximal_cursor_size()
    /// give information about cursor sizes.
    ///
    /// If `x` or `y` are -1, the pixbuf must have options named “x_hot” and “y_hot”, resp.,
    /// containing integer values between 0 and the width resp. height of the pixbuf. (Since:
    /// 3.0)
    ///
    /// On the X backend, support for RGBA cursors requires a sufficently new version of the
    /// X Render extension.
    pub fn new_from_pixbuf(display: &Display, pixbuf: &Pixbuf, x: i32, y: i32) -> Cursor {
        unsafe {
            from_glib_full(
                ffi::gdk_cursor_new_from_pixbuf(display.to_glib_none().0,
                    pixbuf.to_glib_none().0, x, y))
        }
    }

    /// Creates a new cursor by looking up `name` in the current cursor theme.
    pub fn new_from_name(display: &Display, name: &str) -> Option<Cursor> {
        unsafe {
            from_glib_full(ffi::gdk_cursor_new_from_name(display.to_glib_none().0,
                                                         name.to_glib_none().0))
        }
    }

    /// Creates a new cursor from the set of builtin cursors.
    ///
    /// You can find a list of useful ones here:
    /// https://developer.gnome.org/gdk3/stable/gdk3-Cursors.html#gdk-cursor-new-for-display
    pub fn new_for_display(display: &Display, cursor_type: Type) -> Cursor {
        unsafe { 
            from_glib_full(ffi::gdk_cursor_new_for_display(display.to_glib_none().0, cursor_type))
        }
    }

    /// Returns the display on which the Cursor is defined.
    pub fn get_display(&self) -> Display {
        unsafe { from_glib_none(ffi::gdk_cursor_get_display(self.to_glib_none().0)) }
    }

    /// Returns a Pixbuf with the image used to display the cursor.
    ///
    /// Note that depending on the capabilities of the windowing system and on the cursor,
    /// GDK may not be able to obtain the image data. In this case, None is returned.
    pub fn get_image(&self) -> Option<Pixbuf> {
        unsafe { from_glib_full(ffi::gdk_cursor_get_image(self.to_glib_none().0)) }
    }

    /// Returns the cursor type for this cursor.
    pub fn get_cursor_type(&self) -> Type {
        unsafe { ffi::gdk_cursor_get_cursor_type(self.to_glib_none().0) }
    }
}

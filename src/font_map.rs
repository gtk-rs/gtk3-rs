// Copyright 2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use cairo;
use pango;
use FontMap;
use glib::object::IsA;
use glib::translate::*;
use ffi;

pub trait FontMapExtManual {
    fn get_font_type(&self) -> cairo::FontType;
}

impl<O: IsA<FontMap>> FontMapExtManual for O {
    fn get_font_type(&self) -> cairo::FontType {
        unsafe {
            ffi::pango_cairo_font_map_get_font_type(self.to_glib_none().0)
        }
    }
}

impl FontMap {
    pub fn new_for_font_type(fonttype: cairo::FontType) -> Option<pango::FontMap> {
        unsafe {
            from_glib_full(ffi::pango_cairo_font_map_new_for_font_type(fonttype))
        }
    }

    pub fn new() -> Option<pango::FontMap> {
        unsafe {
            from_glib_full(ffi::pango_cairo_font_map_new())
        }
    }
}

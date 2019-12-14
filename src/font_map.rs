// Copyright 2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use crate::FontMap;
use cairo;
use glib::object::IsA;
use glib::translate::*;
use pango;
use pango_cairo_sys;

pub trait FontMapExtManual {
    fn get_font_type(&self) -> cairo::FontType;
}

impl<O: IsA<FontMap>> FontMapExtManual for O {
    fn get_font_type(&self) -> cairo::FontType {
        unsafe {
            pango_cairo_sys::pango_cairo_font_map_get_font_type(self.as_ref().to_glib_none().0)
                .into()
        }
    }
}

impl FontMap {
    pub fn new_for_font_type(fonttype: cairo::FontType) -> Option<pango::FontMap> {
        unsafe {
            from_glib_full(pango_cairo_sys::pango_cairo_font_map_new_for_font_type(
                fonttype.into(),
            ))
        }
    }

    pub fn new() -> Option<pango::FontMap> {
        unsafe { from_glib_full(pango_cairo_sys::pango_cairo_font_map_new()) }
    }

    pub fn set_default(font_map: Option<Self>) {
        unsafe {
            pango_cairo_sys::pango_cairo_font_map_set_default(font_map.as_ref().to_glib_none().0);
        }
    }
}

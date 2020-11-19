// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::object::IsA;
use glib::translate::*;
use pango;
use std::fmt;

glib::glib_wrapper! {
    pub struct FontMap(Interface<ffi::PangoCairoFontMap>) @requires pango::FontMap;

    match fn {
        get_type => || ffi::pango_cairo_font_map_get_type(),
    }
}

impl FontMap {
    pub fn get_default() -> Option<pango::FontMap> {
        unsafe { from_glib_none(ffi::pango_cairo_font_map_get_default()) }
    }
}

pub const NONE_FONT_MAP: Option<&FontMap> = None;

pub trait FontMapExt: 'static {
    fn get_resolution(&self) -> f64;

    fn set_resolution(&self, dpi: f64);
}

impl<O: IsA<FontMap>> FontMapExt for O {
    fn get_resolution(&self) -> f64 {
        unsafe { ffi::pango_cairo_font_map_get_resolution(self.as_ref().to_glib_none().0) }
    }

    fn set_resolution(&self, dpi: f64) {
        unsafe {
            ffi::pango_cairo_font_map_set_resolution(self.as_ref().to_glib_none().0, dpi);
        }
    }
}

impl fmt::Display for FontMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FontMap")
    }
}

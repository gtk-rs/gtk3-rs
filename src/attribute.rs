// Copyright 2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use pango_sys;
use AttrClass;
use Attribute;
use Gravity;
use GravityHint;
use Stretch;
use Style;
use Underline;
use Variant;
use Weight;

impl Attribute {
    #[cfg(any(feature = "v1_38", feature = "dox"))]
    pub fn new_background_alpha(alpha: u16) -> Option<Attribute> {
        unsafe { from_glib_full(pango_sys::pango_attr_background_alpha_new(alpha)) }
    }

    pub fn new_background(red: u16, green: u16, blue: u16) -> Option<Attribute> {
        unsafe { from_glib_full(pango_sys::pango_attr_background_new(red, green, blue)) }
    }

    pub fn new_fallback(enable_fallback: bool) -> Option<Attribute> {
        unsafe {
            from_glib_full(pango_sys::pango_attr_fallback_new(
                enable_fallback.to_glib(),
            ))
        }
    }

    pub fn new_family(family: &str) -> Option<Attribute> {
        unsafe { from_glib_full(pango_sys::pango_attr_family_new(family.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_38", feature = "dox"))]
    pub fn new_foreground_alpha(alpha: u16) -> Option<Attribute> {
        unsafe { from_glib_full(pango_sys::pango_attr_foreground_alpha_new(alpha)) }
    }

    pub fn new_foreground(red: u16, green: u16, blue: u16) -> Option<Attribute> {
        unsafe { from_glib_full(pango_sys::pango_attr_foreground_new(red, green, blue)) }
    }

    pub fn new_gravity_hint(hint: GravityHint) -> Option<Attribute> {
        unsafe { from_glib_full(pango_sys::pango_attr_gravity_hint_new(hint.to_glib())) }
    }

    pub fn new_gravity(gravity: Gravity) -> Option<Attribute> {
        unsafe { from_glib_full(pango_sys::pango_attr_gravity_new(gravity.to_glib())) }
    }

    pub fn new_letter_spacing(letter_spacing: i32) -> Option<Attribute> {
        unsafe { from_glib_full(pango_sys::pango_attr_letter_spacing_new(letter_spacing)) }
    }

    pub fn new_rise(rise: i32) -> Option<Attribute> {
        unsafe { from_glib_full(pango_sys::pango_attr_rise_new(rise)) }
    }

    pub fn new_scale(scale_factor: f64) -> Option<Attribute> {
        unsafe { from_glib_full(pango_sys::pango_attr_scale_new(scale_factor)) }
    }

    pub fn new_size(size: i32) -> Option<Attribute> {
        unsafe { from_glib_full(pango_sys::pango_attr_size_new(size)) }
    }

    pub fn new_size_absolute(size: i32) -> Option<Attribute> {
        unsafe { from_glib_full(pango_sys::pango_attr_size_new_absolute(size)) }
    }

    pub fn new_stretch(stretch: Stretch) -> Option<Attribute> {
        unsafe { from_glib_full(pango_sys::pango_attr_stretch_new(stretch.to_glib())) }
    }

    pub fn new_strikethrough_color(red: u16, green: u16, blue: u16) -> Option<Attribute> {
        unsafe {
            from_glib_full(pango_sys::pango_attr_strikethrough_color_new(
                red, green, blue,
            ))
        }
    }

    pub fn new_strikethrough(strikethrough: bool) -> Option<Attribute> {
        unsafe {
            from_glib_full(pango_sys::pango_attr_strikethrough_new(
                strikethrough.to_glib(),
            ))
        }
    }

    pub fn new_style(style: Style) -> Option<Attribute> {
        unsafe { from_glib_full(pango_sys::pango_attr_style_new(style.to_glib())) }
    }

    pub fn new_underline_color(red: u16, green: u16, blue: u16) -> Option<Attribute> {
        unsafe { from_glib_full(pango_sys::pango_attr_underline_color_new(red, green, blue)) }
    }

    pub fn new_underline(underline: Underline) -> Option<Attribute> {
        unsafe { from_glib_full(pango_sys::pango_attr_underline_new(underline.to_glib())) }
    }

    pub fn new_variant(variant: Variant) -> Option<Attribute> {
        unsafe { from_glib_full(pango_sys::pango_attr_variant_new(variant.to_glib())) }
    }

    pub fn new_weight(weight: Weight) -> Option<Attribute> {
        unsafe { from_glib_full(pango_sys::pango_attr_weight_new(weight.to_glib())) }
    }

    pub fn get_attr_class(&self) -> AttrClass {
        unsafe { from_glib_full((*self.to_glib_none().0).klass) }
    }

    pub fn get_start_index(&self) -> u32 {
        unsafe {
            let stash = self.to_glib_none();
            (*stash.0).start_index
        }
    }

    pub fn get_end_index(&self) -> u32 {
        unsafe {
            let stash = self.to_glib_none();
            (*stash.0).end_index
        }
    }

    pub fn set_start_index(&mut self, index: u32) {
        unsafe {
            let stash = self.to_glib_none_mut();
            (*stash.0).start_index = index;
        }
    }

    pub fn set_end_index(&mut self, index: u32) {
        unsafe {
            let stash = self.to_glib_none_mut();
            (*stash.0).end_index = index;
        }
    }
}

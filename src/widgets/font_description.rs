use std::fmt::{self, Formatter, Display};

use ffi;
use glib::translate::{Stash, FromGlibPtrFull, FromGlibPtrNone, ToGlibPtr, ToGlib, from_glib_full, from_glib_none, from_glib};

use {
    FontMask,
    Gravity,
    Weight,
    Style,
    Variant,
    Stretch,
};

impl<'a> ToGlibPtr<'a, *mut ffi::PangoFontDescription> for &'a FontDescription {
    type Storage = &'a FontDescription;

    fn to_glib_none(&self) -> Stash<'a, *mut ffi::PangoFontDescription, Self> {
        Stash(self.0, *self)
    }
}

impl FromGlibPtrNone<*mut ffi::PangoFontDescription> for FontDescription {
    unsafe fn from_glib_none(ptr: *mut ffi::PangoFontDescription) -> Self {
        let tmp = ffi::pango_font_description_copy(ptr);
        assert!(!tmp.is_null());
        FontDescription(tmp)
    }
}

impl FromGlibPtrFull<*mut ffi::PangoFontDescription> for FontDescription {
    unsafe fn from_glib_full(ptr: *mut ffi::PangoFontDescription) -> Self {
        assert!(!ptr.is_null());
        FontDescription(ptr)
    }
}

impl FromGlibPtrNone<*const ffi::PangoFontDescription> for FontDescription {
    unsafe fn from_glib_none(ptr: *const ffi::PangoFontDescription) -> Self {
        let tmp = ffi::pango_font_description_copy(ptr);
        assert!(!tmp.is_null());
        FontDescription(tmp as *mut _)
    }
}

impl FromGlibPtrFull<*const ffi::PangoFontDescription> for FontDescription {
    unsafe fn from_glib_full(ptr: *const ffi::PangoFontDescription) -> Self {
        assert!(!ptr.is_null());
        FontDescription(ptr as *mut _)
    }
}

pub struct FontDescription(*mut ffi::PangoFontDescription);

impl FontDescription {
    pub fn new() -> FontDescription {
        unsafe { from_glib_full(ffi::pango_font_description_new()) }
    }

    pub fn from_string(name: &str) -> FontDescription {
        unsafe { from_glib_full(ffi::pango_font_description_from_string(name.to_glib_none().0)) }
    }

    pub fn set_family(&mut self, family: &str) {
        unsafe { ffi::pango_font_description_set_family(self.0, family.to_glib_none().0) };
    }

    pub fn get_family(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::pango_font_description_get_family(self.0)) }
    }

    pub fn set_weight(&mut self, weight: Weight) {
        unsafe { ffi::pango_font_description_set_weight(self.0, weight.to_glib()) };
    }

    pub fn get_weight(&self) -> Weight {
        unsafe { from_glib(ffi::pango_font_description_get_weight(self.0)) }
    }

    pub fn set_size(&mut self, size: i32) {
        unsafe { ffi::pango_font_description_set_size(self.0, size) };
    }

    pub fn get_size(&self) -> i32 {
        unsafe { ffi::pango_font_description_get_size(self.0) }
    }

    pub fn set_absolute_size(&mut self, size: f64) {
        unsafe { ffi::pango_font_description_set_absolute_size(self.0, size) };
    }

    pub fn get_size_is_absolute(&self) -> bool {
        unsafe { from_glib(ffi::pango_font_description_get_size_is_absolute(self.0)) }
    }

    pub fn set_gravity(&mut self, gravity: Gravity) {
        unsafe { ffi::pango_font_description_set_gravity(self.0, gravity.to_glib()) };
    }

    pub fn get_gravity(&self) -> Gravity {
        unsafe { from_glib(ffi::pango_font_description_get_gravity(self.0)) }
    }

    pub fn get_set_fields(&self) -> FontMask {
        unsafe { from_glib(ffi::pango_font_description_get_set_fields(self.0)) }
    }

    pub fn unset_fields(&mut self, to_unset: FontMask) {
        unsafe { ffi::pango_font_description_unset_fields(self.0, to_unset.to_glib()) };
    }

    pub fn to_filename(&self) -> String {
        unsafe { from_glib_full(ffi::pango_font_description_to_string(self.0)) }
    }

    pub fn set_style(&self, style: Style) {
        unsafe { ffi::pango_font_description_set_style(self.0, style.to_glib()) }
    }

    pub fn set_variant(&self, variant: Variant) {
        unsafe { ffi::pango_font_description_set_variant(self.0, variant.to_glib()) }
    }

    pub fn set_stretch(&self, stretch: Stretch) {
        unsafe { ffi::pango_font_description_set_stretch(self.0, stretch.to_glib()) }
    }
}

impl Display for FontDescription {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s: String = unsafe { from_glib_full(ffi::pango_font_description_to_string(self.0)) };
        write!(f, "{}", s)
    }
}

impl PartialEq for FontDescription {
    fn eq(&self, other: &FontDescription) -> bool {
        unsafe { from_glib(ffi::pango_font_description_equal(self.0, other.0)) }
    }
}

impl Eq for FontDescription {}

impl Clone for FontDescription {
    fn clone(&self) -> FontDescription {
        unsafe {
            let tmp = ffi::pango_font_description_copy(self.0);
            assert!(!tmp.is_null());
            from_glib_full(tmp)
        }
    }
}

impl Drop for FontDescription {
    fn drop(&mut self) {
        unsafe { ffi::pango_font_description_free(self.0) }
    }
}

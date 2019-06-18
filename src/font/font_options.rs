use ffi;
#[cfg(feature = "use_glib")]
use glib::translate::*;
use std::cmp::PartialEq;
use std::hash;

#[cfg(any(feature = "v1_16", feature = "dox"))]
use font::font_face::to_optional_string;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use std::ffi::CString;

use enums::{Antialias, HintMetrics, HintStyle, Status, SubpixelOrder};

#[cfg(feature = "use_glib")]
glib_wrapper! {
    #[derive(Debug)]
    pub struct FontOptions(Boxed<ffi::cairo_font_options_t>);

    match fn {
        copy => |ptr| {
            let ptr = ffi::cairo_font_options_copy(ptr);
            let status = ffi::cairo_font_options_status(ptr);
            Status::from(status).ensure_valid();
            ptr
        },
        free => |ptr| ffi::cairo_font_options_destroy(ptr),
        get_type => || ffi::gobject::cairo_gobject_font_options_get_type(),
    }
}

#[cfg(not(feature = "use_glib"))]
#[derive(Debug)]
pub struct FontOptions(*mut ffi::cairo_font_options_t);

impl FontOptions {
    pub fn new() -> FontOptions {
        let font_options: FontOptions =
            unsafe { FontOptions::from_raw_full(ffi::cairo_font_options_create()) };
        font_options.ensure_status();
        font_options
    }

    #[cfg(feature = "use_glib")]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_font_options_t) -> FontOptions {
        from_glib_full(ptr)
    }

    #[cfg(not(feature = "use_glib"))]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_font_options_t) -> FontOptions {
        assert!(!ptr.is_null());
        FontOptions(ptr)
    }

    #[cfg(feature = "use_glib")]
    pub fn to_raw_none(&self) -> *mut ffi::cairo_font_options_t {
        mut_override(self.to_glib_none().0)
    }

    #[cfg(not(feature = "use_glib"))]
    pub fn to_raw_none(&self) -> *mut ffi::cairo_font_options_t {
        self.0
    }

    pub fn ensure_status(&self) {
        let status = unsafe { ffi::cairo_font_options_status(self.to_raw_none()) };
        Status::from(status).ensure_valid()
    }

    pub fn merge(&mut self, other: &FontOptions) {
        unsafe { ffi::cairo_font_options_merge(self.to_raw_none(), other.to_raw_none()) }
    }

    pub fn set_antialias(&mut self, antialias: Antialias) {
        unsafe { ffi::cairo_font_options_set_antialias(self.to_raw_none(), antialias.into()) }
    }

    pub fn get_antialias(&self) -> Antialias {
        unsafe { Antialias::from(ffi::cairo_font_options_get_antialias(self.to_raw_none())) }
    }

    pub fn set_subpixel_order(&mut self, order: SubpixelOrder) {
        unsafe { ffi::cairo_font_options_set_subpixel_order(self.to_raw_none(), order.into()) }
    }

    pub fn get_subpixel_order(&self) -> SubpixelOrder {
        unsafe {
            SubpixelOrder::from(ffi::cairo_font_options_get_subpixel_order(
                self.to_raw_none(),
            ))
        }
    }

    pub fn set_hint_style(&mut self, hint_style: HintStyle) {
        unsafe { ffi::cairo_font_options_set_hint_style(self.to_raw_none(), hint_style.into()) }
    }

    pub fn get_hint_style(&self) -> HintStyle {
        unsafe { HintStyle::from(ffi::cairo_font_options_get_hint_style(self.to_raw_none())) }
    }

    pub fn set_hint_metrics(&mut self, hint_metrics: HintMetrics) {
        unsafe { ffi::cairo_font_options_set_hint_metrics(self.to_raw_none(), hint_metrics.into()) }
    }

    pub fn get_hint_metrics(&self) -> HintMetrics {
        unsafe { HintMetrics::from(ffi::cairo_font_options_get_hint_metrics(self.to_raw_none())) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_variations(&self) -> Option<String> {
        unsafe { to_optional_string(ffi::cairo_font_options_get_variations(self.to_raw_none())) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_variations<'a, T: Into<Option<&'a str>>>(&self, variations: T) {
        unsafe {
            let variations = variations.into();
            match variations {
                Some(ref v) => {
                    let v = CString::new(*v).unwrap();
                    ffi::cairo_font_options_set_variations(self.to_raw_none(), v.as_ptr())
                }
                None => ffi::cairo_font_options_set_variations(self.to_raw_none(), 0 as *const _),
            }
        }
    }
}

impl PartialEq for FontOptions {
    fn eq(&self, other: &FontOptions) -> bool {
        unsafe { ffi::cairo_font_options_equal(self.to_raw_none(), other.to_raw_none()).as_bool() }
    }
}

impl Eq for FontOptions {}

impl hash::Hash for FontOptions {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        unsafe { hash::Hash::hash(&ffi::cairo_font_options_hash(self.to_raw_none()), state) }
    }
}

impl Default for FontOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(feature = "use_glib"))]
impl Drop for FontOptions {
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_font_options_destroy(self.to_raw_none());
        }
    }
}

#[cfg(not(feature = "use_glib"))]
impl Clone for FontOptions {
    fn clone(&self) -> FontOptions {
        unsafe { FontOptions::from_raw_full(ffi::cairo_font_options_copy(self.to_raw_none())) }
    }
}

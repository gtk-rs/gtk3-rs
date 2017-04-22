use glib::translate::*;
use std::cmp::PartialEq;
use ffi;

use ffi::enums::{
    Antialias,
    SubpixelOrder,
    HintStyle,
    HintMetrics,
};
use ffi::cairo_font_options_t;

glib_wrapper! {
    pub struct FontOptions(Boxed<ffi::cairo_font_options_t>);

    match fn {
        copy => |ptr| ffi::cairo_font_options_copy(ptr),
        free => |ptr| ffi::cairo_font_options_destroy(ptr),
    }
}

impl FontOptions {
    pub fn new() -> FontOptions {
        let mut font_options: FontOptions = unsafe {
            from_glib_full(ffi::cairo_font_options_create())
        };
        font_options.ensure_status();
        font_options
    }

    #[doc(hidden)]
    pub fn get_ptr(&self) -> *const cairo_font_options_t {
        self.to_glib_none().0
    }

    #[doc(hidden)]
    pub fn get_ptr_mut(&mut self) -> *mut cairo_font_options_t {
        self.to_glib_none_mut().0
    }

    pub fn ensure_status(&mut self) {
        let status = unsafe {
            ffi::cairo_font_options_status(self.get_ptr_mut())
        };
        status.ensure_valid()
    }

    pub fn merge(&mut self, other: &mut FontOptions) {
        unsafe {
            ffi::cairo_font_options_merge(self.get_ptr_mut(), other.get_ptr_mut())
        }
    }

    pub fn hash(&self) -> u64{
        unsafe {
            ffi::cairo_font_options_hash(self.get_ptr()) as u64
        }
    }

    pub fn set_antialias(&mut self, antialias: Antialias) {
        unsafe {
            ffi::cairo_font_options_set_antialias(self.get_ptr_mut(), antialias)
        }
    }

    pub fn get_antialias(&self) -> Antialias {
        unsafe {
            ffi::cairo_font_options_get_antialias(self.get_ptr())
        }
    }

    pub fn set_subpixel_order(&mut self, order: SubpixelOrder) {
        unsafe {
            ffi::cairo_font_options_set_subpixel_order(self.get_ptr_mut(), order)
        }
    }

    pub fn get_subpixel_order(&self) -> SubpixelOrder {
        unsafe {
            ffi::cairo_font_options_get_subpixel_order(self.get_ptr())
        }
    }

    pub fn set_hint_style(&mut self, hint_style: HintStyle) {
        unsafe {
            ffi::cairo_font_options_set_hint_style(self.get_ptr_mut(), hint_style)
        }
    }

    pub fn get_hint_style(&self) -> HintStyle {
        unsafe {
            ffi::cairo_font_options_get_hint_style(self.get_ptr())
        }
    }

    pub fn set_hint_metrics(&mut self, hint_metrics: HintMetrics) {
        unsafe {
            ffi::cairo_font_options_set_hint_metrics(self.get_ptr_mut(), hint_metrics)
        }
    }

    pub fn get_hint_metrics(&self) -> HintMetrics {
        unsafe {
            ffi::cairo_font_options_get_hint_metrics(self.get_ptr())
        }
    }
}

impl PartialEq for FontOptions {
    fn eq(&self, other: &FontOptions) -> bool {
        unsafe {
            ffi::cairo_font_options_equal(self.get_ptr(), other.get_ptr()).as_bool()
        }
    }
}

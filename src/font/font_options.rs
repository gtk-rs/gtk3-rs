use glib::translate::*;
use std::cmp::PartialEq;
use ffi;

use ffi::enums::{
    Antialias,
    SubpixelOrder,
    HintStyle,
    HintMetrics,
};

glib_wrapper! {
    pub struct FontOptions(Boxed<ffi::cairo_font_options_t>);

    match fn {
        copy => |ptr| {
            let ptr = ffi::cairo_font_options_copy(ptr);
            let status = ffi::cairo_font_options_status(ptr);
            status.ensure_valid();
            ptr
        },
        free => |ptr| ffi::cairo_font_options_destroy(ptr),
    }
}

impl FontOptions {
    pub fn new() -> FontOptions {
        let font_options: FontOptions = unsafe {
            from_glib_full(ffi::cairo_font_options_create())
        };
        font_options.ensure_status();
        font_options
    }

    pub fn ensure_status(&self) {
        let status = unsafe {
            ffi::cairo_font_options_status(mut_override(self.to_glib_none().0))
        };
        status.ensure_valid()
    }

    pub fn merge(&mut self, other: &FontOptions) {
        unsafe {
            ffi::cairo_font_options_merge(self.to_glib_none_mut().0, other.to_glib_none().0)
        }
    }

    pub fn hash(&self) -> u64{
        unsafe {
            ffi::cairo_font_options_hash(self.to_glib_none().0) as u64
        }
    }

    pub fn set_antialias(&mut self, antialias: Antialias) {
        unsafe {
            ffi::cairo_font_options_set_antialias(self.to_glib_none_mut().0, antialias)
        }
    }

    pub fn get_antialias(&self) -> Antialias {
        unsafe {
            ffi::cairo_font_options_get_antialias(self.to_glib_none().0)
        }
    }

    pub fn set_subpixel_order(&mut self, order: SubpixelOrder) {
        unsafe {
            ffi::cairo_font_options_set_subpixel_order(self.to_glib_none_mut().0, order)
        }
    }

    pub fn get_subpixel_order(&self) -> SubpixelOrder {
        unsafe {
            ffi::cairo_font_options_get_subpixel_order(self.to_glib_none().0)
        }
    }

    pub fn set_hint_style(&mut self, hint_style: HintStyle) {
        unsafe {
            ffi::cairo_font_options_set_hint_style(self.to_glib_none_mut().0, hint_style)
        }
    }

    pub fn get_hint_style(&self) -> HintStyle {
        unsafe {
            ffi::cairo_font_options_get_hint_style(self.to_glib_none().0)
        }
    }

    pub fn set_hint_metrics(&mut self, hint_metrics: HintMetrics) {
        unsafe {
            ffi::cairo_font_options_set_hint_metrics(self.to_glib_none_mut().0, hint_metrics)
        }
    }

    pub fn get_hint_metrics(&self) -> HintMetrics {
        unsafe {
            ffi::cairo_font_options_get_hint_metrics(self.to_glib_none().0)
        }
    }
}

impl PartialEq for FontOptions {
    fn eq(&self, other: &FontOptions) -> bool {
        unsafe {
            ffi::cairo_font_options_equal(self.to_glib_none().0, other.to_glib_none().0).as_bool()
        }
    }
}

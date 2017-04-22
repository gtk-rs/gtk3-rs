use glib::translate::*;
use std::clone::Clone;
use std::cmp::PartialEq;
use std::ops::Drop;
use ffi;

use ffi::enums::{
    Antialias,
    SubpixelOrder,
    HintStyle,
    HintMetrics,
};
use ffi::cairo_font_options_t;

pub struct FontOptions(*mut cairo_font_options_t);

impl FontOptions {
    pub fn new() -> FontOptions {
        let font_options = unsafe {
            FontOptions(ffi::cairo_font_options_create())
        };
        font_options.ensure_status();
        font_options
    }

    #[doc(hidden)]
    pub fn get_ptr(&self) -> *mut cairo_font_options_t {
        let FontOptions(ptr) = *self;
        ptr
    }

    pub fn ensure_status(&self) {
        let status = unsafe {
            ffi::cairo_font_options_status(self.get_ptr())
        };
        status.ensure_valid()
    }

    pub fn merge(&mut self, other: &mut FontOptions) {
        unsafe {
            ffi::cairo_font_options_merge(self.get_ptr(), other.get_ptr())
        }
    }

    pub fn hash(&self) -> u64{
        unsafe {
            ffi::cairo_font_options_hash(self.get_ptr()) as u64
        }
    }

    pub fn set_antialias(&self, antialias: Antialias) {
        unsafe {
            ffi::cairo_font_options_set_antialias(self.get_ptr(), antialias)
        }
    }

    pub fn get_antialias(&self) -> Antialias {
        unsafe {
            ffi::cairo_font_options_get_antialias(self.get_ptr())
        }
    }

    pub fn set_subpixel_order(&self, order: SubpixelOrder) {
        unsafe {
            ffi::cairo_font_options_set_subpixel_order(self.get_ptr(), order)
        }
    }

    pub fn get_subpixel_order(&self) -> SubpixelOrder {
        unsafe {
            ffi::cairo_font_options_get_subpixel_order(self.get_ptr())
        }
    }

    pub fn set_hint_style(&self, hint_style: HintStyle) {
        unsafe {
            ffi::cairo_font_options_set_hint_style(self.get_ptr(), hint_style)
        }
    }

    pub fn get_hint_style(&self) -> HintStyle {
        unsafe {
            ffi::cairo_font_options_get_hint_style(self.get_ptr())
        }
    }

    pub fn set_hint_metrics(&self, hint_metrics: HintMetrics) {
        unsafe {
            ffi::cairo_font_options_set_hint_metrics(self.get_ptr(), hint_metrics)
        }
    }

    pub fn get_hint_metrics(&self) -> HintMetrics {
        unsafe {
            ffi::cairo_font_options_get_hint_metrics(self.get_ptr())
        }
    }
}

impl<'a> ToGlibPtr<'a, *const cairo_font_options_t> for &'a FontOptions {
    type Storage = &'a FontOptions;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *const cairo_font_options_t, &'a FontOptions> {
        Stash(self.0, *self)
    }
}

impl FromGlibPtrNone<*const cairo_font_options_t> for FontOptions {
    #[inline]
    unsafe fn from_glib_none(ptr: *const cairo_font_options_t) -> Self {
        let ptr = ffi::cairo_font_options_copy(ptr);
        assert!(!ptr.is_null());
        let tmp = FontOptions(ptr);
        tmp.ensure_status();
        tmp
    }
}

impl FromGlibPtrFull<*mut cairo_font_options_t> for FontOptions {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut cairo_font_options_t) -> Self {
        assert!(!ptr.is_null());
        let tmp = FontOptions(ptr);
        tmp.ensure_status();
        tmp
    }
}

impl PartialEq for FontOptions {
    fn eq(&self, other: &FontOptions) -> bool {
        unsafe {
            ffi::cairo_font_options_equal(self.get_ptr(), other.get_ptr()).as_bool()
        }
    }
}

impl Clone for FontOptions {
    fn clone(&self) -> FontOptions {
        unsafe {
            FontOptions(ffi::cairo_font_options_copy(self.get_ptr()))
        }
    }
}

impl Drop for FontOptions {
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_font_options_destroy(self.get_ptr())
        }
    }
}

#[cfg(feature = "use_glib")]
use glib::translate::*;
#[cfg(feature = "use_glib")]
use glib_ffi;
#[cfg(feature = "use_glib")]
use std::ptr;
#[cfg(feature = "use_glib")]
use std::mem;
use libc::c_char;
use ffi;
use std::ffi::{CString, CStr};

use ffi::enums::{
    FontType,
    FontWeight,
    FontSlant,
};

#[cfg(feature = "use_glib")]
glib_wrapper! {
    pub struct FontFace(Shared<ffi::cairo_font_face_t>);

    match fn {
        ref => |ptr| ffi::cairo_font_face_reference(ptr),
        unref => |ptr| ffi::cairo_font_face_destroy(ptr),
    }
}

#[cfg(not(feature = "use_glib"))]
pub struct FontFace(*mut ffi::cairo_font_face_t);

impl FontFace {
    pub fn toy_create(family: &str, slant: FontSlant, weight: FontWeight) -> FontFace {
        let font_face: FontFace = unsafe {
            FontFace::from_raw_full(ffi::cairo_toy_font_face_create(CString::new(family).unwrap().as_ptr(), slant, weight))
        };
        font_face.ensure_status();
        font_face
    }

    #[cfg(feature = "use_glib")]
    #[doc(hidden)]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_font_face_t) -> FontFace {
        from_glib_full(ptr)
    }

    #[cfg(not(feature = "use_glib"))]
    #[doc(hidden)]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_font_face_t) -> FontFace {
        assert!(!ptr.is_null());
        FontFace(ptr)
    }

    #[cfg(feature = "use_glib")]
    #[doc(hidden)]
    pub unsafe fn from_raw_none(ptr: *mut ffi::cairo_font_face_t) -> FontFace {
        from_glib_none(ptr)
    }

    #[cfg(not(feature = "use_glib"))]
    #[doc(hidden)]
    pub unsafe fn from_raw_none(ptr: *mut ffi::cairo_font_face_t) -> FontFace {
        assert!(!ptr.is_null());
        FontFace(ptr)
    }

    #[cfg(feature = "use_glib")]
    #[doc(hidden)]
    pub fn to_raw_none(&self) -> *mut ffi::cairo_font_face_t {
        self.to_glib_none().0
    }

    #[cfg(not(feature = "use_glib"))]
    #[doc(hidden)]
    pub fn to_raw_none(&self) -> *mut ffi::cairo_font_face_t {
        self.0
    }

    pub fn toy_get_family(&self) -> Option<String> {
        unsafe {    
            to_optional_string(ffi::cairo_toy_font_face_get_family(self.to_raw_none()))
        }
    }

    pub fn toy_get_slant(&self) -> FontSlant {
        unsafe {
            ffi::cairo_toy_font_face_get_slant(self.to_raw_none())
        }
    }

    pub fn toy_get_weight(&self) -> FontWeight {
        unsafe {
            ffi::cairo_toy_font_face_get_weight(self.to_raw_none())
        }
    }

    pub fn ensure_status(&self) {
        let status = unsafe {
            ffi::cairo_font_face_status(self.to_raw_none())
        };
        status.ensure_valid()
    }

    pub fn get_type(&self) -> FontType {
        unsafe {
            ffi::cairo_font_face_get_type(self.to_raw_none())
        }
    }

    pub fn get_reference_count(&self) -> usize {
        unsafe {
            ffi::cairo_font_face_get_reference_count(self.to_raw_none()) as usize
        }
    }
}

unsafe fn to_optional_string(str: *const c_char) -> Option<String> {
    if str.is_null() { 
        None
    } else {
        Some(String::from_utf8_lossy(CStr::from_ptr(str).to_bytes()).into_owned())
    }
}


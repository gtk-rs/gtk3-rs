use glib::translate::*;
use ffi;

use ffi::enums::{
    FontType,
    FontWeight,
    FontSlant,
};
use ffi::cairo_font_face_t;

glib_wrapper! {
    pub struct FontFace(Shared<cairo_font_face_t>);

    match fn {
        ref => |ptr| ffi::cairo_font_face_reference(ptr),
        unref => |ptr| ffi::cairo_font_face_destroy(ptr),
    }
}

impl FontFace {
    #[doc(hidden)]
    pub fn get_ptr(&self) -> *mut cairo_font_face_t {
        self.to_glib_none().0
    }

    pub fn toy_create(family: &str, slant: FontSlant, weight: FontWeight) -> FontFace {
        let font_face: FontFace = unsafe {
            from_glib_full(ffi::cairo_toy_font_face_create(family.to_glib_none().0, slant, weight))
        };
        font_face.ensure_status();
        font_face
    }

    pub fn toy_get_family(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::cairo_toy_font_face_get_family(self.get_ptr()))
        }
    }

    pub fn toy_get_slant(&self) -> FontSlant {
        unsafe {
            ffi::cairo_toy_font_face_get_slant(self.get_ptr())
        }
    }

    pub fn toy_get_weight(&self) -> FontWeight {
        unsafe {
            ffi::cairo_toy_font_face_get_weight(self.get_ptr())
        }
    }

    pub fn ensure_status(&self) {
        let status = unsafe {
            ffi::cairo_font_face_status(self.get_ptr())
        };
        status.ensure_valid()
    }

    pub fn get_type(&self) -> FontType {
        unsafe {
            ffi::cairo_font_face_get_type(self.get_ptr())
        }
    }

    pub fn get_reference_count(&self) -> usize {
        unsafe {
            ffi::cairo_font_face_get_reference_count(self.get_ptr()) as usize
        }
    }
}

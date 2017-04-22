use glib::translate::*;
use ffi;

use ffi::enums::{
    FontType,
    FontWeight,
    FontSlant,
};
use ffi::cairo_font_face_t;

pub struct FontFace(*mut cairo_font_face_t);

impl FontFace {
    #[doc(hidden)]
    pub fn get_ptr(&self) -> *mut cairo_font_face_t {
        let FontFace(ptr) = *self;
        ptr
    }

    pub fn toy_create(family: &str, slant: FontSlant, weight: FontWeight) -> FontFace {
        let font_face = FontFace(
            unsafe {
                ffi::cairo_toy_font_face_create(family.to_glib_none().0, slant, weight)
            }
        );
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

    pub fn reference(&self) -> FontFace {
        unsafe {
            FontFace(ffi::cairo_font_face_reference(self.get_ptr()))
        }
    }
}

impl<'a> ToGlibPtr<'a, *const cairo_font_face_t> for &'a FontFace {
    type Storage = &'a FontFace;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *const cairo_font_face_t, &'a FontFace> {
        Stash(self.0, *self)
    }
}

impl FromGlibPtrNone<*mut cairo_font_face_t> for FontFace {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut cairo_font_face_t) -> Self {
        let ptr = ffi::cairo_font_face_reference(ptr);
        assert!(!ptr.is_null());
        let tmp = FontFace(ptr);
        tmp.ensure_status();
        tmp
    }
}

impl FromGlibPtrFull<*mut cairo_font_face_t> for FontFace {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut cairo_font_face_t) -> Self {
        assert!(!ptr.is_null());
        let tmp = FontFace(ptr);
        tmp.ensure_status();
        tmp
    }
}

impl Drop for FontFace {
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_font_face_destroy(self.get_ptr())
        }
    }
}

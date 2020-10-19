use ffi;
#[cfg(feature = "use_glib")]
use glib::translate::*;
use libc::c_char;
use std::ffi::{CStr, CString};
#[cfg(not(feature = "use_glib"))]
use std::ptr;

use enums::{FontSlant, FontType, FontWeight};

#[cfg(any(feature = "freetype", feature = "dox"))]
use enums::FtSynthesize;

use utils::status_to_result;

#[cfg(feature = "use_glib")]
glib_wrapper! {
    #[derive(Debug)]
    pub struct FontFace(Shared<ffi::cairo_font_face_t>);

    match fn {
        ref => |ptr| ffi::cairo_font_face_reference(ptr),
        unref => |ptr| ffi::cairo_font_face_destroy(ptr),
        get_type => || ffi::gobject::cairo_gobject_font_face_get_type(),
    }
}

#[cfg(not(feature = "use_glib"))]
#[derive(Debug)]
pub struct FontFace(ptr::NonNull<ffi::cairo_font_face_t>);

impl FontFace {
    pub fn toy_create(family: &str, slant: FontSlant, weight: FontWeight) -> FontFace {
        let font_face: FontFace = unsafe {
            let family = CString::new(family).unwrap();
            FontFace::from_raw_full(ffi::cairo_toy_font_face_create(
                family.as_ptr(),
                slant.into(),
                weight.into(),
            ))
        };
        let status = unsafe { ffi::cairo_font_face_status(font_face.to_raw_none()) };
        status_to_result(status).expect("Failed to create a FontFace");
        font_face
    }

    #[cfg(feature = "use_glib")]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_font_face_t) -> FontFace {
        from_glib_full(ptr)
    }

    #[cfg(not(feature = "use_glib"))]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_font_face_t) -> FontFace {
        assert!(!ptr.is_null());
        FontFace(ptr::NonNull::new_unchecked(ptr))
    }

    #[cfg(feature = "use_glib")]
    pub unsafe fn from_raw_none(ptr: *mut ffi::cairo_font_face_t) -> FontFace {
        from_glib_none(ptr)
    }

    #[cfg(not(feature = "use_glib"))]
    pub unsafe fn from_raw_none(ptr: *mut ffi::cairo_font_face_t) -> FontFace {
        assert!(!ptr.is_null());
        FontFace(ptr::NonNull::new_unchecked(ptr))
    }

    #[cfg(feature = "use_glib")]
    pub fn to_raw_none(&self) -> *mut ffi::cairo_font_face_t {
        self.to_glib_none().0
    }

    #[cfg(not(feature = "use_glib"))]
    pub fn to_raw_none(&self) -> *mut ffi::cairo_font_face_t {
        self.0.as_ptr()
    }

    pub fn toy_get_family(&self) -> Option<String> {
        unsafe { to_optional_string(ffi::cairo_toy_font_face_get_family(self.to_raw_none())) }
    }

    pub fn toy_get_slant(&self) -> FontSlant {
        unsafe { FontSlant::from(ffi::cairo_toy_font_face_get_slant(self.to_raw_none())) }
    }

    pub fn toy_get_weight(&self) -> FontWeight {
        unsafe { FontWeight::from(ffi::cairo_toy_font_face_get_weight(self.to_raw_none())) }
    }

    pub fn get_type(&self) -> FontType {
        unsafe { FontType::from(ffi::cairo_font_face_get_type(self.to_raw_none())) }
    }

    pub fn get_reference_count(&self) -> usize {
        unsafe { ffi::cairo_font_face_get_reference_count(self.to_raw_none()) as usize }
    }

    #[cfg(any(feature = "freetype", feature = "dox"))]
    pub fn get_synthesize(&self) -> FtSynthesize {
        unsafe { FtSynthesize::from(ffi::cairo_ft_font_face_get_synthesize(self.to_raw_none())) }
    }

    #[cfg(any(feature = "freetype", feature = "dox"))]
    pub fn set_synthesize(&self, synth_flags: FtSynthesize) {
        unsafe { ffi::cairo_ft_font_face_set_synthesize(self.to_raw_none(), synth_flags.into()) }
    }

    #[cfg(any(feature = "freetype", feature = "dox"))]
    pub fn unset_synthesize(&self, synth_flags: FtSynthesize) {
        unsafe { ffi::cairo_ft_font_face_unset_synthesize(self.to_raw_none(), synth_flags.into()) }
    }

    user_data_methods! {
        ffi::cairo_font_face_get_user_data,
        ffi::cairo_font_face_set_user_data,
    }
}

#[cfg(not(feature = "use_glib"))]
impl Drop for FontFace {
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_font_face_destroy(self.to_raw_none());
        }
    }
}

#[cfg(not(feature = "use_glib"))]
impl Clone for FontFace {
    fn clone(&self) -> FontFace {
        unsafe { FontFace::from_raw_none(self.to_raw_none()) }
    }
}

pub(crate) unsafe fn to_optional_string(str: *const c_char) -> Option<String> {
    if str.is_null() {
        None
    } else {
        Some(String::from_utf8_lossy(CStr::from_ptr(str).to_bytes()).into_owned())
    }
}

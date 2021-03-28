// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Attribute;
use crate::EngineLang;
use crate::EngineShape;
use crate::Font;
use crate::Gravity;
use crate::Language;
use crate::Script;
use glib::translate::*;

#[repr(C)]
pub struct Analysis(ffi::PangoAnalysis);

impl Analysis {
    pub fn shape_engine(&self) -> EngineShape {
        unsafe { from_glib_none(self.0.shape_engine) }
    }

    pub fn lang_engine(&self) -> EngineLang {
        unsafe { from_glib_none(self.0.lang_engine) }
    }

    pub fn font(&self) -> Font {
        unsafe { from_glib_none(self.0.font) }
    }

    pub fn level(&self) -> u8 {
        self.0.level
    }

    pub fn gravity(&self) -> Gravity {
        unsafe { from_glib(self.0.gravity as i32) }
    }

    pub fn flags(&self) -> u8 {
        self.0.flags
    }

    pub fn script(&self) -> Script {
        unsafe { from_glib(self.0.script as i32) }
    }

    pub fn language(&self) -> Language {
        unsafe { from_glib_none(self.0.language) }
    }

    pub fn extra_attrs(&self) -> Vec<Attribute> {
        unsafe { FromGlibPtrContainer::from_glib_none(self.0.extra_attrs) }
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::PangoAnalysis> for Analysis {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::PangoAnalysis, Self> {
        let ptr: *const ffi::PangoAnalysis = &self.0;
        Stash(ptr, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::PangoAnalysis> for Analysis {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::PangoAnalysis, Self> {
        let ptr: *mut ffi::PangoAnalysis = &mut self.0;
        StashMut(ptr, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::PangoAnalysis> for Analysis {
    unsafe fn from_glib_none(ptr: *const ffi::PangoAnalysis) -> Self {
        Analysis(*ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::PangoAnalysis> for Analysis {
    unsafe fn from_glib_none(ptr: *mut ffi::PangoAnalysis) -> Self {
        Analysis(*ptr)
    }
}

// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use pango_sys;
use EngineLang;
use EngineShape;
use Font;
use Gravity;
use Language;
use Script;

#[repr(C)]
pub struct Analysis(pango_sys::PangoAnalysis);

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
        from_glib(self.0.gravity as i32)
    }

    pub fn flags(&self) -> u8 {
        self.0.flags
    }

    pub fn script(&self) -> Script {
        from_glib(self.0.script as i32)
    }

    pub fn language(&self) -> Language {
        unsafe { from_glib_none(self.0.language) }
    }

    /*pub fn extra_attrs(&self) -> Vec<LogAttr> {
        unsafe { from_glib_none_num_as_vec(self.0.extra_attrs) }
    }*/
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const pango_sys::PangoAnalysis> for Analysis {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const pango_sys::PangoAnalysis, Self> {
        let ptr: *const pango_sys::PangoAnalysis = &self.0;
        Stash(ptr, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut pango_sys::PangoAnalysis> for Analysis {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut pango_sys::PangoAnalysis, Self> {
        let ptr: *mut pango_sys::PangoAnalysis = &mut self.0;
        StashMut(ptr, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const pango_sys::PangoAnalysis> for Analysis {
    unsafe fn from_glib_none(ptr: *const pango_sys::PangoAnalysis) -> Self {
        Analysis(*ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut pango_sys::PangoAnalysis> for Analysis {
    unsafe fn from_glib_none(ptr: *mut pango_sys::PangoAnalysis) -> Self {
        Analysis(*ptr)
    }
}

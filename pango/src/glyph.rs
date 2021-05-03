// Take a look at the license at the top of the repository in the LICENSE file.

use crate::GlyphItem;
use crate::GlyphString;
use crate::Item;
use glib::translate::*;

impl GlyphString {
    pub fn num_glyphs(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).num_glyphs }
    }

    pub fn glyph_info(&self) -> Vec<GlyphInfo> {
        if self.num_glyphs() < 0 {
            return Vec::new();
        }
        let num_glyphs = self.num_glyphs() as usize;
        unsafe {
            let glyphs: *mut ffi::PangoGlyphInfo = (*self.to_glib_none().0).glyphs;
            FromGlibContainer::from_glib_none_num(glyphs, num_glyphs)
        }
    }
}

impl GlyphItem {
    pub fn item(&self) -> Item {
        unsafe { from_glib_none((*self.to_glib_none().0).item) }
    }

    pub fn glyph_string(&self) -> GlyphString {
        unsafe { from_glib_none((*self.to_glib_none().0).glyphs) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GlyphInfo(*mut ffi::PangoGlyphInfo);

impl GlyphInfo {
    pub fn glyph(&self) -> u32 {
        unsafe { (*self.0).glyph }
    }

    pub fn geometry(&self) -> &GlyphGeometry {
        unsafe { &*(&((*self.0).geometry) as *const _ as *const GlyphGeometry) }
    }
}

impl FromGlibContainerAsVec<*mut ffi::PangoGlyphInfo, *mut ffi::PangoGlyphInfo> for GlyphInfo {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut ffi::PangoGlyphInfo, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }
        let mut res = Vec::with_capacity(num);
        for x in 0..num {
            res.push(from_glib_none(ptr.add(x)));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(
        ptr: *mut ffi::PangoGlyphInfo,
        num: usize,
    ) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib::ffi::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut ffi::PangoGlyphInfo, num: usize) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, num)
    }
}
impl FromGlibContainerAsVec<*mut ffi::PangoGlyphInfo, *const ffi::PangoGlyphInfo> for GlyphInfo {
    unsafe fn from_glib_none_num_as_vec(ptr: *const ffi::PangoGlyphInfo, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }
        let mut res = Vec::with_capacity(num);
        for x in 0..num {
            res.push(from_glib_none(ptr.add(x)));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(
        ptr: *const ffi::PangoGlyphInfo,
        num: usize,
    ) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib::ffi::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *const ffi::PangoGlyphInfo, num: usize) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, num)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut ffi::PangoGlyphInfo> for &'a GlyphInfo {
    type Storage = &'a GlyphInfo;

    fn to_glib_none(&self) -> Stash<'a, *mut ffi::PangoGlyphInfo, Self> {
        Stash(self.0, *self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::PangoGlyphInfo> for GlyphInfo {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::PangoGlyphInfo, Self> {
        StashMut(self.0, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::PangoGlyphInfo> for GlyphInfo {
    unsafe fn from_glib_none(ptr: *mut ffi::PangoGlyphInfo) -> Self {
        assert!(!ptr.is_null());
        Self(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut ffi::PangoGlyphInfo> for GlyphInfo {
    unsafe fn from_glib_full(ptr: *mut ffi::PangoGlyphInfo) -> Self {
        assert!(!ptr.is_null());
        Self(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::PangoGlyphInfo> for GlyphInfo {
    unsafe fn from_glib_none(ptr: *const ffi::PangoGlyphInfo) -> Self {
        assert!(!ptr.is_null());
        Self(ptr as *mut _)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*const ffi::PangoGlyphInfo> for GlyphInfo {
    unsafe fn from_glib_full(ptr: *const ffi::PangoGlyphInfo) -> Self {
        assert!(!ptr.is_null());
        Self(ptr as *mut _)
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct GlyphGeometry(ffi::PangoGlyphGeometry);

impl GlyphGeometry {
    pub fn width(&self) -> i32 {
        self.0.width
    }

    pub fn x_offset(&self) -> i32 {
        self.0.x_offset
    }

    pub fn y_offset(&self) -> i32 {
        self.0.y_offset
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::PangoGlyphGeometry> for GlyphGeometry {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::PangoGlyphGeometry, Self> {
        let ptr: *const ffi::PangoGlyphGeometry = &self.0;
        Stash(ptr, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::PangoGlyphGeometry> for GlyphGeometry {
    unsafe fn from_glib_none(ptr: *const ffi::PangoGlyphGeometry) -> Self {
        GlyphGeometry(*ptr)
    }
}

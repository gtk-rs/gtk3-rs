use glib::translate::*;
use pango_sys;
use GlyphItem;
use GlyphString;
use Item;

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
            let glyphs: *mut pango_sys::PangoGlyphInfo = (*self.to_glib_none().0).glyphs;
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
pub struct GlyphInfo(*mut pango_sys::PangoGlyphInfo);

impl GlyphInfo {
    pub fn glyph(&self) -> u32 {
        unsafe { (*self.0).glyph }
    }

    pub fn geometry(&self) -> &GlyphGeometry {
        unsafe { &*(&((*self.0).geometry) as *const _ as *const GlyphGeometry) }
    }
}

impl FromGlibContainerAsVec<*mut pango_sys::PangoGlyphInfo, *mut pango_sys::PangoGlyphInfo>
    for GlyphInfo
{
    unsafe fn from_glib_none_num_as_vec(
        ptr: *mut pango_sys::PangoGlyphInfo,
        num: usize,
    ) -> Vec<Self> {
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
        ptr: *mut pango_sys::PangoGlyphInfo,
        num: usize,
    ) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib_sys::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(
        ptr: *mut pango_sys::PangoGlyphInfo,
        num: usize,
    ) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, num)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut pango_sys::PangoGlyphInfo> for &'a GlyphInfo {
    type Storage = &'a GlyphInfo;

    fn to_glib_none(&self) -> Stash<'a, *mut pango_sys::PangoGlyphInfo, Self> {
        Stash(self.0, *self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut pango_sys::PangoGlyphInfo> for GlyphInfo {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut pango_sys::PangoGlyphInfo, Self> {
        StashMut(self.0, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut pango_sys::PangoGlyphInfo> for GlyphInfo {
    unsafe fn from_glib_none(ptr: *mut pango_sys::PangoGlyphInfo) -> Self {
        assert!(!ptr.is_null());
        GlyphInfo(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut pango_sys::PangoGlyphInfo> for GlyphInfo {
    unsafe fn from_glib_full(ptr: *mut pango_sys::PangoGlyphInfo) -> Self {
        assert!(!ptr.is_null());
        GlyphInfo(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const pango_sys::PangoGlyphInfo> for GlyphInfo {
    unsafe fn from_glib_none(ptr: *const pango_sys::PangoGlyphInfo) -> Self {
        assert!(!ptr.is_null());
        GlyphInfo(ptr as *mut _)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*const pango_sys::PangoGlyphInfo> for GlyphInfo {
    unsafe fn from_glib_full(ptr: *const pango_sys::PangoGlyphInfo) -> Self {
        assert!(!ptr.is_null());
        GlyphInfo(ptr as *mut _)
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct GlyphGeometry(pango_sys::PangoGlyphGeometry);

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
impl<'a> ToGlibPtr<'a, *const pango_sys::PangoGlyphGeometry> for GlyphGeometry {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const pango_sys::PangoGlyphGeometry, Self> {
        let ptr: *const pango_sys::PangoGlyphGeometry = &self.0;
        Stash(ptr, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const pango_sys::PangoGlyphGeometry> for GlyphGeometry {
    unsafe fn from_glib_none(ptr: *const pango_sys::PangoGlyphGeometry) -> Self {
        GlyphGeometry(*ptr)
    }
}

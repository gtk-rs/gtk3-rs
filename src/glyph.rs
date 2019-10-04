use glib::translate::*;
use pango_sys;
use GlyphItem;
use Item;
use GlyphString;

impl GlyphString {

    pub fn num_glyphs(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).num_glyphs }
    }

    pub fn glyph_info(&self) -> Vec<GlyphInfo> {

        let num_glyphs = self.num_glyphs();
        let mut ret = Vec::new();

        unsafe {
            let glyphs: *mut pango_sys::PangoGlyphInfo = (*self.to_glib_none().0).glyphs;
            if num_glyphs > 0 {
                for x in 0..num_glyphs {
                    ret.push(from_glib_none(
                        glyphs.offset(x as isize),
                    ));
                }
            }
            ret
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
#[repr(C)]
pub struct GlyphInfo(*mut pango_sys::PangoGlyphInfo);

impl GlyphInfo {
    pub fn glyph(&self) -> u32 {
        unsafe { (*self.to_glib_none().0).glyph }
    }

    pub fn geometry(&self) -> &GlyphGeometry {
        unsafe { &*(&((*self.to_glib_none().0).geometry) as *const _ as *const GlyphGeometry) }
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

#[repr(C)]
pub struct GlyphGeometry(pango_sys::PangoGlyphGeometry);

impl GlyphGeometry {
    pub fn width(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).width }
    }

    pub fn x_offset(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).x_offset }
    }

    pub fn y_offset(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).y_offset }
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
    unsafe fn from_glib_none(ptr: *const  pango_sys::PangoGlyphGeometry) -> Self {
        GlyphGeometry(*ptr)
    }
}


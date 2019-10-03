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
        unsafe { Vec::new(std::slice::from_raw_parts((*self.to_glib_none().0).glyphs, self.num_glyphs)) }
    }
}

impl GlyphItem {
    pub fn item(&self) -> Item {
        unsafe { (*self.to_glib_none().0).item) }
    }

    pub fn glyph_string(&self) -> GlyphString {
        unsafe { (*self.to_glib_none().0).glyphs }
    }
}

#[repr(C)]
pub struct GlyphInfo(pango_sys::PangoGlyphInfo);

impl GlyphInfo {
    pub fn glyph(&self) -> u32 {
        unsafe { (*self.to_glib_none().0).glyph }
    }
    
    pub fn geometry(&self) -> GlyphGeometry {
        unsafe { (*self.to_glib_none().0).geometry }
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const pango_sys::PangoGlyphInfo> for GlyphInfo {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const pango_sys::PangoGlyphInfo, Self> {
        let ptr: *const pango_sys::PangoGlyphInfo = &self.0;
        Stash(ptr, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut pango_sys::PangoGlyphInfo> for GlyphInfo {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut pango_sys::PangoGlyphInfo, Self> {
        let ptr: *mut pango_sys::PangoGlyphInfo = &mut self.0;
        StashMut(ptr, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const pango_sys::PangoGlyphInfo> for GlyphInfo {
    unsafe fn from_glib_none(ptr: *const pango_sys::PangoGlyphInfo) -> Self {
        GlyphInfo(*ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut pango_sys::PangoGlyphInfo> for GlyphInfo {
    unsafe fn from_glib_none(ptr: *mut pango_sys::PangoGlyphInfo) -> Self {
        GlyphInfo(*ptr)
    }
}


#[repr(C)]
pub struct GlyphGeometry(pango_sys::PangoGlyphGeometry);

impl GlyphGeometry {
    pub fn width(&self) -> f32 {
        unsafe { (*self.to_glib_none().0).width }
    }

    pub fn x_offset(&self) -> f32 {
        unsafe { (*self.to_glib_none().0).x_offset }
    }

    pub fn y_offset(&self) -> f32 {
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
impl<'a> ToGlibPtrMut<'a, *mut pango_sys::PangoGlyphGeometry> for GlyphGeometry {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut pango_sys::PangoGlyphGeometry, Self> {
        let ptr: *mut pango_sys::PangoGlyphGeometry = &mut self.0;
        StashMut(ptr, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const pango_sys::PangoGlyphGeometry> for GlyphGeometry {
    unsafe fn from_glib_none(ptr: *const pango_sys::PangoGlyphGeometry) -> Self {
        GlyphGeometry(*ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut pango_sys::PangoGlyphGeometry> for GlyphInfo {
    unsafe fn from_glib_none(ptr: *mut pango_sys::PangoGlyphGeometry) -> Self {
        GlyphGeometry(*ptr)
    }
}

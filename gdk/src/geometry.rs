// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Gravity;
use glib::translate::*;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[doc(alias = "GdkGeometry")]
pub struct Geometry {
    pub min_width: i32,
    pub min_height: i32,
    pub max_width: i32,
    pub max_height: i32,
    pub base_width: i32,
    pub base_height: i32,
    pub width_inc: i32,
    pub height_inc: i32,
    pub min_aspect: f64,
    pub max_aspect: f64,
    pub win_gravity: Gravity,
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GdkGeometry> for Geometry {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GdkGeometry, Self> {
        let ptr: *const Geometry = &*self;
        Stash(ptr as *const ffi::GdkGeometry, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GdkGeometry> for Geometry {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GdkGeometry, Self> {
        let ptr: *mut Geometry = &mut *self;
        StashMut(ptr as *mut ffi::GdkGeometry, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::GdkGeometry> for Geometry {
    unsafe fn from_glib_none(ptr: *const ffi::GdkGeometry) -> Self {
        *(ptr as *const Geometry)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::GdkGeometry> for Geometry {
    unsafe fn from_glib_none(ptr: *mut ffi::GdkGeometry) -> Self {
        *(ptr as *mut Geometry)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut ffi::GdkGeometry> for Geometry {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GdkGeometry) -> Self {
        let geom = *(ptr as *mut Geometry);
        glib::ffi::g_free(ptr as *mut _);
        geom
    }
}

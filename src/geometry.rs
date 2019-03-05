// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::translate::*;
use glib_sys;
use Gravity;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
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
impl<'a> ToGlibPtr<'a, *const gdk_sys::GdkGeometry> for Geometry {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const gdk_sys::GdkGeometry, Self> {
        let ptr: *const Geometry = &*self;
        Stash(ptr as *const gdk_sys::GdkGeometry, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut gdk_sys::GdkGeometry> for Geometry {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut gdk_sys::GdkGeometry, Self> {
        let ptr: *mut Geometry = &mut *self;
        StashMut(ptr as *mut gdk_sys::GdkGeometry, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const gdk_sys::GdkGeometry> for Geometry {
    unsafe fn from_glib_none(ptr: *const gdk_sys::GdkGeometry) -> Self {
        *(ptr as *const Geometry)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut gdk_sys::GdkGeometry> for Geometry {
    unsafe fn from_glib_none(ptr: *mut gdk_sys::GdkGeometry) -> Self {
        *(ptr as *mut Geometry)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut gdk_sys::GdkGeometry> for Geometry {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut gdk_sys::GdkGeometry) -> Self {
        let geom = *(ptr as *mut Geometry);
        glib_sys::g_free(ptr as *mut _);
        geom
    }
}

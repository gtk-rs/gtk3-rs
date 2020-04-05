// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::translate::*;
use glib_sys;
use std::mem;

#[derive(Clone)]
#[repr(C)]
pub struct TimeCoord {
    pub time: u32,
    pub axes: [f64; gdk_sys::GDK_MAX_TIMECOORD_AXES as usize],
}

#[doc(hidden)]
impl Uninitialized for TimeCoord {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const gdk_sys::GdkTimeCoord> for TimeCoord {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const gdk_sys::GdkTimeCoord, Self> {
        let ptr: *const TimeCoord = &*self;
        Stash(ptr as *const gdk_sys::GdkTimeCoord, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut gdk_sys::GdkTimeCoord> for TimeCoord {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut gdk_sys::GdkTimeCoord, Self> {
        let ptr: *mut TimeCoord = &mut *self;
        StashMut(ptr as *mut gdk_sys::GdkTimeCoord, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const gdk_sys::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_none(ptr: *const gdk_sys::GdkTimeCoord) -> Self {
        (*(ptr as *const TimeCoord)).clone()
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut gdk_sys::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_none(ptr: *mut gdk_sys::GdkTimeCoord) -> Self {
        (*(ptr as *mut TimeCoord)).clone()
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const gdk_sys::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_borrow(
        ptr: *const gdk_sys::GdkTimeCoord,
    ) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new((*(ptr as *const TimeCoord)).clone())
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut gdk_sys::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_borrow(ptr: *mut gdk_sys::GdkTimeCoord) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new((*(ptr as *mut TimeCoord)).clone())
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut gdk_sys::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_full(ptr: *mut gdk_sys::GdkTimeCoord) -> Self {
        let time_coord = (*(ptr as *mut TimeCoord)).clone();
        glib_sys::g_free(ptr as *mut _);
        time_coord
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*const gdk_sys::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_full(ptr: *const gdk_sys::GdkTimeCoord) -> Self {
        let time_coord = (*(ptr as *const TimeCoord)).clone();
        glib_sys::g_free(ptr as *mut _);
        time_coord
    }
}

impl FromGlibContainerAsVec<gdk_sys::GdkTimeCoord, *mut gdk_sys::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut gdk_sys::GdkTimeCoord, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push((*(ptr.offset(i as isize) as *mut TimeCoord)).clone());
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(
        ptr: *mut gdk_sys::GdkTimeCoord,
        num: usize,
    ) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib_sys::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut gdk_sys::GdkTimeCoord, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push((*(ptr.offset(i as isize) as *mut TimeCoord)).clone());
        }
        glib_sys::g_free(ptr as *mut _);
        res
    }
}

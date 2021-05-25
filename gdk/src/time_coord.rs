// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::mem;

/// A [TimeCoord](crate::TimeCoord) stores a single event in a motion history.
#[derive(Clone)]
#[repr(C)]
pub struct TimeCoord {
    pub time: u32,
    pub axes: [f64; ffi::GDK_MAX_TIMECOORD_AXES as usize],
}

#[doc(hidden)]
impl Uninitialized for TimeCoord {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GdkTimeCoord> for TimeCoord {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GdkTimeCoord, Self> {
        let ptr: *const TimeCoord = &*self;
        Stash(ptr as *const ffi::GdkTimeCoord, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GdkTimeCoord> for TimeCoord {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GdkTimeCoord, Self> {
        let ptr: *mut TimeCoord = &mut *self;
        StashMut(ptr as *mut ffi::GdkTimeCoord, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_none(ptr: *const ffi::GdkTimeCoord) -> Self {
        (*(ptr as *const TimeCoord)).clone()
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_none(ptr: *mut ffi::GdkTimeCoord) -> Self {
        (*(ptr as *mut TimeCoord)).clone()
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_borrow(ptr: *const ffi::GdkTimeCoord) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new((*(ptr as *const TimeCoord)).clone())
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_borrow(ptr: *mut ffi::GdkTimeCoord) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new((*(ptr as *mut TimeCoord)).clone())
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut ffi::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_full(ptr: *mut ffi::GdkTimeCoord) -> Self {
        let time_coord = (*(ptr as *mut TimeCoord)).clone();
        glib::ffi::g_free(ptr as *mut _);
        time_coord
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*const ffi::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_full(ptr: *const ffi::GdkTimeCoord) -> Self {
        let time_coord = (*(ptr as *const TimeCoord)).clone();
        glib::ffi::g_free(ptr as *mut _);
        time_coord
    }
}

impl FromGlibContainerAsVec<ffi::GdkTimeCoord, *mut ffi::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut ffi::GdkTimeCoord, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push((*(ptr.add(i) as *mut TimeCoord)).clone());
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut ffi::GdkTimeCoord, num: usize) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib::ffi::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut ffi::GdkTimeCoord, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push((*(ptr.add(i) as *mut TimeCoord)).clone());
        }
        glib::ffi::g_free(ptr as *mut _);
        res
    }
}

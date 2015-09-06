// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Frame timings — Object holding timing information for a single frame

use glib::translate::*;
use ffi;

pub struct FrameTimings(*mut ffi::GdkFrameTimings);

impl FrameTimings {
    #[inline]
    fn add_ref(&self) {
        unsafe { ffi::gdk_frame_timings_ref(self.0); }
    }

    #[inline]
    fn unref(&self) {
        unsafe { ffi::gdk_frame_timings_unref(self.0) }
    }

    pub fn get_frame_counter(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_frame_counter(self.0) }
    }

    pub fn get_complete(&self) -> bool {
        unsafe { from_glib(ffi::gdk_frame_timings_get_complete(self.0)) }
    }

    pub fn get_frame_time(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_frame_time(self.0) }
    }

    pub fn get_presentation_time(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_presentation_time(self.0) }
    }

    pub fn get_refresh_interval(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_refresh_interval(self.0) }
    }

    pub fn get_predicted_presentation_time(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_predicted_presentation_time(self.0) }
    }
}

impl Clone for FrameTimings {
    #[inline]
    fn clone(&self) -> FrameTimings {
        self.add_ref();
        FrameTimings(self.0)
    }
}

impl Drop for FrameTimings {
    #[inline]
    fn drop(&mut self) {
        self.unref();
    }
}

impl<'a> ToGlibPtr<'a, *mut ffi::GdkFrameTimings> for &'a FrameTimings {
    type Storage = &'a FrameTimings;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut ffi::GdkFrameTimings, &'a FrameTimings> {
        Stash(self.0, *self)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut ffi::GdkFrameTimings {
        self.add_ref();
        self.0
    }
}

impl FromGlibPtr<*mut ffi::GdkFrameTimings> for FrameTimings {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GdkFrameTimings) -> FrameTimings {
        assert!(!ptr.is_null());
        let res = FrameTimings(ptr);
        res.add_ref();
        res
    }

    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GdkFrameTimings) -> FrameTimings {
        assert!(!ptr.is_null());
        FrameTimings(ptr)
    }
}

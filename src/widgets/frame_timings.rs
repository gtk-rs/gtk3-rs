// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Frame timings — Object holding timing information for a single frame

use ::ffi;

#[repr(C)]
pub struct FrameTimings {
    pointer: *mut ffi::C_GdkFrameTimings
}

impl FrameTimings {
    // FIXME: should not be handled by user
    // Since 3.8
    pub fn _ref(&self) -> Option<FrameTimings> {
        let tmp = unsafe { ffi::gdk_frame_timings_ref(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(FrameTimings {
                pointer: tmp
            })
        }
    }

    // FIXME: should not be handled by user
    // Since 3.8
    pub fn unref(&self) {
        unsafe { ffi::gdk_frame_timings_unref(self.pointer) }
    }

    // Since 3.8
    pub fn get_frame_counter(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_frame_counter(self.pointer) }
    }

    // Since 3.8
    pub fn get_complete(&self) -> bool {
        unsafe { ::glib::to_bool(ffi::gdk_frame_timings_get_complete(self.pointer)) }
    }

    pub fn get_frame_time(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_frame_time(self.pointer) }
    }

    // Since 3.8
    pub fn get_presentation_time(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_presentation_time(self.pointer) }
    }

    // Since 3.8
    pub fn get_refresh_interval(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_refresh_interval(self.pointer) }
    }

    // Since 3.8
    pub fn get_predicted_presentation_time(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_predicted_presentation_time(self.pointer) }
    }
}

impl_GObjectFunctions!(FrameTimings, C_GdkFrameTimings);
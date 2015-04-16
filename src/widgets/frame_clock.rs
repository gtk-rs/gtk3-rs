// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Frame clock — Frame clock syncs painting to a window or display

use ffi;

#[repr(C)]
pub struct FrameClock {
    pointer: *mut ffi::C_GdkFrameClock
}

impl FrameClock {
    pub fn get_frame_time(&self) -> i64 {
        unsafe { ffi::gdk_frame_clock_get_frame_time(self.pointer) }
    }

    pub fn request_phase(&self, phase: ::FrameClockPhase) {
        unsafe { ffi::gdk_frame_clock_request_phase(self.pointer, phase) }
    }

    pub fn begin_updating(&self) {
        unsafe { ffi::gdk_frame_clock_begin_updating(self.pointer) }
    }

    pub fn end_updating(&self) {
        unsafe { ffi::gdk_frame_clock_end_updating(self.pointer) }
    }

    pub fn get_frame_counter(&self) -> i64 {
        unsafe { ffi::gdk_frame_clock_get_frame_counter(self.pointer) }
    }

    pub fn get_history_start(&self) -> i64 {
        unsafe { ffi::gdk_frame_clock_get_history_start(self.pointer) }
    }

    pub fn get_timings(&self, frame_counter: i64) -> Option<::FrameTimings> {
        let tmp = unsafe { ffi::gdk_frame_clock_get_timings(self.pointer, frame_counter) };

        if tmp.is_null() {
            None
        } else {
            Some(::FrameTimings::wrap_pointer(tmp))
        }
    }

    pub fn get_current_timings(&self) -> Option<::FrameTimings> {
        let tmp = unsafe { ffi::gdk_frame_clock_get_current_timings(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(::FrameTimings::wrap_pointer(tmp))
        }
    }

    pub fn get_refresh_info(&self, base_time: i64, refresh_interval_return: &mut i64, presentation_time_return: &mut i64) {
        unsafe { ffi::gdk_frame_clock_get_refresh_info(self.pointer, base_time, refresh_interval_return, presentation_time_return) }
    }
}

impl_GObjectFunctions!(FrameClock, C_GdkFrameClock);
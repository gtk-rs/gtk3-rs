// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use frame_timings::FrameTimings;
use ffi;

pub type Phase = ffi::GdkFrameClockPhase;

glib_wrapper! {
    pub struct FrameClock(Object<ffi::GdkFrameClock>);

    match fn {
        get_type => || ffi::gdk_frame_clock_get_type(),
    }
}

impl FrameClock {
    pub fn get_frame_time(&self) -> i64 {
        unsafe { ffi::gdk_frame_clock_get_frame_time(self.to_glib_none().0) }
    }

    pub fn request_phase(&self, phase: Phase) {
        unsafe { ffi::gdk_frame_clock_request_phase(self.to_glib_none().0, phase) }
    }

    pub fn begin_updating(&self) {
        unsafe { ffi::gdk_frame_clock_begin_updating(self.to_glib_none().0) }
    }

    pub fn end_updating(&self) {
        unsafe { ffi::gdk_frame_clock_end_updating(self.to_glib_none().0) }
    }

    pub fn get_frame_counter(&self) -> i64 {
        unsafe { ffi::gdk_frame_clock_get_frame_counter(self.to_glib_none().0) }
    }

    pub fn get_history_start(&self) -> i64 {
        unsafe { ffi::gdk_frame_clock_get_history_start(self.to_glib_none().0) }
    }

    pub fn get_timings(&self, frame_counter: i64) -> Option<FrameTimings> {
        unsafe {
            from_glib_full(ffi::gdk_frame_clock_get_timings(self.to_glib_none().0, frame_counter))
        }
    }

    pub fn get_current_timings(&self) -> Option<FrameTimings> {
        unsafe {
            from_glib_full(
                ffi::gdk_frame_clock_get_current_timings(self.to_glib_none().0))
        }
    }

    pub fn get_refresh_info(&self, base_time: i64) -> (i64, i64) {
        unsafe {
            let mut refresh_interval = 0;
            let mut presentation_time = 0;
            ffi::gdk_frame_clock_get_refresh_info(self.to_glib_none().0, base_time,
                                                  &mut refresh_interval, &mut presentation_time);
            (refresh_interval, presentation_time)
        }
    }
}

// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::translate::*;
use std::num::NonZeroU64;
use FrameTimings;

impl FrameTimings {
    pub fn get_predicted_presentation_time(&self) -> Option<NonZeroU64> {
        let predicted_presentation_time = unsafe {
            gdk_sys::gdk_frame_timings_get_predicted_presentation_time(self.to_glib_none().0)
        };
        // assuming presentation time is always positive
        assert!(predicted_presentation_time >= 0);
        // `0` means the value is not available
        NonZeroU64::new(predicted_presentation_time as u64)
    }

    pub fn get_presentation_time(&self) -> Option<NonZeroU64> {
        let presentation_time =
            unsafe { gdk_sys::gdk_frame_timings_get_presentation_time(self.to_glib_none().0) };
        // assuming presentation time is always positive
        assert!(presentation_time >= 0);
        // `0` means the value is not available
        NonZeroU64::new(presentation_time as u64)
    }

    pub fn get_refresh_interval(&self) -> Option<NonZeroU64> {
        let refresh_interval =
            unsafe { gdk_sys::gdk_frame_timings_get_refresh_interval(self.to_glib_none().0) };
        // assuming refresh interval is always positive
        assert!(refresh_interval >= 0);
        // `0` means the value is not available
        NonZeroU64::new(refresh_interval as u64)
    }
}

// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi;

glib_wrapper! {
    pub struct FrameTimings(Shared<ffi::GdkFrameTimings>);

    match fn {
        ref => |ptr| ffi::gdk_frame_timings_ref(ptr),
        unref => |ptr| ffi::gdk_frame_timings_unref(ptr),
    }
}

impl FrameTimings {
    pub fn get_frame_counter(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_frame_counter(self.to_glib_none().0) }
    }

    pub fn get_complete(&self) -> bool {
        unsafe { from_glib(ffi::gdk_frame_timings_get_complete(self.to_glib_none().0)) }
    }

    pub fn get_frame_time(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_frame_time(self.to_glib_none().0) }
    }

    pub fn get_presentation_time(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_presentation_time(self.to_glib_none().0) }
    }

    pub fn get_refresh_interval(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_refresh_interval(self.to_glib_none().0) }
    }

    pub fn get_predicted_presentation_time(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_predicted_presentation_time(self.to_glib_none().0) }
    }
}

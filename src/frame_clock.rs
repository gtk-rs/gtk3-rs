// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::translate::*;
use FrameClock;

impl FrameClock {
    pub fn get_refresh_info(&self, base_time: i64) -> (i64, i64) {
        unsafe {
            let mut refresh_interval = 0;
            let mut presentation_time = 0;
            gdk_sys::gdk_frame_clock_get_refresh_info(
                self.to_glib_none().0,
                base_time,
                &mut refresh_interval,
                &mut presentation_time,
            );
            (refresh_interval, presentation_time)
        }
    }
}

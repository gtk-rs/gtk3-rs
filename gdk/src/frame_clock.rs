// Take a look at the license at the top of the repository in the LICENSE file.

use crate::FrameClock;
use glib::translate::*;

impl FrameClock {
    pub fn get_refresh_info(&self, base_time: i64) -> (i64, i64) {
        unsafe {
            let mut refresh_interval = 0;
            let mut presentation_time = 0;
            ffi::gdk_frame_clock_get_refresh_info(
                self.to_glib_none().0,
                base_time,
                &mut refresh_interval,
                &mut presentation_time,
            );
            (refresh_interval, presentation_time)
        }
    }
}

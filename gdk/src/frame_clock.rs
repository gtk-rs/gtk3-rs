// Take a look at the license at the top of the repository in the LICENSE file.

use crate::FrameClock;
use glib::translate::*;

impl FrameClock {
    /// Using the frame history stored in the frame clock, finds the last
    /// known presentation time and refresh interval, and assuming that
    /// presentation times are separated by the refresh interval,
    /// predicts a presentation time that is a multiple of the refresh
    /// interval after the last presentation time, and later than `base_time`.
    /// ## `base_time`
    /// base time for determining a presentaton time
    ///
    /// # Returns
    ///
    ///
    /// ## `refresh_interval_return`
    /// a location to store the
    /// determined refresh interval, or [`None`]. A default refresh interval of
    /// 1/60th of a second will be stored if no history is present.
    ///
    /// ## `presentation_time_return`
    /// a location to store the next
    ///  candidate presentation time after the given base time.
    ///  0 will be will be stored if no history is present.
    #[doc(alias = "gdk_frame_clock_get_refresh_info")]
    #[doc(alias = "get_refresh_info")]
    pub fn refresh_info(&self, base_time: i64) -> (i64, i64) {
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

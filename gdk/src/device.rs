// Take a look at the license at the top of the repository in the LICENSE file.

use crate::AxisUse;
use crate::Device;
use crate::TimeCoord;
use crate::Window;
use glib::object::IsA;
use glib::translate::*;

use std::mem;
use std::ptr;

impl Device {
    #[doc(alias = "gdk_device_get_axis")]
    #[doc(alias = "get_axis")]
    pub fn is_axis(&self, axes: &mut [f64], use_: AxisUse, value: &mut f64) -> bool {
        unsafe {
            from_glib(ffi::gdk_device_get_axis(
                self.to_glib_none().0,
                axes.as_mut_ptr(),
                use_.into_glib(),
                value,
            ))
        }
    }

    /// Obtains the motion history for a pointer device; given a starting and
    /// ending timestamp, return all events in the motion history for
    /// the device in the given range of time. Some windowing systems
    /// do not support motion history, in which case, [`false`] will
    /// be returned. (This is not distinguishable from the case where
    /// motion history is supported and no events were found.)
    ///
    /// Note that there is also [Window::set_event_compression](crate::Window::set_event_compression) to get
    /// more motion events delivered directly, independent of the windowing
    /// system.
    /// ## `window`
    /// the window with respect to which which the event coordinates will be reported
    /// ## `start`
    /// starting timestamp for range of events to return
    /// ## `stop`
    /// ending timestamp for the range of events to return
    ///
    /// # Returns
    ///
    /// [`true`] if the windowing system supports motion history and
    ///  at least one event was found.
    ///
    /// ## `events`
    ///
    ///  location to store a newly-allocated array of [TimeCoord](crate::TimeCoord), or
    ///  [`None`]
    #[doc(alias = "gdk_device_get_history")]
    #[doc(alias = "get_history")]
    pub fn history<P: IsA<Window>>(&self, window: &P, start: u32, stop: u32) -> Vec<TimeCoord> {
        unsafe {
            let mut events = ptr::null_mut();
            let mut n_events = mem::MaybeUninit::uninit();
            let ret: bool = from_glib(ffi::gdk_device_get_history(
                self.to_glib_none().0,
                window.as_ref().to_glib_none().0,
                start,
                stop,
                &mut events,
                n_events.as_mut_ptr(),
            ));
            if !ret {
                return Vec::new();
            }
            let n_events = n_events.assume_init() as usize;
            let mut r_events = Vec::with_capacity(n_events);
            for i in 0..n_events {
                r_events.push((*(events.add(i) as *mut TimeCoord)).clone());
            }
            ffi::gdk_device_free_history(events, n_events as _);
            r_events
        }
    }
}

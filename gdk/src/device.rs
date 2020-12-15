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
    pub fn get_axis(&self, axes: &mut [f64], use_: AxisUse, value: &mut f64) -> bool {
        unsafe {
            from_glib(ffi::gdk_device_get_axis(
                self.to_glib_none().0,
                axes.as_mut_ptr(),
                use_.to_glib(),
                value,
            ))
        }
    }

    pub fn get_history<P: IsA<Window>>(&self, window: &P, start: u32, stop: u32) -> Vec<TimeCoord> {
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

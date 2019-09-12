// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::object::IsA;
use glib::translate::*;
use AxisUse;
use Device;
use TimeCoord;
use Window;

use std::mem;
use std::ptr;

impl Device {
    pub fn get_axis(&self, axes: &mut [f64], use_: AxisUse, value: &mut f64) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_device_get_axis(
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
            let ret: bool = from_glib(gdk_sys::gdk_device_get_history(
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
                r_events.push((*(events.offset(i as isize) as *mut TimeCoord)).clone());
            }
            gdk_sys::gdk_device_free_history(events, n_events as _);
            r_events
        }
    }
}

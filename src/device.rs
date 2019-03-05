// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::translate::*;
use AxisUse;
use Device;

impl Device {
    pub fn get_axis(&self, axes: &mut [f64], use_: AxisUse, value: &mut f64) -> bool {
        unsafe { from_glib(gdk_sys::gdk_device_get_axis(self.to_glib_none().0, axes.as_mut_ptr(), use_.to_glib(), value)) }
    }
}

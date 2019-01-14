// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::object::IsA;
use glib::translate::*;
use ffi;
use AxisUse;
use Device;

pub trait DeviceExtManual: 'static {
    fn get_axis(&self, axes: &mut [f64], use_: AxisUse, value: &mut f64) -> bool;
}

impl<O: IsA<Device>> DeviceExtManual for O {
    fn get_axis(&self, axes: &mut [f64], use_: AxisUse, value: &mut f64) -> bool {
        unsafe { from_glib(ffi::gdk_device_get_axis(self.as_ref().to_glib_none().0, axes.as_mut_ptr(), use_.to_glib(), value)) }
    }
}

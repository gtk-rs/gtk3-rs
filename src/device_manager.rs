// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use device::Device;
use display::Display;
use ffi;

glib_wrapper! {
    pub struct DeviceManager(Object<ffi::GdkDeviceManager>);

    match fn {
        get_type => || ffi::gdk_device_manager_get_type(),
    }
}

impl DeviceManager {
    pub fn disable_multidevice() {
        assert_not_initialized!();
        unsafe { ffi::gdk_disable_multidevice() }
    }

    pub fn get_display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_device_manager_get_display(self.to_glib_none().0)) }
    }

    pub fn get_client_pointer(&self) -> Device {
        unsafe {
            from_glib_none(
                ffi::gdk_device_manager_get_client_pointer(self.to_glib_none().0))
        }
    }
}

// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GdkDeviceManager — Functions for handling input devices

use glib::translate::*;
use glib::types::{StaticType, Type};
use device::Device;
use display::Display;
use object::Object;
use ffi;

pub type DeviceManager = Object<ffi::GdkDeviceManager>;

impl StaticType for DeviceManager {
    fn static_type() -> Type { unsafe { from_glib(ffi::gdk_device_manager_get_type()) } }
}

impl DeviceManager {
    pub fn disable_multidevice() {
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

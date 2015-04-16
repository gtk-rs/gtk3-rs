// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GdkDeviceManager — Functions for handling input devices

use ffi;

#[repr(C)]
pub struct DeviceManager {
    pointer: *mut ffi::C_GdkDeviceManager
}

impl DeviceManager {
    pub fn disable_multidevice() {
        unsafe { ffi::gdk_disable_multidevice() }
    }

    pub fn get_display(&self) -> Option<::Display> {
        let tmp = unsafe { ffi::gdk_device_manager_get_display(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(::Display::wrap_pointer(tmp))
        }
    }

    pub fn get_client(&self) -> Option<::Device> {
        let tmp = unsafe { ffi::gdk_device_manager_get_client_pointer(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(::Device::wrap_pointer(tmp))
        }
    }
}

impl_GObjectFunctions!(DeviceManager, C_GdkDeviceManager);
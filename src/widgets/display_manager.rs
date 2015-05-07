// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GdkDisplayManager — Maintains a list of all open GdkDisplays

use ffi;
use glib::translate::ToGlibPtr;

#[repr(C)]
pub struct DisplayManager {
    pointer: *mut ffi::C_GdkDisplayManager
}

impl DisplayManager {
    pub fn get() -> Option<DisplayManager> {
        let tmp = unsafe { ffi::gdk_display_manager_get() };

        if tmp.is_null() {
            None
        } else {
            Some(DisplayManager {
                pointer: tmp
            })
        }
    }

    pub fn get_default_display(&self) -> Option<::Display> {
        let tmp = unsafe { ffi::gdk_display_manager_get_default_display(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(::Display::wrap_pointer(tmp))
        }
    }

    pub fn set_default_display(&self, display: &::Display) {
        unsafe { ffi::gdk_display_manager_set_default_display(self.pointer, display.unwrap_pointer()) }
    }

    pub fn open_display(&self, name: &str) -> Option<::Display> {
        let tmp = unsafe {
            ffi::gdk_display_manager_open_display(self.pointer, name.to_glib_none().0)
        };

        if tmp.is_null() {
            None
        } else {
            Some(::Display::wrap_pointer(tmp))
        }
    }
}

impl_GObjectFunctions!(DisplayManager, C_GdkDisplayManager);
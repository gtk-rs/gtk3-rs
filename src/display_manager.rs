// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GdkDisplayManager — Maintains a list of all open GdkDisplays

use glib::translate::*;
use glib::types::{StaticType, Type};
use display::Display;
use object::Object;
use ffi;

pub type DisplayManager = Object<ffi::GdkDisplayManager>;

impl StaticType for DisplayManager {
    fn static_type() -> Type { unsafe { from_glib(ffi::gdk_display_manager_get_type()) } }
}

impl DisplayManager {
    pub fn get() -> DisplayManager {
        unsafe { from_glib_none(ffi::gdk_display_manager_get()) }
    }

    pub fn get_default_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_display_manager_get_default_display(self.to_glib_none().0))
        }
    }

    pub fn set_default_display(&self, display: &Display) {
        unsafe {
            ffi::gdk_display_manager_set_default_display(self.to_glib_none().0,
                                                         display.to_glib_none().0)
        }
    }

    pub fn open_display(&self, name: &str) -> Option<Display> {
        unsafe {
            from_glib_none(
                ffi::gdk_display_manager_open_display(self.to_glib_none().0, name.to_glib_none().0))
        }
    }
}

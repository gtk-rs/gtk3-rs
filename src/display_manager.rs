// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>


use glib::translate::*;
use display::Display;
use ffi;

glib_wrapper! {
    pub struct DisplayManager(Object<ffi::GdkDisplayManager>);

    match fn {
        get_type => || ffi::gdk_display_manager_get_type(),
    }
}

impl DisplayManager {
    pub fn get() -> DisplayManager {
        assert_initialized_main_thread!();
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

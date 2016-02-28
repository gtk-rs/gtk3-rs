// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use screen::Screen;
use display::Display;
use ffi;

glib_wrapper! {
    pub struct AppLaunchContext(Object<ffi::GdkAppLaunchContext>);

    match fn {
        get_type => || ffi::gdk_app_launch_context_get_type(),
    }
}

// FIXME: should inherit from GAppLaunchContext

impl AppLaunchContext {
    pub fn new() -> AppLaunchContext {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gdk_app_launch_context_new()) }
    }

    pub fn set_display(&self, display: &Display) {
        unsafe {
            ffi::gdk_app_launch_context_set_display(self.to_glib_none().0, display.to_glib_none().0)
        }
    }

    pub fn set_screen(&self, screen: &Screen) {
        unsafe {
            ffi::gdk_app_launch_context_set_screen(self.to_glib_none().0, screen.to_glib_none().0)
        }
    }

    pub fn set_desktop(&self, desktop: i32) {
        unsafe { ffi::gdk_app_launch_context_set_desktop(self.to_glib_none().0, desktop) }
    }

    pub fn set_timestamp(&self, timestamp: u32) {
        unsafe { ffi::gdk_app_launch_context_set_timestamp(self.to_glib_none().0, timestamp) }
    }

    pub fn set_icon_name(&self, icon_name: &str) {
        unsafe {
            ffi::gdk_app_launch_context_set_icon_name(self.to_glib_none().0,
                                                      icon_name.to_glib_none().0)
        }
    }
}

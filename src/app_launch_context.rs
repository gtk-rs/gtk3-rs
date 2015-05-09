// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Application launching — Startup notification for applications

use glib::translate::*;
use glib::types::{StaticType, Type};
use object::Object;
use screen::Screen;
use ffi;

// FIXME: should inherit from GAppLaunchContext
pub type AppLaunchContext = Object<ffi::C_GdkAppLaunchContext>;

impl StaticType for AppLaunchContext {
    fn static_type() -> Type { unsafe { from_glib(ffi::gdk_app_launch_context_get_type()) } }
}

impl AppLaunchContext {
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

    /*pub fn set_icon(&self, icon: GIO::Icon) {
        unsafe { ffi::gdk_app_launch_context_set_timestamp(self.to_glib_none().0, icon.to_glib_none().0) }
    }*/

    pub fn set_icon_name(&self, icon_name: &str) {
        unsafe {
            ffi::gdk_app_launch_context_set_icon_name(self.to_glib_none().0,
                                                      icon_name.to_glib_none().0)
        }
    }
}

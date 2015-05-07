// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Application launching — Startup notification for applications

use ffi;
use libc::c_int;
use glib::translate::ToGlibPtr;

// FIXME: should inherit from GAppLaunchContext
#[repr(C)]
pub struct AppLaunchContext {
    pointer: *mut ffi::C_GdkAppLaunchContext
}

impl AppLaunchContext {
    pub fn set_screen(&self, screen: &::Screen) {
        unsafe { ffi::gdk_app_launch_context_set_screen(self.pointer, screen.unwrap_pointer()) }
    }

    pub fn set_desktop(&self, desktop: i32) {
        unsafe { ffi::gdk_app_launch_context_set_desktop(self.pointer, desktop as c_int) }
    }

    pub fn set_timestamp(&self, timestamp: u32) {
        unsafe { ffi::gdk_app_launch_context_set_timestamp(self.pointer, timestamp) }
    }

    /*pub fn set_icon(&self, icon: GIO::Icon) {
        unsafe { ffi::gdk_app_launch_context_set_timestamp(self.pointer, icon.unwrap_pointer()) }
    }*/

    pub fn set_icon_name(&self, icon_name: &str) {
        unsafe {
            ffi::gdk_app_launch_context_set_icon_name(self.pointer, icon_name.to_glib_none().0)
        }
    }
}

impl_GObjectFunctions!(AppLaunchContext, C_GdkAppLaunchContext);
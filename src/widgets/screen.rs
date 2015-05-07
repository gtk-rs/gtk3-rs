// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GdkScreen — Object representing a physical screen

use glib::translate::*;
use ffi;
use libc::c_int;

#[repr(C)]
pub struct Screen {
    pointer: *mut ffi::C_GdkScreen
}

impl Screen {
    pub fn get_default() -> Option<Screen> {
        let tmp = unsafe { ffi::gdk_screen_get_default() };

        if tmp.is_null() {
            None
        } else {
            Some(Screen {
                pointer: tmp
            })
        }
    }

    pub fn get_system_visual(&self) -> Option<::Visual> {
        let tmp = unsafe { ffi::gdk_screen_get_system_visual(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(::Visual::wrap_pointer(tmp))
        }
    }

    pub fn get_rgba_visual(&self) -> Option<::Visual> {
        let tmp = unsafe { ffi::gdk_screen_get_rgba_visual(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(::Visual::wrap_pointer(tmp))
        }
    }

    pub fn is_composited(&self) -> bool {
        unsafe { ::glib::to_bool(ffi::gdk_screen_is_composited(self.pointer)) }
    }

    pub fn get_root_window(&self) -> Option<::Window> {
        let tmp = unsafe { ffi::gdk_screen_get_root_window(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(::Window::wrap_pointer(tmp))
        }
    }

    pub fn get_display(&self) -> Option<::Display> {
        let tmp = unsafe { ffi::gdk_screen_get_display(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(::Display::wrap_pointer(tmp))
        }
    }

    pub fn get_number(&self) -> i32 {
        unsafe { ffi::gdk_screen_get_number(self.pointer) }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { ffi::gdk_screen_get_width(self.pointer) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { ffi::gdk_screen_get_height(self.pointer) }
    }

    pub fn get_width_mm(&self) -> i32 {
        unsafe { ffi::gdk_screen_get_width_mm(self.pointer) }
    }

    pub fn get_height_mm(&self) -> i32 {
        unsafe { ffi::gdk_screen_get_height_mm(self.pointer) }
    }

    pub fn make_display_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(
                ffi::gdk_screen_make_display_name(self.pointer))
        }
    }

    pub fn get_n_monitors(&self) -> i32 {
        unsafe { ffi::gdk_screen_get_n_monitors(self.pointer) }
    }

    pub fn get_primary_monitor(&self) -> i32 {
        unsafe { ffi::gdk_screen_get_primary_monitor(self.pointer) }
    }

    pub fn get_monitor_geometry(&self, monitor_num: i32, dest: &mut ffi::C_GdkRectangle) {
        unsafe { ffi::gdk_screen_get_monitor_geometry(self.pointer, monitor_num as c_int, dest) }
    }

    pub fn get_monitor_workarea(&self, monitor_num: i32, dest: &mut ffi::C_GdkRectangle) {
        unsafe { ffi::gdk_screen_get_monitor_workarea(self.pointer, monitor_num as c_int, dest) }
    }

    pub fn get_monitor_at_point(&self, x: i32, y: i32) -> i32 {
        unsafe { ffi::gdk_screen_get_monitor_at_point(self.pointer, x as c_int, y as c_int) }
    }

    pub fn get_monitor_at_window(&self, window: &mut ::Window) -> i32 {
        unsafe { ffi::gdk_screen_get_monitor_at_window(self.pointer, window.unwrap_pointer()) }
    }

    pub fn get_monitor_width_mm(&self, monitor_num: i32) -> i32 {
        unsafe { ffi::gdk_screen_get_monitor_width_mm(self.pointer, monitor_num as c_int) }
    }

    pub fn get_monitor_height_mm(&self, monitor_num: i32) -> i32 {
        unsafe { ffi::gdk_screen_get_monitor_height_mm(self.pointer, monitor_num as c_int) }
    }

    pub fn get_monitor_plug_name(&self, monitor_num: i32) -> Option<String> {
        unsafe {
            from_glib_full(
                ffi::gdk_screen_get_monitor_plug_name(self.pointer,
                                                      monitor_num as c_int))
        }
    }

    #[cfg(feature = "gdk_3_10")]
    pub fn get_monitor_scale_factor(&self, monitor_num: i32) -> i32 {
        unsafe { ffi::gdk_screen_get_monitor_scale_factor(self.pointer, monitor_num as c_int) }
    }

    pub fn get_resolution(&self) -> f64 {
        unsafe { ffi::gdk_screen_get_resolution(self.pointer) }
    }

    pub fn set_resolution(&self, dpi: f64) {
        unsafe { ffi::gdk_screen_set_resolution(self.pointer, dpi) }
    }

    pub fn get_active_window(&self) -> Option<::Window> {
        let tmp = unsafe { ffi::gdk_screen_get_active_window(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(::Window::wrap_pointer(tmp))
        }
    }
}

impl_GObjectFunctions!(Screen, C_GdkScreen);

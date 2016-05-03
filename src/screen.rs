// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use display::Display;
use visual::Visual;
use window::Window;
use ffi;
use Rectangle;

glib_wrapper! {
    pub struct Screen(Object<ffi::GdkScreen>);

    match fn {
        get_type => || ffi::gdk_screen_get_type(),
    }
}

impl Screen {
    pub fn get_default() -> Option<Screen> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gdk_screen_get_default()) }
    }

    pub fn get_system_visual(&self) -> Visual {
        unsafe { from_glib_none(ffi::gdk_screen_get_system_visual(self.to_glib_none().0)) }
    }

    pub fn get_rgba_visual(&self) -> Option<Visual> {
        unsafe { from_glib_none(ffi::gdk_screen_get_rgba_visual(self.to_glib_none().0)) }
    }

    pub fn is_composited(&self) -> bool {
        unsafe { from_glib(ffi::gdk_screen_is_composited(self.to_glib_none().0)) }
    }

    pub fn get_root_window(&self) -> Window {
        unsafe { from_glib_none(ffi::gdk_screen_get_root_window(self.to_glib_none().0)) }
    }

    pub fn get_display(&self) -> Display {
        unsafe { from_glib_none(ffi::gdk_screen_get_display(self.to_glib_none().0)) }
    }

    pub fn get_number(&self) -> i32 {
        unsafe { ffi::gdk_screen_get_number(self.to_glib_none().0) }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { ffi::gdk_screen_get_width(self.to_glib_none().0) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { ffi::gdk_screen_get_height(self.to_glib_none().0) }
    }

    pub fn get_width_mm(&self) -> i32 {
        unsafe { ffi::gdk_screen_get_width_mm(self.to_glib_none().0) }
    }

    pub fn get_height_mm(&self) -> i32 {
        unsafe { ffi::gdk_screen_get_height_mm(self.to_glib_none().0) }
    }

    pub fn make_display_name(&self) -> String {
        unsafe {
            from_glib_full(ffi::gdk_screen_make_display_name(self.to_glib_none().0))
        }
    }

    pub fn get_n_monitors(&self) -> i32 {
        unsafe { ffi::gdk_screen_get_n_monitors(self.to_glib_none().0) }
    }

    pub fn get_primary_monitor(&self) -> i32 {
        unsafe { ffi::gdk_screen_get_primary_monitor(self.to_glib_none().0) }
    }

    pub fn get_monitor_geometry(&self, monitor_num: i32) -> Rectangle {
        unsafe {
            let mut ret = Rectangle::uninitialized();
            ffi::gdk_screen_get_monitor_geometry(self.to_glib_none().0, monitor_num,
                ret.to_glib_none_mut().0);
            ret
        }
    }

    pub fn get_monitor_workarea(&self, monitor_num: i32) -> Rectangle {
        unsafe {
            let mut ret = Rectangle::uninitialized();
            ffi::gdk_screen_get_monitor_workarea(self.to_glib_none().0, monitor_num,
                ret.to_glib_none_mut().0);
            ret
        }
    }

    pub fn get_monitor_at_point(&self, x: i32, y: i32) -> i32 {
        unsafe { ffi::gdk_screen_get_monitor_at_point(self.to_glib_none().0, x, y) }
    }

    pub fn get_monitor_at_window(&self, window: &Window) -> i32 {
        unsafe {
            ffi::gdk_screen_get_monitor_at_window(self.to_glib_none().0, window.to_glib_none().0)
        }
    }

    pub fn get_monitor_width_mm(&self, monitor_num: i32) -> i32 {
        unsafe { ffi::gdk_screen_get_monitor_width_mm(self.to_glib_none().0, monitor_num) }
    }

    pub fn get_monitor_height_mm(&self, monitor_num: i32) -> i32 {
        unsafe { ffi::gdk_screen_get_monitor_height_mm(self.to_glib_none().0, monitor_num) }
    }

    pub fn get_monitor_plug_name(&self, monitor_num: i32) -> Option<String> {
        unsafe {
            from_glib_full(
                ffi::gdk_screen_get_monitor_plug_name(self.to_glib_none().0, monitor_num))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_monitor_scale_factor(&self, monitor_num: i32) -> i32 {
        unsafe { ffi::gdk_screen_get_monitor_scale_factor(self.to_glib_none().0, monitor_num) }
    }

    pub fn get_resolution(&self) -> f64 {
        unsafe { ffi::gdk_screen_get_resolution(self.to_glib_none().0) }
    }

    pub fn set_resolution(&self, dpi: f64) {
        unsafe { ffi::gdk_screen_set_resolution(self.to_glib_none().0, dpi) }
    }

    pub fn get_active_window(&self) -> Option<Window> {
        unsafe {
            from_glib_full(ffi::gdk_screen_get_active_window(self.to_glib_none().0))
        }
    }
}

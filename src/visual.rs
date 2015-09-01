// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Visuals — Low-level display hardware information

use std::ptr;
use std::slice;
use glib::translate::*;
use glib::types::{StaticType, Type};
use object::Object;
use screen::Screen;
use ffi;

pub type Visual = Object<ffi::GdkVisual>;

impl StaticType for Visual {
    fn static_type() -> Type { unsafe { from_glib(ffi::gdk_visual_get_type()) } }
}

impl Visual {
    pub fn query_depths() -> Vec<i32> {
        let mut ptr = ptr::null_mut();
        let mut count = 0;

        unsafe {
            ffi::gdk_query_depths(&mut ptr, &mut count);
            Vec::from(
                slice::from_raw_parts(ptr as *const i32, count as usize))
        }
    }

    pub fn get_bits_per_rgb(&self) -> i32 {
        unsafe { ffi::gdk_visual_get_bits_per_rgb(self.to_glib_none().0) }
    }

    pub fn get_blue_pixel_details(&self, mask: &mut u32, shift: &mut i32, precision: &mut i32) {
        unsafe {
            ffi::gdk_visual_get_blue_pixel_details(
                self.to_glib_none().0,
                mask,
                shift,
                precision)
        }
    }

    pub fn get_colormap_size(&self) -> i32 {
        unsafe { ffi::gdk_visual_get_colormap_size(self.to_glib_none().0) }
    }

    pub fn get_depth(&self) -> i32 {
        unsafe { ffi::gdk_visual_get_depth(self.to_glib_none().0) }
    }

    pub fn get_green_pixel_details(&self, mask: &mut u32, shift: &mut i32, precision: &mut i32) {
        unsafe {
            ffi::gdk_visual_get_green_pixel_details(
                self.to_glib_none().0,
                mask,
                shift,
                precision)
        }
    }

    pub fn get_red_pixel_details(&self, mask: &mut u32, shift: &mut i32, precision: &mut i32) {
        unsafe {
            ffi::gdk_visual_get_red_pixel_details(
                self.to_glib_none().0,
                mask,
                shift,
                precision)
        }
    }

    pub fn get_best_depth() -> i32 {
        unsafe { ffi::gdk_visual_get_best_depth() }
    }

    pub fn get_system() -> Visual {
        unsafe { from_glib_none(ffi::gdk_visual_get_system()) }
    }

    pub fn get_best() -> Visual {
        unsafe { from_glib_none(ffi::gdk_visual_get_best()) }
    }

    pub fn get_best_with_depth(depth: i32) -> Option<Visual> {
        unsafe { from_glib_none(ffi::gdk_visual_get_best_with_depth(depth)) }
    }

    pub fn get_screen(&self) -> Screen {
        unsafe { from_glib_none(ffi::gdk_visual_get_screen(self.to_glib_none().0)) }
    }
}

// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Visuals — Low-level display hardware information

use std::slice;
use ffi;
use libc::{c_int};

#[repr(C)]
pub struct Visual {
    pointer: *mut ffi::C_GdkVisual
}

impl Visual {
    pub fn query_depths() -> Vec<i32> {
        let mut tmp = ::std::ptr::null_mut();
        let mut count = 0i32;

        unsafe {
            ffi::gdk_query_depths(&mut tmp, &mut count);
            Vec::from(
                slice::from_raw_parts(tmp as *const i32, count as usize))
        }
    }

    pub fn get_bits_per_rgb(&self) -> i32 {
        unsafe { ffi::gdk_visual_get_bits_per_rgb(self.pointer) }
    }

    pub fn get_blue_pixel_details(&self, mask: &mut u32, shift: &mut i32, precision: &mut i32) {
        unsafe { ffi::gdk_visual_get_blue_pixel_details(self.pointer, mask, shift as *mut c_int, precision as *mut c_int) }
    }

    pub fn get_colormap_size(&self) -> i32 {
        unsafe { ffi::gdk_visual_get_colormap_size(self.pointer) }
    }

    pub fn get_depth(&self) -> i32 {
        unsafe { ffi::gdk_visual_get_depth(self.pointer) }
    }

    pub fn get_green_pixel_details(&self, mask: &mut u32, shift: &mut i32, precision: &mut i32) {
        unsafe { ffi::gdk_visual_get_green_pixel_details(self.pointer, mask, shift as *mut c_int, precision as *mut c_int) }
    }

    pub fn get_red_pixel_details(&self, mask: &mut u32, shift: &mut i32, precision: &mut i32) {
        unsafe { ffi::gdk_visual_get_red_pixel_details(self.pointer, mask, shift as *mut c_int, precision as *mut c_int) }
    }

    pub fn get_best_depth() -> i32 {
        unsafe { ffi::gdk_visual_get_best_depth() }
    }

    pub fn get_system() -> Option<Visual> {
        let tmp = unsafe { ffi::gdk_visual_get_system() };

        if tmp.is_null() {
            None
        } else {
            Some(Visual {
                pointer: tmp
            })
        }
    }

    pub fn get_best() -> Option<Visual> {
        let tmp = unsafe { ffi::gdk_visual_get_best() };

        if tmp.is_null() {
            None
        } else {
            Some(Visual {
                pointer: tmp
            })
        }
    }

    pub fn get_best_with_depth(depth: i32) -> Option<Visual> {
        let tmp = unsafe { ffi::gdk_visual_get_best_with_depth(depth as c_int) };

        if tmp.is_null() {
            None
        } else {
            Some(Visual {
                pointer: tmp
            })
        }
    }

    pub fn get_screen(&self) -> Option<::Screen> {
        let tmp = unsafe { ffi::gdk_visual_get_screen(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(::Screen::wrap_pointer(tmp))
        }
    }
}

impl_GObjectFunctions!(Visual, C_GdkVisual);
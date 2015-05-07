// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Animations — Animated images.

use ffi;
use glib::{to_bool, self};
use std;

pub struct PixbufAnimationIter {
    pointer: *mut ffi::C_GdkPixbufAnimationIter
}

impl PixbufAnimationIter {
    pub fn advance(&mut self, start_time: &glib::TimeVal) -> bool {
        unsafe { glib::to_bool(ffi::gdk_pixbuf_animation_iter_advance(self.unwrap_pointer(),
            std::mem::transmute(start_time))) }
    }

    pub fn get_pixbuf(&self) -> Option<::Pixbuf> {
        let tmp = unsafe { ffi::gdk_pixbuf_animation_iter_get_pixbuf(self.unwrap_pointer()) };

        if tmp.is_null() {
            None
        } else {
            Some(::Pixbuf::wrap_pointer(tmp))
        }
    }

    pub fn get_delay_time(&mut self) -> i32 {
        unsafe { ffi::gdk_pixbuf_animation_iter_get_delay_time(self.unwrap_pointer()) }
    }

    pub fn on_currently_loading_frame(&mut self) -> bool {
        unsafe { glib::to_bool(ffi::gdk_pixbuf_animation_iter_on_currently_loading_frame(self.unwrap_pointer())) }
    }
}

impl_GObjectFunctions!(PixbufAnimationIter, C_GdkPixbufAnimationIter);
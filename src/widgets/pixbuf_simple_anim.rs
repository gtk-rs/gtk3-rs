// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GdkPixbufSimpleAnim — An opaque struct representing a simple animation. 

use ffi;
use glib::{to_bool, self};

pub struct PixbufSimpleAnim {
    pointer: *mut ffi::C_GdkPixbufSimpleAnim
}

impl PixbufSimpleAnim {
    pub fn new(width: i32, height: i32, rate: f32) -> Option<PixbufSimpleAnim> {
        let tmp = unsafe { ffi::gdk_pixbuf_simple_anim_new(width, height, rate) };

        if tmp.is_null() {
            None
        } else {
            Some(PixbufSimpleAnim::wrap_pointer(tmp))
        }
    }

    pub fn add_frame(&mut self, pixbuf: &mut ::Pixbuf) {
        unsafe { ffi::gdk_pixbuf_simple_anim_add_frame(self.unwrap_pointer(), pixbuf.unwrap_pointer()) }
    }

    pub fn set_loop(&mut self, loop_: bool) {
        unsafe { ffi::gdk_pixbuf_simple_anim_set_loop(self.unwrap_pointer(), glib::to_gboolean(loop_)) }
    }

    pub fn get_loop(&mut self) -> bool {
        unsafe { glib::to_bool(ffi::gdk_pixbuf_simple_anim_get_loop(self.unwrap_pointer())) }
    }
}

impl_GObjectFunctions!(PixbufSimpleAnim, C_GdkPixbufSimpleAnim);
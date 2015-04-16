// Copyright 2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi::PangoRectangle;
use libc::c_int;
//use std::default::Default;

/// The PangoRectangle structure represents a rectangle. It is frequently used to represent the
/// logical or ink extents of a single glyph or section of text. (See, for instance,
/// pango_font_get_glyph_extents()).
pub trait Rectangle {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Self;
}

impl Rectangle for PangoRectangle {
    fn new(x: i32, y: i32, width: i32, height: i32) -> PangoRectangle {
        PangoRectangle {
            x: x as c_int,
            y: y as c_int,
            width: width as c_int,
            height: height as c_int,
        }
    }
}

/*impl Default for Rectangle {
    fn default() -> Rectangle {
        Rectangle {
            x: 0,
            y: 0,
            width: 0,
            height: 0
        }
    }
}*/
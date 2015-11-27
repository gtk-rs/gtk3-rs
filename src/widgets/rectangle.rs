// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi::PangoRectangle;
use libc::c_int;
//use std::default::Default;

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
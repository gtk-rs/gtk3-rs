// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Rectangles — Simple graphical data type

use ffi;
use gdk_ffi::C_GdkRectangle;
use glib::to_bool;

pub trait Rectangle {
    fn intersect(&self, other: &C_GdkRectangle, dest: &mut C_GdkRectangle) -> bool;
    fn union(&self, other: &C_GdkRectangle, dest: &mut C_GdkRectangle);
}

impl Rectangle for C_GdkRectangle {
    fn intersect(&self, other: &C_GdkRectangle, dest: &mut C_GdkRectangle) -> bool {
        unsafe { to_bool(ffi::gdk_rectangle_intersect(self, other, dest)) }
    }

    fn union(&self, other: &C_GdkRectangle, dest: &mut C_GdkRectangle) {
        unsafe { ffi::gdk_rectangle_union(self, other, dest) }
    }
}

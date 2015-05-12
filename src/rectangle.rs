// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Rectangles — Simple graphical data type

use ffi;
use gdk_ffi::GdkRectangle;
use glib::to_bool;

pub trait Rectangle {
    fn intersect(&self, other: &GdkRectangle, dest: &mut GdkRectangle) -> bool;
    fn union(&self, other: &GdkRectangle, dest: &mut GdkRectangle);
}

impl Rectangle for GdkRectangle {
    fn intersect(&self, other: &GdkRectangle, dest: &mut GdkRectangle) -> bool {
        unsafe { to_bool(ffi::gdk_rectangle_intersect(self, other, dest)) }
    }

    fn union(&self, other: &GdkRectangle, dest: &mut GdkRectangle) {
        unsafe { ffi::gdk_rectangle_union(self, other, dest) }
    }
}

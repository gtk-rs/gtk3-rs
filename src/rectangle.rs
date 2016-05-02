// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi;
use Rectangle;

pub trait RectangleExt {
    fn intersect(&self, other: &Self) -> Option<Self>
        where Self: Sized;
    fn union(&self, other: &Self) -> Self;
}

impl RectangleExt for Rectangle {
    fn intersect(&self, other: &Rectangle) -> Option<Rectangle> {
        unsafe {
            let mut ret = Rectangle::uninitialized();
            if from_glib(ffi::gdk_rectangle_intersect(self.to_glib_none().0, other.to_glib_none().0,
                    ret.to_glib_none_mut().0)) {
                Some(ret)
            }
            else {
                None
            }
        }
    }

    fn union(&self, other: &Rectangle) -> Rectangle {
        unsafe {
            let mut ret = Rectangle::uninitialized();
            ffi::gdk_rectangle_union(self.to_glib_none().0, other.to_glib_none().0,
                ret.to_glib_none_mut().0);
            ret
        }
    }
}

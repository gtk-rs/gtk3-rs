// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::mem;
use glib::translate::*;
use cairo::RectangleInt;
use ffi;

pub trait RectangleExt {
    fn intersect(&self, other: &Self) -> Option<Self>
        where Self: Sized;
    fn union(&self, other: &Self) -> Self;
}

impl RectangleExt for RectangleInt {
    fn intersect(&self, other: &RectangleInt) -> Option<RectangleInt> {
        unsafe {
            let mut res = mem::uninitialized();
            if from_glib(ffi::gdk_rectangle_intersect(self, other, &mut res)) {
                Some(res)
            }
            else {
                None
            }
        }
    }

    fn union(&self, other: &RectangleInt) -> RectangleInt {
        unsafe {
            let mut res = mem::uninitialized();
            ffi::gdk_rectangle_union(self, other, &mut res);
            res
        }
    }
}

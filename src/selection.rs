// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;

use glib;
use glib::object::IsA;
use glib::translate::*;

use ::Object as AtkObject;
use Selection;

pub trait SelectionExtManual {
    fn ref_selection(&self, i: i32) -> Option<AtkObject>;
}

impl<O: IsA<Selection> + IsA<glib::object::Object>> SelectionExtManual for O {
    fn ref_selection(&self, i: i32) -> Option<AtkObject> {
        unsafe {
            from_glib_full(ffi::atk_selection_ref_selection(self.to_glib_none().0, i))
        }
    }
}

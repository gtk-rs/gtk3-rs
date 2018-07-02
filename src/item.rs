// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::ToGlibPtr;

use Analysis;
use Item;

impl Item {
    pub fn offset(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).offset }
    }

    pub fn length(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).length }
    }

    pub fn num_chars(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).num_chars }
    }

    pub fn analysis(&self) -> &Analysis {
        unsafe { &*(&((*self.to_glib_none().0).analysis) as *const _ as *const Analysis) }
    }
}

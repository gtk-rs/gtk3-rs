// Copyright 2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Attribute;
use AttrClass;
use glib::translate::*;

impl Attribute {
    pub fn get_attr_class(&self) -> AttrClass {
        unsafe {
            from_glib_full((*self.to_glib_none().0).klass)
        }
    }

    pub fn get_start_index(&self) -> u32 {
        unsafe {
            let stash = self.to_glib_none();
            (*stash.0).start_index
        }
    }

    pub fn get_end_index(&self) -> u32 {
        unsafe {
            let stash = self.to_glib_none();
            (*stash.0).end_index
        }
    }

    pub fn set_start_index(&mut self, index: u32) {
        unsafe {
            let stash = self.to_glib_none_mut();
            (*stash.0).start_index = index;
        }
    }

    pub fn set_end_index(&mut self, index: u32) {
        unsafe {
            let stash = self.to_glib_none_mut();
            (*stash.0).end_index = index;
        }
    }
}

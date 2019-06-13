// Copyright 2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use pango_sys;
use std::mem;
use AttrList;
use Attribute;

impl AttrList {
    pub fn change(&self, attr: Attribute) {
        unsafe {
            pango_sys::pango_attr_list_change(
                self.to_glib_none().0,
                attr.to_glib_none().0 as *mut _,
            );
            mem::forget(attr); //As attr transferred fully
        }
    }

    pub fn insert(&self, attr: Attribute) {
        unsafe {
            pango_sys::pango_attr_list_insert(
                self.to_glib_none().0,
                attr.to_glib_none().0 as *mut _,
            );
            mem::forget(attr); //As attr transferred fully
        }
    }

    pub fn insert_before(&self, attr: Attribute) {
        unsafe {
            pango_sys::pango_attr_list_insert_before(
                self.to_glib_none().0,
                attr.to_glib_none().0 as *mut _,
            );
            mem::forget(attr); //As attr transferred fully
        }
    }
}

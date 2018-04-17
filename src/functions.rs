// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Item;

use ffi;
use glib_ffi;
use std::ptr;

use glib::translate::*;

pub fn reorder_items(logical_items: &[&Item]) -> Vec<Item> {
    unsafe {
        let stash_vec: Vec<_> = logical_items.iter().rev().map(|v| v.to_glib_none()).collect();
        let mut list: *mut glib_ffi::GList = ptr::null_mut();
        for stash in &stash_vec {
            list = glib_ffi::g_list_prepend(list, Ptr::to(stash.0));
        }

        FromGlibPtrContainer::from_glib_full(ffi::pango_reorder_items(list))
    }
}
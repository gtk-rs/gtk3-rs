// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use libc::c_int;

pub struct Item {
    pointer: *mut ffi::PangoItem
}

impl Item {
    pub fn new() -> Option<Item> {
        let tmp = unsafe { ffi::pango_item_new() };

        if tmp.is_null() {
            None
        } else {
            Some(Item {
                pointer: tmp
            })
        }
    }

    pub fn copy(&self) -> Option<Item> {
        let tmp = unsafe { ffi::pango_item_copy(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(Item {
                pointer: tmp
            })
        }
    }

    pub fn split(&self, split_index: i32, split_offset: i32) -> Option<Item> {
        let tmp = unsafe { ffi::pango_item_split(self.pointer, split_index as c_int, split_offset as c_int) };

        if tmp.is_null() {
            None
        } else {
            Some(Item {
                pointer: tmp
            })
        }
    }
}

impl Drop for Item {
    fn drop(&mut self) {
        if !self.pointer.is_null() {
            unsafe { ffi::pango_item_free(self.pointer) };
            self.pointer = ::std::ptr::null_mut();
        }
    }
}
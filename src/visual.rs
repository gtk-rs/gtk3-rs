// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use std::ptr;
use std::slice;
use Visual;

impl Visual {
    pub fn query_depths() -> Vec<i32> {
        assert_initialized_main_thread!();
        let mut ptr = ptr::null_mut();
        let mut count = 0;

        unsafe {
            gdk_sys::gdk_query_depths(&mut ptr, &mut count);
            Vec::from(slice::from_raw_parts(ptr as *const i32, count as usize))
        }
    }
}

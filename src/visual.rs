// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ptr;
use std::slice;
use glib::object::IsA;
use ffi;
use Visual;

pub trait VisualExtManual: 'static {
    fn query_depths() -> Vec<i32>;
}

impl<O: IsA<Visual>> VisualExtManual for O {
    fn query_depths() -> Vec<i32> {
        assert_initialized_main_thread!();
        let mut ptr = ptr::null_mut();
        let mut count = 0;

        unsafe {
            ffi::gdk_query_depths(&mut ptr, &mut count);
            Vec::from(
                slice::from_raw_parts(ptr as *const i32, count as usize))
        }
    }
}

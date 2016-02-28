// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib_ffi;
use std::mem;

pub use glib_ffi::GTimeVal as TimeVal;

pub fn get_current_time() -> TimeVal {
    unsafe {
        let mut ret = mem::uninitialized();
        glib_ffi::g_get_current_time(&mut ret);
        ret
    }
}

// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi;
use libc::c_uint;
use std::mem::transmute;

pub fn keyval_name(keyval: u32) -> Option<String> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(ffi::gdk_keyval_name(keyval as c_uint))
    }
}

pub fn keyval_to_unicode(keyval: u32) -> Option<char> {
    skip_assert_initialized!();
    unsafe {
        let c: char = transmute(ffi::gdk_keyval_to_unicode(keyval));
        if c != '\0' {
            Some(c)
        } else {
            None
        }
    }
}

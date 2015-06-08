// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Keyboard Handling Functions

use glib::translate::*;
use ffi;
use libc::c_uint;
use std::mem::transmute;

pub fn keyval_name(keyval: u32) -> Option<String> {
    unsafe {
        from_glib_none(ffi::gdk_keyval_name(keyval as c_uint))
    }
}

pub fn keyval_to_unicode(keyval: u32) -> Option<char> {
    unsafe {
        let c: char = transmute(ffi::gdk_keyval_to_unicode(keyval));
        if c != '\0' {
            Some(c)
        } else {
            None
        }
    }
}

// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;

pub trait FFIGObject {
    fn unwrap_gobject(&self) -> *mut ffi::C_GObject;
    fn wrap_object(object: *mut ffi::C_GObject) -> Self;
}

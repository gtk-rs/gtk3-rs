// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gobject_ffi;

pub trait FFIGObject {
    fn unwrap_gobject(&self) -> *mut gobject_ffi::GObject;
    fn wrap_object(object: *mut gobject_ffi::GObject) -> Self;
}

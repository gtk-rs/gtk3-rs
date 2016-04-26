// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Resource;
use ffi;
use glib::translate::*;

pub fn resources_register(resource: &Resource) {
    unsafe { ffi::g_resources_register(resource.to_glib_none().0) }
}

pub fn resources_unregister(resource: &Resource) {
    unsafe { ffi::g_resources_unregister(resource.to_glib_none().0) }
}

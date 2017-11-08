// Copyright 2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Resource;
use ffi;
use glib_ffi;
use glib;
use glib::translate::*;
use std::ptr;
use std::mem;

impl Resource {
    pub fn new_from_data(data: &glib::Bytes) -> Result<Resource, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();

            // Create a copy of data if it is not pointer-aligned
            // https://bugzilla.gnome.org/show_bug.cgi?id=790030
            let mut data = data.clone();
            let data_ptr = glib_ffi::g_bytes_get_data(data.to_glib_none().0, ptr::null_mut());
            if data_ptr as usize % mem::align_of::<*const u8>() != 0 {
                data = glib::Bytes::from(&*data);
            }

            let ret = ffi::g_resource_new_from_data(data.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

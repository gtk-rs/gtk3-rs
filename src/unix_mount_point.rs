// Copyright 2013-2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use std::mem;
use UnixMountPoint;

impl UnixMountPoint {
    #[cfg(any(unix, feature = "dox"))]
    pub fn get_mount_points() -> (Vec<UnixMountPoint>, u64) {
        unsafe {
            let mut time_read = mem::MaybeUninit::uninit();
            let ret = FromGlibPtrContainer::from_glib_full(gio_sys::g_unix_mount_points_get(
                time_read.as_mut_ptr(),
            ));
            let time_read = time_read.assume_init();
            (ret, time_read)
        }
    }

    pub fn is_changed_since(time: u64) -> bool {
        unsafe { from_glib(gio_sys::g_unix_mount_points_changed_since(time)) }
    }
}

// Take a look at the license at the top of the repository in the LICENSE file.

use crate::UnixMountPoint;
use glib::translate::*;
use std::mem;

impl UnixMountPoint {
    #[cfg(any(unix, feature = "dox"))]
    pub fn get_mount_points() -> (Vec<UnixMountPoint>, u64) {
        unsafe {
            let mut time_read = mem::MaybeUninit::uninit();
            let ret = FromGlibPtrContainer::from_glib_full(ffi::g_unix_mount_points_get(
                time_read.as_mut_ptr(),
            ));
            let time_read = time_read.assume_init();
            (ret, time_read)
        }
    }

    pub fn is_changed_since(time: u64) -> bool {
        unsafe { from_glib(ffi::g_unix_mount_points_changed_since(time)) }
    }
}

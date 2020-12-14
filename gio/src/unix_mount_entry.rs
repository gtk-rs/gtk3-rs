// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Icon;
use crate::UnixMountEntry;
use glib::translate::*;
use glib::GString;
use std::cmp;
use std::mem;

impl UnixMountEntry {
    pub fn new_at<P: AsRef<std::path::Path>>(mount_path: P) -> (UnixMountEntry, u64) {
        unsafe {
            let mut time_read = mem::MaybeUninit::uninit();
            let ret = from_glib_full(ffi::g_unix_mount_at(
                mount_path.as_ref().to_glib_none().0,
                time_read.as_mut_ptr(),
            ));
            let time_read = time_read.assume_init();
            (ret, time_read)
        }
    }

    #[cfg(any(feature = "v2_52", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_52")))]
    pub fn new_for<P: AsRef<std::path::Path>>(file_path: P) -> (UnixMountEntry, u64) {
        unsafe {
            let mut time_read = mem::MaybeUninit::uninit();
            let ret = from_glib_full(ffi::g_unix_mount_for(
                file_path.as_ref().to_glib_none().0,
                time_read.as_mut_ptr(),
            ));
            let time_read = time_read.assume_init();
            (ret, time_read)
        }
    }

    pub fn get_mounts() -> (Vec<UnixMountEntry>, u64) {
        unsafe {
            let mut time_read = mem::MaybeUninit::uninit();
            let ret = FromGlibPtrContainer::from_glib_full(ffi::g_unix_mounts_get(
                time_read.as_mut_ptr(),
            ));
            let time_read = time_read.assume_init();
            (ret, time_read)
        }
    }

    pub fn compare(&self, mount2: &UnixMountEntry) -> i32 {
        unsafe {
            ffi::g_unix_mount_compare(
                mut_override(self.to_glib_none().0),
                mut_override(mount2.to_glib_none().0),
            )
        }
    }

    pub fn get_device_path(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(ffi::g_unix_mount_get_device_path(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    pub fn get_fs_type(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::g_unix_mount_get_fs_type(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    pub fn unix_mount_get_mount_path(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(ffi::g_unix_mount_get_mount_path(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    pub fn get_options(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::g_unix_mount_get_options(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    pub fn get_root_path(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(ffi::g_unix_mount_get_root_path(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    pub fn guess_can_eject(&self) -> bool {
        unsafe {
            from_glib(ffi::g_unix_mount_guess_can_eject(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    pub fn guess_icon(&self) -> Option<Icon> {
        unsafe {
            from_glib_full(ffi::g_unix_mount_guess_icon(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    pub fn guess_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::g_unix_mount_guess_name(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    pub fn guess_should_display(&self) -> bool {
        unsafe {
            from_glib(ffi::g_unix_mount_guess_should_display(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    pub fn guess_symbolic_icon(&self) -> Option<Icon> {
        unsafe {
            from_glib_full(ffi::g_unix_mount_guess_symbolic_icon(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    pub fn is_readonly(&self) -> bool {
        unsafe {
            from_glib(ffi::g_unix_mount_is_readonly(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    pub fn is_system_internal(&self) -> bool {
        unsafe {
            from_glib(ffi::g_unix_mount_is_system_internal(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    pub fn is_changed_since(time: u64) -> bool {
        unsafe { from_glib(ffi::g_unix_mounts_changed_since(time)) }
    }
}

impl PartialEq for UnixMountEntry {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.compare(other) == 0
    }
}

impl Eq for UnixMountEntry {}

impl PartialOrd for UnixMountEntry {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.compare(other).partial_cmp(&0)
    }
}

impl Ord for UnixMountEntry {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.compare(other).cmp(&0)
    }
}

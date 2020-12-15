// Take a look at the license at the top of the repository in the LICENSE file.

use crate::FileInfo;
use glib::translate::*;
use std::mem;
use std::time::{Duration, SystemTime};

impl FileInfo {
    #[cfg_attr(feature = "v2_62", deprecated)]
    pub fn get_modification_time(&self) -> SystemTime {
        unsafe {
            let mut result = mem::MaybeUninit::uninit();
            ffi::g_file_info_get_modification_time(self.to_glib_none().0, result.as_mut_ptr());
            let result = result.assume_init();

            if result.tv_sec > 0 {
                let duration = Duration::from_secs(result.tv_sec as u64)
                    + Duration::from_millis(result.tv_usec as u64);
                SystemTime::UNIX_EPOCH + duration
            } else {
                let duration = Duration::from_secs((-result.tv_sec) as u64)
                    + Duration::from_millis(result.tv_usec as u64);
                SystemTime::UNIX_EPOCH - duration
            }
        }
    }

    #[cfg_attr(feature = "v2_62", deprecated)]
    pub fn set_modification_time(&self, mtime: SystemTime) {
        let diff = mtime
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("failed to convert time");
        unsafe {
            ffi::g_file_info_set_modification_time(
                self.to_glib_none().0,
                mut_override(&glib::ffi::GTimeVal {
                    tv_sec: diff.as_secs() as _,
                    tv_usec: diff.subsec_micros() as _,
                }),
            );
        }
    }
}

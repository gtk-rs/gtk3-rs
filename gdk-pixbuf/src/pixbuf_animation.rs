// Take a look at the license at the top of the repository in the LICENSE file.

use crate::PixbufAnimation;
use crate::PixbufAnimationIter;
use glib::object::IsA;
use glib::translate::*;

use std::ptr;
use std::time::SystemTime;

pub trait PixbufAnimationExtManual {
    #[doc(alias = "gdk_pixbuf_animation_get_iter")]
    fn get_iter(&self, start_time: Option<SystemTime>) -> PixbufAnimationIter;
}

impl<T: IsA<PixbufAnimation>> PixbufAnimationExtManual for T {
    fn get_iter(&self, start_time: Option<SystemTime>) -> PixbufAnimationIter {
        let start_time = start_time.map(|s| {
            let diff = s
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("failed to convert time");
            glib::ffi::GTimeVal {
                tv_sec: diff.as_secs() as _,
                tv_usec: diff.subsec_micros() as _,
            }
        });

        unsafe {
            from_glib_full(ffi::gdk_pixbuf_animation_get_iter(
                self.as_ref().to_glib_none().0,
                start_time
                    .as_ref()
                    .map(|t| t as *const glib::ffi::GTimeVal)
                    .unwrap_or(ptr::null()),
            ))
        }
    }
}

// Take a look at the license at the top of the repository in the LICENSE file.

use super::Pixbuf;
use glib::translate::*;

use std::time::SystemTime;

glib::wrapper! {
    pub struct PixbufAnimationIter(Object<ffi::GdkPixbufAnimationIter>);

    match fn {
        get_type => || ffi::gdk_pixbuf_animation_iter_get_type(),
    }
}

impl PixbufAnimationIter {
    #[doc(alias = "gdk_pixbuf_animation_iter_advance")]
    pub fn advance(&self, start_time: SystemTime) -> bool {
        let diff = start_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("failed to convert time");

        unsafe {
            from_glib(ffi::gdk_pixbuf_animation_iter_advance(
                self.to_glib_none().0,
                &glib::ffi::GTimeVal {
                    tv_sec: diff.as_secs() as _,
                    tv_usec: diff.subsec_micros() as _,
                },
            ))
        }
    }

    #[doc(alias = "gdk_pixbuf_animation_iter_get_pixbuf")]
    pub fn get_pixbuf(&self) -> Pixbuf {
        unsafe {
            from_glib_none(ffi::gdk_pixbuf_animation_iter_get_pixbuf(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_pixbuf_animation_iter_get_delay_time")]
    pub fn get_delay_time(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_animation_iter_get_delay_time(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_pixbuf_animation_iter_on_currently_loading_frame")]
    pub fn on_currently_loading_frame(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_pixbuf_animation_iter_on_currently_loading_frame(
                self.to_glib_none().0,
            ))
        }
    }
}

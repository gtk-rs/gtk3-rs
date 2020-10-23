// Copyright 2013-2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use super::Pixbuf;
use gdk_pixbuf_sys;
use glib::translate::*;

use std::time::SystemTime;

glib_wrapper! {
    pub struct PixbufAnimationIter(Object<gdk_pixbuf_sys::GdkPixbufAnimationIter, PixbufAnimationIterClass>);

    match fn {
        get_type => || gdk_pixbuf_sys::gdk_pixbuf_animation_iter_get_type(),
    }
}

impl PixbufAnimationIter {
    pub fn advance(&self, start_time: SystemTime) -> bool {
        let diff = start_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("failed to convert time");

        unsafe {
            from_glib(gdk_pixbuf_sys::gdk_pixbuf_animation_iter_advance(
                self.to_glib_none().0,
                &glib_sys::GTimeVal {
                    tv_sec: diff.as_secs() as _,
                    tv_usec: diff.subsec_micros() as _,
                },
            ))
        }
    }

    pub fn get_pixbuf(&self) -> Pixbuf {
        unsafe {
            from_glib_none(gdk_pixbuf_sys::gdk_pixbuf_animation_iter_get_pixbuf(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_delay_time(&self) -> i32 {
        unsafe { gdk_pixbuf_sys::gdk_pixbuf_animation_iter_get_delay_time(self.to_glib_none().0) }
    }

    pub fn on_currently_loading_frame(&self) -> bool {
        unsafe {
            from_glib(
                gdk_pixbuf_sys::gdk_pixbuf_animation_iter_on_currently_loading_frame(
                    self.to_glib_none().0,
                ),
            )
        }
    }
}

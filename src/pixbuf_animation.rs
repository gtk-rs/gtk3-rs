// Copyright 2013-2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use PixbufAnimation;
use PixbufAnimationIter;
use glib::object::IsA;
use glib::translate::*;
use glib::TimeVal;

pub trait PixbufAnimationExtManual {
    fn get_iter(&self, start_time: Option<TimeVal>) -> PixbufAnimationIter;
}

impl<T: IsA<PixbufAnimation>> PixbufAnimationExtManual for T {
    fn get_iter(&self, start_time: Option<TimeVal>) -> PixbufAnimationIter {
        unsafe {
            from_glib_full(gdk_pixbuf_sys::gdk_pixbuf_animation_get_iter(
                self.as_ref().to_glib_none().0,
                start_time.as_ref().to_glib_none().0,
            ))
        }
    }
}

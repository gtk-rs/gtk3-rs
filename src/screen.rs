// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Screen;
use ffi;
use cairo;

use glib;
use glib::object::IsA;
use glib::translate::*;

pub trait ScreenExtManual: 'static {
    fn get_font_options(&self) -> Option<cairo::FontOptions>;

    fn get_setting(&self, name: &str) -> Option<glib::Value>;
}

impl<O: IsA<Screen>> ScreenExtManual for O {
    fn get_font_options(&self) -> Option<cairo::FontOptions> {
        unsafe {
            from_glib_none(mut_override(ffi::gdk_screen_get_font_options(self.as_ref().to_glib_none().0)))
        }
    }

    fn get_setting(&self, name: &str) -> Option<glib::Value> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let done: bool = from_glib(ffi::gdk_screen_get_setting(self.as_ref().to_glib_none().0,
                                                                   name.to_glib_none().0,
                                                                   value.to_glib_none_mut().0));

            if done == true {
                Some(value)
            } else {
                None
            }
        }
    }
}

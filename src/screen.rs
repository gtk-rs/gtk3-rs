// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use cairo;
use gdk_sys;
use glib;
use glib::translate::*;
use Screen;

impl Screen {
    pub fn get_font_options(&self) -> Option<cairo::FontOptions> {
        unsafe {
            from_glib_none(mut_override(gdk_sys::gdk_screen_get_font_options(
                self.to_glib_none().0,
            )))
        }
    }

    pub fn get_setting(&self, name: &str) -> Option<glib::Value> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let done: bool = from_glib(gdk_sys::gdk_screen_get_setting(
                self.to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none_mut().0,
            ));

            if done == true {
                Some(value)
            } else {
                None
            }
        }
    }
}

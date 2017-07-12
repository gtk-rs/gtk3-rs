// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Screen;
use ffi;
use cairo;

use glib::translate::*;

impl Screen {
    pub fn get_font_options(&self) -> Option<cairo::FontOptions> {
        unsafe {
            from_glib_none(mut_override(ffi::gdk_screen_get_font_options(self.to_glib_none().0)))
        }
    }
}

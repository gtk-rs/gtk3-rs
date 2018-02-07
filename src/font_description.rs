// Copyright 2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi;
use FontDescription;

impl FontDescription {
    pub fn set_family_static(&mut self, family: &'static str) {
        unsafe {
            ffi::pango_font_description_set_family_static(self.to_glib_none_mut().0, family.to_glib_none().0);
        }
    }
}

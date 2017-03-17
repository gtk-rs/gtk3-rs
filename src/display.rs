// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;
use atom::Atom;
use Display;

impl Display {
    pub fn request_selection_notification(&self, selection: &Atom) -> bool {
        unsafe {
            from_glib(
                ffi::gdk_display_request_selection_notification(self.to_glib_none().0,
                    selection.to_glib_none().0))
        }
    }
}

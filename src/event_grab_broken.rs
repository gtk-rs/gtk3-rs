// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;

#[derive(Clone, Debug)]
pub struct EventGrabBroken(::Event);

event_wrapper!(EventGrabBroken, GdkEventGrabBroken);
event_subtype!(EventGrabBroken, GrabBroken);

impl EventGrabBroken {
    pub fn is_keyboard(&self) -> bool {
        from_glib(self.as_ref().keyboard)
    }

    pub fn is_implicit(&self) -> bool {
        from_glib(self.as_ref().implicit)
    }

    pub fn get_grab_window(&self) -> Option<::Window> {
        unsafe { from_glib_none(self.as_ref().grab_window) }
    }
}

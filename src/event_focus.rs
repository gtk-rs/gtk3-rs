// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventFocus(::Event);

event_wrapper!(EventFocus, GdkEventFocus);
event_subtype!(EventFocus, gdk_sys::GDK_FOCUS_CHANGE);

impl EventFocus {
    pub fn get_in(&self) -> bool {
        from_glib(self.as_ref().in_ as _)
    }
}

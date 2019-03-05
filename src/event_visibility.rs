// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventVisibility(::Event);

event_wrapper!(EventVisibility, GdkEventVisibility);
event_subtype!(EventVisibility, gdk_sys::GDK_VISIBILITY_NOTIFY);

impl EventVisibility {
    pub fn get_state(&self) -> ::VisibilityState {
        from_glib(self.as_ref().state)
    }
}

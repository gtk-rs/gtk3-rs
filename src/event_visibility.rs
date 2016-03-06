// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;

#[derive(Clone, Debug)]
pub struct EventVisibility(::Event);

event_wrapper!(EventVisibility, GdkEventVisibility);
event_subtype!(EventVisibility, VisibilityNotify);

impl EventVisibility {
    pub fn get_state(&self) -> ::VisibilityState {
        self.as_ref().state
    }
}

// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;

#[derive(Clone, Debug)]
pub struct EventWindowState(::Event);

event_wrapper!(EventWindowState, GdkEventWindowState);
event_subtype!(EventWindowState, WindowState);

impl EventWindowState {
    pub fn get_changed_mask(&self) -> ::WindowState {
        self.as_ref().changed_mask
    }

    pub fn get_new_window_state(&self) -> ::WindowState {
        self.as_ref().new_window_state
    }
}

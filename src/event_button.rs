// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;

#[derive(Clone, Debug)]
pub struct EventButton(::Event);

event_wrapper!(EventButton, GdkEventButton);
event_subtype!(EventButton, ButtonPress | DoubleButtonPress | TripleButtonPress | ButtonRelease);

impl EventButton {
    pub fn get_position(&self) -> (f64, f64) {
        let x = self.as_ref().x;
        let y = self.as_ref().y;
        (x, y)
    }

    pub fn get_state(&self) -> ::ModifierType {
        from_glib(self.as_ref().state)
    }

    pub fn get_time(&self) -> u32 {
        self.as_ref().time
    }

    pub fn get_button(&self) -> u32 {
        self.as_ref().button
    }
}

// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;

#[derive(Clone, Debug)]
pub struct EventConfigure(::Event);

event_wrapper!(EventConfigure, GdkEventConfigure);
event_subtype!(EventConfigure, Configure);

impl EventConfigure {
    pub fn get_position(&self) -> (i32, i32) {
        (self.as_ref().x, self.as_ref().y)
    }

    pub fn get_size(&self) -> (u32, u32) {
        let width = self.as_ref().width;
        let height = self.as_ref().height;
        assert!(width >= 0 && height >= 0, "Unexpected negative value");
        (width as u32, height as u32)
    }
}

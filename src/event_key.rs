// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;

#[derive(Clone, Debug)]
pub struct EventKey(::Event);

event_wrapper!(EventKey, GdkEventKey);
event_subtype!(EventKey, KeyPress | KeyRelease);

impl EventKey {
    pub fn get_time(&self) -> u32 {
        self.as_ref().time
    }

    pub fn get_state(&self) -> ::ModifierType {
        from_glib(self.as_ref().state)
    }

    pub fn get_keyval(&self) -> ::enums::key::Key {
        self.as_ref().keyval as ::enums::key::Key
    }

    pub fn get_length(&self) -> u32 {
        let length = self.as_ref().length;
        assert!(length >= 0, "Unexpected negative value");
        length as u32
    }

    pub fn get_hardware_keycode(&self) -> u16 {
        self.as_ref().hardware_keycode
    }

    pub fn get_group(&self) -> u8 {
        self.as_ref().group
    }

    // TODO: add getter for is_modifier
}

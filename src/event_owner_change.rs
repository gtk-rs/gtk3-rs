// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;

#[derive(Clone, Debug)]
pub struct EventOwnerChange(::Event);

event_wrapper!(EventOwnerChange, GdkEventOwnerChange);
event_subtype!(EventOwnerChange, OwnerChange);

impl EventOwnerChange {
    pub fn get_owner(&self) -> Option<::Window> {
        unsafe { from_glib_none(self.as_ref().owner) }
    }

    pub fn get_reason(&self) -> ::OwnerChange {
        self.as_ref().reason
    }

    pub fn get_selection(&self) -> ::Atom {
        unsafe { from_glib_none(self.as_ref().selection) }
    }

    pub fn get_time(&self) -> u32 {
        self.as_ref().time
    }

    pub fn get_selection_time(&self) -> u32 {
        self.as_ref().selection_time
    }
}

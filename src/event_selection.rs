// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;

#[derive(Clone, Debug)]
pub struct EventSelection(::Event);

event_wrapper!(EventSelection, GdkEventSelection);
event_subtype!(EventSelection, SelectionClear | SelectionNotify | SelectionRequest);

impl EventSelection {
    pub fn get_selection(&self) -> ::Atom {
        unsafe { from_glib_none(self.as_ref().selection) }
    }

    pub fn get_target(&self) -> ::Atom {
        unsafe { from_glib_none(self.as_ref().target) }
    }

    pub fn get_property(&self) -> ::Atom {
        unsafe { from_glib_none(self.as_ref().property) }
    }

    pub fn get_time(&self) -> u32 {
        self.as_ref().time
    }

    pub fn get_requestor(&self) -> Option<::Window> {
        unsafe { from_glib_none(self.as_ref().requestor) }
    }
}

// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventSelection(::Event);

event_wrapper!(EventSelection, GdkEventSelection);
event_subtype!(
    EventSelection,
    gdk_sys::GDK_SELECTION_CLEAR | gdk_sys::GDK_SELECTION_NOTIFY | gdk_sys::GDK_SELECTION_REQUEST
);

impl EventSelection {
    pub fn get_selection(&self) -> ::Atom {
        unsafe { from_glib_none(self.as_ref().selection as *mut _) }
    }

    pub fn get_target(&self) -> ::Atom {
        unsafe { from_glib_none(self.as_ref().target as *mut _) }
    }

    pub fn get_property(&self) -> ::Atom {
        unsafe { from_glib_none(self.as_ref().property as *mut _) }
    }

    pub fn get_time(&self) -> u32 {
        self.as_ref().time
    }

    pub fn get_requestor(&self) -> Option<::Window> {
        unsafe { from_glib_none(self.as_ref().requestor) }
    }
}

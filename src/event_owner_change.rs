// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventOwnerChange(::Event);

event_wrapper!(EventOwnerChange, GdkEventOwnerChange);
event_subtype!(EventOwnerChange, gdk_sys::GDK_OWNER_CHANGE);

impl EventOwnerChange {
    pub fn get_owner(&self) -> Option<::Window> {
        unsafe { from_glib_none(self.as_ref().owner) }
    }

    pub fn get_reason(&self) -> ::OwnerChange {
        from_glib(self.as_ref().reason)
    }

    pub fn get_selection(&self) -> ::Atom {
        unsafe { from_glib_none(self.as_ref().selection as *mut _) }
    }

    pub fn get_time(&self) -> u32 {
        self.as_ref().time
    }

    pub fn get_selection_time(&self) -> u32 {
        self.as_ref().selection_time
    }
}

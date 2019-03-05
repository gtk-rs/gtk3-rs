// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventProperty(::Event);

event_wrapper!(EventProperty, GdkEventProperty);
event_subtype!(EventProperty, gdk_sys::GDK_PROPERTY_NOTIFY);

impl EventProperty {
    pub fn get_atom(&self) -> ::Atom {
        unsafe { from_glib_none(self.as_ref().atom) }
    }

    pub fn get_time(&self) -> u32 {
        self.as_ref().time
    }

    pub fn get_state(&self) -> ::PropertyState {
        from_glib(self.as_ref().state)
    }
}

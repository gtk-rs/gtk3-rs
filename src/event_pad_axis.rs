// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventPadAxis(::Event);

event_wrapper!(EventPadAxis, GdkEventPadAxis);
event_subtype!(EventPadAxis, gdk_sys::GDK_PAD_RING | gdk_sys::GDK_PAD_STRIP);

impl EventPadAxis {
    pub fn get_time(&self) -> u32 {
        self.as_ref().time
    }

    pub fn get_group(&self) -> u32 {
        self.as_ref().group
    }

    pub fn get_index(&self) -> u32 {
        self.as_ref().index
    }

    pub fn get_mode(&self) -> u32 {
        self.as_ref().mode
    }

    pub fn get_value(&self) -> f64 {
        self.as_ref().value
    }
}

// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventPadButton(::Event);

event_wrapper!(EventPadButton, GdkEventPadButton);
event_subtype!(
    EventPadButton,
    gdk_sys::GDK_PAD_BUTTON_PRESS | gdk_sys::GDK_PAD_BUTTON_RELEASE
);

impl EventPadButton {
    pub fn get_time(&self) -> u32 {
        self.as_ref().time
    }

    pub fn get_group(&self) -> u32 {
        self.as_ref().group
    }

    pub fn get_button(&self) -> u32 {
        self.as_ref().button
    }

    pub fn get_mode(&self) -> u32 {
        self.as_ref().mode
    }
}

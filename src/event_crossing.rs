// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventCrossing(::Event);

event_wrapper!(EventCrossing, GdkEventCrossing);
event_subtype!(
    EventCrossing,
    gdk_sys::GDK_ENTER_NOTIFY | gdk_sys::GDK_LEAVE_NOTIFY
);

impl EventCrossing {
    pub fn get_position(&self) -> (f64, f64) {
        let x = self.as_ref().x;
        let y = self.as_ref().y;
        (x, y)
    }

    pub fn get_subwindow(&self) -> Option<::Window> {
        unsafe { from_glib_none(self.as_ref().subwindow) }
    }

    pub fn get_mode(&self) -> ::CrossingMode {
        from_glib(self.as_ref().mode)
    }

    pub fn get_detail(&self) -> ::NotifyType {
        from_glib(self.as_ref().detail)
    }

    pub fn get_state(&self) -> ::ModifierType {
        from_glib(self.as_ref().state)
    }

    pub fn get_time(&self) -> u32 {
        self.as_ref().time
    }

    pub fn get_root(&self) -> (f64, f64) {
        let x_root = self.as_ref().x_root;
        let y_root = self.as_ref().y_root;
        (x_root, y_root)
    }

    pub fn get_focus(&self) -> bool {
        from_glib(self.as_ref().focus)
    }
}

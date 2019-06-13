// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventTouch(::Event);

event_wrapper!(EventTouch, GdkEventTouch);
event_subtype!(
    EventTouch,
    gdk_sys::GDK_TOUCH_BEGIN
        | gdk_sys::GDK_TOUCH_UPDATE
        | gdk_sys::GDK_TOUCH_END
        | gdk_sys::GDK_TOUCH_CANCEL
);

impl EventTouch {
    pub fn get_time(&self) -> u32 {
        self.as_ref().time
    }

    pub fn get_position(&self) -> (f64, f64) {
        let x = self.as_ref().x;
        let y = self.as_ref().y;
        (x, y)
    }

    pub fn get_state(&self) -> ::ModifierType {
        from_glib(self.as_ref().state)
    }

    pub fn is_emulating_pointer(&self) -> bool {
        from_glib(self.as_ref().emulating_pointer)
    }

    pub fn get_device(&self) -> Option<::Device> {
        unsafe { from_glib_none(self.as_ref().device) }
    }

    pub fn get_axes(&self) -> Option<(f64, f64)> {
        let axes = self.as_ref().axes;

        if axes.is_null() {
            None
        } else {
            unsafe { Some((*axes, *axes.offset(1))) }
        }
    }

    pub fn get_root(&self) -> (f64, f64) {
        let x_root = self.as_ref().x_root;
        let y_root = self.as_ref().y_root;
        (x_root, y_root)
    }

    pub fn get_event_sequence(&self) -> Option<::EventSequence> {
        unsafe { from_glib_none(self.as_ref().sequence) }
    }
}

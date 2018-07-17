// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi;

#[derive(Clone, Debug)]
pub struct EventMotion(::Event);

event_wrapper!(EventMotion, GdkEventMotion);
event_subtype!(EventMotion, ffi::GDK_MOTION_NOTIFY);

impl EventMotion {
    pub fn get_position(&self) -> (f64, f64) {
        let x = self.as_ref().x;
        let y = self.as_ref().y;
        (x, y)
    }

    pub fn get_state(&self) -> ::ModifierType {
        from_glib(self.as_ref().state)
    }

    pub fn get_time(&self) -> u32 {
        self.as_ref().time
    }

    pub fn request_motions(&self) {
        unsafe { ffi::gdk_event_request_motions(self.as_ref()) }
    }

    pub fn get_device(&self) -> Option<::Device> {
        unsafe { from_glib_none(self.as_ref().device) }
    }
}

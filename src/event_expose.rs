// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use cairo;
use gdk_sys;
use glib::translate::*;
use Rectangle;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventExpose(::Event);

event_wrapper!(EventExpose, GdkEventExpose);
event_subtype!(EventExpose, gdk_sys::GDK_EXPOSE | gdk_sys::GDK_DAMAGE);

impl EventExpose {
    pub fn get_region(&self) -> Option<cairo::Region> {
        unsafe { from_glib_none(self.as_ref().region) }
    }

    pub fn get_count(&self) -> u32 {
        self.as_ref().count as u32
    }

    pub fn get_area(&self) -> Rectangle {
        unsafe { from_glib_none(&self.as_ref().area as *const _) }
    }
}

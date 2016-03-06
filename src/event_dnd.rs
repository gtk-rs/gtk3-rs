// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;

#[derive(Clone, Debug)]
pub struct EventDND(::Event);

event_wrapper!(EventDND, GdkEventDND);
event_subtype!(EventDND, DragEnter | DragLeave | DragMotion | DragStatus | DropStart | DropFinished);

impl EventDND {
    pub fn get_context(&self) -> Option<::DragContext> {
        unsafe { from_glib_none(self.as_ref().context) }
    }

    pub fn get_time(&self) -> u32 {
        self.as_ref().time
    }
}

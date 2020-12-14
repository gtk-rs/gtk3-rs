// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventWindowState(crate::Event);

event_wrapper!(EventWindowState, GdkEventWindowState);
event_subtype!(EventWindowState, ffi::GDK_WINDOW_STATE);

impl EventWindowState {
    pub fn get_changed_mask(&self) -> crate::WindowState {
        unsafe { from_glib(self.as_ref().changed_mask) }
    }

    pub fn get_new_window_state(&self) -> crate::WindowState {
        unsafe { from_glib(self.as_ref().new_window_state) }
    }
}

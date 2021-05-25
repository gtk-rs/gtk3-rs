// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

/// Generated when a pointer or keyboard grab is broken. On X11, this happens
/// when the grab window becomes unviewable (i.e. it or one of its ancestors
/// is unmapped), or if the same application grabs the pointer or keyboard
/// again. Note that implicit grabs (which are initiated by button presses)
/// can also cause [EventGrabBroken](crate::EventGrabBroken) events.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventGrabBroken(crate::Event);

event_wrapper!(EventGrabBroken, GdkEventGrabBroken);
event_subtype!(EventGrabBroken, ffi::GDK_GRAB_BROKEN);

impl EventGrabBroken {
    pub fn is_keyboard(&self) -> bool {
        unsafe { from_glib(self.as_ref().keyboard) }
    }

    pub fn is_implicit(&self) -> bool {
        unsafe { from_glib(self.as_ref().implicit) }
    }

    #[doc(alias = "get_grab_window")]
    pub fn grab_window(&self) -> Option<crate::Window> {
        unsafe { from_glib_none(self.as_ref().grab_window) }
    }
}

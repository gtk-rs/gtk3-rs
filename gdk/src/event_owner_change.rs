// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventOwnerChange(crate::Event);

event_wrapper!(EventOwnerChange, GdkEventOwnerChange);
event_subtype!(EventOwnerChange, ffi::GDK_OWNER_CHANGE);

impl EventOwnerChange {
    pub fn get_owner(&self) -> Option<crate::Window> {
        unsafe { from_glib_none(self.as_ref().owner) }
    }

    pub fn get_reason(&self) -> crate::OwnerChange {
        unsafe { from_glib(self.as_ref().reason) }
    }

    pub fn get_selection(&self) -> crate::Atom {
        unsafe { from_glib_none(self.as_ref().selection as *mut _) }
    }

    pub fn get_time(&self) -> u32 {
        self.as_ref().time
    }

    pub fn get_selection_time(&self) -> u32 {
        self.as_ref().selection_time
    }
}

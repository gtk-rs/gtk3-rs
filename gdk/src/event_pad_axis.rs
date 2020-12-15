// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventPadAxis(crate::Event);

event_wrapper!(EventPadAxis, GdkEventPadAxis);
event_subtype!(EventPadAxis, ffi::GDK_PAD_RING | ffi::GDK_PAD_STRIP);

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

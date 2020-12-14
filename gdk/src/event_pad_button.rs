// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventPadButton(crate::Event);

event_wrapper!(EventPadButton, GdkEventPadButton);
event_subtype!(
    EventPadButton,
    ffi::GDK_PAD_BUTTON_PRESS | ffi::GDK_PAD_BUTTON_RELEASE
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

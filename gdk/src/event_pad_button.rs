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
    pub fn time(&self) -> u32 {
        self.as_ref().time
    }

    pub fn group(&self) -> u32 {
        self.as_ref().group
    }

    pub fn button(&self) -> u32 {
        self.as_ref().button
    }

    pub fn mode(&self) -> u32 {
        self.as_ref().mode
    }
}

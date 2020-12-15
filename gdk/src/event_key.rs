// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventKey(crate::Event);

event_wrapper!(EventKey, GdkEventKey);
event_subtype!(EventKey, ffi::GDK_KEY_PRESS | ffi::GDK_KEY_RELEASE);

impl EventKey {
    pub fn get_time(&self) -> u32 {
        self.as_ref().time
    }

    pub fn get_state(&self) -> crate::ModifierType {
        unsafe { from_glib(self.as_ref().state) }
    }

    pub fn get_keyval(&self) -> crate::keys::Key {
        unsafe { from_glib(self.as_ref().keyval) }
    }

    pub fn get_length(&self) -> u32 {
        let length = self.as_ref().length;
        assert!(length >= 0, "Unexpected negative value");
        length as u32
    }

    pub fn get_hardware_keycode(&self) -> u16 {
        self.as_ref().hardware_keycode
    }

    pub fn get_group(&self) -> u8 {
        self.as_ref().group
    }

    pub fn get_is_modifier(&self) -> bool {
        self.as_ref().is_modifier & 1 != 0
    }
}

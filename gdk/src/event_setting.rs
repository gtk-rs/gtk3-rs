// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use glib::GString;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventSetting(crate::Event);

event_wrapper!(EventSetting, GdkEventSetting);
event_subtype!(EventSetting, ffi::GDK_SETTING);

impl EventSetting {
    pub fn name(&self) -> Option<GString> {
        unsafe { from_glib_none(self.as_ref().name) }
    }

    pub fn action(&self) -> crate::SettingAction {
        unsafe { from_glib(self.as_ref().action) }
    }
}

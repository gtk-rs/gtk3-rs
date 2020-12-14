// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventTouch(crate::Event);

event_wrapper!(EventTouch, GdkEventTouch);
event_subtype!(
    EventTouch,
    ffi::GDK_TOUCH_BEGIN | ffi::GDK_TOUCH_UPDATE | ffi::GDK_TOUCH_END | ffi::GDK_TOUCH_CANCEL
);

impl EventTouch {
    pub fn get_time(&self) -> u32 {
        self.as_ref().time
    }

    pub fn get_position(&self) -> (f64, f64) {
        let x = self.as_ref().x;
        let y = self.as_ref().y;
        (x, y)
    }

    pub fn get_state(&self) -> crate::ModifierType {
        unsafe { from_glib(self.as_ref().state) }
    }

    pub fn is_emulating_pointer(&self) -> bool {
        unsafe { from_glib(self.as_ref().emulating_pointer) }
    }

    pub fn get_device(&self) -> Option<crate::Device> {
        unsafe { from_glib_none(self.as_ref().device) }
    }

    pub fn get_axes(&self) -> Option<(f64, f64)> {
        let axes = self.as_ref().axes;

        if axes.is_null() {
            None
        } else {
            unsafe { Some((*axes, *axes.offset(1))) }
        }
    }

    pub fn get_root(&self) -> (f64, f64) {
        let x_root = self.as_ref().x_root;
        let y_root = self.as_ref().y_root;
        (x_root, y_root)
    }

    pub fn get_event_sequence(&self) -> Option<crate::EventSequence> {
        unsafe { from_glib_none(self.as_ref().sequence) }
    }
}

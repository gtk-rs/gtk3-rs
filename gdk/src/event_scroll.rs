// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventScroll(crate::Event);

event_wrapper!(EventScroll, GdkEventScroll);
event_subtype!(EventScroll, ffi::GDK_SCROLL);

impl EventScroll {
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

    pub fn get_device(&self) -> Option<crate::Device> {
        unsafe { from_glib_none(self.as_ref().device) }
    }

    pub fn get_direction(&self) -> crate::ScrollDirection {
        unsafe { from_glib(self.as_ref().direction) }
    }

    pub fn get_root(&self) -> (f64, f64) {
        let x_root = self.as_ref().x_root;
        let y_root = self.as_ref().y_root;
        (x_root, y_root)
    }

    pub fn get_delta(&self) -> (f64, f64) {
        let dx = self.as_ref().delta_x;
        let dy = self.as_ref().delta_y;
        (dx, dy)
    }

    pub fn get_is_stop(&self) -> bool {
        self.as_ref().is_stop & 1 != 0
    }
}

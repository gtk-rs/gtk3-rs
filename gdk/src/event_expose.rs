// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Rectangle;
use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventExpose(crate::Event);

event_wrapper!(EventExpose, GdkEventExpose);
event_subtype!(EventExpose, ffi::GDK_EXPOSE | ffi::GDK_DAMAGE);

impl EventExpose {
    pub fn get_region(&self) -> Option<cairo::Region> {
        unsafe { from_glib_none(self.as_ref().region) }
    }

    pub fn get_count(&self) -> u32 {
        self.as_ref().count as u32
    }

    pub fn get_area(&self) -> Rectangle {
        unsafe { from_glib_none(&self.as_ref().area as *const _) }
    }
}

// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

/// Proximity events are generated when using GDK’s wrapper for the
/// XInput extension. The XInput extension is an add-on for standard X
/// that allows you to use nonstandard devices such as graphics tablets.
/// A proximity event indicates that the stylus has moved in or out of
/// contact with the tablet, or perhaps that the user’s finger has moved
/// in or out of contact with a touch screen.
///
/// This event type will be used pretty rarely. It only is important for
/// XInput aware programs that are drawing their own cursor.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventProximity(crate::Event);

event_wrapper!(EventProximity, GdkEventProximity);
event_subtype!(
    EventProximity,
    ffi::GDK_PROXIMITY_IN | ffi::GDK_PROXIMITY_OUT
);

impl EventProximity {
    #[doc(alias = "get_time")]
    pub fn time(&self) -> u32 {
        self.as_ref().time
    }

    #[doc(alias = "get_device")]
    pub fn device(&self) -> Option<crate::Device> {
        unsafe { from_glib_none(self.as_ref().device) }
    }
}

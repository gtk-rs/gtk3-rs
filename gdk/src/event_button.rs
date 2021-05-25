// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

/// Used for button press and button release events. The
/// `type` field will be one of [EventType::ButtonPress](crate::EventType::ButtonPress),
/// [EventType::_2buttonPress](crate::EventType::_2buttonPress), [EventType::_3buttonPress](crate::EventType::_3buttonPress) or [EventType::ButtonRelease](crate::EventType::ButtonRelease),
///
/// Double and triple-clicks result in a sequence of events being received.
/// For double-clicks the order of events will be:
///
/// - [EventType::ButtonPress](crate::EventType::ButtonPress)
/// - [EventType::ButtonRelease](crate::EventType::ButtonRelease)
/// - [EventType::ButtonPress](crate::EventType::ButtonPress)
/// - [EventType::_2buttonPress](crate::EventType::_2buttonPress)
/// - [EventType::ButtonRelease](crate::EventType::ButtonRelease)
///
/// Note that the first click is received just like a normal
/// button press, while the second click results in a [EventType::_2buttonPress](crate::EventType::_2buttonPress)
/// being received just after the [EventType::ButtonPress](crate::EventType::ButtonPress).
///
/// Triple-clicks are very similar to double-clicks, except that
/// [EventType::_3buttonPress](crate::EventType::_3buttonPress) is inserted after the third click. The order of the
/// events is:
///
/// - [EventType::ButtonPress](crate::EventType::ButtonPress)
/// - [EventType::ButtonRelease](crate::EventType::ButtonRelease)
/// - [EventType::ButtonPress](crate::EventType::ButtonPress)
/// - [EventType::_2buttonPress](crate::EventType::_2buttonPress)
/// - [EventType::ButtonRelease](crate::EventType::ButtonRelease)
/// - [EventType::ButtonPress](crate::EventType::ButtonPress)
/// - [EventType::_3buttonPress](crate::EventType::_3buttonPress)
/// - [EventType::ButtonRelease](crate::EventType::ButtonRelease)
///
/// For a double click to occur, the second button press must occur within
/// 1/4 of a second of the first. For a triple click to occur, the third
/// button press must also occur within 1/2 second of the first button press.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventButton(crate::Event);

event_wrapper!(EventButton, GdkEventButton);
event_subtype!(
    EventButton,
    ffi::GDK_BUTTON_PRESS
        | ffi::GDK_DOUBLE_BUTTON_PRESS
        | ffi::GDK_TRIPLE_BUTTON_PRESS
        | ffi::GDK_BUTTON_RELEASE
);

impl EventButton {
    #[doc(alias = "get_position")]
    pub fn position(&self) -> (f64, f64) {
        let x = self.as_ref().x;
        let y = self.as_ref().y;
        (x, y)
    }

    #[doc(alias = "get_state")]
    pub fn state(&self) -> crate::ModifierType {
        unsafe { from_glib(self.as_ref().state) }
    }

    #[doc(alias = "get_time")]
    pub fn time(&self) -> u32 {
        self.as_ref().time
    }

    #[doc(alias = "get_button")]
    pub fn button(&self) -> u32 {
        self.as_ref().button
    }

    #[doc(alias = "get_device")]
    pub fn device(&self) -> Option<crate::Device> {
        unsafe { from_glib_none(self.as_ref().device) }
    }

    #[doc(alias = "get_axes")]
    pub fn axes(&self) -> Option<(f64, f64)> {
        let axes = self.as_ref().axes;

        if axes.is_null() {
            None
        } else {
            unsafe { Some((*axes, *axes.offset(1))) }
        }
    }

    #[doc(alias = "get_root")]
    pub fn root(&self) -> (f64, f64) {
        let x_root = self.as_ref().x_root;
        let y_root = self.as_ref().y_root;
        (x_root, y_root)
    }
}

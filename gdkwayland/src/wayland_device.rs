// Take a look at the license at the top of the repository in the LICENSE file.

use gdk::prelude::*;
use gdk::DevicePadFeature;
use glib::translate::*;

use wayland_client::backend::ObjectId;
use wayland_client::protocol::{wl_keyboard::WlKeyboard, wl_pointer::WlPointer, wl_seat::WlSeat};
use wayland_client::Proxy;

glib::wrapper! {
    #[doc(alias = "GdkWaylandDevice")]
    pub struct WaylandDevice(Object<ffi::GdkWaylandDevice>) @extends gdk::Device;

    match fn {
        type_ => || ffi::gdk_wayland_device_get_type(),
    }
}

impl WaylandDevice {
    #[doc(alias = "gdk_wayland_device_pad_set_feedback")]
    pub fn pad_set_feedback(&self, element: DevicePadFeature, idx: usize, label: &str) {
        unsafe {
            ffi::gdk_wayland_device_pad_set_feedback(
                self.to_glib_none().0,
                element.into_glib(),
                idx as _,
                label.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "gdk_wayland_device_get_wl_seat")]
    #[doc(alias = "get_wl_seat")]
    pub fn wl_seat(&self) -> Option<WlSeat> {
        unsafe {
            let seat_ptr = ffi::gdk_wayland_device_get_wl_seat(self.to_glib_none().0);
            if seat_ptr.is_null() {
                None
            } else {
                let display = self.display().unsafe_cast::<crate::WaylandDisplay>();
                let cnx = display.connection();
                let id = ObjectId::from_ptr(WlSeat::interface(), seat_ptr as *mut _).unwrap();

                WlSeat::from_id(&cnx, id).ok()
            }
        }
    }

    #[doc(alias = "gdk_wayland_device_get_wl_keyboard")]
    #[doc(alias = "get_wl_keyboard")]
    pub fn wl_keyboard(&self) -> Option<WlKeyboard> {
        let display = self.display().downcast::<crate::WaylandDisplay>().unwrap();
        unsafe {
            let keyboard_ptr = ffi::gdk_wayland_device_get_wl_keyboard(self.to_glib_none().0);
            if keyboard_ptr.is_null() {
                None
            } else {
                let cnx = display.connection();
                let id =
                    ObjectId::from_ptr(WlKeyboard::interface(), keyboard_ptr as *mut _).unwrap();

                WlKeyboard::from_id(&cnx, id).ok()
            }
        }
    }

    #[doc(alias = "gdk_wayland_device_get_wl_pointer")]
    #[doc(alias = "get_wl_pointer")]
    pub fn wl_pointer(&self) -> Option<WlPointer> {
        let display = self.display().downcast::<crate::WaylandDisplay>().unwrap();
        unsafe {
            let pointer_ptr = ffi::gdk_wayland_device_get_wl_pointer(self.to_glib_none().0);
            if pointer_ptr.is_null() {
                None
            } else {
                let cnx = display.connection();
                let id = ObjectId::from_ptr(WlPointer::interface(), pointer_ptr as *mut _).unwrap();

                WlPointer::from_id(&cnx, id).ok()
            }
        }
    }
}

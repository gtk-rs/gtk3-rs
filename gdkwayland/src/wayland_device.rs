// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(any(feature = "v3_22", feature = "dox"))]
use gdk::DevicePadFeature;
use glib::translate::*;

use wayland_client::protocol::{wl_keyboard::WlKeyboard, wl_pointer::WlPointer, wl_seat::WlSeat};
use wayland_client::sys::client::wl_proxy;
use wayland_client::Proxy;

glib::wrapper! {
    #[doc(alias = "GdkWaylandDevice")]
    pub struct WaylandDevice(Object<ffi::GdkWaylandDevice>) @extends gdk::Device;

    match fn {
        type_ => || ffi::gdk_wayland_device_get_type(),
    }
}

impl WaylandDevice {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
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
    pub fn wl_seat(&self) -> WlSeat {
        unsafe {
            let ptr = ffi::gdk_wayland_device_get_wl_seat(self.to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }

    #[doc(alias = "gdk_wayland_device_get_wl_keyboard")]
    #[doc(alias = "get_wl_keyboard")]
    pub fn wl_keyboard(&self) -> WlKeyboard {
        unsafe {
            let ptr = ffi::gdk_wayland_device_get_wl_keyboard(self.to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }

    #[doc(alias = "gdk_wayland_device_get_wl_pointer")]
    #[doc(alias = "get_wl_pointer")]
    pub fn wl_pointer(&self) -> WlPointer {
        unsafe {
            let ptr = ffi::gdk_wayland_device_get_wl_pointer(self.to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }
}

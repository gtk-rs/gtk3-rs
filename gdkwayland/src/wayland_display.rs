// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use wayland_client::protocol::{wl_compositor::WlCompositor, wl_display::WlDisplay};
use wayland_client::sys::client::wl_proxy;
use wayland_client::Proxy;

glib::wrapper! {
    #[doc(alias = "GdkWaylandDisplay")]
    pub struct WaylandDisplay(Object<ffi::GdkWaylandDisplay>) @extends gdk::Display;

    match fn {
        type_ => || ffi::gdk_wayland_display_get_type(),
    }
}

impl WaylandDisplay {
    #[doc(alias = "gdk_wayland_display_get_wl_compositor")]
    #[doc(alias = "get_wl_compositor")]
    pub fn wl_compositor(&self) -> WlCompositor {
        unsafe {
            let ptr = ffi::gdk_wayland_display_get_wl_compositor(self.to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }

    #[doc(alias = "gdk_wayland_display_get_wl_display")]
    #[doc(alias = "get_wl_display")]
    pub fn wl_display(&self) -> WlDisplay {
        unsafe {
            let ptr = ffi::gdk_wayland_display_get_wl_display(self.to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }

    #[doc(alias = "gdk_wayland_display_set_cursor_theme")]
    pub fn set_cursor_theme(&self, theme: &str, size: isize) {
        unsafe {
            ffi::gdk_wayland_display_set_cursor_theme(
                self.to_glib_none().0,
                theme.to_glib_none().0,
                size as _,
            )
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    #[doc(alias = "gdk_wayland_display_set_startup_notification_id")]
    pub fn set_startup_notification_id(&self, startup_id: &str) {
        unsafe {
            ffi::gdk_wayland_display_set_startup_notification_id(
                self.to_glib_none().0,
                startup_id.to_glib_none().0,
            )
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    #[doc(alias = "gdk_wayland_display_prefers_ssd")]
    pub fn prefers_ssd(&self) -> bool {
        unsafe { from_glib(ffi::gdk_wayland_display_prefers_ssd(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    #[doc(alias = "gdk_wayland_display_query_registry")]
    pub fn query_registry(&self, global: &str) -> bool {
        unsafe {
            from_glib(ffi::gdk_wayland_display_query_registry(
                self.to_glib_none().0,
                global.to_glib_none().0,
            ))
        }
    }
}

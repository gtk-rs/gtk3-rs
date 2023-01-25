// Take a look at the license at the top of the repository in the LICENSE file.

use glib::once_cell::sync::Lazy;
use glib::translate::*;
use glib::{ObjectExt, Quark};

use wayland_client::backend::ObjectId;
use wayland_client::protocol::{wl_compositor::WlCompositor, wl_display::WlDisplay};
use wayland_client::Proxy;

static WAYLAND_DISPLAY_CONNECTION_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_str("gtk-rs-wayland-display-connection-quark"));

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
    pub fn wl_compositor(&self) -> Option<WlCompositor> {
        unsafe {
            let compositor_ptr = ffi::gdk_wayland_display_get_wl_compositor(self.to_glib_none().0);
            if compositor_ptr.is_null() {
                None
            } else {
                let cnx = self.connection();
                let id = ObjectId::from_ptr(WlCompositor::interface(), compositor_ptr as *mut _)
                    .unwrap();

                WlCompositor::from_id(&cnx, id).ok()
            }
        }
    }

    #[doc(alias = "gdk_wayland_display_get_wl_display")]
    #[doc(alias = "get_wl_display")]
    pub fn wl_display(&self) -> Option<WlDisplay> {
        unsafe {
            let display_ptr = ffi::gdk_wayland_display_get_wl_display(self.to_glib_none().0);
            if display_ptr.is_null() {
                None
            } else {
                let cnx = self.connection();
                let id = ObjectId::from_ptr(WlDisplay::interface(), display_ptr as *mut _).unwrap();

                WlDisplay::from_id(&cnx, id).ok()
            }
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

    #[doc(alias = "gdk_wayland_display_set_startup_notification_id")]
    pub fn set_startup_notification_id(&self, startup_id: &str) {
        unsafe {
            ffi::gdk_wayland_display_set_startup_notification_id(
                self.to_glib_none().0,
                startup_id.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "gdk_wayland_display_prefers_ssd")]
    pub fn prefers_ssd(&self) -> bool {
        unsafe { from_glib(ffi::gdk_wayland_display_prefers_ssd(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_wayland_display_query_registry")]
    pub fn query_registry(&self, global: &str) -> bool {
        unsafe {
            from_glib(ffi::gdk_wayland_display_query_registry(
                self.to_glib_none().0,
                global.to_glib_none().0,
            ))
        }
    }

    pub(crate) fn connection(&self) -> wayland_client::Connection {
        unsafe {
            match self
                .qdata::<Option<wayland_client::Connection>>(*WAYLAND_DISPLAY_CONNECTION_QUARK)
            {
                Some(conn) => conn.as_ref().clone().unwrap(),
                None => {
                    let display_ptr =
                        ffi::gdk_wayland_display_get_wl_display(self.to_glib_none().0);
                    let backend = wayland_backend::sys::client::Backend::from_foreign_display(
                        display_ptr as *mut _,
                    );
                    let conn = wayland_client::Connection::from_backend(backend);
                    self.set_qdata(*WAYLAND_DISPLAY_CONNECTION_QUARK, conn.clone());

                    conn
                }
            }
        }
    }
}

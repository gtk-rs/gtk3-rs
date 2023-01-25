// Take a look at the license at the top of the repository in the LICENSE file.

use gdk::prelude::*;
use glib::translate::ToGlibPtr;

use wayland_client::backend::ObjectId;
use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::Proxy;

glib::wrapper! {
    #[doc(alias = "GdkWaylandMonitor")]
    pub struct WaylandMonitor(Object<ffi::GdkWaylandMonitor>) @extends gdk::Monitor;

    match fn {
        type_ => || ffi::gdk_wayland_monitor_get_type(),
    }
}

impl WaylandMonitor {
    #[doc(alias = "gdk_wayland_monitor_get_wl_output")]
    #[doc(alias = "get_wl_output")]
    pub fn wl_output(&self) -> Option<WlOutput> {
        unsafe {
            let output_ptr = ffi::gdk_wayland_monitor_get_wl_output(self.to_glib_none().0);
            if output_ptr.is_null() {
                None
            } else {
                let display = self.display()?.unsafe_cast::<crate::WaylandDisplay>();
                let cnx = display.connection();
                let id = ObjectId::from_ptr(WlOutput::interface(), output_ptr as *mut _).unwrap();

                WlOutput::from_id(&cnx, id).ok()
            }
        }
    }
}

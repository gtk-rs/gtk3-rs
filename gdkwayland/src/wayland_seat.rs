// Take a look at the license at the top of the repository in the LICENSE file.

use gdk::prelude::*;
use glib::translate::ToGlibPtr;

use wayland_client::backend::ObjectId;
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_client::Proxy;

glib::wrapper! {
    #[doc(alias = "GdkWaylandSeat")]
    pub struct WaylandSeat(Object<ffi::GdkWaylandSeat>) @extends gdk::Seat;

    match fn {
        type_ => || ffi::gdk_wayland_seat_get_type(),
    }
}

impl WaylandSeat {
    #[doc(alias = "gdk_wayland_seat_get_wl_seat")]
    #[doc(alias = "get_wl_seat")]
    pub fn wl_seat(&self) -> Option<WlSeat> {
        unsafe {
            let seat_ptr = ffi::gdk_wayland_seat_get_wl_seat(self.to_glib_none().0);
            if seat_ptr.is_null() {
                None
            } else {
                let display = self.display()?.unsafe_cast::<crate::WaylandDisplay>();
                let cnx = display.connection();
                let id = ObjectId::from_ptr(WlSeat::interface(), seat_ptr as *mut _).unwrap();

                WlSeat::from_id(&cnx, id).ok()
            }
        }
    }
}

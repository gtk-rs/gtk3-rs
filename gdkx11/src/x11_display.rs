// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{X11Display, ffi};

use glib::translate::ToGlibPtr;
use x11::xlib;

impl X11Display {
    #[doc(alias = "gdk_x11_display_get_xdisplay")]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn xdisplay(&self) -> *mut xlib::Display {
        unsafe { ffi::gdk_x11_display_get_xdisplay(self.to_glib_none().0) }
    }
}

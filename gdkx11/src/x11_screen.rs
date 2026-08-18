// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{X11Screen, ffi};

use glib::translate::ToGlibPtr;
use x11::xlib;

impl X11Screen {
    #[doc(alias = "gdk_x11_screen_get_xscreen")]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn xscreen(&self) -> *mut xlib::Screen {
        unsafe { ffi::gdk_x11_screen_get_xscreen(self.to_glib_none().0) }
    }
}

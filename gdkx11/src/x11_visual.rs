// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{X11Visual, ffi};

use glib::translate::ToGlibPtr;
use x11::xlib;

impl X11Visual {
    #[doc(alias = "gdk_x11_visual_get_xvisual")]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn xvisual(&self) -> *mut xlib::Visual {
        unsafe { ffi::gdk_x11_visual_get_xvisual(self.to_glib_none().0) }
    }
}

// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Invisible;
use glib::translate::{from_glib_none, ToGlibPtr};
use glib::IsA;

// For some reasons, it's not generated...
pub trait InvisibleExtManual: 'static {
    #[doc(alias = "gtk_invisible_get_screen")]
    fn get_screen(&self) -> Option<gdk::Screen>;
}

impl<T: IsA<Invisible>> InvisibleExtManual for T {
    fn get_screen(&self) -> Option<gdk::Screen> {
        unsafe {
            from_glib_none(ffi::gtk_invisible_get_screen(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

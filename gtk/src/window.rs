// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Window;
use glib::object::IsA;
use glib::translate::*;

pub trait GtkWindowExtManual: 'static {
    /// Presents a window to the user. This function should not be used
    /// as when it is called, it is too late to gather a valid timestamp
    /// to allow focus stealing prevention to work correctly.
    #[doc(alias = "gtk_window_present")]
    fn present(&self);
}

#[cfg(target_os = "macos")]
extern "C" {
    fn macos_force_foreground_level();
}

impl<O: IsA<Window>> GtkWindowExtManual for O {
    fn present(&self) {
        unsafe {
            ffi::gtk_window_present(self.as_ref().to_glib_none().0);
        }
        // This is a super wonderful hack to actually make this function work as expected.
        #[cfg(target_os = "macos")]
        unsafe {
            macos_force_foreground_level();
        }
    }
}

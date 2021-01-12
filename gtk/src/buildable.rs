// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Buildable;
use glib::translate::*;
use glib::IsA;

pub trait BuildableExtManual: 'static {
    #[doc(alias = "gtk_buildable_get_name")]
    fn get_buildable_name(&self) -> Option<String>;

    #[doc(alias = "gtk_buildable_set_name")]
    fn set_buildable_name(&self, name: &str);
}

impl<O: IsA<Buildable>> BuildableExtManual for O {
    fn get_buildable_name(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::gtk_buildable_get_name(self.as_ref().to_glib_none().0)) }
    }

    fn set_buildable_name(&self, name: &str) {
        unsafe {
            ffi::gtk_buildable_set_name(self.as_ref().to_glib_none().0, name.to_glib_none().0);
        }
    }
}

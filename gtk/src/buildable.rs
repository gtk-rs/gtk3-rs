// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Buildable;
use glib::object::IsA;
use glib::translate::*;

mod sealed {
    pub trait Sealed {}
    impl<T: glib::object::IsA<crate::Buildable>> Sealed for T {}
}

pub trait BuildableExtManual: IsA<Buildable> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_buildable_get_name")]
    #[doc(alias = "get_buildable_name")]
    fn buildable_name(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::gtk_buildable_get_name(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gtk_buildable_set_name")]
    fn set_buildable_name(&self, name: &str) {
        unsafe {
            ffi::gtk_buildable_set_name(self.as_ref().to_glib_none().0, name.to_glib_none().0);
        }
    }
}

impl<O: IsA<Buildable>> BuildableExtManual for O {}

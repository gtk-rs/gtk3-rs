// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Widget;
use gio::AppInfo;
use glib::object::IsA;
use glib::translate::*;

glib::glib_wrapper! {
    pub struct AppChooser(Interface<ffi::GtkAppChooser>) @requires Widget;

    match fn {
        get_type => || ffi::gtk_app_chooser_get_type(),
    }
}

pub trait AppChooserExt: 'static {
    fn get_app_info(&self) -> Option<AppInfo>;
    fn get_content_type(&self) -> Option<String>;
    fn refresh(&self);
}

impl<O: IsA<AppChooser>> AppChooserExt for O {
    fn get_app_info(&self) -> Option<AppInfo> {
        unsafe {
            from_glib_full(ffi::gtk_app_chooser_get_app_info(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_content_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_app_chooser_get_content_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn refresh(&self) {
        unsafe { ffi::gtk_app_chooser_refresh(self.as_ref().to_glib_none().0) }
    }
}

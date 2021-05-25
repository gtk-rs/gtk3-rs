// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Widget;
use gio::AppInfo;
use glib::object::IsA;
use glib::translate::*;

glib::wrapper! {
    /// [AppChooser](crate::AppChooser) is an interface that can be implemented by widgets which
    /// allow the user to choose an application (typically for the purpose of
    /// opening a file). The main objects that implement this interface are
    /// [AppChooserWidget](crate::AppChooserWidget), [AppChooserDialog](crate::AppChooserDialog) and [AppChooserButton](crate::AppChooserButton).
    ///
    /// Applications are represented by GIO [gio::AppInfo](crate::gio::AppInfo) objects here.
    /// GIO has a concept of recommended and fallback applications for a
    /// given content type. Recommended applications are those that claim
    /// to handle the content type itself, while fallback also includes
    /// applications that handle a more generic content type. GIO also
    /// knows the default and last-used application for a given content
    /// type. The [AppChooserWidget](crate::AppChooserWidget) provides detailed control over
    /// whether the shown list of applications should include default,
    /// recommended or fallback applications.
    ///
    /// To obtain the application that has been selected in a [AppChooser](crate::AppChooser),
    /// use [AppChooserExt::app_info](crate::prelude::AppChooserExt::app_info).
    ///
    /// # Implements
    ///
    /// [AppChooserExt](crate::prelude::AppChooserExt), [WidgetExt](crate::prelude::WidgetExt), [glib::ObjectExt](trait@glib::ObjectExt), [BuildableExt](crate::prelude::BuildableExt), [WidgetExtManual](trait@crate::prelude::WidgetExtManual), [BuildableExtManual](trait@crate::prelude::BuildableExtManual)
    pub struct AppChooser(Interface<ffi::GtkAppChooser>) @requires Widget;

    match fn {
        type_ => || ffi::gtk_app_chooser_get_type(),
    }
}

/// Trait containing all `AppChooser` methods.
///
/// # Implementors
///
/// [AppChooserButton](crate::AppChooserButton), [AppChooserDialog](crate::AppChooserDialog), [AppChooserWidget](crate::AppChooserWidget), [AppChooser](crate::AppChooser)
pub trait AppChooserExt: 'static {
    /// Returns the currently selected application.
    ///
    /// # Returns
    ///
    /// a [gio::AppInfo](crate::gio::AppInfo) for the currently selected
    ///  application, or [`None`] if none is selected. Free with `g_object_unref`
    #[doc(alias = "gtk_app_chooser_get_app_info")]
    #[doc(alias = "get_app_info")]
    fn app_info(&self) -> Option<AppInfo>;

    /// Returns the current value of the `GtkAppChooser:::content-type` property.
    ///
    /// # Returns
    ///
    /// the content type of `self`. Free with `g_free`
    #[doc(alias = "gtk_app_chooser_get_content_type")]
    #[doc(alias = "get_content_type")]
    fn content_type(&self) -> Option<String>;

    /// Reloads the list of applications.
    #[doc(alias = "gtk_app_chooser_refresh")]
    fn refresh(&self);
}

impl<O: IsA<AppChooser>> AppChooserExt for O {
    fn app_info(&self) -> Option<AppInfo> {
        unsafe {
            from_glib_full(ffi::gtk_app_chooser_get_app_info(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn content_type(&self) -> Option<String> {
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

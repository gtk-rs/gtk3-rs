// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RadioToolButton;
use crate::ToolItem;
use glib::object::{Cast, ObjectType};
use glib::translate::*;
use glib::ToValue;
use std::ptr;

impl RadioToolButton {
    /// Creates a new [RadioToolButton](crate::RadioToolButton), adding it to `group`.
    /// ## `group`
    /// An
    ///  existing radio button group, or [`None`] if you are creating a new group
    ///
    /// # Returns
    ///
    /// The new [RadioToolButton](crate::RadioToolButton)
    #[doc(alias = "gtk_radio_tool_button_new")]
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_radio_tool_button_new(ptr::null_mut())).unsafe_cast()
        }
    }

    #[doc(alias = "gtk_radio_tool_button_new_from_stock")]
    pub fn from_stock(stock_id: &str) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_radio_tool_button_new_from_stock(
                ptr::null_mut(),
                stock_id.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    pub fn join_group(&self, group: Option<&RadioToolButton>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut _,
                "group".to_glib_none().0,
                group.to_value().to_glib_none().0,
            );
        }
    }
}

impl Default for RadioToolButton {
    fn default() -> Self {
        Self::new()
    }
}

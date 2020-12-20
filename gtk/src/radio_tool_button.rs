// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RadioToolButton;
use crate::ToolItem;
use glib::object::{Cast, ObjectType};
use glib::translate::*;
use glib::Value;
use std::ptr;

impl RadioToolButton {
    #[doc(alias = "gtk_radio_tool_button_new")]
    pub fn new() -> RadioToolButton {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_radio_tool_button_new(ptr::null_mut())).unsafe_cast()
        }
    }

    #[doc(alias = "gtk_radio_tool_button_new_from_stock")]
    pub fn from_stock(stock_id: &str) -> RadioToolButton {
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
                Value::from(group).to_glib_none().0,
            );
        }
    }
}

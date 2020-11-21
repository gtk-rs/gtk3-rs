// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use crate::RadioToolButton;
use crate::ToolItem;
use glib::object::{Cast, ObjectType};
use glib::translate::*;
use glib::Value;
use gobject_sys;
use std::ptr;

impl RadioToolButton {
    pub fn new() -> RadioToolButton {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_radio_tool_button_new(ptr::null_mut())).unsafe_cast()
        }
    }

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
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut _,
                "group".to_glib_none().0,
                Value::from(group).to_glib_none().0,
            );
        }
    }
}

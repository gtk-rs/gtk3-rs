// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::DBusInterfaceInfo;
use glib::translate::*;
use std::ptr;

glib::glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DBusNodeInfo(Shared<ffi::GDBusNodeInfo>);

    match fn {
        ref => |ptr| ffi::g_dbus_node_info_ref(ptr),
        unref => |ptr| ffi::g_dbus_node_info_unref(ptr),
        get_type => || ffi::g_dbus_node_info_get_type(),
    }
}

impl DBusNodeInfo {
    pub fn new_for_xml(xml_data: &str) -> Result<DBusNodeInfo, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_node_info_new_for_xml(xml_data.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn generate_xml(&self, indent: u32, string_builder: &mut glib::String) {
        unsafe {
            ffi::g_dbus_node_info_generate_xml(
                self.to_glib_none().0,
                indent,
                string_builder.to_glib_none_mut().0,
            );
        }
    }

    pub fn lookup_interface(&self, name: &str) -> Option<DBusInterfaceInfo> {
        unsafe {
            from_glib_none(ffi::g_dbus_node_info_lookup_interface(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }
}

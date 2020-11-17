// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib;
use glib::translate::*;

glib::glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SettingsSchemaKey(Shared<ffi::GSettingsSchemaKey>);

    match fn {
        ref => |ptr| ffi::g_settings_schema_key_ref(ptr),
        unref => |ptr| ffi::g_settings_schema_key_unref(ptr),
        get_type => || ffi::g_settings_schema_key_get_type(),
    }
}

impl SettingsSchemaKey {
    pub fn get_default_value(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_settings_schema_key_get_default_value(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_description(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_settings_schema_key_get_description(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    pub fn get_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::g_settings_schema_key_get_name(self.to_glib_none().0)) }
    }

    pub fn get_range(&self) -> Option<glib::Variant> {
        unsafe { from_glib_full(ffi::g_settings_schema_key_get_range(self.to_glib_none().0)) }
    }

    pub fn get_summary(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_settings_schema_key_get_summary(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_value_type(&self) -> Option<glib::VariantType> {
        unsafe {
            from_glib_none(ffi::g_settings_schema_key_get_value_type(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn range_check(&self, value: &glib::Variant) -> bool {
        unsafe {
            from_glib(ffi::g_settings_schema_key_range_check(
                self.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }
}

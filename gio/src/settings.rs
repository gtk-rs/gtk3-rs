// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Settings, SettingsBindFlags, SettingsExt};
use glib::prelude::*;
use glib::translate::{from_glib_borrow, from_glib_none, ToGlib, ToGlibPtr};
use glib::variant::FromVariant;
use glib::{BoolError, IsA, ToVariant};

#[must_use]
pub struct BindingBuilder<'a> {
    settings: &'a Settings,
    key: &'a str,
    object: &'a glib::Object,
    property: &'a str,
    flags: SettingsBindFlags,
    get_mapping: Option<Box<dyn Fn(&glib::Variant, glib::Type) -> Option<glib::Value>>>,
    set_mapping: Option<Box<dyn Fn(&glib::Value, glib::VariantType) -> Option<glib::Variant>>>,
}

impl<'a> BindingBuilder<'a> {
    pub fn flags(mut self, flags: SettingsBindFlags) -> Self {
        self.flags = flags;
        self
    }

    pub fn get_mapping<F: Fn(&glib::Variant, glib::Type) -> Option<glib::Value> + 'static>(
        mut self,
        f: F,
    ) -> Self {
        self.get_mapping = Some(Box::new(f));
        self
    }

    pub fn set_mapping<
        F: Fn(&glib::Value, glib::VariantType) -> Option<glib::Variant> + 'static,
    >(
        mut self,
        f: F,
    ) -> Self {
        self.set_mapping = Some(Box::new(f));
        self
    }

    pub fn build(self) {
        type Mappings = (
            Option<Box<dyn Fn(&glib::Variant, glib::Type) -> Option<glib::Value>>>,
            Option<Box<dyn Fn(&glib::Value, glib::VariantType) -> Option<glib::Variant>>>,
        );
        unsafe extern "C" fn bind_with_mapping_get_trampoline(
            value: *mut glib::gobject_ffi::GValue,
            variant: *mut glib::ffi::GVariant,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let user_data = &*(user_data as *const Mappings);
            let f = user_data.0.as_ref().unwrap();
            let value = &mut *(value as *mut glib::Value);
            if let Some(v) = f(&*from_glib_borrow(variant), value.type_()) {
                *value = v;
                true
            } else {
                false
            }
            .to_glib()
        }
        unsafe extern "C" fn bind_with_mapping_set_trampoline(
            value: *const glib::gobject_ffi::GValue,
            variant_type: *const glib::ffi::GVariantType,
            user_data: glib::ffi::gpointer,
        ) -> *mut glib::ffi::GVariant {
            let user_data = &*(user_data as *const Mappings);
            let f = user_data.1.as_ref().unwrap();
            let value = &*(value as *const glib::Value);
            f(value, from_glib_none(variant_type)).to_glib_full()
        }
        unsafe extern "C" fn destroy_closure(ptr: *mut libc::c_void) {
            Box::<Mappings>::from_raw(ptr as *mut _);
        }
        let get_trampoline: Option<unsafe extern "C" fn(_, _, _) -> _> =
            if self.get_mapping.is_none() {
                None
            } else {
                Some(bind_with_mapping_get_trampoline)
            };
        let set_trampoline: Option<unsafe extern "C" fn(_, _, _) -> _> =
            if self.set_mapping.is_none() {
                None
            } else {
                Some(bind_with_mapping_set_trampoline)
            };
        let mappings: Mappings = (self.get_mapping, self.set_mapping);
        unsafe {
            ffi::g_settings_bind_with_mapping(
                self.settings.to_glib_none().0,
                self.key.to_glib_none().0,
                self.object.to_glib_none().0,
                self.property.to_glib_none().0,
                self.flags.to_glib(),
                get_trampoline,
                set_trampoline,
                Box::into_raw(Box::new(mappings)) as *mut libc::c_void,
                Some(destroy_closure),
            )
        }
    }
}

pub trait SettingsExtManual {
    fn get<U: FromVariant>(&self, key: &str) -> U;

    fn set<U: ToVariant>(&self, key: &str, value: &U) -> Result<(), BoolError>;

    #[doc(alias = "g_settings_bind")]
    #[doc(alias = "g_settings_bind_with_mapping")]
    fn bind<'a, P: IsA<glib::Object>>(
        &'a self,
        key: &'a str,
        object: &'a P,
        property: &'a str,
    ) -> BindingBuilder<'a>;
}

impl<O: IsA<Settings>> SettingsExtManual for O {
    fn get<U: FromVariant>(&self, key: &str) -> U {
        let val = self.get_value(key);
        FromVariant::from_variant(&val).unwrap_or_else(|| {
            panic!(
                "Type mismatch: Expected '{}' got '{}'",
                U::static_variant_type().to_str(),
                val.type_()
            )
        })
    }

    fn set<U: ToVariant>(&self, key: &str, value: &U) -> Result<(), BoolError> {
        self.set_value(key, &value.to_variant())
    }

    fn bind<'a, P: IsA<glib::Object>>(
        &'a self,
        key: &'a str,
        object: &'a P,
        property: &'a str,
    ) -> BindingBuilder<'a> {
        BindingBuilder {
            settings: self.upcast_ref(),
            key,
            object: object.upcast_ref(),
            property,
            flags: SettingsBindFlags::DEFAULT,
            get_mapping: None,
            set_mapping: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env::set_var;
    use std::process::Command;
    use std::str::from_utf8;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn set_env() {
        INIT.call_once(|| {
            let output = Command::new("glib-compile-schemas")
                .args(&[
                    &format!("{}/tests", env!("CARGO_MANIFEST_DIR")),
                    "--targetdir",
                    env!("OUT_DIR"),
                ])
                .output()
                .unwrap();

            if !output.status.success() {
                println!("Failed to generate GSchema!");
                println!(
                    "glib-compile-schemas stdout: {}",
                    from_utf8(&output.stdout).unwrap()
                );
                println!(
                    "glib-compile-schemas stderr: {}",
                    from_utf8(&output.stderr).unwrap()
                );
                panic!("Can't test without GSchemas!");
            }

            set_var("GSETTINGS_SCHEMA_DIR", env!("OUT_DIR"));
            set_var("GSETTINGS_BACKEND", "memory");
        });
    }

    #[test]
    #[serial_test::serial]
    fn string_get() {
        set_env();
        let settings = Settings::new("com.github.gtk-rs.test");
        assert_eq!(settings.get::<String>("test-string").as_str(), "Good");
    }

    #[test]
    #[serial_test::serial]
    fn bool_set_get() {
        set_env();
        let settings = Settings::new("com.github.gtk-rs.test");
        settings.set("test-bool", &false).unwrap();
        assert!(!settings.get::<bool>("test-bool"));
    }

    #[test]
    #[should_panic]
    #[serial_test::serial]
    fn wrong_type() {
        set_env();
        let settings = Settings::new("com.github.gtk-rs.test");
        settings.get::<u8>("test-string");
    }
}

use glib::variant::FromVariant;
use glib::{BoolError, IsA, ToVariant};
use {Settings, SettingsExt};

pub trait SettingsExtManual {
    fn get<U: FromVariant>(&self, key: &str) -> U;

    fn set<U: ToVariant>(&self, key: &str, value: &U) -> Result<(), BoolError>;
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
    #[serial]
    fn string_get() {
        set_env();
        let settings = Settings::new("com.github.gtk-rs.test");
        assert_eq!(settings.get::<String>("test-string").as_str(), "Good");
    }

    #[test]
    #[serial]
    fn bool_set_get() {
        set_env();
        let settings = Settings::new("com.github.gtk-rs.test");
        settings.set("test-bool", &false).unwrap();
        assert!(!settings.get::<bool>("test-bool"));
    }

    #[test]
    #[should_panic]
    #[serial]
    fn wrong_type() {
        set_env();
        let settings = Settings::new("com.github.gtk-rs.test");
        settings.get::<u8>("test-string");
    }
}

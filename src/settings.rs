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
        FromVariant::from_variant(&val).expect(&format!(
            "Type mismatch: Expected '{}' got '{}'",
            U::static_variant_type().to_str(),
            val.type_()
        ))
    }

    fn set<U: ToVariant>(&self, key: &str, value: &U) -> Result<(), BoolError> {
        self.set_value(key, &value.to_variant())
    }
}

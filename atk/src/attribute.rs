// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use glib::GString;
use std::fmt;

/// AtkAttribute is a string name/value pair representing a generic
/// attribute. This can be used to expose additional information from
/// an accessible object as a whole (see [AtkObjectExt::attributes](crate::prelude::AtkObjectExt::attributes))
/// or an document (see [DocumentExt::attributes](crate::prelude::DocumentExt::attributes)). In the case of
/// text attributes (see [TextExt::default_attributes](crate::prelude::TextExt::default_attributes)),
/// AtkTextAttribute enum defines all the possible text attribute
/// names. You can use `atk_text_attribute_get_name` to get the string
/// name from the enum value. See also `atk_text_attribute_for_name`
/// and `atk_text_attribute_get_value` for more information.
///
/// A string name/value pair representing a generic attribute.
pub struct Attribute {
    pub name: GString,
    pub value: GString,
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Attribute")
            .field("name", &self.name)
            .field("value", &self.value)
            .finish()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::AtkAttribute> for Attribute {
    unsafe fn from_glib(value: ffi::AtkAttribute) -> Self {
        skip_assert_initialized!();
        Self {
            name: from_glib_full(value.name),
            value: from_glib_full(value.value),
        }
    }
}

#[doc(hidden)]
impl IntoGlib for Attribute {
    type GlibType = ffi::AtkAttribute;

    fn into_glib(self) -> ffi::AtkAttribute {
        ffi::AtkAttribute {
            name: self.name.to_glib_none().0,
            value: self.value.to_glib_none().0,
        }
    }
}

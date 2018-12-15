// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::GString;
use glib::translate::*;
use ffi;
use std::fmt;

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
    fn from_glib(value: ffi::AtkAttribute) -> Self {
        skip_assert_initialized!();
        unsafe {
            Attribute {
                name: from_glib_full(value.name),
                value: from_glib_full(value.value),
            }
        }
    }
}

#[doc(hidden)]
impl ToGlib for Attribute {
    type GlibType = ffi::AtkAttribute;

    fn to_glib(&self) -> ffi::AtkAttribute {
        ffi::AtkAttribute {
            name: self.name.to_glib_none().0,
            value: self.value.to_glib_none().0
        }
    }
}

// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use atk_sys;
use glib::translate::*;
use glib::GString;
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
impl FromGlib<atk_sys::AtkAttribute> for Attribute {
    fn from_glib(value: atk_sys::AtkAttribute) -> Self {
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
    type GlibType = atk_sys::AtkAttribute;

    fn to_glib(&self) -> atk_sys::AtkAttribute {
        atk_sys::AtkAttribute {
            name: self.name.to_glib_none().0,
            value: self.value.to_glib_none().0,
        }
    }
}

// Copyright (C) 2017-2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gobject_ffi;

use translate::*;
use Type;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PropertyMutability {
    Readable,
    Writable,
    ReadWrite,
}

impl Into<gobject_ffi::GParamFlags> for PropertyMutability {
    fn into(self) -> gobject_ffi::GParamFlags {
        use self::PropertyMutability::*;

        match self {
            Readable => gobject_ffi::G_PARAM_READABLE,
            Writable => gobject_ffi::G_PARAM_WRITABLE,
            ReadWrite => gobject_ffi::G_PARAM_READWRITE,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Property<'a> {
    Boolean(&'a str, &'a str, &'a str, bool, PropertyMutability),
    Int(
        &'a str,
        &'a str,
        &'a str,
        (i32, i32),
        i32,
        PropertyMutability,
    ),
    Int64(
        &'a str,
        &'a str,
        &'a str,
        (i64, i64),
        i64,
        PropertyMutability,
    ),
    UInt(
        &'a str,
        &'a str,
        &'a str,
        (u32, u32),
        u32,
        PropertyMutability,
    ),
    UInt64(
        &'a str,
        &'a str,
        &'a str,
        (u64, u64),
        u64,
        PropertyMutability,
    ),
    Float(
        &'a str,
        &'a str,
        &'a str,
        (f32, f32),
        f32,
        PropertyMutability,
    ),
    Double(
        &'a str,
        &'a str,
        &'a str,
        (f64, f64),
        f64,
        PropertyMutability,
    ),
    String(
        &'a str,
        &'a str,
        &'a str,
        Option<&'a str>,
        PropertyMutability,
    ),
    Boxed(&'a str, &'a str, &'a str, fn() -> Type, PropertyMutability),
    Object(&'a str, &'a str, &'a str, fn() -> Type, PropertyMutability),
    Variant(
        &'a str,
        &'a str,
        &'a str,
        fn() -> ::VariantType,
        Option<&'a ::Variant>,
        PropertyMutability,
    ),
}

impl<'a> Into<*mut gobject_ffi::GParamSpec> for &'a Property<'a> {
    fn into(self) -> *mut gobject_ffi::GParamSpec {
        unsafe {
            match *self {
                Property::Boolean(name, nick, description, default, mutability) => {
                    gobject_ffi::g_param_spec_boolean(
                        name.to_glib_none().0,
                        nick.to_glib_none().0,
                        description.to_glib_none().0,
                        default.to_glib(),
                        mutability.into(),
                    )
                }
                Property::Int(name, nick, description, (min, max), default, mutability) => {
                    gobject_ffi::g_param_spec_int(
                        name.to_glib_none().0,
                        nick.to_glib_none().0,
                        description.to_glib_none().0,
                        min,
                        max,
                        default,
                        mutability.into(),
                    )
                }
                Property::Int64(name, nick, description, (min, max), default, mutability) => {
                    gobject_ffi::g_param_spec_int64(
                        name.to_glib_none().0,
                        nick.to_glib_none().0,
                        description.to_glib_none().0,
                        min,
                        max,
                        default,
                        mutability.into(),
                    )
                }
                Property::UInt(name, nick, description, (min, max), default, mutability) => {
                    gobject_ffi::g_param_spec_uint(
                        name.to_glib_none().0,
                        nick.to_glib_none().0,
                        description.to_glib_none().0,
                        min,
                        max,
                        default,
                        mutability.into(),
                    )
                }
                Property::UInt64(name, nick, description, (min, max), default, mutability) => {
                    gobject_ffi::g_param_spec_uint64(
                        name.to_glib_none().0,
                        nick.to_glib_none().0,
                        description.to_glib_none().0,
                        min,
                        max,
                        default,
                        mutability.into(),
                    )
                }
                Property::Float(name, nick, description, (min, max), default, mutability) => {
                    gobject_ffi::g_param_spec_float(
                        name.to_glib_none().0,
                        nick.to_glib_none().0,
                        description.to_glib_none().0,
                        min,
                        max,
                        default,
                        mutability.into(),
                    )
                }
                Property::Double(name, nick, description, (min, max), default, mutability) => {
                    gobject_ffi::g_param_spec_double(
                        name.to_glib_none().0,
                        nick.to_glib_none().0,
                        description.to_glib_none().0,
                        min,
                        max,
                        default,
                        mutability.into(),
                    )
                }
                Property::String(name, nick, description, default, mutability) => {
                    gobject_ffi::g_param_spec_string(
                        name.to_glib_none().0,
                        nick.to_glib_none().0,
                        description.to_glib_none().0,
                        default.to_glib_none().0,
                        mutability.into(),
                    )
                }
                Property::Boxed(name, nick, description, get_type, mutability) => {
                    gobject_ffi::g_param_spec_boxed(
                        name.to_glib_none().0,
                        nick.to_glib_none().0,
                        description.to_glib_none().0,
                        get_type().to_glib(),
                        mutability.into(),
                    )
                }
                Property::Object(name, nick, description, get_type, mutability) => {
                    gobject_ffi::g_param_spec_object(
                        name.to_glib_none().0,
                        nick.to_glib_none().0,
                        description.to_glib_none().0,
                        get_type().to_glib(),
                        mutability.into(),
                    )
                }
                Property::Variant(name, nick, description, get_type, default, mutability) => {
                    gobject_ffi::g_param_spec_variant(
                        name.to_glib_none().0,
                        nick.to_glib_none().0,
                        description.to_glib_none().0,
                        get_type().to_glib_none().0,
                        default.to_glib_none().0,
                        mutability.into(),
                    )
                }
            }
        }
    }
}

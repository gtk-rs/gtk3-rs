// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use translate::{FromGlib, ToGlib, from_glib};
use glib_ffi;
use gobject_ffi;

/// A GLib or GLib-based library type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    /// An invalid `Type` used as error return value in some functions
    Invalid,
    /// The fundamental type corresponding to the unit type `()`
    Unit,
    /// The fundamental type corresponding to `i8`
    I8,
    /// The fundamental type corresponding to `u8`
    U8,
    /// The fundamental type corresponding to `bool`
    Bool,
    /// The fundamental type corresponding to `i32`
    I32,
    /// The fundamental type corresponding to `u32`
    U32,
    /// The fundamental type corresponding to `isize`
    ISize,
    /// The fundamental type corresponding to `usize`
    USize,
    /// The fundamental type corresponding to `i64`
    I64,
    /// The fundamental type corresponding to `u64`
    U64,
    /// The fundamental type corresponding to `f32`
    F32,
    /// The fundamental type corresponding to `f64`
    F64,
    /// The fundamental type corresponding to `String`
    String,
    /// The fundamental type corresponding to a pointer
    Pointer,
    /// The fundamental type of GVariant
    Variant,
    /// The fundamental type from which all interfaces are derived
    BaseInterface,
    /// The fundamental type from which all enumeration types are derived
    BaseEnum,
    /// The fundamental type from which all flags types are derived
    BaseFlags,
    /// The fundamental type from which all boxed types are derived
    BaseBoxed,
    /// The fundamental type from which all `GParamSpec` types are derived
    BaseParamSpec,
    /// The fundamental type from which all objects are derived
    BaseObject,
    /// A non-fundamental type identified by value of type `usize`
    Other(usize),
}

pub trait StaticType {
    fn static_type() -> Type;
}

pub trait InstanceType {
    fn instance_type(&self) -> Type;
}

#[inline]
pub fn instance_of<C: StaticType>(ptr: glib_ffi::gconstpointer) -> bool {
    unsafe {
        from_glib(
            gobject_ffi::g_type_check_instance_is_a(
                ptr as *mut _, <C as StaticType>::static_type().to_glib()))
    }
}

impl FromGlib<glib_ffi::GType> for Type {
    #[inline]
    fn from_glib(val: glib_ffi::GType) -> Type {
        use self::Type::*;
        match val {
            gobject_ffi::G_TYPE_INVALID => Invalid,
            gobject_ffi::G_TYPE_NONE => Unit,
            gobject_ffi::G_TYPE_INTERFACE => BaseInterface,
            gobject_ffi::G_TYPE_CHAR => I8,
            gobject_ffi::G_TYPE_UCHAR => U8,
            gobject_ffi::G_TYPE_BOOLEAN => Bool,
            gobject_ffi::G_TYPE_INT => I32,
            gobject_ffi::G_TYPE_UINT => U32,
            gobject_ffi::G_TYPE_LONG => ISize,
            gobject_ffi::G_TYPE_ULONG => USize,
            gobject_ffi::G_TYPE_INT64 => I64,
            gobject_ffi::G_TYPE_UINT64 => U64,
            gobject_ffi::G_TYPE_ENUM => BaseEnum,
            gobject_ffi::G_TYPE_FLAGS => BaseFlags,
            gobject_ffi::G_TYPE_FLOAT => F32,
            gobject_ffi::G_TYPE_DOUBLE => F64,
            gobject_ffi::G_TYPE_STRING => String,
            gobject_ffi::G_TYPE_POINTER => Pointer,
            gobject_ffi::G_TYPE_BOXED => BaseBoxed,
            gobject_ffi::G_TYPE_PARAM => BaseParamSpec,
            gobject_ffi::G_TYPE_OBJECT => BaseObject,
            gobject_ffi::G_TYPE_VARIANT => Variant,
            x => Other(x as usize),
        }
    }
}

impl ToGlib for Type {
    type GlibType = glib_ffi::GType;

    fn to_glib(&self) -> glib_ffi::GType {
        use self::Type::*;
        match *self {
            Invalid => gobject_ffi::G_TYPE_INVALID,
            Unit => gobject_ffi::G_TYPE_NONE,
            BaseInterface => gobject_ffi::G_TYPE_INTERFACE,
            I8 => gobject_ffi::G_TYPE_CHAR,
            U8 => gobject_ffi::G_TYPE_UCHAR,
            Bool => gobject_ffi::G_TYPE_BOOLEAN,
            I32 => gobject_ffi::G_TYPE_INT,
            U32 => gobject_ffi::G_TYPE_UINT,
            ISize => gobject_ffi::G_TYPE_LONG,
            USize => gobject_ffi::G_TYPE_ULONG,
            I64 => gobject_ffi::G_TYPE_INT64,
            U64 => gobject_ffi::G_TYPE_UINT64,
            BaseEnum => gobject_ffi::G_TYPE_ENUM,
            BaseFlags => gobject_ffi::G_TYPE_FLAGS,
            F32 => gobject_ffi::G_TYPE_FLOAT,
            F64 => gobject_ffi::G_TYPE_DOUBLE,
            String => gobject_ffi::G_TYPE_STRING,
            Pointer => gobject_ffi::G_TYPE_POINTER,
            BaseBoxed => gobject_ffi::G_TYPE_BOXED,
            BaseParamSpec => gobject_ffi::G_TYPE_PARAM,
            BaseObject => gobject_ffi::G_TYPE_OBJECT,
            Variant => gobject_ffi::G_TYPE_VARIANT,
            Other(x) => x as glib_ffi::GType,
        }
    }
}

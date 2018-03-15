// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Runtime type information.

use translate::{FromGlib, FromGlibContainerAsVec, ToGlib, ToGlibPtr, ToGlibPtrMut, ToGlibContainerFromSlice, from_glib, from_glib_none};
use ffi as glib_ffi;
use gobject_ffi;
use value::{Value, FromValue, FromValueOptional, SetValue};

use std::fmt;
use std::mem;
use std::ptr;

/// A GLib or GLib-based library type
#[derive(Clone, Copy, PartialEq, Eq)]
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
    /// The fundamental type corresponding to C `long`
    ILong,
    /// The fundamental type corresponding to C `unsigned long`
    ULong,
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

impl Type {
    pub fn name(&self) -> String {
        unsafe {
            from_glib_none(gobject_ffi::g_type_name(self.to_glib()))
        }
    }

    pub fn is_a(&self, other: &Type) -> bool {
        unsafe {
            from_glib(gobject_ffi::g_type_is_a(self.to_glib(), other.to_glib()))
        }
    }

    pub fn parent(&self) -> Option<Self> {
        unsafe {
            let parent = gobject_ffi::g_type_parent(self.to_glib());
            if parent == gobject_ffi::G_TYPE_INVALID {
                None
            } else {
                Some(from_glib(parent))
            }
        }
    }

    pub fn children(&self) -> Vec<Self> {
        unsafe {
            let mut n_children = 0u32;
            let children = gobject_ffi::g_type_children(self.to_glib(), &mut n_children);

            FromGlibContainerAsVec::from_glib_full_num_as_vec(children, n_children as usize)
        }
    }

    pub fn interfaces(&self) -> Vec<Self> {
        unsafe {
            let mut n_interfaces = 0u32;
            let interfaces = gobject_ffi::g_type_interfaces(self.to_glib(), &mut n_interfaces);

            FromGlibContainerAsVec::from_glib_full_num_as_vec(interfaces, n_interfaces as usize)
        }
    }
    pub fn interface_prerequisites(&self) -> Vec<Self> {
        unsafe {
            let mut n_prereqs = 0u32;
            let prereqs = gobject_ffi::g_type_interface_prerequisites(self.to_glib(), &mut n_prereqs);

            FromGlibContainerAsVec::from_glib_full_num_as_vec(prereqs, n_prereqs as usize)
        }
    }

    pub fn from_name<'a, P: Into<&'a str>>(name: P) -> Option<Self> {
        unsafe {
            let type_ = gobject_ffi::g_type_from_name(name.into().to_glib_none().0);
            if type_ == gobject_ffi::G_TYPE_INVALID {
                None
            } else {
                Some(from_glib(type_))
            }
        }
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name())
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name())
    }
}

/// Types that are supported by GLib dynamic typing.
pub trait StaticType {
    /// Returns the type identifier of `Self`.
    fn static_type() -> Type;
}

impl StaticType for Type {
    fn static_type() -> Type {
        unsafe {
            from_glib(gobject_ffi::g_gtype_get_type())
        }
    }
}

impl<'a> FromValueOptional<'a> for Type {
    unsafe fn from_value_optional(value: &'a Value) -> Option<Self> {
        Some(from_glib(gobject_ffi::g_value_get_gtype(value.to_glib_none().0)))
    }
}

impl<'a> FromValue<'a> for Type {
    unsafe fn from_value(value: &'a Value) -> Self {
        from_glib(gobject_ffi::g_value_get_gtype(value.to_glib_none().0))
    }
}

impl SetValue for Type {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_gtype(value.to_glib_none_mut().0, this.to_glib())
    }
}

impl<'a, T: ?Sized + StaticType> StaticType for &'a T {
    fn static_type() -> Type {
        T::static_type()
    }
}

macro_rules! builtin {
    ($name:ident, $val:ident) => {
        impl StaticType for $name {
            fn static_type() -> Type {
                Type::$val
            }
        }
    }
}

builtin!(bool, Bool);
builtin!(i8, I8);
builtin!(u8, U8);
builtin!(i32, I32);
builtin!(u32, U32);
builtin!(i64, I64);
builtin!(u64, U64);
builtin!(f32, F32);
builtin!(f64, F64);
builtin!(str, String);
builtin!(String, String);

impl<'a> StaticType for [&'a str] {
    fn static_type() -> Type {
        unsafe {
            from_glib(glib_ffi::g_strv_get_type())
        }
    }
}

impl StaticType for Vec<String> {
    fn static_type() -> Type {
        unsafe {
            from_glib(glib_ffi::g_strv_get_type())
        }
    }
}

#[inline]
pub unsafe fn instance_of<C: StaticType>(ptr: glib_ffi::gconstpointer) -> bool {
    from_glib(
        gobject_ffi::g_type_check_instance_is_a(
            ptr as *mut _, <C as StaticType>::static_type().to_glib()))
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
            gobject_ffi::G_TYPE_LONG => ILong,
            gobject_ffi::G_TYPE_ULONG => ULong,
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
            ILong => gobject_ffi::G_TYPE_LONG,
            ULong => gobject_ffi::G_TYPE_ULONG,
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

impl<'a> ToGlibContainerFromSlice<'a, *mut glib_ffi::GType> for Type {
    type Storage = Option<Vec<glib_ffi::GType>>;

    fn to_glib_none_from_slice(t: &'a [Type]) -> (*mut glib_ffi::GType, Self::Storage) {
        let mut vec = t.iter().map(|v| v.to_glib()).collect::<Vec<_>>();

        (vec.as_mut_ptr(), Some(vec))
    }

    fn to_glib_container_from_slice(t: &'a [Type]) -> (*mut glib_ffi::GType, Self::Storage) {
        (Self::to_glib_full_from_slice(t), None)
    }

    fn to_glib_full_from_slice(t: &[Type]) -> *mut glib_ffi::GType {
        if t.len() == 0 {
            return ptr::null_mut();
        }

        unsafe {
            let res = glib_ffi::g_malloc0(mem::size_of::<glib_ffi::GType>() * (t.len() + 1)) as *mut glib_ffi::GType;
            for (i, v) in t.iter().enumerate() {
                *res.offset(i as isize) = v.to_glib();
            }
            res
        }
    }
}


impl FromGlibContainerAsVec<Type, *const glib_ffi::GType> for Type {
    unsafe fn from_glib_none_num_as_vec(ptr: *const glib_ffi::GType, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib(*ptr.offset(i as isize)));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(_: *const glib_ffi::GType, _: usize) -> Vec<Self> {
        // Can't really free a *const
        unimplemented!();
    }

    unsafe fn from_glib_full_num_as_vec(_: *const glib_ffi::GType, _: usize) -> Vec<Self> {
        // Can't really free a *const
        unimplemented!();
    }
}

impl FromGlibContainerAsVec<Type, *mut glib_ffi::GType> for Type {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut glib_ffi::GType, num: usize) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *const _, num)
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut glib_ffi::GType, num: usize) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib_ffi::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut glib_ffi::GType, num: usize) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, num)
    }
}

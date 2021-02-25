// Take a look at the license at the top of the repository in the LICENSE file.

//! Runtime type information.

use crate::translate::{
    from_glib, FromGlib, FromGlibContainerAsVec, ToGlib, ToGlibContainerFromSlice, ToGlibPtr,
    ToGlibPtrMut,
};
use crate::value::{FromValue, FromValueOptional, SetValue, Value};

use std::fmt;
use std::mem;
use std::ptr;

/// A GLib or GLib-based library type
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Type(ffi::GType);

impl Type {
    /// An invalid `Type` used as error return value in some functions
    pub const INVALID: Self = Self(gobject_ffi::G_TYPE_INVALID);

    /// The fundamental type corresponding to the unit type `()`
    pub const UNIT: Self = Self(gobject_ffi::G_TYPE_NONE);

    /// The fundamental type corresponding to `i8`
    pub const I8: Self = Self(gobject_ffi::G_TYPE_CHAR);

    /// The fundamental type corresponding to `u8`
    pub const U8: Self = Self(gobject_ffi::G_TYPE_UCHAR);

    /// The fundamental type corresponding to `bool`
    pub const BOOL: Self = Self(gobject_ffi::G_TYPE_BOOLEAN);

    /// The fundamental type corresponding to `i32`
    pub const I32: Self = Self(gobject_ffi::G_TYPE_INT);

    /// The fundamental type corresponding to `u32`
    pub const U32: Self = Self(gobject_ffi::G_TYPE_UINT);

    /// The fundamental type corresponding to C `long`
    pub const I_LONG: Self = Self(gobject_ffi::G_TYPE_LONG);

    /// The fundamental type corresponding to C `unsigned long`
    pub const U_LONG: Self = Self(gobject_ffi::G_TYPE_ULONG);

    /// The fundamental type corresponding to `i64`
    pub const I64: Self = Self(gobject_ffi::G_TYPE_INT64);

    /// The fundamental type corresponding to `u64`
    pub const U64: Self = Self(gobject_ffi::G_TYPE_UINT64);

    /// The fundamental type corresponding to `f32`
    pub const F32: Self = Self(gobject_ffi::G_TYPE_FLOAT);

    /// The fundamental type corresponding to `f64`
    pub const F64: Self = Self(gobject_ffi::G_TYPE_DOUBLE);

    /// The fundamental type corresponding to `String`
    pub const STRING: Self = Self(gobject_ffi::G_TYPE_STRING);

    /// The fundamental type corresponding to a pointer
    pub const POINTER: Self = Self(gobject_ffi::G_TYPE_POINTER);

    /// The fundamental type of GVariant
    pub const VARIANT: Self = Self(gobject_ffi::G_TYPE_VARIANT);

    /// The fundamental type from which all interfaces are derived
    pub const INTERFACE: Self = Self(gobject_ffi::G_TYPE_INTERFACE);

    /// The fundamental type from which all enumeration types are derived
    pub const ENUM: Self = Self(gobject_ffi::G_TYPE_ENUM);

    /// The fundamental type from which all flags types are derived
    pub const FLAGS: Self = Self(gobject_ffi::G_TYPE_FLAGS);

    /// The fundamental type from which all boxed types are derived
    pub const BOXED: Self = Self(gobject_ffi::G_TYPE_BOXED);

    /// The fundamental type from which all `GParamSpec` types are derived
    pub const PARAM_SPEC: Self = Self(gobject_ffi::G_TYPE_PARAM);

    /// The fundamental type from which all objects are derived
    pub const OBJECT: Self = Self(gobject_ffi::G_TYPE_OBJECT);

    #[doc(alias = "g_type_name")]
    pub fn name<'a>(self) -> &'a str {
        match self.to_glib() {
            gobject_ffi::G_TYPE_INVALID => "<invalid>",
            x => unsafe {
                let ptr = gobject_ffi::g_type_name(x);
                std::ffi::CStr::from_ptr(ptr).to_str().unwrap()
            },
        }
    }

    #[doc(alias = "g_type_qname")]
    pub fn qname(self) -> crate::Quark {
        match self.to_glib() {
            gobject_ffi::G_TYPE_INVALID => crate::Quark::from_string("<invalid>"),
            x => unsafe { from_glib(gobject_ffi::g_type_qname(x)) },
        }
    }

    #[doc(alias = "g_type_is_a")]
    pub fn is_a(self, other: Self) -> bool {
        unsafe { from_glib(gobject_ffi::g_type_is_a(self.to_glib(), other.to_glib())) }
    }

    #[doc(alias = "g_type_parent")]
    pub fn parent(self) -> Option<Self> {
        unsafe {
            let parent: Self = from_glib(gobject_ffi::g_type_parent(self.to_glib()));
            Some(parent).filter(|t| t.is_valid())
        }
    }

    #[doc(alias = "g_type_children")]
    pub fn children(self) -> Vec<Self> {
        unsafe {
            let mut n_children = 0u32;
            let children = gobject_ffi::g_type_children(self.to_glib(), &mut n_children);

            FromGlibContainerAsVec::from_glib_full_num_as_vec(children, n_children as usize)
        }
    }

    #[doc(alias = "g_type_interfaces")]
    pub fn interfaces(self) -> Vec<Self> {
        unsafe {
            let mut n_interfaces = 0u32;
            let interfaces = gobject_ffi::g_type_interfaces(self.to_glib(), &mut n_interfaces);

            FromGlibContainerAsVec::from_glib_full_num_as_vec(interfaces, n_interfaces as usize)
        }
    }

    #[doc(alias = "g_type_interface_prerequisites")]
    pub fn interface_prerequisites(self) -> Vec<Self> {
        match self {
            t if !t.is_a(Self::INTERFACE) => vec![],
            _ => unsafe {
                let mut n_prereqs = 0u32;
                let prereqs =
                    gobject_ffi::g_type_interface_prerequisites(self.to_glib(), &mut n_prereqs);

                FromGlibContainerAsVec::from_glib_full_num_as_vec(prereqs, n_prereqs as usize)
            },
        }
    }

    #[doc(alias = "g_type_from_name")]
    pub fn from_name<'a, P: Into<&'a str>>(name: P) -> Option<Self> {
        unsafe {
            let type_: Self =
                from_glib(gobject_ffi::g_type_from_name(name.into().to_glib_none().0));
            Some(type_).filter(|t| t.is_valid())
        }
    }

    /// Checks that the type is not [`INVALID`](Self::INVALID)
    #[inline]
    pub fn is_valid(self) -> bool {
        self != Self::INVALID
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
    #[doc(alias = "g_gtype_get_type")]
    fn static_type() -> Type {
        unsafe { from_glib(gobject_ffi::g_gtype_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for Type {
    unsafe fn from_value_optional(value: &'a Value) -> Option<Self> {
        Some(from_glib(gobject_ffi::g_value_get_gtype(
            value.to_glib_none().0,
        )))
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

impl<'a, T: ?Sized + StaticType> StaticType for &'a mut T {
    fn static_type() -> Type {
        T::static_type()
    }
}

macro_rules! builtin {
    ($name:ty, $val:ident) => {
        impl StaticType for $name {
            fn static_type() -> Type {
                Type::$val
            }
        }
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ILong(pub libc::c_long);

impl std::ops::Deref for ILong {
    type Target = libc::c_long;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ILong {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<libc::c_long> for ILong {
    fn from(v: libc::c_long) -> ILong {
        ILong(v)
    }
}

impl From<ILong> for libc::c_long {
    fn from(v: ILong) -> libc::c_long {
        v.0
    }
}

impl PartialEq<libc::c_long> for ILong {
    fn eq(&self, other: &libc::c_long) -> bool {
        &self.0 == other
    }
}

impl PartialEq<ILong> for libc::c_long {
    fn eq(&self, other: &ILong) -> bool {
        self == &other.0
    }
}

impl PartialOrd<libc::c_long> for ILong {
    fn partial_cmp(&self, other: &libc::c_long) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<ILong> for libc::c_long {
    fn partial_cmp(&self, other: &ILong) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ULong(pub libc::c_ulong);

impl std::ops::Deref for ULong {
    type Target = libc::c_ulong;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ULong {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<libc::c_ulong> for ULong {
    fn from(v: libc::c_ulong) -> ULong {
        ULong(v)
    }
}

impl From<ULong> for libc::c_ulong {
    fn from(v: ULong) -> libc::c_ulong {
        v.0
    }
}

impl PartialEq<libc::c_ulong> for ULong {
    fn eq(&self, other: &libc::c_ulong) -> bool {
        &self.0 == other
    }
}

impl PartialEq<ULong> for libc::c_ulong {
    fn eq(&self, other: &ULong) -> bool {
        self == &other.0
    }
}

impl PartialOrd<libc::c_ulong> for ULong {
    fn partial_cmp(&self, other: &libc::c_ulong) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<ULong> for libc::c_ulong {
    fn partial_cmp(&self, other: &ULong) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

builtin!(bool, BOOL);
builtin!(i8, I8);
builtin!(u8, U8);
builtin!(i32, I32);
builtin!(u32, U32);
builtin!(i64, I64);
builtin!(u64, U64);
builtin!(ILong, I_LONG);
builtin!(ULong, U_LONG);
builtin!(f32, F32);
builtin!(f64, F64);
builtin!(str, STRING);
builtin!(String, STRING);

impl<'a> StaticType for [&'a str] {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_strv_get_type()) }
    }
}

impl StaticType for Vec<String> {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_strv_get_type()) }
    }
}

impl StaticType for () {
    fn static_type() -> Type {
        Type::UNIT
    }
}

#[inline]
pub unsafe fn instance_of<C: StaticType>(ptr: ffi::gconstpointer) -> bool {
    from_glib(gobject_ffi::g_type_check_instance_is_a(
        ptr as *mut _,
        <C as StaticType>::static_type().to_glib(),
    ))
}

impl FromGlib<ffi::GType> for Type {
    #[inline]
    unsafe fn from_glib(val: ffi::GType) -> Type {
        Self(val)
    }
}

impl ToGlib for Type {
    type GlibType = ffi::GType;

    #[inline]
    fn to_glib(&self) -> ffi::GType {
        self.0
    }
}

impl<'a> ToGlibContainerFromSlice<'a, *mut ffi::GType> for Type {
    type Storage = Option<Vec<ffi::GType>>;

    fn to_glib_none_from_slice(t: &'a [Type]) -> (*mut ffi::GType, Self::Storage) {
        let mut vec = t.iter().map(ToGlib::to_glib).collect::<Vec<_>>();

        (vec.as_mut_ptr(), Some(vec))
    }

    fn to_glib_container_from_slice(t: &'a [Type]) -> (*mut ffi::GType, Self::Storage) {
        (Self::to_glib_full_from_slice(t), None)
    }

    fn to_glib_full_from_slice(t: &[Type]) -> *mut ffi::GType {
        if t.is_empty() {
            return ptr::null_mut();
        }

        unsafe {
            let res =
                ffi::g_malloc0(mem::size_of::<ffi::GType>() * (t.len() + 1)) as *mut ffi::GType;
            for (i, v) in t.iter().enumerate() {
                *res.add(i) = v.to_glib();
            }
            res
        }
    }
}

impl FromGlibContainerAsVec<Type, *const ffi::GType> for Type {
    unsafe fn from_glib_none_num_as_vec(ptr: *const ffi::GType, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib(*ptr.add(i)));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(_: *const ffi::GType, _: usize) -> Vec<Self> {
        // Can't really free a *const
        unimplemented!();
    }

    unsafe fn from_glib_full_num_as_vec(_: *const ffi::GType, _: usize) -> Vec<Self> {
        // Can't really free a *const
        unimplemented!();
    }
}

impl FromGlibContainerAsVec<Type, *mut ffi::GType> for Type {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut ffi::GType, num: usize) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *const _, num)
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut ffi::GType, num: usize) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        ffi::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut ffi::GType, num: usize) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InitiallyUnowned;
    use std::collections::{BTreeSet, HashSet};

    #[test]
    fn invalid() {
        let invalid = Type::INVALID;

        assert_eq!(invalid.name(), "<invalid>");
        assert_eq!(invalid.qname(), crate::Quark::from_string("<invalid>"));
        assert!(invalid.is_a(Type::INVALID));
        assert!(!invalid.is_a(Type::STRING));
        assert_eq!(invalid.parent(), None);
        assert_eq!(invalid.children(), vec![]);
        assert_eq!(invalid.interfaces(), vec![]);
        assert_eq!(invalid.interface_prerequisites(), vec![]);
        assert!(!invalid.is_valid());
        dbg!(&invalid);
    }

    #[test]
    fn hash() {
        // Get this first so the type is registered
        let iu_type = InitiallyUnowned::static_type();

        let set = Type::OBJECT.children().into_iter().collect::<HashSet<_>>();
        assert!(set.contains(&iu_type));
    }

    #[test]
    fn ord() {
        // Get this first so the type is registered
        let iu_type = InitiallyUnowned::static_type();
        assert!(Type::OBJECT < iu_type);

        let set = Type::OBJECT.children().into_iter().collect::<BTreeSet<_>>();
        assert!(set.contains(&iu_type));
    }
}

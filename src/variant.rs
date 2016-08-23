// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! `Variant` binding and helper traits.
//!
//! [`Variant`](struct.Variant.html) is an immutable dynamically-typed generic
//! container. Its type and value are defined at construction and never change.
//!
//! `Variant` types are described by [`VariantType`](../struct.VariantType.html)
//! "type strings".
//!
//! Although `GVariant` supports arbitrarily complex types, this binding is
//! currently limited to the basic ones: `bool`, `u8`, `i16`, `u16`, `i32`,
//! `u32`, `i64`, `u64`, `f64` and `&str`/`String`.
//!
//! # Examples
//!
//! ```
//! use glib::prelude::*; // or `use gtk::prelude::*;`
//! use glib::Variant;
//!
//! // Using the `ToVariant` trait.
//! let num = 10.to_variant();
//!
//! // `is` tests the type of the value.
//! assert!(num.is::<i32>());
//!
//! // `get` tries to extract the value.
//! assert_eq!(num.get::<i32>(), Some(10));
//! assert_eq!(num.get::<u32>(), None);
//!
//! // `Variant` implements `From`
//! let hello = Variant::from("Hello!");
//!
//! // `get_str` tries to borrow a string slice.
//! assert_eq!(hello.get_str(), Some("Hello!"));
//! assert_eq!(num.get_str(), None);
//! ```

use VariantTy;
use ffi as glib_ffi;
use translate::*;
use std::borrow::Cow;
use std::cmp::{PartialEq, Eq};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::slice;
use std::str;

glib_wrapper! {
    /// A generic immutable value capable of carrying various types.
    ///
    /// See the [module documentation](index.html) for more details.
    pub struct Variant(Shared<glib_ffi::GVariant>);

    match fn {
        ref => |ptr| glib_ffi::g_variant_ref_sink(ptr),
        unref => |ptr| glib_ffi::g_variant_unref(ptr),
    }
}

impl Variant {
    /// Returns the type of the value.
    pub fn type_(&self) -> &VariantTy {
        unsafe { VariantTy::from_ptr(glib_ffi::g_variant_get_type(self.to_glib_none().0)) }
    }

    /// Returns `true` if the type of the value corresponds to `T`.
    #[inline]
    pub fn is<T: StaticVariantType>(&self) -> bool {
        self.type_() == T::static_variant_type()
    }

    /// Tries to extract a value of type `T`.
    ///
    /// Returns `Some` if `T` matches the variant's type.
    #[inline]
    pub fn get<T: FromVariant>(&self) -> Option<T> {
        T::from_variant(self)
    }

    /// Tries to extract a `&str`.
    ///
    /// Returns `Some` if the variant has a string type (`s`, `o` or `g` type
    /// strings).
    pub fn get_str(&self) -> Option<&str> {
        unsafe {
            match self.type_().to_str() {
                "s" | "o" | "g" => {
                    let mut len = 0;
                    let ptr = glib_ffi::g_variant_get_string(self.to_glib_none().0, &mut len);
                    let ret = str::from_utf8_unchecked(
                        slice::from_raw_parts(ptr as *const u8, len as usize));
                    Some(ret)
                }
                _ => None,
            }
        }
    }
}

unsafe impl Send for Variant { }
unsafe impl Sync for Variant { }

impl fmt::Debug for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("Variant {{ ptr: {:?}, type: \"{}\", value: {} }}",
            self.to_glib_none().0, self.type_(), self))
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let serialized: String = unsafe {
            from_glib_full(glib_ffi::g_variant_print(self.to_glib_none().0, false.to_glib()))
        };
        f.write_str(&serialized)
    }
}

impl PartialEq for Variant {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            from_glib(
                glib_ffi::g_variant_equal(self.to_glib_none().0 as *const _,
                    other.to_glib_none().0 as *const _))
        }
    }
}

impl Eq for Variant { }

impl Hash for Variant {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            state.write_u32(glib_ffi::g_variant_hash(self.to_glib_none().0 as *const _))
        }
    }
}

/// Converts to `Variant`.
pub trait ToVariant {
    /// Returns a `Variant` clone of `self`.
    fn to_variant(&self) -> Variant;
}

/// Extracts a value.
pub trait FromVariant: Sized + StaticVariantType {
    /// Tries to extract a value.
    ///
    /// Returns `Some` if the variant's type matches `Self`.
    fn from_variant(variant: &Variant) -> Option<Self>;
}

/// Returns `VariantType` of `Self`.
pub trait StaticVariantType {
    /// Returns the `VariantType` corresponding to `Self`.
    fn static_variant_type() -> Cow<'static, VariantTy>;
}

impl<'a, T: ?Sized + ToVariant> ToVariant for &'a T {
    fn to_variant(&self) -> Variant {
        <T as ToVariant>::to_variant(self)
    }
}

impl<'a, T: ?Sized + StaticVariantType> StaticVariantType for &'a T {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        <T as StaticVariantType>::static_variant_type()
    }
}

macro_rules! impl_numeric {
    ($name:ty, $type_str:expr, $new_fn:ident, $get_fn:ident) => {
        impl StaticVariantType for $name {
            fn static_variant_type() -> Cow<'static, VariantTy> {
                unsafe { VariantTy::from_str_unchecked($type_str).into() }
            }
        }

        impl ToVariant for $name {
            fn to_variant(&self) -> Variant {
                unsafe { from_glib_none(glib_ffi::$new_fn(*self)) }
            }
        }

        impl FromVariant for $name {
            fn from_variant(variant: &Variant) -> Option<Self> {
                unsafe {
                    if variant.is::<Self>() {
                        Some(glib_ffi::$get_fn(variant.to_glib_none().0))
                    } else {
                        None
                    }
                }
            }
        }
    }
}

impl_numeric!(u8, "y", g_variant_new_byte, g_variant_get_byte);
impl_numeric!(i16, "n", g_variant_new_int16, g_variant_get_int16);
impl_numeric!(u16, "q", g_variant_new_uint16, g_variant_get_uint16);
impl_numeric!(i32, "i", g_variant_new_int32, g_variant_get_int32);
impl_numeric!(u32, "u", g_variant_new_uint32, g_variant_get_uint32);
impl_numeric!(i64, "x", g_variant_new_int64, g_variant_get_int64);
impl_numeric!(u64, "t", g_variant_new_uint64, g_variant_get_uint64);
impl_numeric!(f64, "d", g_variant_new_double, g_variant_get_double);

impl StaticVariantType for bool {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        unsafe { VariantTy::from_str_unchecked("b").into() }
    }
}

impl ToVariant for bool {
    fn to_variant(&self) -> Variant {
        unsafe { from_glib_none(glib_ffi::g_variant_new_boolean(self.to_glib())) }
    }
}

impl FromVariant for bool {
    fn from_variant(variant: &Variant) -> Option<Self> {
        unsafe {
            if variant.is::<Self>() {
                Some(from_glib(glib_ffi::g_variant_get_boolean(variant.to_glib_none().0)))
            } else {
                None
            }
        }
    }
}

impl StaticVariantType for String {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        unsafe { VariantTy::from_str_unchecked("s").into() }
    }
}

impl ToVariant for String {
    fn to_variant(&self) -> Variant {
        self[..].to_variant()
    }
}

impl FromVariant for String {
    fn from_variant(variant: &Variant) -> Option<Self> {
        variant.get_str().map(String::from)
    }
}

impl StaticVariantType for str {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        unsafe { VariantTy::from_str_unchecked("s").into() }
    }
}

impl ToVariant for str {
    #[cfg(feature = "v2_38")]
    fn to_variant(&self) -> Variant {
        unsafe { from_glib_none(glib_ffi::g_variant_new_take_string(self.to_glib_full())) }
    }

    #[cfg(not(feature = "v2_38"))]
    fn to_variant(&self) -> Variant {
        unsafe { from_glib_none(glib_ffi::g_variant_new_string(self.to_glib_none().0)) }
    }
}

impl<T: ToVariant> From<T> for Variant {
    fn from(value: T) -> Variant {
        value.to_variant()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    macro_rules! unsigned {
        ($name:ident, $ty:ident) => {
            #[test]
            fn $name() {
                let mut n = $ty::max_value();
                while n > 0  {
                    let v = Variant::from(n);
                    assert_eq!(v.get(), Some(n));
                    n /= 2;
                }
            }
        }
    }

    macro_rules! signed {
        ($name:ident, $ty:ident) => {
            #[test]
            fn $name() {
                let mut n = $ty::max_value();
                while n > 0  {
                    let v = Variant::from(n);
                    assert_eq!(v.get(), Some(n));
                    let v = Variant::from(-n);
                    assert_eq!(v.get(), Some(-n));
                    n /= 2;
                }
            }
        }
    }

    unsigned!(test_u8, u8);
    unsigned!(test_u16, u16);
    unsigned!(test_u32, u32);
    unsigned!(test_u64, u64);
    signed!(test_i16, i16);
    signed!(test_i32, i32);
    signed!(test_i64, i64);

    #[test]
    fn test_str() {
        let s = "this is a test";
        let v = Variant::from(s);
        assert_eq!(v.get_str(), Some(s));
    }

    #[test]
    fn test_string() {
        let s = String::from("this is a test");
        let v = Variant::from(s.clone());
        assert_eq!(v.get(), Some(s.clone()));
    }

    #[test]
    fn test_eq() {
        let v1 = Variant::from("this is a test");
        let v2 = Variant::from("this is a test");
        let v3 = Variant::from("test");
        assert_eq!(v1, v2);
        assert!(v1 != v3);
    }

    #[test]
    fn test_hash() {
        let v1 = Variant::from("this is a test");
        let v2 = Variant::from("this is a test");
        let v3 = Variant::from("test");
        let mut set = HashSet::new();
        set.insert(v1);
        assert!(set.contains(&v2));
        assert!(!set.contains(&v3));
    }
}

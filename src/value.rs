// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! `Value` binding and helper traits.
//!
//! The type of a [`Value`](struct.Value.html) is dynamic in that it generally
//! isn't known at compile time but once created a `Value` can't change its
//! type.
//!
//! [`TypedValue`](struct.TypedValue.html) has a statically known type and
//! dereferences to `Value` so it can be used everywhere `Value` references are
//! accepted.
//!
//! Supported types are `bool`, `i8`, `u8`, `i32`, `u32`, `i64`, `u64`, `f32`,
//! `f64`, `String` and objects (`T: IsA<Object>`).
//!
//! # Examples
//!
//! ```
//! use glib::prelude::*; // or `use gtk::prelude::*;`
//! use glib::{TypedValue, Value};
//!
//! // Value and TypedValue implement From<&i32>, From<&str>
//! // and From<Option<&str>>. Another option is the `ToValue` trait.
//! let mut num = 10.to_value();
//! let mut hello = Value::from("Hello!");
//! let none: Option<&str> = None;
//! let str_none = Value::from(none.clone());
//! let typed_str_none = TypedValue::from(none);
//!
//! // `is` tests the type of the value.
//! assert!(num.is::<i32>());
//! assert!(hello.is::<String>());
//!
//! // `get` tries to get a value of specific type and returns None
//! // if the type doesn't match or the value is None.
//! assert_eq!(num.get(), Some(10));
//! assert_eq!(num.get::<String>(), None);
//! assert_eq!(hello.get(), Some(String::from("Hello!")));
//! assert_eq!(hello.get::<String>(), Some(String::from("Hello!")));
//! assert_eq!(str_none.get::<String>(), None);
//!
//! // `typed` tries to convert a `Value` to `TypedValue`.
//! let mut typed_num = num.downcast::<i32>().unwrap();
//! let mut typed_hello = hello.downcast::<String>().unwrap();
//!
//! // `str_none` is not an `i32`
//! assert!(str_none.downcast::<i32>().is_err());
//!
//! // `get`
//! assert!(typed_hello.get().unwrap() == "Hello!");
//! assert!(typed_str_none.get() == None);
//!
//! // Numeric types can't have value `None`, `get` always returns `Some`.
//! // Such types have `get_some`, which avoids unnecessary `unwrap`ping.
//! assert_eq!(typed_num.get().unwrap(), 10);
//! assert_eq!(typed_num.get_some(), 10);
//!
//! // `set_none` sets the value to `None` if the type supports it.
//! typed_hello.set_none();
//! assert!(typed_hello.get().is_none());
//!
//! // `set` takes an optional reference for types that support `None`.
//! typed_hello.set(Some("Hello again!"));
//! assert!(typed_hello.get().unwrap() == "Hello again!");
//!
//! // `set_some` is the only setter for types that don't support `None`.
//! typed_num.set_some(&20);
//! assert_eq!(typed_num.get_some(), 20);
//! ```

use std::borrow::Borrow;
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};

use object::{Downcast, IsA, Object};
use translate::*;
use types::{StaticType, Type};

use ffi as glib_ffi;
use gobject_ffi;

/// A generic value capable of carrying various types.
///
/// Once created the type of the value can't be changed.
///
/// Some types (e.g. `String` and objects) support `None` values while others
/// (e.g. numeric types) don't.
///
/// See the [module documentation](index.html) for more details.
pub struct Value(gobject_ffi::GValue);

impl Value {
    /// Tries to downcast to a `TypedValue`.
    ///
    /// Returns `Ok(TypedValue<T>)` if the value carries a type corresponding
    /// to `T` and `Err(self)` otherwise.
    pub fn downcast<T: FromValueOptional + SetValue>(self) -> Result<TypedValue<T>, Self> {
        unsafe {
            let ok = from_glib(
                gobject_ffi::g_type_check_value_holds(mut_override(self.to_glib_none().0),
                    T::static_type().to_glib()));
            if ok {
                Ok(TypedValue(self, PhantomData))
            }
            else {
                Err(self)
            }
        }
    }

    /// Tries to get a value of type `T`.
    ///
    /// Returns `Some` if the type is correct and the value is not `None`.
    ///
    /// This function doesn't distinguish between type mismatches and correctly
    /// typed `None` values. Use `downcast` or `is` for that.
    pub fn get<T: FromValueOptional>(&self) -> Option<T> {
        unsafe {
           let ok = from_glib(
               gobject_ffi::g_type_check_value_holds(mut_override(self.to_glib_none().0),
                   T::static_type().to_glib()));
           if ok {
               T::from_value_optional(self)
           }
           else {
               None
           }
        }
    }

    /// Returns `true` if the type of the value corresponds to `T`.
    #[inline]
    pub fn is<T: FromValueOptional + SetValue>(&self) -> bool {
        self.type_() == T::static_type()
    }

    /// Returns the type of the value.
    pub fn type_(&self) -> Type {
        unsafe {
            // FIXME: make this safe by making GValue::g_type public
            let type_ = *(&self.0 as *const gobject_ffi::GValue as *const glib_ffi::GType);
            from_glib(type_)
        }
    }

    /// Returns whether `Value`s of type `src` can be transformed to type `dst`.
    pub fn type_transformable(src: Type, dst: Type) -> bool {
        unsafe {
            from_glib(gobject_ffi::g_value_type_transformable(src.to_glib(), dst.to_glib()))
        }
    }

    fn into_raw(mut self) -> gobject_ffi::GValue {
        unsafe {
            let ret = mem::replace(&mut self.0, mem::uninitialized());
            mem::forget(self);
            ret
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        unsafe {
            // FIXME: make this safer by making GValue::g_type public
            let type_ = *(&self.0 as *const gobject_ffi::GValue as *const glib_ffi::GType);
            let mut ret = Value::uninitialized();
            gobject_ffi::g_value_init(ret.to_glib_none_mut().0, type_);
            gobject_ffi::g_value_copy(self.to_glib_none().0, ret.to_glib_none_mut().0);
            ret
        }
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        unsafe { gobject_ffi::g_value_unset(self.to_glib_none_mut().0) }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        unsafe {
            let s: String = from_glib_full(
                gobject_ffi::g_strdup_value_contents(self.to_glib_none().0));
            write!(f, "Value({})", s)
        }
    }
}

impl<'a, T: ?Sized + SetValueOptional> From<Option<&'a T>> for Value {
    #[inline]
    fn from(value: Option<&'a T>) -> Self {
        value.to_value()
    }
}

impl<'a, T: ?Sized + SetValue> From<&'a T> for Value {
    #[inline]
    fn from(value: &'a T) -> Self {
        value.to_value()
    }
}

impl<T> From<TypedValue<T>> for Value {
    fn from(value: TypedValue<T>) -> Self {
        value.0
    }
}

impl Uninitialized for Value {
    unsafe fn uninitialized() -> Value {
        Value(mem::zeroed())
    }
}

impl<'a> ToGlibPtr<'a, *const gobject_ffi::GValue> for Value {
    type Storage = &'a Value;

    fn to_glib_none(&'a self) -> Stash<'a, *const gobject_ffi::GValue, Self> {
        Stash(&self.0, self)
    }
}

impl<'a> ToGlibPtrMut<'a, *mut gobject_ffi::GValue> for Value {
    type Storage = &'a mut Value;

    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut gobject_ffi::GValue, Self> {
        StashMut(&mut self.0, self)
    }
}

impl<'a> ToGlibPtr<'a, *mut gobject_ffi::GValue> for &'a [&'a ToValue] {
    type Storage = ValueArray;

    fn to_glib_none(&'a self) -> Stash<'a, *mut gobject_ffi::GValue, Self> {
        let mut values: Vec<gobject_ffi::GValue> = self.iter()
            .map(|v| v.to_value().into_raw())
            .collect();
        Stash(values.as_mut_ptr(), ValueArray(values))
    }
}

pub struct ValueArray(Vec<gobject_ffi::GValue>);

impl Drop for ValueArray {
    fn drop(&mut self) {
        unsafe {
            for value in &mut self.0 {
                gobject_ffi::g_value_unset(value);
            }
        }
    }
}

/// A statically typed [`Value`](struct.Value.html).
///
/// It dereferences to `Value` and can be used everywhere `Value` references are
/// accepted.
///
/// See the [module documentation](index.html) for more details.
pub struct TypedValue<T>(Value, PhantomData<*const T>);

impl<T: FromValueOptional + SetValue> TypedValue<T> {
    /// Returns the value.
    ///
    /// Types that don't support a `None` value always return `Some`. See
    /// `get_some`.
    pub fn get(&self) -> Option<T> {
        unsafe { T::from_value_optional(self) }
    }

    /// Returns the value.
    ///
    /// This method is only available for types that don't support a `None`
    /// value.
    pub fn get_some(&self) -> T where T: FromValue {
        unsafe { T::from_value(self) }
    }

    /// Sets the value.
    ///
    /// This method is only available for types that support a `None` value.
    pub fn set<U: ?Sized + SetValueOptional>(&mut self, value: Option<&U>) where T: Borrow<U> {
        unsafe { SetValueOptional::set_value_optional(self, value) }
    }

    /// Sets the value to `None`.
    ///
    /// This method is only available for types that support a `None` value.
    pub fn set_none(&mut self) where T: SetValueOptional {
        unsafe { T::set_value_optional(self, None) }
    }

    /// Sets the value.
    pub fn set_some<U: ?Sized + SetValue>(&mut self, value: &U) where T: Borrow<U> {
        unsafe { SetValue::set_value(self, value) }
    }
}

impl<T> Clone for TypedValue<T> {
    fn clone(&self) -> Self {
        TypedValue(self.0.clone(), PhantomData)
    }
}

impl<T> fmt::Debug for TypedValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "TypedValue({:?})", self.0)
    }
}

impl<T> Deref for TypedValue<T> {
    type Target = Value;

    fn deref(&self) -> &Value {
        &self.0
    }
}

impl<T> DerefMut for TypedValue<T> {
    fn deref_mut(&mut self) -> &mut Value {
        &mut self.0
    }
}

impl<'a, T: FromValueOptional + SetValueOptional> From<Option<&'a T>> for TypedValue<T> {
    fn from(value: Option<&'a T>) -> Self {
        TypedValue(Value::from(value), PhantomData)
    }
}

impl<'a, T: FromValueOptional + SetValue> From<&'a T> for TypedValue<T> {
    fn from(value: &'a T) -> Self {
        TypedValue(Value::from(value), PhantomData)
    }
}

impl<'a> From<Option<&'a str>> for TypedValue<String> {
    fn from(value: Option<&'a str>) -> Self {
        TypedValue(Value::from(value), PhantomData)
    }
}

impl<'a> From<&'a str> for TypedValue<String> {
    fn from(value: &'a str) -> Self {
        TypedValue(Value::from(value), PhantomData)
    }
}

/// Converts to `Value`.
pub trait ToValue {
    /// Returns a `Value` clone of `self`.
    fn to_value(&self) -> Value;

    /// Returns the type identifer of `self`.
    ///
    /// This is the type of the value to be returned by `to_value`.
    fn to_value_type(&self) -> Type;
}

impl<T: SetValueOptional> ToValue for Option<T> {
    fn to_value(&self) -> Value {
        unsafe {
            let mut ret = Value::uninitialized();
            gobject_ffi::g_value_init(ret.to_glib_none_mut().0, T::static_type().to_glib());
            T::set_value_optional(&mut ret, self.as_ref());
            ret
        }
    }

    #[inline]
    fn to_value_type(&self) -> Type {
        T::static_type()
    }
}

impl<T: ?Sized + SetValue> ToValue for T {
    fn to_value(&self) -> Value {
        unsafe {
            let mut ret = Value::uninitialized();
            gobject_ffi::g_value_init(ret.to_glib_none_mut().0, T::static_type().to_glib());
            T::set_value(&mut ret, self);
            ret
        }
    }

    #[inline]
    fn to_value_type(&self) -> Type {
        T::static_type()
    }
}

impl ToValue for Value {
    fn to_value(&self) -> Value {
        self.clone()
    }

    fn to_value_type(&self) -> Type {
        self.type_()
    }
}

/// Extracts a value.
///
/// Types that don't support a `None` value always return `Some`.
pub trait FromValueOptional: StaticType + Sized {
    unsafe fn from_value_optional(&Value) -> Option<Self>;
}

/// Extracts a value.
///
/// Only implemented for types that don't support a `None` value.
pub trait FromValue: FromValueOptional {
    unsafe fn from_value(&Value) -> Self;
}

/// Sets a value.
///
/// Only implemented for types that support a `None` value.
pub trait SetValueOptional: SetValue {
    unsafe fn set_value_optional(&mut Value, Option<&Self>);
}

/// Sets a value.
pub trait SetValue: StaticType {
    unsafe fn set_value(&mut Value, &Self);
}

impl FromValueOptional for String {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        from_glib_none(gobject_ffi::g_value_get_string(value.to_glib_none().0))
    }
}

impl SetValue for str {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_take_string(value.to_glib_none_mut().0, this.to_glib_full())
    }
}

impl SetValueOptional for str {
    unsafe fn set_value_optional(value: &mut Value, this: Option<&Self>) {
        gobject_ffi::g_value_take_string(value.to_glib_none_mut().0, this.to_glib_full())
    }
}

impl<'a, T: ?Sized + SetValue> SetValue for &'a T {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        SetValue::set_value(value, *this)
    }
}

impl<'a, T: ?Sized + SetValueOptional> SetValueOptional for &'a T {
    unsafe fn set_value_optional(value: &mut Value, this: Option<&Self>) {
        SetValueOptional::set_value_optional(value, this.map(|v| *v))
    }
}

impl SetValue for String {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_take_string(value.to_glib_none_mut().0, this.to_glib_full())
    }
}

impl SetValueOptional for String {
    unsafe fn set_value_optional(value: &mut Value, this: Option<&Self>) {
        gobject_ffi::g_value_take_string(value.to_glib_none_mut().0, this.to_glib_full())
    }
}

impl<T: IsA<Object>> FromValueOptional for T {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Option::<Object>::from_glib_full(gobject_ffi::g_value_dup_object(value.to_glib_none().0))
            .map(|o| o.downcast_unchecked())
    }
}

impl<T: IsA<Object>> SetValue for T {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_object(value.to_glib_none_mut().0, this.to_glib_none().0)
    }
}

impl<T: IsA<Object>> SetValueOptional for T {
    unsafe fn set_value_optional(value: &mut Value, this: Option<&Self>) {
        gobject_ffi::g_value_set_object(value.to_glib_none_mut().0, this.to_glib_none().0)
    }
}

impl FromValueOptional for bool {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(from_glib(gobject_ffi::g_value_get_boolean(value.to_glib_none().0)))
    }
}

impl FromValue for bool {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_boolean(value.to_glib_none().0))
    }
}

impl SetValue for bool {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_boolean(value.to_glib_none_mut().0, this.to_glib())
    }
}

macro_rules! numeric {
    ($name:ident, $get:ident, $set:ident) => {
        impl FromValueOptional for $name {
            unsafe fn from_value_optional(value: &Value) -> Option<Self> {
                Some(gobject_ffi::$get(value.to_glib_none().0))
            }
        }

        impl FromValue for $name {
            unsafe fn from_value(value: &Value) -> Self {
                gobject_ffi::$get(value.to_glib_none().0)
            }
        }

        impl SetValue for $name {
            unsafe fn set_value(value: &mut Value, this: &Self) {
                gobject_ffi::$set(value.to_glib_none_mut().0, *this)
            }
        }
    }
}

numeric!(i8, g_value_get_schar, g_value_set_schar);
numeric!(u8, g_value_get_uchar, g_value_set_uchar);
numeric!(i32, g_value_get_int, g_value_set_int);
numeric!(u32, g_value_get_uint, g_value_set_uint);
numeric!(i64, g_value_get_int64, g_value_set_int64);
numeric!(u64, g_value_get_uint64, g_value_set_uint64);
numeric!(f32, g_value_get_float, g_value_set_float);
numeric!(f64, g_value_get_double, g_value_set_double);

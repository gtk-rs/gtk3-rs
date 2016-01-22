// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::borrow::Borrow;
use std::fmt;
use std::marker::PhantomData;
use std::mem;

use object::{Downcast, IsA, Object};
use translate::*;
use types::StaticType;

use glib_ffi;
use gobject_ffi;

/// A generic value capable of carrying various types.
///
/// Once created the type of the value can't be changed.
///
/// Some types (e.g. `String` and objects) support `None` values while others
/// (e.g. numeric types) don't.
pub struct Value(Box<gobject_ffi::GValue>);

impl Value {
    /// Tries to return a typed borrow of the value.
    ///
    /// Returns `Some(TypedValue<T>)` if the value carries a type corresponding
    /// to `T` and `None` otherwise.
    pub fn typed<T: FromValueOptional>(&self) -> Option<TypedValue<T>> {
        unsafe {
            let ok = gobject_ffi::g_type_check_value_holds(mut_override(self.to_glib_none().0),
                T::static_type().to_glib());
            some_if(ok, || TypedValue(self, PhantomData))
        }
    }

    /// Tries to return a typed mutable borrow of the value.
    ///
    /// Returns `Some(TypedValueMut<T>)` if the value carries a type
    /// corresponding to `T` and `None` otherwise.
    pub fn typed_mut<T: FromValueOptional + SetValue>(&mut self) -> Option<TypedValueMut<T>> {
        unsafe {
            let ok = gobject_ffi::g_type_check_value_holds(self.to_glib_none_mut().0,
                T::static_type().to_glib());
            some_if(ok, move || TypedValueMut(self, PhantomData))
        }
    }

    /// Tries to get a value of type `T`.
    ///
    /// Returns `Some` if the type is correct and the value is not `None`.
    ///
    /// This function doesn't distinguish between type mismatches and correctly
    /// typed `None` values. Use `typed` and `typed_mut` for that.
    pub fn get<T: FromValueOptional>(&self) -> Option<T> {
        self.typed::<T>().and_then(|v| v.get())
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        unsafe {
            // FIXME: make this safer by making GValue::g_type public
            let type_ = *(&*self.0 as *const gobject_ffi::GValue as *const glib_ffi::GType);
            let mut ret = Value(Box::new(mem::zeroed()));
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
            write!(f, "{}", s)
        }
    }
}

impl<T: SetValueOptional> From<Option<T>> for Value {
    fn from(value: Option<T>) -> Value {
        unsafe {
            let mut ret = Value::uninitialized();
            gobject_ffi::g_value_init(ret.to_glib_none_mut().0, T::static_type().to_glib());
            T::set_value_optional(&mut ret, value);
            ret
        }
    }
}

impl<T: SetValue> From<T> for Value {
    fn from(value: T) -> Value {
        unsafe {
            let mut ret = Value::uninitialized();
            gobject_ffi::g_value_init(ret.to_glib_none_mut().0, T::static_type().to_glib());
            T::set_value(&mut ret, value);
            ret
        }
    }
}

impl Uninitialized for Value {
    unsafe fn uninitialized() -> Value {
        Value(Box::new(mem::zeroed()))
    }
}

impl<'a> ToGlibPtr<'a, *const gobject_ffi::GValue> for Value {
    type Storage = &'a Value;

    fn to_glib_none(&'a self) -> Stash<'a, *const gobject_ffi::GValue, Self> {
        Stash(&*self.0, self)
    }
}

impl<'a> ToGlibPtrMut<'a, *mut gobject_ffi::GValue> for Value {
    type Storage = &'a mut Value;

    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut gobject_ffi::GValue, Self> {
        StashMut(&mut *self.0, self)
    }
}

/// A typed borrow of a `Value`.
pub struct TypedValue<'a, T>(&'a Value, PhantomData<*const T>);

impl<'a, T: FromValueOptional> TypedValue<'a, T> {
    /// Returns the value.
    ///
    /// Types that don't support a `None` value always return `Some`. See
    /// `get_some`.
    pub fn get(&self) -> Option<T> {
        unsafe { T::from_value_optional(self.0) }
    }

    /// Returns the value.
    ///
    /// This method is only available for types that don't support a `None`
    /// value.
    pub fn get_some(&self) -> T where T: FromValue {
        unsafe { T::from_value(self.0) }
    }
}

pub struct TypedValueMut<'a, T>(&'a mut Value, PhantomData<*const T>);

impl<'a, T: FromValueOptional + SetValue> TypedValueMut<'a, T> {
    /// Returns the value.
    ///
    /// Types that don't support a `None` value always return `Some`. See
    /// `get_some`.
    pub fn get(&self) -> Option<T> {
        unsafe { T::from_value_optional(self.0) }
    }

    /// Returns the value.
    ///
    /// This method is only available for types that don't support a `None`
    /// value.
    pub fn get_some(&self) -> T where T: FromValue {
        unsafe { T::from_value(self.0) }
    }

    /// Sets the value.
    ///
    /// This method is only available for types that support a `None` value.
    pub fn set<'x, U: ?Sized>(&mut self, value: Option<&'x U>)
    where T: Borrow<U>, &'x U: SetValueOptional {
        unsafe { SetValueOptional::set_value_optional(self.0, value) }
    }

    /// Sets the value.
    pub fn set_some<'x, U: ?Sized>(&mut self, value: &'x U)
    where T: Borrow<U>, &'x U: SetValue {
        unsafe { SetValue::set_value(self.0, value) }
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
    unsafe fn set_value_optional(&mut Value, Option<Self>);
}

/// Sets a value.
pub trait SetValue: StaticType + Sized {
    unsafe fn set_value(&mut Value, Self);
}

impl FromValueOptional for String {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        from_glib_none(gobject_ffi::g_value_get_string(value.to_glib_none().0))
    }
}

impl<'a> SetValue for &'a str {
    unsafe fn set_value(value: &mut Value, this: Self) {
        gobject_ffi::g_value_take_string(value.to_glib_none_mut().0, this.to_glib_full())
    }
}

impl<'a> SetValueOptional for &'a str {
    unsafe fn set_value_optional(value: &mut Value, this: Option<Self>) {
        gobject_ffi::g_value_take_string(value.to_glib_none_mut().0, this.to_glib_full())
    }
}

impl SetValue for String {
    unsafe fn set_value(value: &mut Value, this: Self) {
        gobject_ffi::g_value_take_string(value.to_glib_none_mut().0, this.to_glib_full())
    }
}

impl SetValueOptional for String {
    unsafe fn set_value_optional(value: &mut Value, this: Option<Self>) {
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
    unsafe fn set_value(value: &mut Value, this: Self) {
        gobject_ffi::g_value_set_object(value.to_glib_none_mut().0, this.to_glib_none().0)
    }
}

impl<T: IsA<Object>> SetValueOptional for T {
    unsafe fn set_value_optional(value: &mut Value, this: Option<Self>) {
        gobject_ffi::g_value_set_object(value.to_glib_none_mut().0, this.to_glib_none().0)
    }
}

impl<'a, T: IsA<Object>> SetValue for &'a T {
    unsafe fn set_value(value: &mut Value, this: Self) {
        gobject_ffi::g_value_set_object(value.to_glib_none_mut().0, this.to_glib_none().0)
    }
}

impl<'a, T: IsA<Object>> SetValueOptional for &'a T {
    unsafe fn set_value_optional(value: &mut Value, this: Option<Self>) {
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
    unsafe fn set_value(value: &mut Value, this: Self) {
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
            unsafe fn set_value(value: &mut Value, this: Self) {
                gobject_ffi::$set(value.to_glib_none_mut().0, this)
            }
        }

        impl<'a> SetValue for &'a $name {
            unsafe fn set_value(value: &mut Value, this: Self) {
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

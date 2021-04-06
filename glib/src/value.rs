// Take a look at the license at the top of the repository in the LICENSE file.

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
//! [`SendValue`](struct.SendValue.html) is a version of [`Value`](struct.Value.html)
//! that can only store types that implement `Send` and as such implements `Send` itself. It
//! dereferences to `Value` so it can be used everywhere `Value` references are accepted.
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
//! // `get` tries to get an optional value of the specified type
//! // and returns an `Err` if the type doesn't match.
//! assert_eq!(num.get(), Ok(Some(10)));
//! assert!(num.get::<String>().is_err());
//! assert_eq!(hello.get(), Ok(Some(String::from("Hello!"))));
//! assert_eq!(hello.get::<String>(), Ok(Some(String::from("Hello!"))));
//! assert_eq!(str_none.get::<String>(), Ok(None));
//!
//! // `get_some` tries to get a value of the specified non-optional type
//! // and returns an `Err` if the type doesn't match.
//! assert_eq!(num.get_some::<i32>(), Ok(10));
//! assert!(num.get_some::<bool>().is_err());
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

use libc::{c_char, c_void};
use std::borrow::Borrow;
use std::error;
use std::ffi::CStr;
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;
use std::ptr;

use crate::gstring::GString;
use crate::translate::*;
use crate::types::{StaticType, Type};

/// An error returned from the [`get`](struct.Value.html#method.get)
/// or [`get_some`](struct.Value.html#method.get_some) functions on a [`Value`](struct.Value.html)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GetError {
    pub actual: Type,
    pub requested: Type,
}

impl GetError {
    pub fn new_type_mismatch(actual: Type, requested: Type) -> Self {
        GetError { actual, requested }
    }
}

impl fmt::Display for GetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "GetError: Value type mismatch. Actual {:?}, requested {:?}",
            self.actual, self.requested,
        )
    }
}

impl error::Error for GetError {}

/// A generic value capable of carrying various types.
///
/// Once created the type of the value can't be changed.
///
/// Some types (e.g. `String` and objects) support `None` values while others
/// (e.g. numeric types) don't.
///
/// `Value` does not implement the `Send` trait, but [`SendValue`](struct.SendValue.html) can be
/// used instead.
///
/// See the [module documentation](index.html) for more details.
// TODO: Should use impl !Send for Value {} once stable
#[repr(transparent)]
pub struct Value(pub(crate) gobject_ffi::GValue);

impl Value {
    /// Creates a new `Value` that is initialized with `type_`
    pub fn from_type(type_: Type) -> Self {
        unsafe {
            assert_eq!(
                gobject_ffi::g_type_check_is_value_type(type_.to_glib()),
                ffi::GTRUE
            );
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, type_.to_glib());
            value
        }
    }

    /// Tries to downcast to a `TypedValue`.
    ///
    /// Returns `Ok(TypedValue<T>)` if the value carries a type corresponding
    /// to `T` and `Err(self)` otherwise.
    pub fn downcast<'a, T: FromValueOptional<'a> + SetValue>(self) -> Result<TypedValue<T>, Self> {
        unsafe {
            let ok = from_glib(gobject_ffi::g_type_check_value_holds(
                mut_override(self.to_glib_none().0),
                T::static_type().to_glib(),
            ));
            if ok {
                Ok(TypedValue(self, PhantomData))
            } else {
                Err(self)
            }
        }
    }

    /// Tries to downcast to a `&TypedValue`.
    ///
    /// Returns `Some(&TypedValue<T>)` if the value carries a type corresponding
    /// to `T` and `None` otherwise.
    pub fn downcast_ref<'a, T: FromValueOptional<'a> + SetValue>(&self) -> Option<&TypedValue<T>> {
        unsafe {
            let ok = from_glib(gobject_ffi::g_type_check_value_holds(
                mut_override(self.to_glib_none().0),
                T::static_type().to_glib(),
            ));
            if ok {
                // This cast is safe because Value and TypedValue have the same
                // representation: the only difference is the zero-sized phantom data
                Some(&*(self as *const Value as *const TypedValue<T>))
            } else {
                None
            }
        }
    }

    /// Tries to get a possibly optional value of type `T`.
    ///
    /// Returns `Ok` if the type is correct.
    pub fn get<'a, T: FromValueOptional<'a>>(&'a self) -> Result<Option<T>, GetError> {
        unsafe {
            let ok = from_glib(gobject_ffi::g_type_check_value_holds(
                mut_override(self.to_glib_none().0),
                T::static_type().to_glib(),
            ));
            if ok {
                Ok(T::from_value_optional(self))
            } else {
                Err(GetError::new_type_mismatch(self.type_(), T::static_type()))
            }
        }
    }

    /// Tries to get a value of type `T`.
    ///
    /// This method is only available for types that don't support a `None`
    /// value.
    ///
    /// Returns `Ok` if the type is correct.
    pub fn get_some<'a, T: FromValue<'a>>(&'a self) -> Result<T, GetError> {
        unsafe {
            let ok = from_glib(gobject_ffi::g_type_check_value_holds(
                mut_override(self.to_glib_none().0),
                T::static_type().to_glib(),
            ));
            if ok {
                Ok(T::from_value(self))
            } else {
                Err(GetError::new_type_mismatch(self.type_(), T::static_type()))
            }
        }
    }

    /// Returns `true` if the type of the value corresponds to `T`
    /// or is a sub-type of `T`.
    #[inline]
    pub fn is<'a, T: FromValueOptional<'a> + SetValue>(&self) -> bool {
        self.type_().is_a(T::static_type())
    }

    /// Returns the type of the value.
    pub fn type_(&self) -> Type {
        unsafe { from_glib(self.0.g_type) }
    }

    /// Returns whether `Value`s of type `src` can be transformed to type `dst`.
    pub fn type_transformable(src: Type, dst: Type) -> bool {
        unsafe {
            from_glib(gobject_ffi::g_value_type_transformable(
                src.to_glib(),
                dst.to_glib(),
            ))
        }
    }

    /// Tries to transform the value into a value of the target type
    pub fn transform<T: StaticType + SetValue>(&self) -> Option<Value> {
        unsafe {
            let mut dest = Value::from_type(T::static_type());
            if from_glib(gobject_ffi::g_value_transform(
                self.to_glib_none().0,
                dest.to_glib_none_mut().0,
            )) {
                Some(dest)
            } else {
                None
            }
        }
    }

    #[doc(hidden)]
    pub fn into_raw(self) -> gobject_ffi::GValue {
        unsafe {
            let s = mem::ManuallyDrop::new(self);
            ptr::read(&s.0)
        }
    }

    pub fn try_into_send_value<'a, T: Send + FromValueOptional<'a> + SetValue>(
        self,
    ) -> Result<SendValue, Self> {
        self.downcast::<T>().map(TypedValue::into_send_value)
    }

    fn content_debug_string(&self) -> GString {
        unsafe { from_glib_full(gobject_ffi::g_strdup_value_contents(self.to_glib_none().0)) }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        unsafe {
            let mut ret = Value::from_type(from_glib(self.0.g_type));
            gobject_ffi::g_value_copy(self.to_glib_none().0, ret.to_glib_none_mut().0);
            ret
        }
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        // Before GLib 2.48, unsetting a zeroed GValue would give critical warnings
        // https://bugzilla.gnome.org/show_bug.cgi?id=755766
        if self.type_().is_valid() {
            unsafe { gobject_ffi::g_value_unset(self.to_glib_none_mut().0) }
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "({}) {}", self.type_(), self.content_debug_string())
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

impl From<SendValue> for Value {
    fn from(value: SendValue) -> Self {
        value.0
    }
}

impl Uninitialized for Value {
    unsafe fn uninitialized() -> Value {
        mem::zeroed()
    }
}

impl StaticType for Value {
    fn static_type() -> Type {
        unsafe { from_glib(gobject_ffi::g_value_get_type()) }
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const gobject_ffi::GValue> for Value {
    type Storage = &'a Value;

    fn to_glib_none(&'a self) -> Stash<'a, *const gobject_ffi::GValue, Self> {
        Stash(&self.0, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut gobject_ffi::GValue> for Value {
    type Storage = &'a mut Value;

    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut gobject_ffi::GValue, Self> {
        StashMut(&mut self.0, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut gobject_ffi::GValue> for &'a [&'a dyn ToValue] {
    type Storage = ValueArray;

    fn to_glib_none(&'a self) -> Stash<'a, *mut gobject_ffi::GValue, Self> {
        let mut values: Vec<gobject_ffi::GValue> =
            self.iter().map(|v| v.to_value().into_raw()).collect();
        Stash(values.as_mut_ptr(), ValueArray(values))
    }
}

#[doc(hidden)]
impl<'a> ToGlibContainerFromSlice<'a, *mut gobject_ffi::GValue> for &'a Value {
    type Storage = &'a [&'a Value];

    fn to_glib_none_from_slice(t: &'a [&'a Value]) -> (*mut gobject_ffi::GValue, &'a [&'a Value]) {
        (t.as_ptr() as *mut gobject_ffi::GValue, t)
    }

    fn to_glib_container_from_slice(
        t: &'a [&'a Value],
    ) -> (*mut gobject_ffi::GValue, &'a [&'a Value]) {
        if t.is_empty() {
            return (ptr::null_mut(), t);
        }

        unsafe {
            let res = ffi::g_malloc(mem::size_of::<gobject_ffi::GValue>() * t.len())
                as *mut gobject_ffi::GValue;
            ptr::copy_nonoverlapping(t.as_ptr() as *const gobject_ffi::GValue, res, t.len());
            (res, t)
        }
    }

    fn to_glib_full_from_slice(t: &[&'a Value]) -> *mut gobject_ffi::GValue {
        if t.is_empty() {
            return ptr::null_mut();
        }

        unsafe {
            let res = ffi::g_malloc0(mem::size_of::<gobject_ffi::GValue>() * t.len())
                as *mut gobject_ffi::GValue;
            for (i, v) in t.iter().enumerate() {
                gobject_ffi::g_value_init(res.add(i), v.type_().to_glib());
                gobject_ffi::g_value_copy(v.to_glib_none().0, res.add(i));
            }
            res
        }
    }
}

#[doc(hidden)]
impl<'a> ToGlibContainerFromSlice<'a, *const gobject_ffi::GValue> for &'a Value {
    type Storage = &'a [&'a Value];

    fn to_glib_none_from_slice(
        t: &'a [&'a Value],
    ) -> (*const gobject_ffi::GValue, &'a [&'a Value]) {
        let (ptr, storage) =
            ToGlibContainerFromSlice::<'a, *mut gobject_ffi::GValue>::to_glib_none_from_slice(t);
        (ptr as *const _, storage)
    }

    fn to_glib_container_from_slice(
        _: &'a [&'a Value],
    ) -> (*const gobject_ffi::GValue, &'a [&'a Value]) {
        unimplemented!()
    }

    fn to_glib_full_from_slice(_: &[&'a Value]) -> *const gobject_ffi::GValue {
        unimplemented!()
    }
}

macro_rules! from_glib {
    ($name:ident, $wrap:expr) => {
        impl FromGlibPtrNone<*const gobject_ffi::GValue> for $name {
            unsafe fn from_glib_none(ptr: *const gobject_ffi::GValue) -> Self {
                let mut ret = Value::from_type(from_glib((*ptr).g_type));
                gobject_ffi::g_value_copy(ptr, ret.to_glib_none_mut().0);
                $wrap(ret)
            }
        }

        impl FromGlibPtrNone<*mut gobject_ffi::GValue> for $name {
            unsafe fn from_glib_none(ptr: *mut gobject_ffi::GValue) -> Self {
                from_glib_none(ptr as *const _)
            }
        }

        impl FromGlibPtrFull<*mut gobject_ffi::GValue> for $name {
            unsafe fn from_glib_full(ptr: *mut gobject_ffi::GValue) -> Self {
                let mut ret = Value::uninitialized();
                ptr::swap(&mut ret.0, ptr);
                ffi::g_free(ptr as *mut c_void);
                $wrap(ret)
            }
        }

        impl FromGlibContainerAsVec<*mut gobject_ffi::GValue, *mut *mut gobject_ffi::GValue>
            for $name
        {
            unsafe fn from_glib_none_num_as_vec(
                ptr: *mut *mut gobject_ffi::GValue,
                num: usize,
            ) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push(from_glib_none(ptr::read(ptr.add(i))));
                }
                res
            }

            unsafe fn from_glib_container_num_as_vec(
                ptr: *mut *mut gobject_ffi::GValue,
                num: usize,
            ) -> Vec<Self> {
                let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
                ffi::g_free(ptr as *mut _);
                res
            }

            unsafe fn from_glib_full_num_as_vec(
                ptr: *mut *mut gobject_ffi::GValue,
                num: usize,
            ) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push(from_glib_full(ptr::read(ptr.add(i))));
                }
                ffi::g_free(ptr as *mut _);
                res
            }
        }

        impl FromGlibPtrArrayContainerAsVec<*mut gobject_ffi::GValue, *mut *mut gobject_ffi::GValue>
            for $name
        {
            unsafe fn from_glib_none_as_vec(ptr: *mut *mut gobject_ffi::GValue) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_container_as_vec(ptr: *mut *mut gobject_ffi::GValue) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_full_as_vec(ptr: *mut *mut gobject_ffi::GValue) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, c_ptr_array_len(ptr))
            }
        }

        impl FromGlibContainerAsVec<*mut gobject_ffi::GValue, *const *mut gobject_ffi::GValue>
            for $name
        {
            unsafe fn from_glib_none_num_as_vec(
                ptr: *const *mut gobject_ffi::GValue,
                num: usize,
            ) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *mut *mut _, num)
            }

            unsafe fn from_glib_container_num_as_vec(
                _: *const *mut gobject_ffi::GValue,
                _: usize,
            ) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }

            unsafe fn from_glib_full_num_as_vec(
                _: *const *mut gobject_ffi::GValue,
                _: usize,
            ) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }
        }

        impl
            FromGlibPtrArrayContainerAsVec<
                *mut gobject_ffi::GValue,
                *const *mut gobject_ffi::GValue,
            > for $name
        {
            unsafe fn from_glib_none_as_vec(ptr: *const *mut gobject_ffi::GValue) -> Vec<Self> {
                FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr as *mut *mut _)
            }

            unsafe fn from_glib_container_as_vec(_: *const *mut gobject_ffi::GValue) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }

            unsafe fn from_glib_full_as_vec(_: *const *mut gobject_ffi::GValue) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }
        }
    };
}

from_glib!(Value, |v| v);

pub struct ValueArray(Vec<gobject_ffi::GValue>);

impl Drop for ValueArray {
    fn drop(&mut self) {
        unsafe {
            for value in &mut self.0 {
                // Before GLib 2.48, unsetting a zeroed GValue would give critical warnings
                // https://bugzilla.gnome.org/show_bug.cgi?id=755766
                if value.g_type != gobject_ffi::G_TYPE_INVALID {
                    gobject_ffi::g_value_unset(value);
                }
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
#[derive(Clone)]
#[repr(transparent)]
pub struct TypedValue<T>(Value, PhantomData<*const T>);

impl<'a, T: FromValueOptional<'a> + SetValue> TypedValue<T> {
    /// Returns the value.
    ///
    /// Types that don't support a `None` value always return `Some`. See
    /// `get_some`.
    pub fn get(&'a self) -> Option<T> {
        unsafe { T::from_value_optional(self) }
    }

    /// Returns the value.
    ///
    /// This method is only available for types that don't support a `None`
    /// value.
    pub fn get_some(&'a self) -> T
    where
        T: FromValue<'a>,
    {
        unsafe { T::from_value(self) }
    }

    /// Sets the value.
    ///
    /// This method is only available for types that support a `None` value.
    pub fn set<U: ?Sized + SetValueOptional>(&mut self, value: Option<&U>)
    where
        T: Borrow<U>,
    {
        unsafe { SetValueOptional::set_value_optional(&mut self.0, value) }
    }

    /// Sets the value to `None`.
    ///
    /// This method is only available for types that support a `None` value.
    pub fn set_none(&mut self)
    where
        T: SetValueOptional,
    {
        unsafe { T::set_value_optional(&mut self.0, None) }
    }

    /// Sets the value.
    pub fn set_some<U: ?Sized + SetValue>(&mut self, value: &U)
    where
        T: Borrow<U>,
    {
        unsafe { SetValue::set_value(&mut self.0, value) }
    }
}

impl<T> fmt::Debug for TypedValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_tuple("TypedValue").field(&self.0).finish()
    }
}

impl<'a, T: FromValueOptional<'a> + SetValue + Send> TypedValue<T> {
    pub fn into_send_value(self) -> SendValue {
        SendValue(self.0)
    }
}

impl<T> Deref for TypedValue<T> {
    type Target = Value;

    fn deref(&self) -> &Value {
        &self.0
    }
}

unsafe impl<T: Send> Send for TypedValue<T> {}
unsafe impl<T: Sync> Sync for TypedValue<T> {}

impl<'a, T: FromValueOptional<'a> + SetValueOptional> From<Option<&'a T>> for TypedValue<T> {
    fn from(value: Option<&'a T>) -> Self {
        TypedValue(Value::from(value), PhantomData)
    }
}

impl<'a, T: FromValueOptional<'a> + SetValue> From<&'a T> for TypedValue<T> {
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

impl<'a> From<TypedValue<&'a str>> for TypedValue<String> {
    fn from(value: TypedValue<&str>) -> Self {
        TypedValue(value.0, PhantomData)
    }
}

impl<'a> From<TypedValue<String>> for TypedValue<&'a str> {
    fn from(value: TypedValue<String>) -> Self {
        TypedValue(value.0, PhantomData)
    }
}

#[doc(hidden)]
impl<'a, T: 'a> ToGlibPtrMut<'a, *mut gobject_ffi::GValue> for TypedValue<T> {
    type Storage = &'a mut TypedValue<T>;

    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut gobject_ffi::GValue, Self> {
        StashMut(&mut (self.0).0, self)
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
            let mut ret = Value::from_type(T::static_type());
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
            let mut ret = Value::from_type(T::static_type());
            T::set_value(&mut ret, self);
            ret
        }
    }

    #[inline]
    fn to_value_type(&self) -> Type {
        T::static_type()
    }
}

/// A version of [`Value`](struct.Value.html) for storing `Send` types, that implements Send
/// itself.
///
/// See the [module documentation](index.html) for more details.
#[derive(Clone)]
#[repr(transparent)]
pub struct SendValue(Value);

unsafe impl Send for SendValue {}

impl SendValue {
    /// Tries to downcast to a `TypedValue`.
    ///
    /// Returns `Ok(TypedValue<T>)` if the value carries a type corresponding
    /// to `T` and `Err(self)` otherwise.
    pub fn downcast<'a, T: FromValueOptional<'a> + SetValue + Send>(
        self,
    ) -> Result<TypedValue<T>, Self> {
        self.0.downcast().map_err(SendValue)
    }

    /// Tries to downcast to a `&TypedValue`.
    ///
    /// Returns `Some(&TypedValue<T>)` if the value carries a type corresponding
    /// to `T` and `None` otherwise.
    pub fn downcast_ref<'a, T: FromValueOptional<'a> + SetValue>(&self) -> Option<&TypedValue<T>> {
        unsafe {
            let ok = from_glib(gobject_ffi::g_type_check_value_holds(
                mut_override(self.to_glib_none().0),
                T::static_type().to_glib(),
            ));
            if ok {
                // This cast is safe because SendValue and TypedValue have the same
                // representation: the only difference is the zero-sized phantom data
                Some(&*(self as *const SendValue as *const TypedValue<T>))
            } else {
                None
            }
        }
    }

    #[doc(hidden)]
    pub fn into_raw(self) -> gobject_ffi::GValue {
        self.0.into_raw()
    }
}

impl fmt::Debug for SendValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        <Value as fmt::Debug>::fmt(&self.0, f)
    }
}

impl Deref for SendValue {
    type Target = Value;

    fn deref(&self) -> &Value {
        &self.0
    }
}

impl<'a, T: ?Sized + SetValueOptional + Send> From<Option<&'a T>> for SendValue {
    #[inline]
    fn from(value: Option<&'a T>) -> Self {
        SendValue(value.to_value())
    }
}

impl<'a, T: ?Sized + SetValue + Send> From<&'a T> for SendValue {
    #[inline]
    fn from(value: &'a T) -> Self {
        SendValue(value.to_value())
    }
}

impl<T: Send> From<TypedValue<T>> for SendValue {
    fn from(value: TypedValue<T>) -> Self {
        SendValue(value.0)
    }
}

from_glib!(SendValue, SendValue);

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut gobject_ffi::GValue> for SendValue {
    type Storage = &'a mut SendValue;

    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut gobject_ffi::GValue, Self> {
        StashMut(&mut (self.0).0, self)
    }
}

/// Converts to `SendValue`.
pub trait ToSendValue: Send + ToValue {
    /// Returns a `SendValue` clone of `self`.
    fn to_send_value(&self) -> SendValue;
}

impl<T: SetValueOptional + Send + ToValue> ToSendValue for Option<T> {
    fn to_send_value(&self) -> SendValue {
        SendValue(self.to_value())
    }
}

impl<T: ?Sized + SetValue + Send + ToValue> ToSendValue for T {
    fn to_send_value(&self) -> SendValue {
        SendValue(self.to_value())
    }
}

/// Extracts a value.
///
/// Types that don't support a `None` value always return `Some`.
pub trait FromValueOptional<'a>: StaticType + Sized {
    /// # Safety
    ///
    /// The caller is responsible for ensuring the given `Value` is of a suitable
    /// type for this conversion.
    unsafe fn from_value_optional(value: &'a Value) -> Option<Self>;
}

/// Extracts a value.
///
/// Only implemented for types that don't support a `None` value.
pub trait FromValue<'a>: FromValueOptional<'a> {
    /// # Safety
    ///
    /// The caller is responsible for ensuring the given `Value` is of a suitable
    /// type for this conversion.
    unsafe fn from_value(value: &'a Value) -> Self;
}

/// Sets a value.
///
/// Only implemented for types that support a `None` value.
pub trait SetValueOptional: SetValue {
    /// # Safety
    ///
    /// The caller is responsible for ensuring the given `Value` is of a suitable
    /// type for this conversion.
    unsafe fn set_value_optional(value: &mut Value, new_value: Option<&Self>);
}

/// Sets a value.
pub trait SetValue: StaticType {
    /// # Safety
    ///
    /// The caller is responsible for ensuring the given `Value` is of a suitable
    /// type for this conversion.
    unsafe fn set_value(value: &mut Value, new_value: &Self);
}

impl<'a> FromValueOptional<'a> for String {
    unsafe fn from_value_optional(value: &'a Value) -> Option<Self> {
        from_glib_none(gobject_ffi::g_value_get_string(value.to_glib_none().0))
    }
}

impl<'a> FromValueOptional<'a> for &'a str {
    unsafe fn from_value_optional(value: &'a Value) -> Option<Self> {
        let cstr = gobject_ffi::g_value_get_string(value.to_glib_none().0);
        if cstr.is_null() {
            None
        } else {
            CStr::from_ptr(cstr).to_str().ok()
        }
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

impl<'a> FromValue<'a> for Value {
    unsafe fn from_value(value: &'a Value) -> Self {
        from_glib_none(
            gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *const gobject_ffi::GValue
        )
    }
}

impl<'a> FromValueOptional<'a> for Value {
    unsafe fn from_value_optional(value: &'a Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for &'a Value {
    unsafe fn from_value(value: &'a Value) -> Self {
        &*(gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *const Value)
    }
}

impl<'a> FromValueOptional<'a> for &'a Value {
    unsafe fn from_value_optional(value: &'a Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl SetValue for Value {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            this.to_glib_none().0 as ffi::gpointer,
        )
    }
}

impl SetValueOptional for Value {
    unsafe fn set_value_optional(value: &mut Value, this: Option<&Self>) {
        if let Some(this) = this {
            gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                this.to_glib_none().0 as ffi::gpointer,
            );
        } else {
            gobject_ffi::g_value_set_boxed(value.to_glib_none_mut().0, ptr::null_mut());
        }
    }
}

impl<'a> FromValueOptional<'a> for Vec<String> {
    unsafe fn from_value_optional(value: &'a Value) -> Option<Self> {
        Some(<Vec<String> as FromValue>::from_value(value))
    }
}

impl<'a> FromValue<'a> for Vec<String> {
    unsafe fn from_value(value: &'a Value) -> Self {
        let ptr = gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *const *const c_char;
        FromGlibPtrContainer::from_glib_none(ptr)
    }
}

impl<'a> FromValueOptional<'a> for Vec<GString> {
    unsafe fn from_value_optional(value: &'a Value) -> Option<Self> {
        Some(<Vec<GString> as FromValue>::from_value(value))
    }
}

impl<'a> FromValue<'a> for Vec<GString> {
    unsafe fn from_value(value: &'a Value) -> Self {
        let ptr = gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *const *const c_char;
        FromGlibPtrContainer::from_glib_none(ptr)
    }
}

impl<'a> SetValue for [&'a str] {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        let ptr: *mut *mut c_char = this.to_glib_full();
        gobject_ffi::g_value_take_boxed(value.to_glib_none_mut().0, ptr as *const c_void)
    }
}

impl<'a> SetValueOptional for [&'a str] {
    unsafe fn set_value_optional(value: &mut Value, this: Option<&Self>) {
        let ptr: *mut *mut c_char = this.to_glib_full();
        gobject_ffi::g_value_take_boxed(value.to_glib_none_mut().0, ptr as *const c_void)
    }
}

impl SetValue for Vec<String> {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        let ptr: *mut *mut c_char = this.to_glib_full();
        gobject_ffi::g_value_take_boxed(value.to_glib_none_mut().0, ptr as *const c_void)
    }
}

impl SetValueOptional for Vec<String> {
    #[allow(clippy::redundant_closure)]
    unsafe fn set_value_optional(value: &mut Value, this: Option<&Self>) {
        let ptr: *mut *mut c_char = this.map(|v| v.to_glib_full()).unwrap_or(ptr::null_mut());
        gobject_ffi::g_value_take_boxed(value.to_glib_none_mut().0, ptr as *const c_void)
    }
}

impl<'a, T: ?Sized + SetValue> SetValue for &'a T {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        SetValue::set_value(value, *this)
    }
}

impl<'a, T: ?Sized + SetValueOptional> SetValueOptional for &'a T {
    unsafe fn set_value_optional(value: &mut Value, this: Option<&Self>) {
        SetValueOptional::set_value_optional(value, this.cloned())
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

impl<'a> FromValueOptional<'a> for bool {
    unsafe fn from_value_optional(value: &'a Value) -> Option<Self> {
        Some(from_glib(gobject_ffi::g_value_get_boolean(
            value.to_glib_none().0,
        )))
    }
}

impl<'a> FromValue<'a> for bool {
    unsafe fn from_value(value: &'a Value) -> Self {
        from_glib(gobject_ffi::g_value_get_boolean(value.to_glib_none().0))
    }
}

impl SetValue for bool {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_boolean(value.to_glib_none_mut().0, this.to_glib())
    }
}

macro_rules! numeric {
    ($name:ty, $get:expr, $set:expr) => {
        impl<'a> FromValueOptional<'a> for $name {
            unsafe fn from_value_optional(value: &'a Value) -> Option<Self> {
                Some($get(value.to_glib_none().0))
            }
        }

        impl<'a> FromValue<'a> for $name {
            unsafe fn from_value(value: &'a Value) -> Self {
                $get(value.to_glib_none().0)
            }
        }

        impl SetValue for $name {
            unsafe fn set_value(value: &mut Value, this: &Self) {
                $set(value.to_glib_none_mut().0, *this)
            }
        }
    };
}

numeric!(
    i8,
    gobject_ffi::g_value_get_schar,
    gobject_ffi::g_value_set_schar
);
numeric!(
    u8,
    gobject_ffi::g_value_get_uchar,
    gobject_ffi::g_value_set_uchar
);
numeric!(
    i32,
    gobject_ffi::g_value_get_int,
    gobject_ffi::g_value_set_int
);
numeric!(
    u32,
    gobject_ffi::g_value_get_uint,
    gobject_ffi::g_value_set_uint
);
numeric!(
    i64,
    gobject_ffi::g_value_get_int64,
    gobject_ffi::g_value_set_int64
);
numeric!(
    u64,
    gobject_ffi::g_value_get_uint64,
    gobject_ffi::g_value_set_uint64
);
numeric!(
    crate::ILong,
    |v| gobject_ffi::g_value_get_long(v).into(),
    |v, i: crate::ILong| gobject_ffi::g_value_set_long(v, i.0)
);
numeric!(
    crate::ULong,
    |v| gobject_ffi::g_value_get_ulong(v).into(),
    |v, i: crate::ULong| gobject_ffi::g_value_set_ulong(v, i.0)
);
numeric!(
    f32,
    gobject_ffi::g_value_get_float,
    gobject_ffi::g_value_set_float
);
numeric!(
    f64,
    gobject_ffi::g_value_get_double,
    gobject_ffi::g_value_set_double
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_value() {
        use std::thread;

        let v = SendValue::from(&1i32);

        // Must compile, while it must fail with Value
        thread::spawn(move || drop(v)).join().unwrap();
    }

    #[test]
    fn test_strv() {
        let v = vec!["123", "456"].to_value();
        assert_eq!(
            v.get::<Vec<GString>>(),
            Ok(Some(vec![GString::from("123"), GString::from("456")]))
        );

        let v = vec![String::from("123"), String::from("456")].to_value();
        assert_eq!(
            v.get::<Vec<GString>>(),
            Ok(Some(vec![GString::from("123"), GString::from("456")]))
        );
    }

    #[test]
    fn test_get() {
        let v = 123.to_value();
        assert_eq!(v.get(), Ok(Some(123)));
        assert_eq!(v.get_some::<i32>(), Ok(123));
        assert_eq!(
            v.get::<&str>(),
            Err(GetError::new_type_mismatch(Type::I32, Type::STRING))
        );
        assert_eq!(
            v.get_some::<bool>(),
            Err(GetError::new_type_mismatch(Type::I32, Type::BOOL))
        );

        let some_v = Some("test").to_value();
        assert_eq!(some_v.get::<&str>(), Ok(Some("test")));
        assert_eq!(some_v.get::<&str>(), Ok(Some("test")));
        assert_eq!(
            some_v.get::<i32>(),
            Err(GetError::new_type_mismatch(Type::STRING, Type::I32))
        );

        let none_str: Option<&str> = None;
        let none_v = none_str.to_value();
        assert_eq!(none_v.get::<&str>(), Ok(None));
        assert_eq!(
            none_v.get::<i32>(),
            Err(GetError::new_type_mismatch(Type::STRING, Type::I32))
        );
    }

    #[test]
    fn test_transform() {
        let v = 123.to_value();
        let v2 = v
            .transform::<String>()
            .expect("Failed to transform to string");
        assert_eq!(v2.get::<&str>(), Ok(Some("123")));
    }

    #[test]
    fn test_debug() {
        fn value_debug_string<T: ToValue>(val: T) -> String {
            format!("{:?}", val.to_value())
        }

        assert_eq!(value_debug_string(1u32), "(guint) 1");
        assert_eq!(value_debug_string(2i32), "(gint) 2");
        assert_eq!(value_debug_string(false), "(gboolean) FALSE");
        assert_eq!(value_debug_string("FooBar"), r#"(gchararray) "FooBar""#);
    }
}

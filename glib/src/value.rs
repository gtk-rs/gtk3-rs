// Take a look at the license at the top of the repository in the LICENSE file.

//! `Value` binding and helper traits.
//!
//! The type of a [`Value`](struct.Value.html) is dynamic in that it generally
//! isn't known at compile time but once created a `Value` can't change its
//! type.
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
//! use glib::Value;
//!
//! // Value implements From<&i32>, From<&str> // and From<Option<&str>>.
//! // Another option is the `ToValue` trait.
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
use std::error;
use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::ops::Deref;
use std::ptr;

use crate::gstring::GString;
use crate::translate::*;
use crate::types::{StaticType, Type};
use crate::{ParamFlags, ParamSpec};

/// A type that can be stored in `Value`s.
pub trait ValueType: ToValue + for<'r> FromValue<'r> /* TODO: TryFrom<Value> + Into<Value> */ {
    /// Type to get the `Type` from.
    ///
    /// This exists only for handling optional types.
    // FIXME: Should default to Self once associated type defaults are stabilized
    // https://github.com/rust-lang/rust/issues/29661
    type Type: StaticType;
}

/// A type that can be stored in `Value`s and is optional.
///
/// These are types were storing an `Option` is valid. Examples are `String` and all object types.
pub trait ValueTypeOptional:
    ValueType + ToValueOptional + for<'r> FromValue<'r, Error = WrongValueTypeOrNoneError>
{
}

impl<'r, T: ValueTypeOptional> ValueType for Option<T>
where
    Self: for<'s> FromValue<'s, Error = WrongValueTypeError>,
{
    type Type = T::Type;
}

/// A type for which a `ParamSpec` exists and that can be used for object properties.
pub trait UsableAsParam: ValueType {
    /// Create a new default `ParamSpec` for this type.
    ///
    /// For types that allow specifiying a default value or valid range, these will be initialized
    /// with the equivalent of `None` or `0` and the whole valid range.
    fn param_spec(name: &str, nick: &str, blurb: &str, flags: ParamFlags) -> ParamSpec;
}

/// A type for which a `ParamSpec` exists that allows specifying a default value.
pub trait UsableAsParamWithDefault: UsableAsParam {
    /// Create a new `ParamSpec` for this type with a default value.
    ///
    /// For types that allow specifying a valid range, these will be initialized with the whole
    /// valid range.
    fn param_spec_with_default(
        name: &str,
        nick: &str,
        blurb: &str,
        // FIXME: Should probably be `Self`
        default: &Self,
        flags: ParamFlags,
    ) -> ParamSpec;
}

/// A type for which a `ParamSpec` exists that allows specifying a default value and valid range.
pub trait UsableAsParamWithMinMax: UsableAsParamWithDefault {
    /// Create a new `ParamSpec` for this type.
    fn param_spec_with_min_max(
        name: &str,
        nick: &str,
        blurb: &str,
        min: &Self,
        max: &Self,
        default: &Self,
        flags: ParamFlags,
    ) -> ParamSpec;
}

/// An error returned from the [`get`](struct.Value.html#method.get)
/// function on a [`Value`](struct.Value.html) for non-optional types
/// and `Option`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WrongValueTypeError {
    pub actual: Type,
    pub requested: Type,
}

impl WrongValueTypeError {
    pub fn check<T: StaticType>(value: &Value) -> Result<(), WrongValueTypeError> {
        unsafe {
            if gobject_ffi::g_type_check_value_holds(&value.0, T::static_type().0) == ffi::GFALSE {
                Err(WrongValueTypeError {
                    actual: from_glib(value.0.g_type),
                    requested: T::static_type(),
                })
            } else {
                Ok(())
            }
        }
    }

    pub fn type_mismatch(actual: Type, requested: Type) -> Self {
        WrongValueTypeError { actual, requested }
    }
}

impl fmt::Display for WrongValueTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Value type mismatch. Actual {:?}, requested {:?}",
            self.actual, self.requested,
        )
    }
}

impl error::Error for WrongValueTypeError {}

/// An error returned from the [`get`](struct.Value.html#method.get)
/// function on a [`Value`](struct.Value.html) for optional types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WrongValueTypeOrNoneError {
    WrongValueType(WrongValueTypeError),
    UnexpectedNone,
}

impl fmt::Display for WrongValueTypeOrNoneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WrongValueTypeOrNoneError::WrongValueType(err) => err.fmt(f),
            WrongValueTypeOrNoneError::UnexpectedNone => write!(f, "Unexpected None",),
        }
    }
}

impl error::Error for WrongValueTypeOrNoneError {}

impl From<WrongValueTypeError> for WrongValueTypeOrNoneError {
    fn from(err: WrongValueTypeError) -> Self {
        WrongValueTypeOrNoneError::WrongValueType(err)
    }
}

/// Trait to retrieve the contained value from a `Value`.
///
/// Usually this would not be used directly but from the [`get`](struct.Value.html#method.get)
/// function on a [`Value`](struct.Value.html)
pub trait FromValue<'a>: Sized {
    /// Error type if a value can't be retrieved.
    type Error: error::Error;

    /// Checks if the correct type is contained in the value
    fn check(value: &'a Value) -> Result<(), Self::Error>;

    /// Try getting the contained value from a `Value`.
    fn from_value(value: &'a Value) -> Result<Self, Self::Error>;
}

/// Blanket implementation for all optional types.
impl<'a, T: FromValue<'a, Error = WrongValueTypeOrNoneError>> FromValue<'a> for Option<T> {
    type Error = WrongValueTypeError;

    fn check(value: &'a Value) -> Result<(), Self::Error> {
        match T::check(value) {
            Err(WrongValueTypeOrNoneError::WrongValueType(err)) => Err(err),
            Err(WrongValueTypeOrNoneError::UnexpectedNone) => Ok(()),
            Ok(_) => Ok(()),
        }
    }

    fn from_value(value: &'a Value) -> Result<Self, Self::Error> {
        match T::from_value(value) {
            Err(WrongValueTypeOrNoneError::WrongValueType(err)) => Err(err),
            Err(WrongValueTypeOrNoneError::UnexpectedNone) => Ok(None),
            Ok(v) => Ok(Some(v)),
        }
    }
}

/// Trait to convert a value to a `Value`.
pub trait ToValue {
    /// Convert a value to a `Value`.
    fn to_value(&self) -> Value;

    /// Returns the type identifer of `self`.
    ///
    /// This is the type of the value to be returned by `to_value`.
    fn to_value_type(&self) -> Type;
}

impl<'a, T: ToValue + ?Sized> From<&'a T> for Value {
    fn from(v: &'a T) -> Self {
        v.to_value()
    }
}

/// Trait to convert an `Option` to a `Value` for optional types.
pub trait ToValueOptional: ToValue + Sized {
    /// Convert an `Option` to a `Value`.
    fn to_value_optional(s: &Option<Self>) -> Value;
}

/// Blanket implementation for all optional types.
impl<T: ToValueOptional + StaticType> ToValue for Option<T> {
    fn to_value(&self) -> Value {
        T::to_value_optional(self)
    }

    fn to_value_type(&self) -> Type {
        T::static_type()
    }
}

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

    /// Creates a new `Value` that is initialized for a given `ValueType`.
    pub fn for_value_type<T: ValueType>() -> Self {
        Value::from_type(T::Type::static_type())
    }

    /// Tries to get a possibly optional value of type `T`.
    ///
    /// Returns `Ok` if the value could be retrieved.
    pub fn get<'a, T: FromValue<'a>>(&'a self) -> Result<T, T::Error> {
        T::from_value(self)
    }

    /// Returns `true` if the type of the value corresponds to `T`
    /// or is a sub-type of `T`.
    #[inline]
    pub fn is<'a, T: ValueType>(&self) -> bool {
        T::check(self).is_ok()
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
    pub fn transform<T: ValueType>(&self) -> Result<Value, crate::BoolError> {
        unsafe {
            let mut dest = Value::for_value_type::<T>();
            if from_glib(gobject_ffi::g_value_transform(
                self.to_glib_none().0,
                dest.to_glib_none_mut().0,
            )) {
                Ok(dest)
            } else {
                Err(crate::bool_error!(
                    "Can't transform value of type '{}' into '{}'",
                    self.type_(),
                    T::Type::static_type()
                ))
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

    pub fn try_into_send_value<'a, T: Send + ValueType>(self) -> Result<SendValue, Self> {
        if T::check(&self).is_err() {
            Err(self)
        } else {
            Ok(SendValue(self))
        }
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
        if self.type_() != Type::Invalid {
            unsafe { gobject_ffi::g_value_unset(self.to_glib_none_mut().0) }
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        unsafe {
            let s: GString =
                from_glib_full(gobject_ffi::g_strdup_value_contents(self.to_glib_none().0));

            f.debug_tuple("Value").field(&s).finish()
        }
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

/// A version of [`Value`](struct.Value.html) for storing `Send` types, that implements Send
/// itself.
///
/// See the [module documentation](index.html) for more details.
#[derive(Clone)]
#[repr(transparent)]
pub struct SendValue(Value);

unsafe impl Send for SendValue {}

impl SendValue {
    #[doc(hidden)]
    pub fn into_raw(self) -> gobject_ffi::GValue {
        self.0.into_raw()
    }
}

impl fmt::Debug for SendValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_tuple("SendValue").field(&self.0).finish()
    }
}

impl Deref for SendValue {
    type Target = Value;

    fn deref(&self) -> &Value {
        &self.0
    }
}

impl<'a, T: ToSendValue + ?Sized> From<&'a T> for SendValue {
    fn from(v: &'a T) -> Self {
        v.to_send_value()
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

impl<T: ToValue + Send> ToSendValue for T {
    fn to_send_value(&self) -> Value {
        SendValue(self.to_value())
    }
}

// FIXME: Isn't this overlapping?
impl<T: ToValueOptional + Send> ToSendValue for Option<T> {
    fn to_send_value(&self) -> Value {
        SendValue(T::to_value_optional(self))
    }
}

impl<'a> ValueType for &'a str {
    type Type = String;
}

impl<'a> FromValue<'a> for &'a str {
    type Error = WrongValueTypeOrNoneError;

    fn check(value: &'a Value) -> Result<(), Self::Error> {
        WrongValueTypeError::check::<String>(value)?;

        unsafe {
            let ptr = gobject_ffi::g_value_get_string(value.to_glib_none().0);
            if ptr.is_null() {
                return Err(WrongValueTypeOrNoneError::UnexpectedNone);
            }
        }

        Ok(())
    }

    fn from_value(value: &'a Value) -> Result<Self, Self::Error> {
        Self::check(value)?;

        unsafe {
            let ptr = gobject_ffi::g_value_get_string(value.to_glib_none().0);
            Ok(CStr::from_ptr(ptr).to_str().expect("Invalid UTF-8"))
        }
    }
}

impl ToValue for str {
    fn to_value(&self) -> Value {
        unsafe {
            let mut value = Value::from_type(<String>::static_type());

            gobject_ffi::g_value_take_string(value.to_glib_none_mut().0, self.to_glib_full());

            value
        }
    }

    fn to_value_type(&self) -> Type {
        String::static_type()
    }
}

impl ToValue for &str {
    fn to_value(&self) -> Value {
        (*self).to_value()
    }

    fn to_value_type(&self) -> Type {
        String::static_type()
    }
}

impl ToValueOptional for &str {
    fn to_value_optional(s: &Option<Self>) -> Value {
        let mut value = Value::for_value_type::<String>();
        unsafe {
            if let Some(s) = s {
                gobject_ffi::g_value_take_string(value.to_glib_none_mut().0, s.to_glib_full());
            } else {
                gobject_ffi::g_value_set_string(value.to_glib_none_mut().0, std::ptr::null());
            }
        }

        value
    }
}

impl ValueType for String {
    type Type = String;
}

impl<'a> FromValue<'a> for String {
    type Error = WrongValueTypeOrNoneError;

    fn check(value: &'a Value) -> Result<(), Self::Error> {
        <&str>::check(value)
    }

    fn from_value(value: &'a Value) -> Result<Self, Self::Error> {
        <&str>::from_value(value).map(String::from)
    }
}

impl ToValue for String {
    fn to_value(&self) -> Value {
        <&str>::to_value(self.as_str())
    }

    fn to_value_type(&self) -> Type {
        String::static_type()
    }
}

impl ToValueOptional for String {
    fn to_value_optional(s: &Option<Self>) -> Value {
        <&str>::to_value_optional(s.as_deref())
    }
}

impl UsableAsParam for String {
    fn param_spec(name: &str, nick: &str, blurb: &str, flags: ParamFlags) -> ParamSpec {
        ParamSpec::string(name, nick, blurb, flags, Some(String::new()))
    }
}

impl UsableAsParamWithDefault for String {
    fn param_spec_with_default(
        name: &str,
        nick: &str,
        blurb: &str,
        default: &String,
        flags: ParamFlags,
    ) -> ParamSpec {
        ParamSpec::string(name, nick, blurb, flags, default)
    }
}

impl UsableAsParam for Option<String> {
    fn param_spec(name: &str, nick: &str, blurb: &str, flags: ParamFlags) -> ParamSpec {
        ParamSpec::string(name, nick, blurb, flags, None)
    }
}

impl UsableAsParamWithDefault for Option<String> {
    fn param_spec_with_default(
        name: &str,
        nick: &str,
        blurb: &str,
        default: &Option<String>,
        flags: ParamFlags,
    ) -> ParamSpec {
        ParamSpec::string(name, nick, blurb, flags, default)
    }
}

impl ValueType for Vec<String> {
    type Type = Vec<String>;
}

impl<'a> FromValue<'a> for Vec<String> {
    type Error = WrongValueTypeError;

    fn check(value: &'a Value) -> Result<(), Self::Error> {
        WrongValueTypeError::check::<Vec<String>>(value)
    }

    fn from_value(value: &'a Value) -> Result<Self, Self::Error> {
        Self::check(value)?;

        unsafe {
            let ptr =
                gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *const *const c_char;
            Ok(FromGlibPtrContainer::from_glib_none(ptr))
        }
    }
}

impl ToValue for Vec<String> {
    fn to_value(&self) -> Value {
        unsafe {
            let mut value = Value::for_value_type::<Vec<String>>();
            let ptr: *mut *mut c_char = self.to_glib_full();
            gobject_ffi::g_value_take_boxed(value.to_glib_none_mut().0, ptr as *const c_void);
            value
        }
    }

    fn to_value_type(&self) -> Type {
        <Vec<String>>::static_type()
    }
}

impl UsableAsParam for Vec<String> {
    fn param_spec(name: &str, nick: &str, blurb: &str, flags: ParamFlags) -> ParamSpec {
        ParamSpec::boxed(name, nick, blurb, <Vec<String>>::static_type(), flags)
    }
}

impl<'a> ToValue for [&'a str] {
    fn to_value(&self) -> Value {
        unsafe {
            let mut value = Value::for_value_type::<Vec<String>>();
            let ptr: *mut *mut c_char = self.to_glib_full();
            gobject_ffi::g_value_take_boxed(value.to_glib_none_mut().0, ptr as *const c_void);
            value
        }
    }

    fn to_value_type(&self) -> Type {
        <Vec<String>>::static_type()
    }
}

impl<'a> ToValue for &'a [&'a str] {
    fn to_value(&self) -> Value {
        unsafe {
            let mut value = Value::for_value_type::<Vec<String>>();
            let ptr: *mut *mut c_char = self.to_glib_full();
            gobject_ffi::g_value_take_boxed(value.to_glib_none_mut().0, ptr as *const c_void);
            value
        }
    }

    fn to_value_type(&self) -> Type {
        <Vec<String>>::static_type()
    }
}

impl ValueType for bool {
    type Type = Self;
}

impl<'a> FromValue<'a> for bool {
    type Error = WrongValueTypeError;

    fn check(value: &'a Value) -> Result<(), Self::Error> {
        WrongValueTypeError::check::<bool>(value)
    }

    fn from_value(value: &'a Value) -> Result<Self, Self::Error> {
        Self::check(value)?;
        unsafe {
            Ok(from_glib(gobject_ffi::g_value_get_boolean(
                value.to_glib_none().0,
            )))
        }
    }
}

impl ToValue for bool {
    fn to_value(&self) -> Value {
        let mut value = Value::for_value_type::<bool>();
        unsafe {
            gobject_ffi::g_value_set_boolean(&mut value.0, self.to_glib());
        }
        value
    }

    fn to_value_type(&self) -> Type {
        Self::static_type()
    }
}

impl UsableAsParam for bool {
    fn param_spec(name: &str, nick: &str, blurb: &str, flags: ParamFlags) -> ParamSpec {
        ParamSpec::boolean(name, nick, blurb, flags, false)
    }
}

impl UsableAsParamWithDefault for bool {
    fn param_spec_with_default(
        name: &str,
        nick: &str,
        blurb: &str,
        default: &bool,
        flags: ParamFlags,
    ) -> ParamSpec {
        ParamSpec::boolean(name, nick, blurb, flags, default)
    }
}

macro_rules! numeric {
    ($name:ty, $get:expr, $set:expr, $default:expr, $min:expr, $max:expr, $param_spec_new:expr) => {
        impl ValueType for $name {
            type Type = Self;
        }

        impl<'a> FromValue<'a> for $name {
            type Error = WrongValueTypeError;

            fn check(value: &'a Value) -> Result<(), Self::Error> {
                WrongValueTypeError::check::<Self>(value)
            }

            fn from_value(value: &'a Value) -> Result<Self, Self::Error> {
                Self::check(value)?;
                unsafe { Ok($get(value.to_glib_none().0)) }
            }
        }

        impl ToValue for $name {
            fn to_value(&self) -> Value {
                let mut value = Value::for_value_type::<$name>();
                unsafe {
                    $set(&mut value.0, *self);
                }
                value
            }

            fn to_value_type(&self) -> Type {
                Self::static_type()
            }
        }

        impl UsableAsParam for $name {
            fn param_spec(name: &str, nick: &str, blurb: &str, flags: ParamFlags) -> ParamSpec {
                $param_spec_new(name, nick, blurb, flags, $min, $max, $default)
            }
        }

        impl UsableAsParamWithDefault for $name {
            fn param_spec_with_default(
                name: &str,
                nick: &str,
                blurb: &str,
                default: &$name,
                flags: ParamFlags,
            ) -> ParamSpec {
                $param_spec_new(name, nick, blurb, flags, $min, $max, default)
            }
        }

        impl UsableAsParamWithMinMax for $name {
            fn param_spec_with_min_max(
                name: &str,
                nick: &str,
                blurb: &str,
                default: &$name,
                min: &$name,
                max: &$name,
                flags: ParamFlags,
            ) -> ParamSpec {
                $param_spec_new(name, nick, blurb, flags, min, max, default)
            }
        }
    };
}

numeric!(
    i8,
    gobject_ffi::g_value_get_schar,
    gobject_ffi::g_value_set_schar,
    0,
    i8::MIN,
    i8::MAX,
    ParamSpec::i8
);
numeric!(
    u8,
    gobject_ffi::g_value_get_uchar,
    gobject_ffi::g_value_set_uchar,
    0,
    u8::MIN,
    u8::MAX,
    ParamSpec::u8
);
numeric!(
    i32,
    gobject_ffi::g_value_get_int,
    gobject_ffi::g_value_set_int,
    0,
    i32::MIN,
    i32::MAX,
    ParamSpec::i32
);
numeric!(
    u32,
    gobject_ffi::g_value_get_uint,
    gobject_ffi::g_value_set_uint,
    0,
    u32::MIN,
    u32::MAX,
    ParamSpec::u32
);
numeric!(
    i64,
    gobject_ffi::g_value_get_int64,
    gobject_ffi::g_value_set_int64,
    0,
    i64::MIN,
    i64::MAX,
    ParamSpec::i64
);
numeric!(
    u64,
    gobject_ffi::g_value_get_uint64,
    gobject_ffi::g_value_set_uint64,
    0,
    u64::MIN,
    u64::MAX,
    ParamSpec::u64
);
numeric!(
    crate::ILong,
    |v| gobject_ffi::g_value_get_long(v).into(),
    |v, i: crate::ILong| gobject_ffi::g_value_set_long(v, i.0),
    crate::ILong(0),
    crate::ILong(libc::c_long::MIN),
    crate::ILong(libc::c_long::MAX),
    ParamSpec::long
);
numeric!(
    crate::ULong,
    |v| gobject_ffi::g_value_get_ulong(v).into(),
    |v, i: crate::ULong| gobject_ffi::g_value_set_ulong(v, i.0),
    crate::ULong(0),
    crate::ULong(libc::c_ulong::MIN),
    crate::ULong(libc::c_ulong::MAX),
    ParamSpec::ulong
);
numeric!(
    f32,
    gobject_ffi::g_value_get_float,
    gobject_ffi::g_value_set_float,
    0.0,
    -f32::MAX,
    f32::MAX,
    ParamSpec::float
);
numeric!(
    f64,
    gobject_ffi::g_value_get_double,
    gobject_ffi::g_value_set_double,
    0.0,
    -f64::MIN,
    f64::MAX,
    ParamSpec::double
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
        assert_eq!(v.get(), Ok(123));
        assert_eq!(
            v.get::<&str>(),
            Err(WrongValueTypeError::type_mismatch(Type::I32, Type::String))
        );
        assert_eq!(
            v.get::<Option<&str>>(),
            Err(WrongValueTypeError::type_mismatch(Type::I32, Type::String))
        );
        assert_eq!(
            v.get::<bool>(),
            Err(WrongValueTypeError::type_mismatch(Type::I32, Type::Bool))
        );

        let some_v = "test".to_value();
        assert_eq!(some_v.get::<&str>(), Ok("test"));
        assert_eq!(some_v.get::<Option<&str>>(), Ok(Some("test")));
        assert_eq!(
            some_v.get::<i32>(),
            Err(WrongValueTypeError::type_mismatch(Type::String, Type::I32))
        );

        let none_str: Option<&str> = None;
        let none_v = none_str.to_value();
        assert_eq!(none_v.get::<Option<&str>>(), Ok(None));
        assert_eq!(
            none_v.get::<&str>(),
            Err(WrongValueTypeOrNoneError::UnexpectedNone)
        );
        assert_eq!(
            none_v.get::<i32>(),
            Err(WrongValueTypeError::type_mismatch(Type::String, Type::I32))
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
}

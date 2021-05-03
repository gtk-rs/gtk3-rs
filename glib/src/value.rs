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
//! // Value implement From<&i32>, From<&str> and From<Option<&str>>.
//! // Another option is the `ToValue` trait.
//! let mut num = 10.to_value();
//! let mut hello = Value::from("Hello!");
//! let none: Option<&str> = None;
//! let str_none = none.to_value();
//!
//! // `is` tests the type of the value.
//! assert!(num.is::<i32>());
//! assert!(hello.is::<String>());
//!
//! // `get` tries to get an optional value of the specified type
//! // and returns an `Err` if the type doesn't match.
//! assert_eq!(num.get(), Ok(10));
//! assert!(num.get::<String>().is_err());
//! assert_eq!(hello.get(), Ok(String::from("Hello!")));
//! assert_eq!(hello.get::<String>(), Ok(String::from("Hello!")));
//! assert_eq!(str_none.get::<Option<String>>(), Ok(None));
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

/// A type that can be stored in `Value`s.
pub trait ValueType: ToValue + FromValue<'static> + 'static {
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
    ValueType + ToValueOptional + FromValueOptional<'static> + StaticType
{
}

impl<T, C> ValueType for Option<T>
where
    T: FromValue<'static, Checker = C> + ValueTypeOptional + StaticType + 'static,
    C: ValueTypeChecker<Error = ValueTypeMismatchOrNoneError>,
{
    type Type = T::Type;
}

/// Trait for `Value` type checkers.
pub unsafe trait ValueTypeChecker {
    type Error: std::error::Error + Send + Sized + 'static;

    fn check(value: &Value) -> Result<(), Self::Error>;
}

/// An error returned from the [`get`](struct.Value.html#method.get) function
/// on a [`Value`](struct.Value.html) for non-optional types an `Option`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ValueTypeMismatchError {
    actual: Type,
    requested: Type,
}

impl ValueTypeMismatchError {
    pub fn new(actual: Type, requested: Type) -> Self {
        Self { actual, requested }
    }
}

impl ValueTypeMismatchError {
    pub fn actual_type(&self) -> Type {
        self.actual
    }

    pub fn requested_type(&self) -> Type {
        self.requested
    }
}

impl fmt::Display for ValueTypeMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Value type mismatch. Actual {:?}, requested {:?}",
            self.actual_type(),
            self.requested_type(),
        )
    }
}

impl error::Error for ValueTypeMismatchError {}

/// Generic `Value` type checker for types.
pub struct GenericValueTypeChecker<T>(std::marker::PhantomData<T>);

unsafe impl<T: StaticType> ValueTypeChecker for GenericValueTypeChecker<T> {
    type Error = ValueTypeMismatchError;

    fn check(value: &Value) -> Result<(), Self::Error> {
        unsafe {
            if gobject_ffi::g_type_check_value_holds(&value.0, T::static_type().into_glib())
                == ffi::GFALSE
            {
                Err(ValueTypeMismatchError::new(
                    Type::from_glib(value.0.g_type),
                    T::static_type(),
                ))
            } else {
                Ok(())
            }
        }
    }
}

/// An error returned from the [`get`](struct.Value.html#method.get)
/// function on a [`Value`](struct.Value.html) for optional types.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ValueTypeMismatchOrNoneError {
    WrongValueType(ValueTypeMismatchError),
    UnexpectedNone,
}

impl fmt::Display for ValueTypeMismatchOrNoneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::WrongValueType(err) => err.fmt(f),
            Self::UnexpectedNone => write!(f, "Unexpected None",),
        }
    }
}

impl error::Error for ValueTypeMismatchOrNoneError {}

impl From<ValueTypeMismatchError> for ValueTypeMismatchOrNoneError {
    fn from(err: ValueTypeMismatchError) -> Self {
        Self::WrongValueType(err)
    }
}

/// Generic `Value` type checker for optional types.
pub struct GenericValueTypeOrNoneChecker<T>(std::marker::PhantomData<T>);

unsafe impl<T: StaticType> ValueTypeChecker for GenericValueTypeOrNoneChecker<T> {
    type Error = ValueTypeMismatchOrNoneError;

    fn check(value: &Value) -> Result<(), Self::Error> {
        GenericValueTypeChecker::<T>::check(value)?;

        unsafe {
            // Values are always zero-initialized so even if pointers are only 32 bits then the
            // whole 64 bit value will be 0 for NULL pointers.
            if value.0.data[0].v_uint64 == 0 {
                return Err(Self::Error::UnexpectedNone);
            }
        }

        Ok(())
    }
}

/// Trait to retrieve the contained value from a `Value`.
///
/// Usually this would not be used directly but from the [`get`](struct.Value.html#method.get)
/// function on a [`Value`](struct.Value.html)
pub unsafe trait FromValue<'a>: Sized {
    /// Value type checker.
    type Checker: ValueTypeChecker;

    /// Get the contained value from a `Value`.
    ///
    /// # Safety
    /// `Self::Checker::check()` must be called first and must not fail.
    unsafe fn from_value(value: &'a Value) -> Self;
}

/// Trait for types that implement `FromValue` and are Optional.
///
/// This trait is auto-implemented for the appropriate types and is sealed.
pub trait FromValueOptional<'a>: private::FromValueOptionalSealed<'a> {}

impl<'a, T, C> FromValueOptional<'a> for T
where
    T: FromValue<'a, Checker = C>,
    C: ValueTypeChecker<Error = ValueTypeMismatchOrNoneError>,
{
}

mod private {
    pub trait FromValueOptionalSealed<'a> {}

    impl<'a, T, C> FromValueOptionalSealed<'a> for T
    where
        T: super::FromValue<'a, Checker = C>,
        C: super::ValueTypeChecker<Error = super::ValueTypeMismatchOrNoneError>,
    {
    }
}

/// Blanket implementation for all optional types.
unsafe impl<'a, T, C> FromValue<'a> for Option<T>
where
    T: FromValue<'a, Checker = C> + StaticType,
    C: ValueTypeChecker<Error = ValueTypeMismatchOrNoneError>,
{
    type Checker = GenericValueTypeChecker<T>;

    unsafe fn from_value(value: &'a Value) -> Self {
        if let Err(ValueTypeMismatchOrNoneError::UnexpectedNone) = T::Checker::check(value) {
            None
        } else {
            Some(T::from_value(value))
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
    fn value_type(&self) -> Type;
}

/// Blanket implementation for all references.
impl<T: ToValue + StaticType> ToValue for &T {
    fn to_value(&self) -> Value {
        T::to_value(*self)
    }

    fn value_type(&self) -> Type {
        T::static_type()
    }
}

/// Trait to convert an `Option` to a `Value` for optional types.
pub trait ToValueOptional {
    /// Convert an `Option` to a `Value`.
    #[allow(clippy::wrong_self_convention)]
    fn to_value_optional(s: Option<&Self>) -> Value;
}

/// Blanket implementation for all optional types.
impl<T: ToValueOptional + StaticType> ToValue for Option<T> {
    fn to_value(&self) -> Value {
        T::to_value_optional(self.as_ref())
    }

    fn value_type(&self) -> Type {
        T::static_type()
    }
}

impl<T: ToValueOptional + StaticType> StaticType for Option<T> {
    fn static_type() -> Type {
        T::static_type()
    }
}

impl<T: ToValueOptional + StaticType + ?Sized> ToValueOptional for &T {
    fn to_value_optional(s: Option<&Self>) -> Value {
        <T as ToValueOptional>::to_value_optional(s.as_ref().map(|s| **s))
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
                gobject_ffi::g_type_check_is_value_type(type_.into_glib()),
                ffi::GTRUE
            );
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, type_.into_glib());
            value
        }
    }

    /// Creates a new `Value` that is initialized for a given `ValueType`.
    pub fn for_value_type<T: ValueType>() -> Self {
        Value::from_type(T::Type::static_type())
    }

    /// Tries to get a value of type `T`.
    ///
    /// Returns `Ok` if the type is correct.
    pub fn get<'a, T>(&'a self) -> Result<T, <<T as FromValue>::Checker as ValueTypeChecker>::Error>
    where
        T: FromValue<'a>,
    {
        unsafe {
            T::Checker::check(self)?;
            Ok(T::from_value(self))
        }
    }

    /// Returns `true` if the type of the value corresponds to `T`
    /// or is a sub-type of `T`.
    #[inline]
    pub fn is<T: StaticType>(&self) -> bool {
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
                src.into_glib(),
                dst.into_glib(),
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

    /// Consumes `Value` and returns the corresponding `GValue`.
    pub fn into_raw(self) -> gobject_ffi::GValue {
        unsafe {
            let s = mem::ManuallyDrop::new(self);
            ptr::read(&s.0)
        }
    }

    pub fn try_into_send_value<T: Send + StaticType>(self) -> Result<SendValue, Self> {
        if self.type_().is_a(T::static_type()) {
            Ok(SendValue(self))
        } else {
            Err(self)
        }
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

impl<'a, T: ?Sized + ToValue> From<&'a T> for Value {
    #[inline]
    fn from(value: &'a T) -> Self {
        value.to_value()
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
                gobject_ffi::g_value_init(res.add(i), v.type_().into_glib());
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
    /// Consumes `SendValue` and returns the corresponding `GValue`.
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

impl<'a, T: ?Sized + ToSendValue> From<&'a T> for SendValue {
    #[inline]
    fn from(value: &'a T) -> Self {
        value.to_send_value()
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

impl<T: Send + ToValue + ?Sized> ToSendValue for T {
    fn to_send_value(&self) -> SendValue {
        SendValue(self.to_value())
    }
}

impl ValueType for &'static str {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for &'a str {
    type Checker = GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a Value) -> Self {
        let ptr = gobject_ffi::g_value_get_string(value.to_glib_none().0);
        CStr::from_ptr(ptr).to_str().expect("Invalid UTF-8")
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

    fn value_type(&self) -> Type {
        String::static_type()
    }
}

impl ToValue for &str {
    fn to_value(&self) -> Value {
        (*self).to_value()
    }

    fn value_type(&self) -> Type {
        String::static_type()
    }
}

impl ToValueOptional for str {
    fn to_value_optional(s: Option<&Self>) -> Value {
        let mut value = Value::for_value_type::<String>();
        unsafe {
            gobject_ffi::g_value_take_string(value.to_glib_none_mut().0, s.to_glib_full());
        }

        value
    }
}

impl ValueType for String {
    type Type = String;
}

unsafe impl<'a> FromValue<'a> for String {
    type Checker = GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a Value) -> Self {
        String::from(<&str>::from_value(value))
    }
}

impl ToValue for String {
    fn to_value(&self) -> Value {
        <&str>::to_value(&self.as_str())
    }

    fn value_type(&self) -> Type {
        String::static_type()
    }
}

impl ToValueOptional for String {
    fn to_value_optional(s: Option<&Self>) -> Value {
        <str>::to_value_optional(s.as_ref().map(|s| s.as_str()))
    }
}

impl ValueType for Vec<String> {
    type Type = Vec<String>;
}

unsafe impl<'a> FromValue<'a> for Vec<String> {
    type Checker = GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a Value) -> Self {
        let ptr = gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *const *const c_char;
        FromGlibPtrContainer::from_glib_none(ptr)
    }
}

impl ToValue for Vec<String> {
    fn to_value(&self) -> Value {
        unsafe {
            let mut value = Value::for_value_type::<Self>();
            let ptr: *mut *mut c_char = self.to_glib_full();
            gobject_ffi::g_value_take_boxed(value.to_glib_none_mut().0, ptr as *const c_void);
            value
        }
    }

    fn value_type(&self) -> Type {
        <Vec<String>>::static_type()
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

    fn value_type(&self) -> Type {
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

    fn value_type(&self) -> Type {
        <Vec<String>>::static_type()
    }
}

impl ValueType for bool {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for bool {
    type Checker = GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a Value) -> Self {
        from_glib(gobject_ffi::g_value_get_boolean(value.to_glib_none().0))
    }
}

impl ToValue for bool {
    fn to_value(&self) -> Value {
        let mut value = Value::for_value_type::<Self>();
        unsafe {
            gobject_ffi::g_value_set_boolean(&mut value.0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

macro_rules! numeric {
    ($name:ty, $get:expr, $set:expr) => {
        impl ValueType for $name {
            type Type = Self;
        }

        unsafe impl<'a> FromValue<'a> for $name {
            type Checker = GenericValueTypeChecker<Self>;

            unsafe fn from_value(value: &'a Value) -> Self {
                $get(value.to_glib_none().0)
            }
        }

        impl ToValue for $name {
            fn to_value(&self) -> Value {
                let mut value = Value::for_value_type::<Self>();
                unsafe {
                    $set(&mut value.0, *self);
                }
                value
            }

            fn value_type(&self) -> Type {
                Self::static_type()
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

impl ValueType for Value {
    type Type = Value;
}

unsafe impl<'a> FromValue<'a> for Value {
    type Checker = GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a Value) -> Self {
        let ptr = gobject_ffi::g_value_get_boxed(value.to_glib_none().0);
        from_glib_none(ptr as *const gobject_ffi::GValue)
    }
}

impl ToValue for Value {
    fn to_value(&self) -> Value {
        unsafe {
            let mut value = Value::from_type(<Value>::static_type());

            gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                self.to_glib_none().0 as ffi::gconstpointer,
            );

            value
        }
    }

    fn value_type(&self) -> Type {
        Value::static_type()
    }
}

impl ToValueOptional for Value {
    fn to_value_optional(s: Option<&Self>) -> Value {
        let mut value = Value::for_value_type::<Self>();
        unsafe {
            gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                s.to_glib_none().0 as ffi::gconstpointer,
            );
        }

        value
    }
}

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
            Ok(vec![GString::from("123"), GString::from("456")])
        );

        let v = vec![String::from("123"), String::from("456")].to_value();
        assert_eq!(
            v.get::<Vec<GString>>(),
            Ok(vec![GString::from("123"), GString::from("456")])
        );
    }

    #[test]
    fn test_from_to_value() {
        let v = 123.to_value();
        assert_eq!(v.get(), Ok(123));
        assert_eq!(
            v.get::<&str>(),
            Err(ValueTypeMismatchError::new(Type::I32, Type::STRING).into())
        );
        assert_eq!(
            v.get::<bool>(),
            Err(ValueTypeMismatchError::new(Type::I32, Type::BOOL))
        );

        // Check if &str / str / Option<&str> etc can be converted and retrieved
        let v_str = "test".to_value();
        assert_eq!(v_str.get::<&str>(), Ok("test"));
        assert_eq!(v_str.get::<Option<&str>>(), Ok(Some("test")));
        assert_eq!(
            v_str.get::<i32>(),
            Err(ValueTypeMismatchError::new(Type::STRING, Type::I32))
        );

        let some_v = Some("test").to_value();
        assert_eq!(some_v.get::<&str>(), Ok("test"));
        assert_eq!(some_v.get::<Option<&str>>(), Ok(Some("test")));
        assert_eq!(
            some_v.get::<i32>(),
            Err(ValueTypeMismatchError::new(Type::STRING, Type::I32))
        );

        let none_str: Option<&str> = None;
        let none_v = none_str.to_value();
        assert_eq!(none_v.get::<Option<&str>>(), Ok(None));
        assert_eq!(
            none_v.get::<i32>(),
            Err(ValueTypeMismatchError::new(Type::STRING, Type::I32))
        );

        // Check if owned T and Option<T> can be converted and retrieved
        let v_str = String::from("test").to_value();
        assert_eq!(v_str.get::<String>(), Ok(String::from("test")));
        assert_eq!(
            v_str.get::<Option<String>>(),
            Ok(Some(String::from("test")))
        );
        assert_eq!(
            v_str.get::<i32>(),
            Err(ValueTypeMismatchError::new(Type::STRING, Type::I32))
        );

        let some_v = Some(String::from("test")).to_value();
        assert_eq!(some_v.get::<String>(), Ok(String::from("test")));
        assert_eq!(
            some_v.get::<Option<String>>(),
            Ok(Some(String::from("test")))
        );
        assert_eq!(
            some_v.get::<i32>(),
            Err(ValueTypeMismatchError::new(Type::STRING, Type::I32))
        );

        let none_str: Option<String> = None;
        let none_v = none_str.to_value();
        assert_eq!(none_v.get::<Option<String>>(), Ok(None));
        assert_eq!(
            none_v.get::<i32>(),
            Err(ValueTypeMismatchError::new(Type::STRING, Type::I32))
        );

        // Check if &T and Option<&T> can be converted and retrieved
        let v_str = (&String::from("test")).to_value();
        assert_eq!(v_str.get::<String>(), Ok(String::from("test")));
        assert_eq!(
            v_str.get::<Option<String>>(),
            Ok(Some(String::from("test")))
        );
        assert_eq!(
            v_str.get::<i32>(),
            Err(ValueTypeMismatchError::new(Type::STRING, Type::I32))
        );

        let some_v = Some(&String::from("test")).to_value();
        assert_eq!(some_v.get::<String>(), Ok(String::from("test")));
        assert_eq!(
            some_v.get::<Option<String>>(),
            Ok(Some(String::from("test")))
        );
        assert_eq!(
            some_v.get::<i32>(),
            Err(ValueTypeMismatchError::new(Type::STRING, Type::I32))
        );

        let none_str: Option<&String> = None;
        let none_v = none_str.to_value();
        assert_eq!(none_v.get::<Option<String>>(), Ok(None));
        assert_eq!(
            none_v.get::<i32>(),
            Err(ValueTypeMismatchError::new(Type::STRING, Type::I32))
        );
    }

    #[test]
    fn test_transform() {
        let v = 123.to_value();
        let v2 = v
            .transform::<String>()
            .expect("Failed to transform to string");
        assert_eq!(v2.get::<&str>(), Ok("123"));
    }

    #[test]
    fn test_into_raw() {
        unsafe {
            let mut v = 123.to_value().into_raw();
            assert_eq!(gobject_ffi::g_type_check_value(&v), ffi::GTRUE);
            assert_eq!(gobject_ffi::g_value_get_int(&v), 123);
            gobject_ffi::g_value_unset(&mut v);
        }
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

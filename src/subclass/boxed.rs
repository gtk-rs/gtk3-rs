// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Module for registering boxed types for Rust types.

use glib_sys;
use gobject_sys;
use std::ops;
use translate::*;
use value::*;

/// Trait for defining boxed types.
///
/// Links together the type name with the type itself.
///
/// See [`register_boxed_type`] for registering an implementation of this trait
/// with the type system.
///
/// [`register_boxed_type`]: fn.register_boxed_type.html
pub trait BoxedType: Clone + Sized + 'static {
    /// Boxed type name.
    ///
    /// This must be unique in the whole process.
    const NAME: &'static str;

    /// Returns the type ID.
    ///
    /// This is usually defined via the [`GBoxed!`] derive macro.
    ///
    /// [`GBoxed!`]: ../../derive.GBoxed.html
    fn get_type() -> ::Type;
}

/// Register a boxed `glib::Type` ID for `T`.
///
/// This must be called only once and will panic on a second call.
///
/// See [`GBoxed!`] for defining a function that ensures that
/// this is only called once and returns the type id.
///
/// [`GBoxed!`]: ../../derive.GBoxed.html
pub fn register_boxed_type<T: BoxedType>() -> ::Type {
    unsafe extern "C" fn boxed_copy<T: BoxedType>(v: glib_sys::gpointer) -> glib_sys::gpointer {
        let v = &*(v as *mut T);
        let copy = Box::new(v.clone());

        Box::into_raw(copy) as glib_sys::gpointer
    }
    unsafe extern "C" fn boxed_free<T: BoxedType>(v: glib_sys::gpointer) {
        let v = v as *mut T;
        let _ = Box::from_raw(v);
    }
    unsafe {
        use std::ffi::CString;

        let type_name = CString::new(T::NAME).unwrap();
        if gobject_sys::g_type_from_name(type_name.as_ptr()) != gobject_sys::G_TYPE_INVALID {
            panic!(
                "Type {} has already been registered",
                type_name.to_str().unwrap()
            );
        }

        from_glib(gobject_sys::g_boxed_type_register_static(
            type_name.as_ptr(),
            Some(boxed_copy::<T>),
            Some(boxed_free::<T>),
        ))
    }
}

/// Wrapper struct for storing any `BoxedType` in `glib::Value`.
///
/// Instead of this the [`GBoxed!`] derive macro can be used to
/// directly implement the relevant traits on the type itself.
///
/// [`GBoxed!`]: ../../derive.GBoxed.html
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Boxed<T: BoxedType>(pub T);

impl<T: BoxedType> ops::Deref for Boxed<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: BoxedType> ops::DerefMut for Boxed<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: BoxedType> ::StaticType for Boxed<T> {
    fn static_type() -> ::Type {
        T::get_type()
    }
}

impl<T: BoxedType> SetValue for Boxed<T> {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        let ptr: *mut Boxed<T> = Box::into_raw(Box::new(this.clone()));
        gobject_sys::g_value_take_boxed(value.to_glib_none_mut().0, ptr as *mut _);
    }
}

impl<T: BoxedType> SetValueOptional for Boxed<T> {
    unsafe fn set_value_optional(value: &mut Value, this: Option<&Self>) {
        let this = this.expect("None not allowed");
        let ptr: *mut Boxed<T> = Box::into_raw(Box::new(this.clone()));
        gobject_sys::g_value_take_boxed(value.to_glib_none_mut().0, ptr as *mut _);
    }
}

impl<'a, T: BoxedType> FromValueOptional<'a> for &'a Boxed<T> {
    unsafe fn from_value_optional(value: &'a Value) -> Option<Self> {
        let ptr = gobject_sys::g_value_get_boxed(value.to_glib_none().0);
        assert!(!ptr.is_null());
        Some(&*(ptr as *mut Boxed<T>))
    }
}

impl<'a, T: BoxedType> FromValue<'a> for &'a Boxed<T> {
    unsafe fn from_value(value: &'a Value) -> Self {
        let ptr = gobject_sys::g_value_get_boxed(value.to_glib_none().0);
        assert!(!ptr.is_null());
        &*(ptr as *mut Boxed<T>)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // GBoxed macro assumes 'glib' is in scope
    use crate as glib;

    #[derive(Clone, Debug, PartialEq, Eq, glib::GBoxed)]
    #[gboxed(type_name = "MyBoxed")]
    struct MyBoxed(String);

    #[test]
    fn test_register() {
        assert_ne!(::Type::Invalid, MyBoxed::get_type());
    }

    #[test]
    fn test_value_boxed() {
        assert_ne!(::Type::Invalid, MyBoxed::get_type());

        let b = Boxed(MyBoxed(String::from("abc")));
        let v = b.to_value();
        let b2 = v.get_some::<&Boxed<MyBoxed>>().unwrap();
        assert_eq!(&b, b2);
    }

    #[test]
    fn test_value() {
        assert_ne!(::Type::Invalid, MyBoxed::get_type());

        let b = MyBoxed(String::from("abc"));
        let v = b.to_value();
        let b2 = v.get_some::<&MyBoxed>().unwrap();
        assert_eq!(&b, b2);
    }
}

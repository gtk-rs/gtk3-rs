// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! `IMPL` Object wrapper implementation and `Object` binding.

use translate::*;
use types::{self, StaticType};
use wrapper::{UnsafeFrom, Wrapper};
use gobject_ffi;
use std::mem;
use std::ptr;

use Value;
use value::SetValue;
use Type;
use BoolError;
use Closure;

/// Upcasting and downcasting support.
///
/// Provides conversions up and down the class hierarchy tree.
pub trait Cast: IsA<Object> {
    /// Upcasts an object to a superclass or interface `T`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let button = gtk::Button::new();
    /// let widget = button.upcast::<gtk::Widget>();
    /// ```
    #[inline]
    fn upcast<T>(self) -> T
    where T: StaticType + UnsafeFrom<ObjectRef> + Wrapper,
          Self: IsA<T> {
        unsafe { T::from(self.into()) }
    }

    /// Tries to downcast to a subclass or interface implementor `T`.
    ///
    /// Returns `Ok(T)` if the object is an instance of `T` and `Err(self)`
    /// otherwise.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let button = gtk::Button::new();
    /// let widget = button.upcast::<gtk::Widget>();
    /// assert!(widget.downcast::<gtk::Button>().is_ok());
    /// ```
    #[inline]
    fn downcast<T>(self) -> Result<T, Self>
    where Self: Sized + Downcast<T> {
        Downcast::downcast(self)
    }

    /// Returns `true` if the object is an instance of (can be downcast to) `T`.
    fn is<T>(&self) -> bool where Self: Downcast<T> {
        Downcast::can_downcast(self)
    }
}

impl<T: IsA<Object>> Cast for T { }

/// Declares the "is a" relationship.
///
/// `Self` is said to implement `T`.
///
/// For instance, since originally `GtkWidget` is a subclass of `GObject` and
/// implements the `GtkBuildable` interface, `gtk::Widget` implements
/// `IsA<glib::Object>` and `IsA<gtk::Buildable>`.
///
///
/// The trait can only be implemented if the appropriate `ToGlibPtr`
/// implementations exist.
///
/// `T` always implements `IsA<T>`.
pub trait IsA<T: StaticType + UnsafeFrom<ObjectRef> + Wrapper>: StaticType + Wrapper +
    Into<ObjectRef> + UnsafeFrom<ObjectRef> +
    for<'a> ToGlibPtr<'a, *mut <T as Wrapper>::GlibType> { }

impl<T> IsA<T> for T
where T: StaticType + Wrapper + Into<ObjectRef> + UnsafeFrom<ObjectRef> +
    for<'a> ToGlibPtr<'a, *mut <T as Wrapper>::GlibType> { }

/// Downcasts support.
pub trait Downcast<T> {
    /// Checks if it's possible to downcast to `T`.
    ///
    /// Returns `true` if the instance implements `T` and `false` otherwise.
    fn can_downcast(&self) -> bool;
    /// Tries to downcast to `T`.
    ///
    /// Returns `Ok(T)` if the instance implements `T` and `Err(Self)` otherwise.
    fn downcast(self) -> Result<T, Self> where Self: Sized;
    /// Downcasts to `T` unconditionally.
    ///
    /// Panics if compiled with `debug_assertions` and the instance doesn't implement `T`.
    unsafe fn downcast_unchecked(self) -> T;
}

impl<Super: IsA<Super>, Sub: IsA<Super>> Downcast<Sub> for Super {
    #[inline]
    fn can_downcast(&self) -> bool {
        types::instance_of::<Sub>(self.to_glib_none().0 as *const _)
    }

    #[inline]
    fn downcast(self) -> Result<Sub, Super> {
        unsafe {
            if !types::instance_of::<Sub>(self.to_glib_none().0 as *const _) {
                return Err(self);
            }
            Ok(Sub::from(self.into()))
        }
    }

    #[inline]
    unsafe fn downcast_unchecked(self) -> Sub {
        debug_assert!(types::instance_of::<Sub>(self.to_glib_none().0 as *const _));
        Sub::from(self.into())
    }
}

#[doc(hidden)]
pub use gobject_ffi::GObject;

glib_wrapper! {
    #[doc(hidden)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ObjectRef(Shared<GObject>);

    match fn {
        ref => |ptr| gobject_ffi::g_object_ref_sink(ptr),
        unref => |ptr| gobject_ffi::g_object_unref(ptr),
    }
}

/// Wrapper implementations for Object types. See `glib_wrapper!`.
#[macro_export]
macro_rules! glib_object_wrapper {
    ([$($attr:meta)*] $name:ident, $ffi_name:path, @get_type $get_type_expr:expr) => {
        $(#[$attr])*
        #[derive(Clone, Debug, Hash)]
        pub struct $name($crate::object::ObjectRef, ::std::marker::PhantomData<$ffi_name>);

        #[doc(hidden)]
        impl Into<$crate::object::ObjectRef> for $name {
            fn into(self) -> $crate::object::ObjectRef {
                self.0
            }
        }

        #[doc(hidden)]
        impl $crate::wrapper::UnsafeFrom<$crate::object::ObjectRef> for $name {
            unsafe fn from(t: $crate::object::ObjectRef) -> Self {
                $name(t, ::std::marker::PhantomData)
            }
        }

        #[doc(hidden)]
        impl $crate::translate::GlibPtrDefault for $name {
            type GlibType = *mut $ffi_name;
        }

        #[doc(hidden)]
        impl $crate::wrapper::Wrapper for $name {
            type GlibType = $ffi_name;
        }

        #[doc(hidden)]
        impl<'a> $crate::translate::ToGlibPtr<'a, *const $ffi_name> for $name {
            type Storage = <$crate::object::ObjectRef as
                $crate::translate::ToGlibPtr<'a, *mut $crate::object::GObject>>::Storage;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::translate::Stash<'a, *const $ffi_name, Self> {
                let stash = self.0.to_glib_none();
                $crate::translate::Stash(stash.0 as *const _, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *const $ffi_name {
                self.0.to_glib_full() as *const _
            }
        }

        #[doc(hidden)]
        impl<'a> $crate::translate::ToGlibPtr<'a, *mut $ffi_name> for $name {
            type Storage = <$crate::object::ObjectRef as
                $crate::translate::ToGlibPtr<'a, *mut $crate::object::GObject>>::Storage;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::translate::Stash<'a, *mut $ffi_name, Self> {
                let stash = self.0.to_glib_none();
                $crate::translate::Stash(stash.0 as *mut _, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *mut $ffi_name {
                self.0.to_glib_full() as *mut _
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibPtrNone<*mut $ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_none(ptr: *mut $ffi_name) -> Self {
                debug_assert!($crate::types::instance_of::<Self>(ptr as *const _));
                $name($crate::translate::from_glib_none(ptr as *mut _), ::std::marker::PhantomData)
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibPtrFull<*mut $ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_full(ptr: *mut $ffi_name) -> Self {
                debug_assert!($crate::types::instance_of::<Self>(ptr as *const _));
                $name($crate::translate::from_glib_full(ptr as *mut _), ::std::marker::PhantomData)
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibPtrBorrow<*mut $ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_borrow(ptr: *mut $ffi_name) -> Self {
                debug_assert!($crate::types::instance_of::<Self>(ptr as *const _));
                $name($crate::translate::from_glib_borrow(ptr as *mut _),
                      ::std::marker::PhantomData)
            }
        }

        impl $crate::types::StaticType for $name {
            fn static_type() -> $crate::types::Type {
                unsafe { $crate::translate::from_glib($get_type_expr) }
            }
        }

        impl<T: $crate::object::IsA<$crate::object::Object>> ::std::cmp::PartialEq<T> for $name {
            #[inline]
            fn eq(&self, other: &T) -> bool {
                use $crate::translate::ToGlibPtr;
                self.0.to_glib_none().0 == other.to_glib_none().0
            }
        }

        #[doc(hidden)]
        impl<'a> $crate::value::FromValueOptional<'a> for $name {
            unsafe fn from_value_optional(value: &$crate::Value) -> Option<Self> {
                Option::<$name>::from_glib_full(gobject_ffi::g_value_dup_object(value.to_glib_none().0) as *mut $ffi_name)
                    .map(|o| $crate::object::Downcast::downcast_unchecked(o))
            }
        }

        #[doc(hidden)]
        impl $crate::value::SetValue for $name {
            unsafe fn set_value(value: &mut $crate::Value, this: &Self) {
                gobject_ffi::g_value_set_object(value.to_glib_none_mut().0, $crate::translate::ToGlibPtr::<*mut $ffi_name>::to_glib_none(this).0 as *mut gobject_ffi::GObject)
            }
        }

        #[doc(hidden)]
        impl $crate::value::SetValueOptional for $name {
            unsafe fn set_value_optional(value: &mut $crate::Value, this: Option<&Self>) {
                gobject_ffi::g_value_set_object(value.to_glib_none_mut().0, $crate::translate::ToGlibPtr::<*mut $ffi_name>::to_glib_none(&this).0 as *mut gobject_ffi::GObject)
            }
        }

        impl ::std::cmp::Eq for $name { }
    };

    (@munch_impls $name:ident, ) => { };

    (@munch_impls $name:ident, $super_name:path) => {
        #[doc(hidden)]
        impl<'a> $crate::translate::ToGlibPtr<'a,
                *mut <$super_name as $crate::wrapper::Wrapper>::GlibType> for $name {
            type Storage = <$crate::object::ObjectRef as
                $crate::translate::ToGlibPtr<'a, *mut $crate::object::GObject>>::Storage;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::translate::Stash<'a,
                    *mut <$super_name as $crate::wrapper::Wrapper>::GlibType, Self> {
                let stash = self.0.to_glib_none();
                debug_assert!($crate::types::instance_of::<$super_name>(stash.0 as *const _));
                $crate::translate::Stash(stash.0 as *mut _, stash.1)
            }

            #[inline]
            fn to_glib_full(&self)
                    -> *mut <$super_name as $crate::wrapper::Wrapper>::GlibType {
                let ptr = self.0.to_glib_full();
                debug_assert!($crate::types::instance_of::<$super_name>(ptr as *const _));
                ptr as *mut _
            }
        }

        impl $crate::object::IsA<$super_name> for $name { }
    };

    (@munch_impls $name:ident, $super_name:path => $super_ffi:path) => {
        #[doc(hidden)]
        impl<'a> $crate::translate::ToGlibPtr<'a, *mut $super_ffi> for $name {
            type Storage = <$crate::object::ObjectRef as
                $crate::translate::ToGlibPtr<'a, *mut $crate::object::GObject>>::Storage;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::translate::Stash<'a, *mut $super_ffi, Self> {
                let stash = self.0.to_glib_none();
                debug_assert!($crate::types::instance_of::<$super_name>(stash.0 as *const _));
                $crate::translate::Stash(stash.0 as *mut _, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *mut $super_ffi {
                let ptr = self.0.to_glib_full();
                debug_assert!($crate::types::instance_of::<$super_name>(ptr as *const _));
                ptr as *mut _
            }
        }

        impl $crate::object::IsA<$super_name> for $name { }
    };

    (@munch_impls $name:ident, $super_name:path, $($implements:tt)*) => {
        glib_object_wrapper!(@munch_impls $name, $super_name);
        glib_object_wrapper!(@munch_impls $name, $($implements)*);
    };

    (@munch_impls $name:ident, $super_name:path => $super_ffi:path, $($implements:tt)*) => {
        glib_object_wrapper!(@munch_impls $name, $super_name => $super_ffi);
        glib_object_wrapper!(@munch_impls $name, $($implements)*);
    };

    ([$($attr:meta)*] $name:ident, $ffi_name:path, @get_type $get_type_expr:expr,
     @implements $($implements:tt)*) => {
        glib_object_wrapper!([$($attr)*] $name, $ffi_name, @get_type $get_type_expr);
        glib_object_wrapper!(@munch_impls $name, $($implements)*);

        #[doc(hidden)]
        impl<'a> $crate::translate::ToGlibPtr<'a, *mut $crate::object::GObject> for $name {
            type Storage = <$crate::object::ObjectRef as
                $crate::translate::ToGlibPtr<'a, *mut $crate::object::GObject>>::Storage;

            #[inline]
            fn to_glib_none(&'a self)
                    -> $crate::translate::Stash<'a, *mut $crate::object::GObject, Self> {
                let stash = self.0.to_glib_none();
                $crate::translate::Stash(stash.0 as *mut _, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *mut $crate::object::GObject {
                (&self.0).to_glib_full() as *mut _
            }
        }

        impl $crate::object::IsA<$crate::object::Object> for $name { }
    };

    ([$($attr:meta)*] $name:ident, $ffi_name:path, @get_type $get_type_expr:expr,
     [$($implements:path),*]) => {
        glib_object_wrapper!([$($attr)*] $name, $ffi_name, @get_type $get_type_expr,
            @implements $($implements),*);
    }
}

glib_object_wrapper! {
    [doc = "The base class in the object hierarchy."]
    Object, GObject, @get_type gobject_ffi::g_object_get_type()
}

pub trait ObjectExt {
    fn get_type(&self) -> Type;

    fn set_property<'a, N: Into<&'a str>>(&self, property_name: N, value: &Value) -> Result<(), BoolError>;
    fn get_property<'a, N: Into<&'a str>>(&self, property_name: N) -> Result<Value, BoolError>;
    fn has_property<'a, N: Into<&'a str>>(&self, property_name: N, type_: Option<Type>) -> bool;

    fn connect<'a, N, F>(&self, signal_name: N, after: bool, callback: F) -> Result<u64, BoolError>
        where N: Into<&'a str>, F: Fn(&[Value]) -> Option<Value> + Send + Sync + 'static;
    fn emit<'a, N: Into<&'a str>>(&self, signal_name: N, args: &[&Value]) -> Result<Option<Value>, BoolError>;
}

fn get_property_type<T: IsA<Object>>(obj: &T, property_name: &str) -> Option<Type> {
    unsafe {
        let obj = obj.to_glib_none().0;
        let klass = (*obj).g_type_instance.g_class as *mut gobject_ffi::GObjectClass;

        let pspec = gobject_ffi::g_object_class_find_property(klass, property_name.to_glib_none().0);
        if pspec.is_null() {
            None
        } else {
            Some(from_glib((*pspec).value_type))
        }
    }
}

impl<T: IsA<Object> + SetValue> ObjectExt for T {
    fn get_type(&self) -> Type {
        unsafe {
            let obj = self.to_glib_none().0;
            let klass = (*obj).g_type_instance.g_class as *mut gobject_ffi::GTypeClass;
            from_glib((*klass).g_type)
        }
    }

    fn set_property<'a, N: Into<&'a str>>(&self, property_name: N, value: &Value) -> Result<(), BoolError> {
        let property_name = property_name.into();

        if !self.has_property(property_name, Some(value.type_())) {
            return Err(BoolError("Invalid property name"));
        }

        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, property_name.to_glib_none().0, value.to_glib_none().0)
        }

        Ok(())
    }

    fn get_property<'a, N: Into<&'a str>>(&self, property_name: N) -> Result<Value, BoolError> {
        let property_name = property_name.into();

        let property_type = match get_property_type(self, property_name) {
            None => return Err(BoolError("Invalid property name")),
            Some(property_type) => property_type,
        };

        unsafe {
            let mut value = Value::uninitialized();

            gobject_ffi::g_value_init(value.to_glib_none_mut().0, property_type.to_glib());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, property_name.to_glib_none().0, value.to_glib_none_mut().0);

            // This can't really happen unless something goes wrong inside GObject
            if value.type_() == ::Type::Invalid {
                Err(BoolError("Failed to get property value"))
            } else {
                Ok(value)
            }
        }
    }

    fn has_property<'a, N: Into<&'a str>>(&self, property_name: N, type_: Option<Type>) -> bool {
        let ptype = get_property_type(self, property_name.into());

        match (ptype, type_) {
            (None, _) => false,
            (Some(_), None) => true,
            (Some(ptype), Some(type_)) => ptype == type_,
        }
    }

    fn connect<'a, N, F>(&self, signal_name: N, after: bool, callback: F) -> Result<u64, BoolError>
        where N: Into<&'a str>, F: Fn(&[Value]) -> Option<Value> + Send + Sync + 'static {
        let signal_name: &str = signal_name.into();

        unsafe {
            let type_ = self.get_type();

            let mut signal_id = 0;
            let mut signal_detail = 0;

            let found: bool = from_glib(gobject_ffi::g_signal_parse_name(signal_name.to_glib_none().0,
                                                                         type_.to_glib(), &mut signal_id,
                                                                         &mut signal_detail, true.to_glib()));

            if !found {
                return Err(BoolError("Signal not found"));
            }

            let mut details = mem::zeroed();
            gobject_ffi::g_signal_query(signal_id, &mut details);
            if details.signal_id != signal_id {
                return Err(BoolError("Signal not found"));
            }

            // This is actually G_SIGNAL_TYPE_STATIC_SCOPE
            let return_type = details.return_type & (!gobject_ffi::G_TYPE_FLAG_RESERVED_ID_BIT);
            let closure = Closure::new(move |values| {
                let ret = callback(values);

                if return_type == gobject_ffi::G_TYPE_NONE {
                    // Silently drop return value, if any
                    None
                } else {
                    // Silently create empty return value
                    if let Some(ret) = ret {
                        if ret.type_().to_glib() == return_type {
                            Some(ret)
                        } else {
                            let mut value = Value::uninitialized();
                            gobject_ffi::g_value_init(value.to_glib_none_mut().0, return_type);
                            Some(value)
                        }
                    } else {
                        let mut value = Value::uninitialized();
                        gobject_ffi::g_value_init(value.to_glib_none_mut().0, return_type);
                        Some(value)
                    }
                }
            });
            let handler = gobject_ffi::g_signal_connect_closure_by_id(self.to_glib_none().0, signal_id, signal_detail,
                                                                      closure.to_glib_none().0, after.to_glib());

            if handler == 0 {
                Err(BoolError("Failed to connect to signal"))
            } else {
                Ok(handler as u64)
            }
        }
    }

    fn emit<'a, N: Into<&'a str>>(&self, signal_name: N, args: &[&Value]) -> Result<Option<Value>, BoolError> {
        let signal_name: &str = signal_name.into();
        unsafe {
            let type_ = self.get_type();

            let mut signal_id = 0;
            let mut signal_detail = 0;

            let found: bool = from_glib(gobject_ffi::g_signal_parse_name(signal_name.to_glib_none().0,
                                                                         type_.to_glib(), &mut signal_id,
                                                                         &mut signal_detail, true.to_glib()));

            if !found {
                return Err(BoolError("Signal not found"));
            }

            let mut details = mem::zeroed();
            gobject_ffi::g_signal_query(signal_id, &mut details);
            if details.signal_id != signal_id {
                return Err(BoolError("Signal not found"));
            }

            if details.n_params != args.len() as u32 {
                return Err(BoolError("Incompatible number of arguments"));
            }

            for i in 0..details.n_params {
                let arg_type = *(details.param_types.offset(i as isize)) & (!gobject_ffi::G_TYPE_FLAG_RESERVED_ID_BIT);
                if arg_type != args[i as usize].type_().to_glib() {
                    return Err(BoolError("Incompatible argument types"));
                }
            }

            let mut c_args: Vec<gobject_ffi::GValue> = Vec::new();
            let instance = Value::from(self);
            c_args.push(ptr::read(instance.to_glib_none().0));
            for arg in args {
                c_args.push(ptr::read(arg.to_glib_none().0));
            }

            let mut return_value = Value::uninitialized();
            if details.return_type != gobject_ffi::G_TYPE_NONE {
                gobject_ffi::g_value_init(return_value.to_glib_none_mut().0, details.return_type);
            }

            gobject_ffi::g_signal_emitv(mut_override(c_args.as_ptr()), signal_id, signal_detail, return_value.to_glib_none_mut().0);

            if return_value.type_() != Type::Unit && return_value.type_() != Type::Invalid {
                Ok(Some(return_value))
            } else {
                Ok(None)
            }
        }
    }
}

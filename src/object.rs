// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! `IMPL` Object wrapper implementation and `Object` binding.

use translate::*;
use types::{self, StaticType};
use wrapper::{UnsafeFrom, Wrapper};
use ffi as glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;
use std::iter;
use std::marker::PhantomData;

use Value;
use value::{ToValue, SetValue};
use Type;
use BoolError;
use Closure;
use SignalHandlerId;

/// Upcasting and downcasting support.
///
/// Provides conversions up and down the class hierarchy tree.
pub trait Cast: IsA<Object> {
    /// Upcasts an object to a superclass or interface `T`.
    ///
    /// *NOTE*: This statically checks at compile-time if casting is possible. It is not always
    /// known at compile-time, whether a specific object implements an interface or not, in which case
    /// `upcast` would fail to compile. `dynamic_cast` can be used in these circumstances, which
    /// is checking the types at runtime.
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
    /// *NOTE*: This statically checks at compile-time if casting is possible. It is not always
    /// known at compile-time, whether a specific object implements an interface or not, in which case
    /// `upcast` would fail to compile. `dynamic_cast` can be used in these circumstances, which
    /// is checking the types at runtime.
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

    /// Returns `true` if the object is an instance of (can be cast to) `T`.
    fn is<T>(&self) -> bool
    where T: StaticType {
        unsafe {
            types::instance_of::<T>(self.to_glib_none().0 as *const _)
        }
    }

    /// Tries to cast to an object of type `T`. This handles upcasting, downcasting
    /// and casting between interface and interface implementors. All checks are performed at
    /// runtime, while `downcast` and `upcast` will do many checks at compile-time already.
    ///
    /// It is not always known at compile-time, whether a specific object implements an interface or
    /// not, and checking as to be performed at runtime.
    ///
    /// Returns `Ok(T)` if the object is an instance of `T` and `Err(self)`
    /// otherwise.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let button = gtk::Button::new();
    /// let widget = button.dynamic_cast::<gtk::Widget>();
    /// assert!(widget.is_ok);
    /// let widget = widget.unwrap();
    /// assert!(widget.dynamic_cast::<gtk::Button>().is_ok());
    /// ```
    #[inline]
    fn dynamic_cast<T>(self) -> Result<T, Self>
    where T: StaticType + UnsafeFrom<ObjectRef> + Wrapper {
        if !self.is::<T>() {
            Err(self)
        } else {
            Ok(unsafe { T::from(self.into()) })
        }
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
pub unsafe trait IsA<T: StaticType + UnsafeFrom<ObjectRef> + Wrapper>: StaticType + Wrapper +
    Into<ObjectRef> + UnsafeFrom<ObjectRef> +
    for<'a> ToGlibPtr<'a, *mut <T as Wrapper>::GlibType> { }

unsafe impl<T> IsA<T> for T
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
        unsafe {
            types::instance_of::<Sub>(self.to_glib_none().0 as *const _)
        }
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

#[doc(hidden)]
pub use gobject_ffi::GObjectClass;

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
    ([$($attr:meta)*] $name:ident, $ffi_name:path, $ffi_class_name:path, @get_type $get_type_expr:expr) => {
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
            type GlibClassType = $ffi_class_name;
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
        impl<'a> $crate::translate::ToGlibContainerFromSlice<'a, *mut *mut $ffi_name> for $name {
            type Storage = (Vec<Stash<'a, *mut $ffi_name, $name>>, Option<Vec<*mut $ffi_name>>);

            fn to_glib_none_from_slice(t: &'a [$name]) -> (*mut *mut $ffi_name, Self::Storage) {
                let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();
                let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
                v_ptr.push(ptr::null_mut() as *mut $ffi_name);

                (v_ptr.as_ptr() as *mut *mut $ffi_name, (v, Some(v_ptr)))
            }

            fn to_glib_container_from_slice(t: &'a [$name]) -> (*mut *mut $ffi_name, Self::Storage) {
                let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();

                let v_ptr = unsafe {
                    let v_ptr = glib_ffi::g_malloc0(mem::size_of::<*mut $ffi_name>() * (t.len() + 1)) as *mut *mut $ffi_name;

                    for (i, s) in v.iter().enumerate() {
                        ptr::write(v_ptr.offset(i as isize), s.0);
                    }

                    v_ptr
                };

                (v_ptr, (v, None))
            }

            fn to_glib_full_from_slice(t: &[$name]) -> *mut *mut $ffi_name {
                unsafe {
                    let v_ptr = glib_ffi::g_malloc0(mem::size_of::<*mut $ffi_name>() * (t.len() + 1)) as *mut *mut $ffi_name;

                    for (i, s) in t.iter().enumerate() {
                        ptr::write(v_ptr.offset(i as isize), s.to_glib_full());
                    }

                    v_ptr
                }
            }
        }

        #[doc(hidden)]
        impl<'a> $crate::translate::ToGlibContainerFromSlice<'a, *const *mut $ffi_name> for $name {
            type Storage = (Vec<Stash<'a, *mut $ffi_name, $name>>, Option<Vec<*mut $ffi_name>>);

            fn to_glib_none_from_slice(t: &'a [$name]) -> (*const *mut $ffi_name, Self::Storage) {
                let (ptr, stash) = $crate::translate::ToGlibContainerFromSlice::<'a, *mut *mut $ffi_name>::to_glib_none_from_slice(t);
                (ptr as *const *mut $ffi_name, stash)
            }

            fn to_glib_container_from_slice(_: &'a [$name]) -> (*const *mut $ffi_name, Self::Storage) {
                // Can't have consumer free a *const pointer
                unimplemented!()
            }

            fn to_glib_full_from_slice(_: &[$name]) -> *const *mut $ffi_name {
                // Can't have consumer free a *const pointer
                unimplemented!()
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
        impl $crate::translate::FromGlibPtrNone<*const $ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_none(ptr: *const $ffi_name) -> Self {
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

        #[doc(hidden)]
        impl $crate::translate::FromGlibContainerAsVec<*mut $ffi_name, *mut *mut $ffi_name> for $name {
            unsafe fn from_glib_none_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push($crate::translate::from_glib_none(ptr::read(ptr.offset(i as isize))));
                }
                res
            }

            unsafe fn from_glib_container_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                let res = $crate::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
                glib_ffi::g_free(ptr as *mut _);
                res
            }

            unsafe fn from_glib_full_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push($crate::translate::from_glib_full(ptr::read(ptr.offset(i as isize))));
                }
                glib_ffi::g_free(ptr as *mut _);
                res
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibPtrArrayContainerAsVec<*mut $ffi_name, *mut *mut $ffi_name> for $name {
            unsafe fn from_glib_none_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, $crate::translate::c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_container_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, $crate::translate::c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_full_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, $crate::translate::c_ptr_array_len(ptr))
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibContainerAsVec<*mut $ffi_name, *const *mut $ffi_name> for $name {
            unsafe fn from_glib_none_num_as_vec(ptr: *const *mut $ffi_name, num: usize) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *mut *mut _, num)
            }

            unsafe fn from_glib_container_num_as_vec(_: *const *mut $ffi_name, _: usize) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }

            unsafe fn from_glib_full_num_as_vec(_: *const *mut $ffi_name, _: usize) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibPtrArrayContainerAsVec<*mut $ffi_name, *const *mut $ffi_name> for $name {
            unsafe fn from_glib_none_as_vec(ptr: *const *mut $ffi_name) -> Vec<Self> {
                $crate::translate::FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr as *mut *mut _)
            }

            unsafe fn from_glib_container_as_vec(_: *const *mut $ffi_name) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }

            unsafe fn from_glib_full_as_vec(_: *const *mut $ffi_name) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
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
                unsafe {
                    debug_assert!($crate::types::instance_of::<$super_name>(stash.0 as *const _));
                }
                $crate::translate::Stash(stash.0 as *mut _, stash.1)
            }

            #[inline]
            fn to_glib_full(&self)
                    -> *mut <$super_name as $crate::wrapper::Wrapper>::GlibType {
                let ptr = self.0.to_glib_full();
                unsafe {
                    debug_assert!($crate::types::instance_of::<$super_name>(ptr as *const _));
                }
                ptr as *mut _
            }
        }

        unsafe impl $crate::object::IsA<$super_name> for $name { }
    };

    (@munch_impls $name:ident, $super_name:path => $super_ffi:path) => {
        #[doc(hidden)]
        impl<'a> $crate::translate::ToGlibPtr<'a, *mut $super_ffi> for $name {
            type Storage = <$crate::object::ObjectRef as
                $crate::translate::ToGlibPtr<'a, *mut $crate::object::GObject>>::Storage;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::translate::Stash<'a, *mut $super_ffi, Self> {
                let stash = self.0.to_glib_none();
                unsafe {
                    debug_assert!($crate::types::instance_of::<$super_name>(stash.0 as *const _));
                }
                $crate::translate::Stash(stash.0 as *mut _, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *mut $super_ffi {
                let ptr = self.0.to_glib_full();
                unsafe {
                    debug_assert!($crate::types::instance_of::<$super_name>(ptr as *const _));
                }
                ptr as *mut _
            }
        }

        unsafe impl $crate::object::IsA<$super_name> for $name { }
    };

    (@munch_impls $name:ident, $super_name:path, $($implements:tt)*) => {
        glib_object_wrapper!(@munch_impls $name, $super_name);
        glib_object_wrapper!(@munch_impls $name, $($implements)*);
    };

    (@munch_impls $name:ident, $super_name:path => $super_ffi:path, $($implements:tt)*) => {
        glib_object_wrapper!(@munch_impls $name, $super_name => $super_ffi);
        glib_object_wrapper!(@munch_impls $name, $($implements)*);
    };

    ([$($attr:meta)*] $name:ident, $ffi_name:path, $ffi_class_name:path, @get_type $get_type_expr:expr,
     @implements $($implements:tt)*) => {
        glib_object_wrapper!([$($attr)*] $name, $ffi_name, $ffi_class_name, @get_type $get_type_expr);
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

        unsafe impl $crate::object::IsA<$crate::object::Object> for $name { }
    };

    ([$($attr:meta)*] $name:ident, $ffi_name:path, $ffi_class_name:path, @get_type $get_type_expr:expr,
     [$($implements:path),*]) => {
        glib_object_wrapper!([$($attr)*] $name, $ffi_name, $ffi_class_name, @get_type $get_type_expr,
            @implements $($implements),*);
    }
}

glib_object_wrapper! {
    [doc = "The base class in the object hierarchy."]
    Object, GObject, GObjectClass, @get_type gobject_ffi::g_object_get_type()
}

pub trait ObjectExt: IsA<Object> {
    fn get_type(&self) -> Type;

    fn set_property<'a, N: Into<&'a str>>(&self, property_name: N, value: &ToValue) -> Result<(), BoolError>;
    fn get_property<'a, N: Into<&'a str>>(&self, property_name: N) -> Result<Value, BoolError>;
    fn has_property<'a, N: Into<&'a str>>(&self, property_name: N, type_: Option<Type>) -> Result<(), BoolError>;
    fn get_property_type<'a, N: Into<&'a str>>(&self, property_name: N) -> Option<Type>;

    fn block_signal(&self, handler_id: &SignalHandlerId);
    fn unblock_signal(&self, handler_id: &SignalHandlerId);
    fn stop_signal_emission(&self, signal_name: &str);

    fn connect<'a, N, F>(&self, signal_name: N, after: bool, callback: F) -> Result<SignalHandlerId, BoolError>
        where N: Into<&'a str>, F: Fn(&[Value]) -> Option<Value> + Send + Sync + 'static;
    fn emit<'a, N: Into<&'a str>>(&self, signal_name: N, args: &[&ToValue]) -> Result<Option<Value>, BoolError>;
    fn disconnect(&self, handler_id: SignalHandlerId);

    fn downgrade(&self) -> WeakRef<Self>;
}

impl<T: IsA<Object> + SetValue> ObjectExt for T {
    fn get_type(&self) -> Type {
        unsafe {
            let obj = self.to_glib_none().0;
            let klass = (*obj).g_type_instance.g_class as *mut gobject_ffi::GTypeClass;
            from_glib((*klass).g_type)
        }
    }

    fn set_property<'a, N: Into<&'a str>>(&self, property_name: N, value: &ToValue) -> Result<(), BoolError> {
        let property_name = property_name.into();

        if let Err(error) = self.has_property(property_name, Some(value.to_value_type())) {
            return Err(error);
        }

        let value = value.to_value();
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, property_name.to_glib_none().0, value.to_glib_none().0)
        }

        Ok(())
    }

    fn get_property<'a, N: Into<&'a str>>(&self, property_name: N) -> Result<Value, BoolError> {
        let property_name = property_name.into();

        let property_type = match self.get_property_type(property_name) {
            None => return Err(BoolError("Invalid property name")),
            Some(property_type) => property_type,
        };

        unsafe {
            let mut value = Value::from_type(property_type);
            gobject_ffi::g_object_get_property(self.to_glib_none().0, property_name.to_glib_none().0, value.to_glib_none_mut().0);

            // This can't really happen unless something goes wrong inside GObject
            if value.type_() == ::Type::Invalid {
                Err(BoolError("Failed to get property value"))
            } else {
                Ok(value)
            }
        }
    }

    fn block_signal(&self, handler_id: &SignalHandlerId) {
        unsafe {
            gobject_ffi::g_signal_handler_block(self.to_glib_none().0, handler_id.to_glib());
        }
    }

    fn unblock_signal(&self, handler_id: &SignalHandlerId) {
        unsafe {
            gobject_ffi::g_signal_handler_unblock(self.to_glib_none().0, handler_id.to_glib());
        }
    }

    fn stop_signal_emission(&self, signal_name: &str) {
        unsafe {
            gobject_ffi::g_signal_stop_emission_by_name(self.to_glib_none().0, signal_name.to_glib_none().0);
        }
    }

    fn disconnect(&self, handler_id: SignalHandlerId) {
        unsafe {
            gobject_ffi::g_signal_handler_disconnect(self.to_glib_none().0, handler_id.to_glib());
        }
    }

    fn has_property<'a, N: Into<&'a str>>(&self, property_name: N, type_: Option<Type>) -> Result<(), BoolError> {
        let property_name = property_name.into();
        let ptype = self.get_property_type(property_name);

        match (ptype, type_) {
            (None, _) => Err(BoolError("Invalid property name")),
            (Some(_), None) => Ok(()),
            (Some(ptype), Some(type_)) => {
                if ptype == type_ {
                    Ok(())
                } else {
                    Err(BoolError("Invalid property type"))
                }
            },
        }
    }

    fn get_property_type<'a, N: Into<&'a str>>(&self, property_name: N) -> Option<Type> {
        let property_name = property_name.into();
        unsafe {
            let obj = self.to_glib_none().0;
            let klass = (*obj).g_type_instance.g_class as *mut gobject_ffi::GObjectClass;

            let pspec = gobject_ffi::g_object_class_find_property(klass, property_name.to_glib_none().0);
            if pspec.is_null() {
                None
            } else {
                Some(from_glib((*pspec).value_type))
            }
        }
    }

    fn connect<'a, N, F>(&self, signal_name: N, after: bool, callback: F) -> Result<SignalHandlerId, BoolError>
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
            let return_type: Type = from_glib(details.return_type & (!gobject_ffi::G_TYPE_FLAG_RESERVED_ID_BIT));
            let closure = Closure::new(move |values| {
                let ret = callback(values);

                if return_type == Type::Unit {
                    if let Some(ret) = ret {
                        panic!("Signal required no return value but got value of type {}", ret.type_().name());
                    }
                    None
                } else {
                    match ret {
                        Some(ret) => {
                            if !ret.type_().is_a(&return_type) {
                                panic!("Signal required return value of type {} but got {}",
                                       return_type.name(), ret.type_().name());
                            }
                            Some(ret)
                        },
                        None => {
                            panic!("Signal required return value of type {} but got None", return_type.name());
                        },
                    }
                }
            });
            let handler = gobject_ffi::g_signal_connect_closure_by_id(self.to_glib_none().0, signal_id, signal_detail,
                                                                      closure.to_glib_none().0, after.to_glib());

            if handler == 0 {
                Err(BoolError("Failed to connect to signal"))
            } else {
                Ok(from_glib(handler))
            }
        }
    }

    fn emit<'a, N: Into<&'a str>>(&self, signal_name: N, args: &[&ToValue]) -> Result<Option<Value>, BoolError> {
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
                if arg_type != args[i as usize].to_value_type().to_glib() {
                    return Err(BoolError("Incompatible argument types"));
                }
            }

            let mut v_args: Vec<Value>;
            let mut s_args: [Value; 10] = mem::zeroed();
            let args = if args.len() < 10 {
                for (i, arg) in iter::once(&(self as &ToValue)).chain(args).enumerate() {
                    s_args[i] = arg.to_value();
                }
                &s_args[0..args.len()+1]
            } else {
                v_args = Vec::with_capacity(args.len() + 1);
                for arg in iter::once(&(self as &ToValue)).chain(args) {
                    v_args.push(arg.to_value());
                }
                v_args.as_slice()
            };

            let mut return_value = Value::uninitialized();
            if details.return_type != gobject_ffi::G_TYPE_NONE {
                gobject_ffi::g_value_init(return_value.to_glib_none_mut().0, details.return_type);
            }

            gobject_ffi::g_signal_emitv(mut_override(args.as_ptr()) as *mut gobject_ffi::GValue,
                signal_id, signal_detail, return_value.to_glib_none_mut().0);

            if return_value.type_() != Type::Unit && return_value.type_() != Type::Invalid {
                Ok(Some(return_value))
            } else {
                Ok(None)
            }
        }
    }

    fn downgrade(&self) -> WeakRef<T> {
        unsafe {
            let w = WeakRef(Box::new(mem::uninitialized()), PhantomData);
            gobject_ffi::g_weak_ref_init(mut_override(&*w.0), self.to_glib_none().0);
            w
        }

    }
}

pub struct WeakRef<T: IsA<Object> + ?Sized>(Box<gobject_ffi::GWeakRef>, PhantomData<*const T>);

impl<T: IsA<Object> + StaticType + UnsafeFrom<ObjectRef> + Wrapper + ?Sized> WeakRef<T> {
    pub fn upgrade(&self) -> Option<T> {
        unsafe {
            let ptr = gobject_ffi::g_weak_ref_get(mut_override(&*self.0));
            if ptr.is_null() {
                None
            } else {
                let obj: Object = from_glib_full(ptr);
                Some(T::from(obj.into()))
            }
        }
    }
}

impl<T: IsA<Object> + ?Sized> Drop for WeakRef<T> {
    fn drop(&mut self) {
        unsafe {
            gobject_ffi::g_weak_ref_clear(mut_override(&*self.0));
        }
    }
}

impl<T: IsA<Object> + ?Sized> Clone for WeakRef<T> {
    fn clone(&self) -> Self {
        unsafe {
            let c = WeakRef(Box::new(mem::uninitialized()), PhantomData);

            let o = gobject_ffi::g_weak_ref_get(mut_override(&*self.0));
            gobject_ffi::g_weak_ref_init(mut_override(&*c.0), o);
            if !o.is_null() {
                gobject_ffi::g_object_unref(o);
            }

            c
        }
    }
}

unsafe impl<T: IsA<Object> + Send + Sync + ?Sized> Send for WeakRef<T> {}
unsafe impl<T: IsA<Object> + Send + Sync + ?Sized> Sync for WeakRef<T> {}

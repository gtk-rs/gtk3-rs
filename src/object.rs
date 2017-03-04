// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! `IMPL` Object wrapper implementation and `Object` binding.

use translate::*;
use types::{self, StaticType};
use wrapper::{UnsafeFrom, Wrapper};
use gobject_ffi;

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
        ref => |ptr| gobject_ffi::g_object_ref(ptr),
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
}

impl<T: IsA<Object>> ObjectExt for T {
}

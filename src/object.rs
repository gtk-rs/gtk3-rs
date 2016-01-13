// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Types that facilitate representing `GObject` descendants.

use translate::*;
use types::{self, StaticType};
use wrapper::{UnsafeFrom, Wrapper};
use gobject_ffi;

/// Upcasting and downcasting support.
///
/// Provides conversions up and down the class hierarchy tree.
pub trait Cast: Upcast<Object> {
    /// Upcasts an object to an ancestor class or interface `T`.
    ///
    /// Example
    ///
    /// ```ignore
    /// let button = gtk::Button::new();
    /// let widget = button.upcast::<gtk::Widget>();
    /// ```
    #[inline]
    fn upcast<T>(self) -> T
    where T: StaticType + UnsafeFrom<ObjectRef> + Wrapper,
          Self: Upcast<T> {
        unsafe { T::from(self.into()) }
    }

    /// Tries to downcast to a descendant class or interface implementor `T`.
    ///
    /// Example
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
}

impl<T: Upcast<Object>> Cast for T { }

/// Declares the "is a" relationship.
///
/// `Self` is said to implement `T`. The trait can only be implemented if the appropriate
/// `ToGlibPtr` implementations exist.
///
/// `T` always implements `Upcast<T>`.
pub trait Upcast<T: StaticType + UnsafeFrom<ObjectRef> + Wrapper>: StaticType + Wrapper +
    Into<ObjectRef> + UnsafeFrom<ObjectRef> +
    for<'a> ToGlibPtr<'a, *mut <T as Wrapper>::GlibType> { }

impl<T> Upcast<T> for T
where T: StaticType + Wrapper + Into<ObjectRef> + UnsafeFrom<ObjectRef> +
    for<'a> ToGlibPtr<'a, *mut <T as Wrapper>::GlibType> { }

/// Downcasts support.
pub trait Downcast<T> {
    /// Tries to downcast to `T`.
    ///
    /// Returns `Ok(T)` if the instance implements `T` and `Err(Self)` otherwise.
    fn downcast(self) -> Result<T, Self> where Self: Sized;
    /// Downcasts to `T` unconditionally.
    ///
    /// Panics if compiled with `debug_assertions` and the instance doesn't implement `T`.
    unsafe fn downcast_unchecked(self) -> T;
}

impl<Super: Upcast<Super>, Sub: Upcast<Super>> Downcast<Sub> for Super {
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
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub struct $name($crate::object::ObjectRef, ::std::marker::PhantomData<$ffi_name>);

        impl Into<$crate::object::ObjectRef> for $name {
            fn into(self) -> $crate::object::ObjectRef {
                self.0
            }
        }

        impl $crate::wrapper::UnsafeFrom<$crate::object::ObjectRef> for $name {
            unsafe fn from(t: $crate::object::ObjectRef) -> Self {
                $name(t, ::std::marker::PhantomData)
            }
        }

        impl $crate::translate::GlibPtrDefault for $name {
            type GlibType = *mut $ffi_name;
        }

        impl $crate::wrapper::Wrapper for $name {
            type GlibType = $ffi_name;
        }

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

        impl $crate::translate::FromGlibPtr<*mut $ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_none(ptr: *mut $ffi_name) -> Self {
                debug_assert!($crate::types::instance_of::<Self>(ptr as *const _));
                $name($crate::translate::from_glib_none(ptr as *mut _), ::std::marker::PhantomData)
            }

            #[inline]
            unsafe fn from_glib_full(ptr: *mut $ffi_name) -> Self {
                debug_assert!($crate::types::instance_of::<Self>(ptr as *const _));
                $name($crate::translate::from_glib_full(ptr as *mut _), ::std::marker::PhantomData)
            }

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
    };

    ([$($attr:meta)*] $name:ident, $ffi_name:path, @get_type $get_type_expr:expr,
     [$($implements:path),*]) => {
        glib_object_wrapper!([$($attr)*] $name, $ffi_name, @get_type $get_type_expr);

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

        impl $crate::object::Upcast<$crate::object::Object> for $name { }

        $(
            impl<'a> $crate::translate::ToGlibPtr<'a,
                    *mut <$implements as $crate::wrapper::Wrapper>::GlibType> for $name {
                type Storage = <$crate::object::ObjectRef as
                    $crate::translate::ToGlibPtr<'a, *mut $crate::object::GObject>>::Storage;

                #[inline]
                fn to_glib_none(&'a self) -> $crate::translate::Stash<'a,
                        *mut <$implements as $crate::wrapper::Wrapper>::GlibType, Self> {
                    let stash = self.0.to_glib_none();
                    debug_assert!($crate::types::instance_of::<$implements>(stash.0 as *const _));
                    $crate::translate::Stash(stash.0 as *mut _, stash.1)
                }

                #[inline]
                fn to_glib_full(&self)
                        -> *mut <$implements as $crate::wrapper::Wrapper>::GlibType {
                    let ptr = self.0.to_glib_full();
                    debug_assert!($crate::types::instance_of::<$implements>(ptr as *const _));
                    ptr as *mut _
                }
            }

            impl $crate::object::Upcast<$implements> for $name { }
        )*
    }
}

glib_object_wrapper! {
    [doc = "The base class in the object hierarchy."]
    Object, GObject, @get_type gobject_ffi::g_object_get_type()
}

pub trait ObjectExt {
}

impl<T: Upcast<Object>> ObjectExt for T {
}

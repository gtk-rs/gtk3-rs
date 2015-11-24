// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Types that facilitate representing `GObject` descendants.

use translate::*;
use types::{self, StaticType};
use wrapper::Wrapper;
use gobject_ffi;

/// Declares the "is a" relationship.
///
/// `Self` is said to implement `T`. The trait can only be implemented if the appropriate
/// `ToGlibPtr` implementations exist.
///
/// `T` always implements `Upcast<T>`.
pub trait Upcast<T: Wrapper>: for<'a> ToGlibPtr<'a, *mut <T as Wrapper>::GlibType>
where T: Wrapper { }

impl<T: Wrapper> Upcast<T> for T
where T: for<'a> ToGlibPtr<'a, *mut <T as Wrapper>::GlibType> { }

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

impl<Super, Sub> Downcast<Sub> for Super
where Super: StaticType + Wrapper,
      Sub: StaticType + Wrapper + Upcast<Super> + FromGlibPtr<*mut <Sub as Wrapper>::GlibType>,
      for<'a> Super: ToGlibPtr<'a, *mut <Super as Wrapper>::GlibType>,
      for<'a> Sub: ToGlibPtr<'a, *mut <Sub as Wrapper>::GlibType> {
    #[inline]
    fn downcast(self) -> Result<Sub, Super> {
        unsafe {
            let src = self.to_glib_none();
            if types::instance_of::<Sub>(src.0 as *const _) {
                return Ok(from_glib_none(src.0 as *mut _))
            }
        }
        Err(self)
    }

    #[inline]
    unsafe fn downcast_unchecked(self) -> Sub {
        let src = (&self).to_glib_none();
        debug_assert!(types::instance_of::<Sub>(src.0 as *const _));
        from_glib_none(src.0 as *mut _)
    }
}

#[doc(hidden)]
pub use gobject_ffi::GObject;

glib_wrapper! {
    #[doc(hidden)]
    pub struct ObjectRef(Refcounted<GObject>);

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
        pub struct $name($crate::object::ObjectRef, ::std::marker::PhantomData<$ffi_name>);

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

        impl Clone for $name {
            fn clone(&self) -> Self {
                $name(self.0.clone(), ::std::marker::PhantomData)
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

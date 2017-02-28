// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! `IMPL` Shared (reference counted) wrapper implementation.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use translate::*;

/// Wrapper implementations for shared types. See `glib_wrapper!`.
#[macro_export]
macro_rules! glib_shared_wrapper {
    ([$($attr:meta)*] $name:ident, $ffi_name:path, @ref $ref_arg:ident $ref_expr:expr,
     @unref $unref_arg:ident $unref_expr:expr) => {
        $(#[$attr])*
        #[derive(Clone)]
        pub struct $name($crate::shared::Shared<$ffi_name, MemoryManager>);

        #[doc(hidden)]
        pub struct MemoryManager;

        impl $crate::shared::SharedMemoryManager<$ffi_name> for MemoryManager {
            #[inline]
            unsafe fn ref_($ref_arg: *mut $ffi_name) {
                $ref_expr;
            }

            #[inline]
            unsafe fn unref($unref_arg: *mut $ffi_name) {
                $unref_expr
            }
        }

        #[doc(hidden)]
        impl $crate::translate::GlibPtrDefault for $name {
            type GlibType = *mut $ffi_name;
        }

        #[doc(hidden)]
        impl<'a> $crate::translate::ToGlibPtr<'a, *mut $ffi_name> for $name {
            type Storage = &'a $crate::shared::Shared<$ffi_name, MemoryManager>;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::translate::Stash<'a, *mut $ffi_name, Self> {
                let stash = self.0.to_glib_none();
                $crate::translate::Stash(stash.0, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *mut $ffi_name {
                (&self.0).to_glib_full()
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibPtrNone<*mut $ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_none(ptr: *mut $ffi_name) -> Self {
                $name($crate::translate::from_glib_none(ptr))
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibPtrFull<*mut $ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_full(ptr: *mut $ffi_name) -> Self {
                $name($crate::translate::from_glib_full(ptr))
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibPtrBorrow<*mut $ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_borrow(ptr: *mut $ffi_name) -> Self {
                $name($crate::translate::from_glib_borrow(ptr))
            }
        }
    }
}

pub trait SharedMemoryManager<T> {
    unsafe fn ref_(ptr: *mut T);
    unsafe fn unref(ptr: *mut T);
}

/// Encapsulates memory management logic for shared types.
pub struct Shared<T, MM: SharedMemoryManager<T>> {
    inner: *mut T,
    borrowed: bool,
    mm: PhantomData<*const MM>,
}

impl<T, MM: SharedMemoryManager<T>> Drop for Shared<T, MM> {
    fn drop(&mut self) {
        if !self.borrowed {
            unsafe { MM::unref(self.inner); }
        }
    }
}

impl<T, MM: SharedMemoryManager<T>> Clone for Shared<T, MM> {
    fn clone(&self) -> Self {
        unsafe { MM::ref_(self.inner); }
        Shared {
            inner: self.inner,
            borrowed: false,
            mm: PhantomData,
        }
    }
}

impl<T, MM: SharedMemoryManager<T>> fmt::Debug for Shared<T, MM> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Shared {{ inner: {:?}, borrowed: {} }}", self.inner, self.borrowed)
    }
}

impl<T, MM: SharedMemoryManager<T>> PartialEq for Shared<T, MM> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T, MM: SharedMemoryManager<T>> Eq for Shared<T, MM> {}

impl<T, MM: SharedMemoryManager<T>> Hash for Shared<T, MM> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.inner.hash(state)
    }
}

impl<'a, T: 'static, MM> ToGlibPtr<'a, *mut T> for Shared<T, MM>
where MM: SharedMemoryManager<T> + 'static {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut T, Self> {
        Stash(self.inner, self)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut T {
        unsafe { MM::ref_(self.inner); }
        self.inner
    }
}

impl<T: 'static, MM: SharedMemoryManager<T>> FromGlibPtrNone<*mut T> for Shared<T, MM> {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut T) -> Self {
        assert!(!ptr.is_null());
        MM::ref_(ptr);
        Shared {
            inner: ptr,
            borrowed: false,
            mm: PhantomData,
        }
    }
}

impl<T: 'static, MM: SharedMemoryManager<T>> FromGlibPtrFull<*mut T> for Shared<T, MM> {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut T) -> Self {
        assert!(!ptr.is_null());
        Shared {
            inner: ptr,
            borrowed: false,
            mm: PhantomData,
        }
    }
}

impl<T: 'static, MM: SharedMemoryManager<T>> FromGlibPtrBorrow<*mut T> for Shared<T, MM> {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut T) -> Self {
        assert!(!ptr.is_null());
        Shared {
            inner: ptr,
            borrowed: true,
            mm: PhantomData,
        }
    }
}

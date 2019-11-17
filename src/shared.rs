// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! `IMPL` Shared (reference counted) wrapper implementation.

use std::cmp;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ptr;
use translate::*;

/// Wrapper implementations for shared types. See `glib_wrapper!`.
#[macro_export]
macro_rules! glib_shared_wrapper {
    ([$($attr:meta)*] $name:ident, $ffi_name:path, @ref $ref_arg:ident $ref_expr:expr,
     @unref $unref_arg:ident $unref_expr:expr,
     @get_type $get_type_expr:expr) => {
        glib_shared_wrapper!([$($attr)*] $name, $ffi_name, @ref $ref_arg $ref_expr,
            @unref $unref_arg $unref_expr);

        impl $crate::types::StaticType for $name {
            fn static_type() -> $crate::types::Type {
                #[allow(unused_unsafe)]
                unsafe { $crate::translate::from_glib($get_type_expr) }
            }
        }

        #[doc(hidden)]
        impl<'a> $crate::value::FromValueOptional<'a> for $name {
            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_value_optional(value: &$crate::Value) -> Option<Self> {
                $crate::translate::from_glib_full($crate::gobject_sys::g_value_dup_boxed($crate::translate::ToGlibPtr::to_glib_none(value).0) as *mut $ffi_name)
            }
        }

        #[doc(hidden)]
        impl $crate::value::SetValue for $name {
            #[allow(clippy::missing_safety_doc)]
            unsafe fn set_value(value: &mut $crate::Value, this: &Self) {
                $crate::gobject_sys::g_value_set_boxed($crate::translate::ToGlibPtrMut::to_glib_none_mut(value).0, $crate::translate::ToGlibPtr::<*mut $ffi_name>::to_glib_none(this).0 as $crate::glib_sys::gpointer)
            }
        }

        #[doc(hidden)]
        impl $crate::value::SetValueOptional for $name {
            #[allow(clippy::missing_safety_doc)]
            unsafe fn set_value_optional(value: &mut $crate::Value, this: Option<&Self>) {
                $crate::gobject_sys::g_value_set_boxed($crate::translate::ToGlibPtrMut::to_glib_none_mut(value).0, $crate::translate::ToGlibPtr::<*mut $ffi_name>::to_glib_none(&this).0 as $crate::glib_sys::gpointer)
            }
        }
    };

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
                let stash = $crate::translate::ToGlibPtr::to_glib_none(&self.0);
                $crate::translate::Stash(stash.0, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *mut $ffi_name {
                $crate::translate::ToGlibPtr::to_glib_full(&self.0)
            }
        }

        #[doc(hidden)]
        impl<'a> $crate::translate::ToGlibContainerFromSlice<'a, *mut *mut $ffi_name> for $name {
            type Storage = (Vec<$crate::translate::Stash<'a, *mut $ffi_name, $name>>, Option<Vec<*mut $ffi_name>>);

            fn to_glib_none_from_slice(t: &'a [$name]) -> (*mut *mut $ffi_name, Self::Storage) {
                let v: Vec<_> = t.iter().map(|s| $crate::translate::ToGlibPtr::to_glib_none(s)).collect();
                let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
                v_ptr.push(::std::ptr::null_mut() as *mut $ffi_name);

                (v_ptr.as_ptr() as *mut *mut $ffi_name, (v, Some(v_ptr)))
            }

            fn to_glib_container_from_slice(t: &'a [$name]) -> (*mut *mut $ffi_name, Self::Storage) {
                let v: Vec<_> = t.iter().map(|s| $crate::translate::ToGlibPtr::to_glib_none(s)).collect();

                let v_ptr = unsafe {
                    let v_ptr = $crate::glib_sys::g_malloc0(::std::mem::size_of::<*mut $ffi_name>() * (t.len() + 1)) as *mut *mut $ffi_name;

                    for (i, s) in v.iter().enumerate() {
                        ::std::ptr::write(v_ptr.add(i), s.0);
                    }

                    v_ptr
                };

                (v_ptr, (v, None))
            }

            fn to_glib_full_from_slice(t: &[$name]) -> *mut *mut $ffi_name {
                unsafe {
                    let v_ptr = $crate::glib_sys::g_malloc0(::std::mem::size_of::<*mut $ffi_name>() * (t.len() + 1)) as *mut *mut $ffi_name;

                    for (i, s) in t.iter().enumerate() {
                        ::std::ptr::write(v_ptr.add(i), $crate::translate::ToGlibPtr::to_glib_full(s));
                    }

                    v_ptr
                }
            }
        }

        #[doc(hidden)]
        impl<'a> $crate::translate::ToGlibContainerFromSlice<'a, *const *mut $ffi_name> for $name {
            type Storage = (Vec<$crate::translate::Stash<'a, *mut $ffi_name, $name>>, Option<Vec<*mut $ffi_name>>);

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
                $name($crate::translate::from_glib_none(ptr))
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibPtrNone<*const $ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_none(ptr: *const $ffi_name) -> Self {
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

        #[doc(hidden)]
        impl $crate::translate::FromGlibPtrBorrow<*const $ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_borrow(ptr: *const $ffi_name) -> Self {
                $crate::translate::from_glib_borrow(ptr as *mut $ffi_name)
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
                    res.push($crate::translate::from_glib_none(::std::ptr::read(ptr.add(i))));
                }
                res
            }

            unsafe fn from_glib_container_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                let res = $crate::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
                $crate::glib_sys::g_free(ptr as *mut _);
                res
            }

            unsafe fn from_glib_full_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push($crate::translate::from_glib_full(::std::ptr::read(ptr.add(i))));
                }
                $crate::glib_sys::g_free(ptr as *mut _);
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
    }
}

pub trait SharedMemoryManager<T> {
    unsafe fn ref_(ptr: *mut T);
    unsafe fn unref(ptr: *mut T);
}

/// Encapsulates memory management logic for shared types.
pub struct Shared<T, MM: SharedMemoryManager<T>> {
    inner: ptr::NonNull<T>,
    borrowed: bool,
    mm: PhantomData<*const MM>,
}

impl<T, MM: SharedMemoryManager<T>> Drop for Shared<T, MM> {
    fn drop(&mut self) {
        if !self.borrowed {
            unsafe {
                MM::unref(self.inner.as_ptr());
            }
        }
    }
}

impl<T, MM: SharedMemoryManager<T>> Clone for Shared<T, MM> {
    fn clone(&self) -> Self {
        unsafe {
            MM::ref_(self.inner.as_ptr());
        }
        Shared {
            inner: self.inner,
            borrowed: false,
            mm: PhantomData,
        }
    }
}

impl<T, MM: SharedMemoryManager<T>> fmt::Debug for Shared<T, MM> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Shared")
            .field("inner", &self.inner)
            .field("borrowed", &self.borrowed)
            .finish()
    }
}

impl<T, MM: SharedMemoryManager<T>> PartialOrd for Shared<T, MM> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T, MM: SharedMemoryManager<T>> Ord for Shared<T, MM> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T, MM: SharedMemoryManager<T>> PartialEq for Shared<T, MM> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T, MM: SharedMemoryManager<T>> Eq for Shared<T, MM> {}

impl<T, MM: SharedMemoryManager<T>> Hash for Shared<T, MM> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.inner.hash(state)
    }
}

impl<'a, T: 'static, MM> ToGlibPtr<'a, *mut T> for Shared<T, MM>
where
    MM: SharedMemoryManager<T> + 'static,
{
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut T, Self> {
        Stash(self.inner.as_ptr(), self)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut T {
        unsafe {
            MM::ref_(self.inner.as_ptr());
        }
        self.inner.as_ptr()
    }
}

impl<T: 'static, MM: SharedMemoryManager<T>> FromGlibPtrNone<*mut T> for Shared<T, MM> {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut T) -> Self {
        assert!(!ptr.is_null());
        MM::ref_(ptr);
        Shared {
            inner: ptr::NonNull::new_unchecked(ptr),
            borrowed: false,
            mm: PhantomData,
        }
    }
}

impl<T: 'static, MM: SharedMemoryManager<T>> FromGlibPtrNone<*const T> for Shared<T, MM> {
    #[inline]
    unsafe fn from_glib_none(ptr: *const T) -> Self {
        assert!(!ptr.is_null());
        MM::ref_(ptr as *mut _);
        Shared {
            inner: ptr::NonNull::new_unchecked(ptr as *mut _),
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
            inner: ptr::NonNull::new_unchecked(ptr),
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
            inner: ptr::NonNull::new_unchecked(ptr),
            borrowed: true,
            mm: PhantomData,
        }
    }
}

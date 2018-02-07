// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! `IMPL` Boxed wrapper implementation.

use std::ops::{Deref, DerefMut};
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use translate::*;

/// Wrapper implementations for Boxed types. See `glib_wrapper!`.
#[macro_export]
macro_rules! glib_boxed_wrapper {
    ([$($attr:meta)*] $name:ident, $ffi_name:path, @copy $copy_arg:ident $copy_expr:expr,
     @free $free_arg:ident $free_expr:expr,
     @get_type $get_type_expr:expr) => {
        glib_boxed_wrapper!([$($attr)*] $name, $ffi_name, @copy $copy_arg $copy_expr,
            @free $free_arg $free_expr);

        impl $crate::types::StaticType for $name {
            fn static_type() -> $crate::types::Type {
                unsafe { $crate::translate::from_glib($get_type_expr) }
            }
        }

        #[doc(hidden)]
        impl<'a> $crate::value::FromValueOptional<'a> for $name {
            unsafe fn from_value_optional(value: &$crate::Value) -> Option<Self> {
                Option::<$name>::from_glib_full(gobject_ffi::g_value_dup_boxed(value.to_glib_none().0) as *mut $ffi_name)
            }
        }

        #[doc(hidden)]
        impl $crate::value::SetValue for $name {
            unsafe fn set_value(value: &mut $crate::Value, this: &Self) {
                gobject_ffi::g_value_set_boxed(value.to_glib_none_mut().0, $crate::translate::ToGlibPtr::<*const $ffi_name>::to_glib_none(this).0 as glib_ffi::gpointer)
            }
        }

        #[doc(hidden)]
        impl $crate::value::SetValueOptional for $name {
            unsafe fn set_value_optional(value: &mut $crate::Value, this: Option<&Self>) {
                gobject_ffi::g_value_set_boxed(value.to_glib_none_mut().0, $crate::translate::ToGlibPtr::<*const $ffi_name>::to_glib_none(&this).0 as glib_ffi::gpointer)
            }
        }
    };

    ([$($attr:meta)*] $name:ident, $ffi_name:path, @copy $copy_arg:ident $copy_expr:expr,
     @free $free_arg:ident $free_expr:expr) => {
        $(#[$attr])*
        #[derive(Clone, Debug)]
        pub struct $name($crate::boxed::Boxed<$ffi_name, MemoryManager>);

        #[doc(hidden)]
        pub struct MemoryManager;

        impl $crate::boxed::BoxedMemoryManager<$ffi_name> for MemoryManager {
            #[inline]
            unsafe fn copy($copy_arg: *const $ffi_name) -> *mut $ffi_name {
                $copy_expr
            }

            #[inline]
            unsafe fn free($free_arg: *mut $ffi_name) {
                $free_expr
            }
        }

        #[doc(hidden)]
        impl $crate::translate::Uninitialized for $name {
            #[inline]
            unsafe fn uninitialized() -> Self {
                $name($crate::boxed::Boxed::uninitialized())
            }
        }

        #[doc(hidden)]
        impl $crate::translate::GlibPtrDefault for $name {
            type GlibType = *mut $ffi_name;
        }

        #[doc(hidden)]
        impl<'a> $crate::translate::ToGlibPtr<'a, *const $ffi_name> for $name {
            type Storage = &'a $crate::boxed::Boxed<$ffi_name, MemoryManager>;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::translate::Stash<'a, *const $ffi_name, Self> {
                let stash = self.0.to_glib_none();
                $crate::translate::Stash(stash.0, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *const $ffi_name {
                (&self.0).to_glib_full()
            }
        }

        #[doc(hidden)]
        impl<'a> $crate::translate::ToGlibPtrMut<'a, *mut $ffi_name> for $name {
            type Storage = &'a mut $crate::boxed::Boxed<$ffi_name, MemoryManager>;

            #[inline]
            fn to_glib_none_mut(&'a mut self) -> $crate::translate::StashMut<'a, *mut $ffi_name, Self> {
                let stash = self.0.to_glib_none_mut();
                $crate::translate::StashMut(stash.0, stash.1)
            }
        }

        #[doc(hidden)]
        impl<'a> $crate::translate::ToGlibContainerFromSlice<'a, *mut *const $ffi_name> for $name {
            type Storage = (Vec<Stash<'a, *const $ffi_name, $name>>, Option<Vec<*const $ffi_name>>);

            fn to_glib_none_from_slice(t: &'a [$name]) -> (*mut *const $ffi_name, Self::Storage) {
                let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();
                let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
                v_ptr.push(ptr::null_mut() as *const $ffi_name);

                (v_ptr.as_ptr() as *mut *const $ffi_name, (v, Some(v_ptr)))
            }

            fn to_glib_container_from_slice(t: &'a [$name]) -> (*mut *const $ffi_name, Self::Storage) {
                let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();

                let v_ptr = unsafe {
                    let v_ptr = glib_ffi::g_malloc0(mem::size_of::<*const $ffi_name>() * (t.len() + 1)) as *mut *const $ffi_name;

                    for (i, s) in v.iter().enumerate() {
                        ptr::write(v_ptr.offset(i as isize), s.0);
                    }

                    v_ptr
                };

                (v_ptr, (v, None))
            }

            fn to_glib_full_from_slice(t: &[$name]) -> *mut *const $ffi_name {
                unsafe {
                    let v_ptr = glib_ffi::g_malloc0(mem::size_of::<*const $ffi_name>() * (t.len() + 1)) as *mut *const $ffi_name;

                    for (i, s) in t.iter().enumerate() {
                        ptr::write(v_ptr.offset(i as isize), s.to_glib_full());
                    }

                    v_ptr
                }
            }
        }

        #[doc(hidden)]
        impl<'a> $crate::translate::ToGlibContainerFromSlice<'a, *const *const $ffi_name> for $name {
            type Storage = (Vec<Stash<'a, *const $ffi_name, $name>>, Option<Vec<*const $ffi_name>>);

            fn to_glib_none_from_slice(t: &'a [$name]) -> (*const *const $ffi_name, Self::Storage) {
                let (ptr, stash) = $crate::translate::ToGlibContainerFromSlice::<'a, *mut *const $ffi_name>::to_glib_none_from_slice(t);
                (ptr as *const *const $ffi_name, stash)
            }

            fn to_glib_container_from_slice(_: &'a [$name]) -> (*const *const $ffi_name, Self::Storage) {
                // Can't have consumer free a *const pointer
                unimplemented!()
            }

            fn to_glib_full_from_slice(_: &[$name]) -> *const *const $ffi_name {
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
    }
}

enum AnyBox<T> {
    Native(Box<T>),
    ForeignOwned(*mut T),
    ForeignBorrowed(*mut T),
}

impl<T> fmt::Debug for AnyBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::AnyBox::*;
        match *self {
            Native(ref b) => write!(f, "Native({:?})", &**b as *const T),
            ForeignOwned(ptr) => write!(f, "ForeignOwned({:?})", ptr),
            ForeignBorrowed(ptr) => write!(f, "ForeignBorrowed({:?})", ptr),
        }
    }
}

/// Memory management functions for a boxed type.
pub trait BoxedMemoryManager<T>: 'static {
    /// Makes a copy.
    unsafe fn copy(ptr: *const T) -> *mut T;
    /// Frees the object.
    unsafe fn free(ptr: *mut T);
}

/// Encapsulates memory management logic for boxed types.
pub struct Boxed<T: 'static, MM: BoxedMemoryManager<T>> {
    inner: AnyBox<T>,
    _dummy: PhantomData<MM>,
}

impl<T: 'static, MM: BoxedMemoryManager<T>> Boxed<T, MM> {
    #[inline]
    pub unsafe fn uninitialized() -> Self {
        Boxed {
            inner: AnyBox::Native(Box::new(mem::uninitialized())),
            _dummy: PhantomData,
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> Uninitialized for Boxed<T, MM> {
    #[inline]
    unsafe fn uninitialized() -> Self {
        Boxed {
            inner: AnyBox::Native(Box::new(mem::uninitialized())),
            _dummy: PhantomData,
        }
    }
}

impl<'a, T: 'static, MM: BoxedMemoryManager<T>> ToGlibPtr<'a, *const T> for Boxed<T, MM> {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const T, Self> {
        use self::AnyBox::*;
        let ptr = match self.inner {
            Native(ref b) => &**b as *const T,
            ForeignOwned(p) | ForeignBorrowed(p) => p as *const T,
        };
        Stash(ptr, self)
    }

    #[inline]
    fn to_glib_full(&self) -> *const T {
        use self::AnyBox::*;
        let ptr = match self.inner {
            Native(ref b) => &**b as *const T,
            ForeignOwned(p) | ForeignBorrowed(p) => p as *const T,
        };
        unsafe { MM::copy(ptr) }
    }
}

impl<'a, T: 'static, MM: BoxedMemoryManager<T>> ToGlibPtrMut<'a, *mut T> for Boxed<T, MM> {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut T, Self> {
        use self::AnyBox::*;
        let ptr = match self.inner {
            Native(ref mut b) => &mut **b as *mut T,
            ForeignOwned(p) | ForeignBorrowed(p) => p,
        };
        StashMut(ptr, self)
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> FromGlibPtrNone<*mut T> for Boxed<T, MM> {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut T) -> Self {
        assert!(!ptr.is_null());
        let ptr = MM::copy(ptr);
        from_glib_full(ptr)
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> FromGlibPtrNone<*const T> for Boxed<T, MM> {
    #[inline]
    unsafe fn from_glib_none(ptr: *const T) -> Self {
        assert!(!ptr.is_null());
        let ptr = MM::copy(ptr);
        from_glib_full(ptr)
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> FromGlibPtrFull<*mut T> for Boxed<T, MM> {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut T) -> Self {
        assert!(!ptr.is_null());
        Boxed {
            inner: AnyBox::ForeignOwned(ptr),
            _dummy: PhantomData,
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> FromGlibPtrBorrow<*mut T> for Boxed<T, MM> {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut T) -> Self {
        assert!(!ptr.is_null());
        Boxed {
            inner: AnyBox::ForeignBorrowed(ptr),
            _dummy: PhantomData,
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> Drop for Boxed<T, MM> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if let AnyBox::ForeignOwned(ptr) = self.inner {
                MM::free(ptr);
            }
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> fmt::Debug for Boxed<T, MM> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Boxed {{ inner: {:?} }}", self.inner)
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> Clone for Boxed<T, MM> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            from_glib_none(self.to_glib_none().0 as *mut T)
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> Deref for Boxed<T, MM> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            // This is safe because the pointer will remain valid while self is borrowed
            &*self.to_glib_none().0
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> DerefMut for Boxed<T, MM> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            // This is safe because the pointer will remain valid while self is borrowed
            &mut *self.to_glib_none_mut().0
        }
    }
}

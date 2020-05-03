// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! `IMPL` Boxed wrapper implementation.

use std::cmp;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use translate::*;

/// Wrapper implementations for Boxed types. See `glib_wrapper!`.
#[macro_export]
macro_rules! glib_boxed_wrapper {
    ([$($attr:meta)*] $name:ident, $ffi_name:path, @copy $copy_arg:ident $copy_expr:expr,
     @free $free_arg:ident $free_expr:expr, @init $init_arg:ident $init_expr:expr, @clear $clear_arg:ident $clear_expr:expr,
     @get_type $get_type_expr:expr) => {
        glib_boxed_wrapper!(@generic_impl [$($attr)*] $name, $ffi_name);
        glib_boxed_wrapper!(@memory_manager_impl $name, $ffi_name, @copy $copy_arg $copy_expr, @free $free_arg $free_expr,
                            @init $init_arg $init_expr, @clear $clear_arg $clear_expr);
        glib_boxed_wrapper!(@value_impl $name, $ffi_name, @get_type $get_type_expr);
    };

    ([$($attr:meta)*] $name:ident, $ffi_name:path, @copy $copy_arg:ident $copy_expr:expr,
     @free $free_arg:ident $free_expr:expr, @init $init_arg:ident $init_expr:expr, @clear $clear_arg:ident $clear_expr:expr) => {
        glib_boxed_wrapper!(@generic_impl [$($attr)*] $name, $ffi_name);
        glib_boxed_wrapper!(@memory_manager_impl $name, $ffi_name, @copy $copy_arg $copy_expr, @free $free_arg $free_expr,
                            @init $init_arg $init_expr, @clear $clear_arg $clear_expr);
    };

    ([$($attr:meta)*] $name:ident, $ffi_name:path, @copy $copy_arg:ident $copy_expr:expr,
     @free $free_arg:ident $free_expr:expr) => {
        glib_boxed_wrapper!(@generic_impl [$($attr)*] $name, $ffi_name);
        glib_boxed_wrapper!(@memory_manager_impl $name, $ffi_name, @copy $copy_arg $copy_expr, @free $free_arg $free_expr);
    };

    ([$($attr:meta)*] $name:ident, $ffi_name:path, @copy $copy_arg:ident $copy_expr:expr,
     @free $free_arg:ident $free_expr:expr, @get_type $get_type_expr:expr) => {
        glib_boxed_wrapper!(@generic_impl [$($attr)*] $name, $ffi_name);
        glib_boxed_wrapper!(@memory_manager_impl $name, $ffi_name, @copy $copy_arg $copy_expr, @free $free_arg $free_expr);
        glib_boxed_wrapper!(@value_impl $name, $ffi_name, @get_type $get_type_expr);
    };

    (@generic_impl [$($attr:meta)*] $name:ident, $ffi_name:path) => {
        $(#[$attr])*
        #[derive(Clone)]
        pub struct $name($crate::boxed::Boxed<$ffi_name, MemoryManager>);
        #[doc(hidden)]
        impl $crate::translate::GlibPtrDefault for $name {
            type GlibType = *mut $ffi_name;
        }

        #[doc(hidden)]
        impl<'a> $crate::translate::ToGlibPtr<'a, *const $ffi_name> for $name {
            type Storage = &'a $crate::boxed::Boxed<$ffi_name, MemoryManager>;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::translate::Stash<'a, *const $ffi_name, Self> {
                let stash = $crate::translate::ToGlibPtr::to_glib_none(&self.0);
                $crate::translate::Stash(stash.0, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *const $ffi_name {
                $crate::translate::ToGlibPtr::to_glib_full(&self.0)
            }
        }

        #[doc(hidden)]
        impl<'a> $crate::translate::ToGlibPtrMut<'a, *mut $ffi_name> for $name {
            type Storage = &'a mut $crate::boxed::Boxed<$ffi_name, MemoryManager>;

            #[inline]
            fn to_glib_none_mut(&'a mut self) -> $crate::translate::StashMut<'a, *mut $ffi_name, Self> {
                let stash = $crate::translate::ToGlibPtrMut::to_glib_none_mut(&mut self.0);
                $crate::translate::StashMut(stash.0, stash.1)
            }
        }

        #[doc(hidden)]
        impl<'a> $crate::translate::ToGlibContainerFromSlice<'a, *mut *const $ffi_name> for $name {
            type Storage = (Vec<$crate::translate::Stash<'a, *const $ffi_name, $name>>, Option<Vec<*const $ffi_name>>);

            fn to_glib_none_from_slice(t: &'a [$name]) -> (*mut *const $ffi_name, Self::Storage) {
                let v: Vec<_> = t.iter().map(|s| $crate::translate::ToGlibPtr::to_glib_none(s)).collect();
                let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
                v_ptr.push(::std::ptr::null_mut() as *const $ffi_name);

                (v_ptr.as_ptr() as *mut *const $ffi_name, (v, Some(v_ptr)))
            }

            fn to_glib_container_from_slice(t: &'a [$name]) -> (*mut *const $ffi_name, Self::Storage) {
                let v: Vec<_> = t.iter().map(|s| $crate::translate::ToGlibPtr::to_glib_none(s)).collect();

                let v_ptr = unsafe {
                    let v_ptr = $crate::glib_sys::g_malloc0(::std::mem::size_of::<*const $ffi_name>() * (t.len() + 1)) as *mut *const $ffi_name;

                    for (i, s) in v.iter().enumerate() {
                        ::std::ptr::write(v_ptr.add(i), s.0);
                    }

                    v_ptr
                };

                (v_ptr, (v, None))
            }

            fn to_glib_full_from_slice(t: &[$name]) -> *mut *const $ffi_name {
                unsafe {
                    let v_ptr = $crate::glib_sys::g_malloc0(::std::mem::size_of::<*const $ffi_name>() * (t.len() + 1)) as *mut *const $ffi_name;

                    for (i, s) in t.iter().enumerate() {
                        ::std::ptr::write(v_ptr.add(i), $crate::translate::ToGlibPtr::to_glib_full(s));
                    }

                    v_ptr
                }
            }
        }

        #[doc(hidden)]
        impl<'a> $crate::translate::ToGlibContainerFromSlice<'a, *const *const $ffi_name> for $name {
            type Storage = (Vec<$crate::translate::Stash<'a, *const $ffi_name, $name>>, Option<Vec<*const $ffi_name>>);

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
            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_none(ptr: *mut $ffi_name) -> Self {
                $name($crate::translate::from_glib_none(ptr))
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibPtrNone<*const $ffi_name> for $name {
            #[inline]
            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_none(ptr: *const $ffi_name) -> Self {
                $name($crate::translate::from_glib_none(ptr))
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibPtrFull<*mut $ffi_name> for $name {
            #[inline]
            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_full(ptr: *mut $ffi_name) -> Self {
                $name($crate::translate::from_glib_full(ptr))
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibPtrFull<*const $ffi_name> for $name {
            #[inline]
            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_full(ptr: *const $ffi_name) -> Self {
                $name($crate::translate::from_glib_full(ptr))
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibPtrBorrow<*mut $ffi_name> for $name {
            #[inline]
            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_borrow(ptr: *mut $ffi_name) -> $crate::translate::Borrowed<Self> {
                $crate::translate::Borrowed::new(
                    $name(
                        $crate::translate::from_glib_borrow::<_, $crate::boxed::Boxed<_, _>>(ptr).into_inner()
                    )
                )
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibPtrBorrow<*const $ffi_name> for $name {
            #[inline]
            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_borrow(ptr: *const $ffi_name) -> $crate::translate::Borrowed<Self> {
                $crate::translate::from_glib_borrow::<_, $name>(ptr as *mut $ffi_name)
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibContainerAsVec<*mut $ffi_name, *mut *mut $ffi_name> for $name {
            #[allow(clippy::missing_safety_doc)]
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

            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_container_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                let res = $crate::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
                $crate::glib_sys::g_free(ptr as *mut _);
                res
            }

            #[allow(clippy::missing_safety_doc)]
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
            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_none_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, $crate::translate::c_ptr_array_len(ptr))
            }

            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_container_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, $crate::translate::c_ptr_array_len(ptr))
            }

            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_full_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, $crate::translate::c_ptr_array_len(ptr))
            }
        }
    };

    (@value_impl $name:ident, $ffi_name:path, @get_type $get_type_expr:expr) => {
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
                $crate::gobject_sys::g_value_set_boxed($crate::translate::ToGlibPtrMut::to_glib_none_mut(value).0, $crate::translate::ToGlibPtr::<*const $ffi_name>::to_glib_none(this).0 as $crate::glib_sys::gpointer)
            }
        }

        #[doc(hidden)]
        impl $crate::value::SetValueOptional for $name {
            #[allow(clippy::missing_safety_doc)]
            unsafe fn set_value_optional(value: &mut $crate::Value, this: Option<&Self>) {
                $crate::gobject_sys::g_value_set_boxed($crate::translate::ToGlibPtrMut::to_glib_none_mut(value).0, $crate::translate::ToGlibPtr::<*const $ffi_name>::to_glib_none(&this).0 as $crate::glib_sys::gpointer)
            }
        }
    };

    (@memory_manager_impl $name:ident, $ffi_name:path, @copy $copy_arg:ident $copy_expr:expr, @free $free_arg:ident $free_expr:expr) => {
        #[doc(hidden)]
        pub enum MemoryManager {}

        impl $crate::boxed::BoxedMemoryManager<$ffi_name> for MemoryManager {
            #[inline]
            unsafe fn copy($copy_arg: *const $ffi_name) -> *mut $ffi_name {
                $copy_expr
            }

            #[inline]
            unsafe fn free($free_arg: *mut $ffi_name) {
                $free_expr
            }

            #[inline]
            unsafe fn init(_: *mut $ffi_name) {
                unimplemented!()
            }

            #[inline]
            unsafe fn clear(_: *mut $ffi_name) {
                unimplemented!()
            }
        }
    };

    (@memory_manager_impl $name:ident, $ffi_name:path, @copy $copy_arg:ident $copy_expr:expr, @free $free_arg:ident $free_expr:expr,
         @init $init_arg:ident $init_expr:expr, @clear $clear_arg:ident $clear_expr:expr) => {
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

            #[inline]
            unsafe fn init($init_arg: *mut $ffi_name) {
                $init_expr
            }

            #[inline]
            unsafe fn clear($clear_arg: *mut $ffi_name) {
                $clear_expr
            }
        }

        #[doc(hidden)]
        impl $crate::translate::Uninitialized for $name {
            #[inline]
            unsafe fn uninitialized() -> Self {
                $name($crate::boxed::Boxed::uninitialized())
            }
        }
    };
}

enum AnyBox<T> {
    Native(Box<T>),
    Foreign(ptr::NonNull<T>),
}

impl<T> fmt::Debug for AnyBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::AnyBox::*;
        match *self {
            Native(ref b) => f.debug_tuple("Native").field(&(&**b as *const T)).finish(),
            Foreign(ptr) => f.debug_tuple("Foreign").field(&ptr).finish(),
        }
    }
}

// The safety docs really belong in the glib_wrapper!() macro for Boxed<T>
#[allow(clippy::missing_safety_doc)]
/// Memory management functions for a boxed type.
pub trait BoxedMemoryManager<T>: 'static {
    /// Makes a copy.
    unsafe fn copy(ptr: *const T) -> *mut T;
    /// Frees the object.
    unsafe fn free(ptr: *mut T);
    /// Initializes an already allocated object.
    unsafe fn init(ptr: *mut T);
    /// Clears and frees all memory of the object, but not the object itself.
    unsafe fn clear(ptr: *mut T);
}

/// Encapsulates memory management logic for boxed types.
pub struct Boxed<T: 'static, MM: BoxedMemoryManager<T>> {
    inner: AnyBox<T>,
    _dummy: PhantomData<MM>,
}

impl<T: 'static, MM: BoxedMemoryManager<T>> Uninitialized for Boxed<T, MM> {
    #[inline]
    unsafe fn uninitialized() -> Self {
        Boxed {
            inner: {
                let mut inner = mem::MaybeUninit::zeroed();
                MM::init(inner.as_mut_ptr());

                AnyBox::Native(Box::new(inner.assume_init()))
            },
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
            Foreign(p) => p.as_ptr(),
        };
        Stash(ptr, self)
    }

    #[inline]
    fn to_glib_full(&self) -> *const T {
        use self::AnyBox::*;
        let ptr = match self.inner {
            Native(ref b) => &**b as *const T,
            Foreign(p) => p.as_ptr(),
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
            Foreign(p) => p.as_ptr(),
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
            inner: AnyBox::Foreign(ptr::NonNull::new_unchecked(ptr)),
            _dummy: PhantomData,
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> FromGlibPtrFull<*const T> for Boxed<T, MM> {
    #[inline]
    unsafe fn from_glib_full(ptr: *const T) -> Self {
        assert!(!ptr.is_null());
        Boxed {
            inner: AnyBox::Foreign(ptr::NonNull::new_unchecked(ptr as *mut T)),
            _dummy: PhantomData,
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> FromGlibPtrBorrow<*mut T> for Boxed<T, MM> {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut T) -> Borrowed<Self> {
        assert!(!ptr.is_null());
        Borrowed::new(Boxed {
            inner: AnyBox::Foreign(ptr::NonNull::new_unchecked(ptr)),
            _dummy: PhantomData,
        })
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> Drop for Boxed<T, MM> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            match self.inner {
                AnyBox::Foreign(ptr) => {
                    MM::free(ptr.as_ptr());
                }
                AnyBox::Native(ref mut box_) => {
                    MM::clear(&mut **box_);
                }
            }
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> fmt::Debug for Boxed<T, MM> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Boxed").field("inner", &self.inner).finish()
    }
}

impl<T, MM: BoxedMemoryManager<T>> PartialOrd for Boxed<T, MM> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.to_glib_none().0.partial_cmp(&other.to_glib_none().0)
    }
}

impl<T, MM: BoxedMemoryManager<T>> Ord for Boxed<T, MM> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.to_glib_none().0.cmp(&other.to_glib_none().0)
    }
}

impl<T, MM: BoxedMemoryManager<T>> PartialEq for Boxed<T, MM> {
    fn eq(&self, other: &Self) -> bool {
        self.to_glib_none().0 == other.to_glib_none().0
    }
}

impl<T, MM: BoxedMemoryManager<T>> Eq for Boxed<T, MM> {}

impl<T, MM: BoxedMemoryManager<T>> Hash for Boxed<T, MM> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.to_glib_none().0.hash(state)
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> Clone for Boxed<T, MM> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { from_glib_none(self.to_glib_none().0 as *mut T) }
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

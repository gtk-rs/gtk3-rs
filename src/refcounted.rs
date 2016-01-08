use std::marker::PhantomData;
use translate::*;

/// Wrapper implementations for refcounted types. See `glib_wrapper!`.
#[macro_export]
macro_rules! glib_refcounted_wrapper {
    ([$($attr:meta)*] $name:ident, $ffi_name:path, @ref $ref_arg:ident $ref_expr:expr,
     @unref $unref_arg:ident $unref_expr:expr) => {
        $(#[$attr])*
        pub struct $name($crate::refcounted::Refcounted<$ffi_name, MemoryManager>);

        #[doc(hidden)]
        pub struct MemoryManager;

        impl $crate::refcounted::RefcountedMemoryManager<$ffi_name> for MemoryManager {
            #[inline]
            unsafe fn ref_($ref_arg: *mut $ffi_name) {
                $ref_expr;
            }

            #[inline]
            unsafe fn unref($unref_arg: *mut $ffi_name) {
                $unref_expr
            }
        }

        impl<'a> $crate::translate::ToGlibPtr<'a, *mut $ffi_name> for $name {
            type Storage = &'a $crate::refcounted::Refcounted<$ffi_name, MemoryManager>;

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

        impl $crate::translate::FromGlibPtr<*mut $ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_none(ptr: *mut $ffi_name) -> Self {
                $name($crate::translate::from_glib_none(ptr))
            }

            #[inline]
            unsafe fn from_glib_full(ptr: *mut $ffi_name) -> Self {
                $name($crate::translate::from_glib_full(ptr))
            }

            #[inline]
            unsafe fn from_glib_borrow(ptr: *mut $ffi_name) -> Self {
                $name($crate::translate::from_glib_borrow(ptr))
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                $name(self.0.clone())
            }
        }
    }
}

pub trait RefcountedMemoryManager<T> {
    unsafe fn ref_(ptr: *mut T);
    unsafe fn unref(ptr: *mut T);
}

/// Encapsulates memory management logic for refcounted types.
#[derive(Debug)]
pub struct Refcounted<T, MM: RefcountedMemoryManager<T>> {
    inner: *mut T,
    borrowed: bool,
    mm: PhantomData<*const MM>,
}

impl<T, MM: RefcountedMemoryManager<T>> Drop for Refcounted<T, MM> {
    fn drop(&mut self) {
        if !self.borrowed {
            unsafe { MM::unref(self.inner); }
        }
    }
}

impl<T, MM: RefcountedMemoryManager<T>> Clone for Refcounted<T, MM> {
    fn clone(&self) -> Self {
        unsafe { MM::ref_(self.inner); }
        Refcounted {
            inner: self.inner,
            borrowed: false,
            mm: PhantomData,
        }
    }
}

impl<'a, T: 'static, MM> ToGlibPtr<'a, *mut T> for Refcounted<T, MM>
where MM: RefcountedMemoryManager<T> + 'static {
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

impl<T: 'static, MM: RefcountedMemoryManager<T>> FromGlibPtr<*mut T> for Refcounted<T, MM> {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut T) -> Self {
        assert!(!ptr.is_null());
        MM::ref_(ptr);
        Refcounted {
            inner: ptr,
            borrowed: false,
            mm: PhantomData,
        }
    }

    #[inline]
    unsafe fn from_glib_full(ptr: *mut T) -> Self {
        assert!(!ptr.is_null());
        Refcounted {
            inner: ptr,
            borrowed: false,
            mm: PhantomData,
        }
    }

    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut T) -> Self {
        assert!(!ptr.is_null());
        Refcounted {
            inner: ptr,
            borrowed: true,
            mm: PhantomData,
        }
    }
}

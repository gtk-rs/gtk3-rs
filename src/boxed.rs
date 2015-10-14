use std::marker::PhantomData;
use std::mem;
use translate::*;

enum AnyBox<T> {
    Native(Box<T>),
    Foreign(*mut T),
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

impl<'a, T: 'static, MM: BoxedMemoryManager<T>> ToGlibPtr<'a, *const T> for &'a Boxed<T, MM> {
    type Storage = Self;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *const T, Self> {
        let ptr = match self.inner {
            AnyBox::Native(ref b) => &**b as *const T,
            AnyBox::Foreign(p) => p as *const T,
        };
        Stash(ptr, *self)
    }
}

impl<'a, T: 'static, MM: BoxedMemoryManager<T>> ToGlibPtrMut<'a, *mut T> for Boxed<T, MM> {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut T, Self> {
        let ptr = match self.inner {
            AnyBox::Native(ref mut b) => &mut **b as *mut T,
            AnyBox::Foreign(p) => p,
        };
        StashMut(ptr, self)
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> FromGlibPtr<*mut T> for Boxed<T, MM> {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut T) -> Self {
        assert!(!ptr.is_null());
        let ptr = MM::copy(ptr);
        from_glib_full(ptr)
    }

    #[inline]
    unsafe fn from_glib_full(ptr: *mut T) -> Self {
        assert!(!ptr.is_null());
        Boxed {
            inner: AnyBox::Foreign(ptr),
            _dummy: PhantomData,
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> Drop for Boxed<T, MM> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if let AnyBox::Foreign(ptr) = self.inner {
                MM::free(ptr);
            }
        }
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

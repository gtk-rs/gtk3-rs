// Copyright 2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Types that facilitate representing `GObject` descendants.

use std::marker::PhantomData;
use std::ptr;
use translate::*;
use types::{self, Type, StaticType};
use ffi;

/// A reference to any GObject descendant.
#[allow(raw_pointer_derive)]
#[derive(Debug, PartialEq, Eq)]
pub struct Ref(*mut ffi::GObject);

impl Ref {
    #[inline]
    fn add_ref(&self) { unsafe { ffi::g_object_ref_sink(self.0 as *mut _); } }

    #[inline]
    fn unref(&self) { unsafe { ffi::g_object_unref(self.0 as *mut _); } }

    /// Transfer: none constructor.
    #[inline]
    pub fn from_glib_none(ptr: *mut ffi::GObject) -> Ref {
        let r = Ref(ptr);
        r.add_ref();
        r
    }

    /// Transfer: full constructor.
    #[inline]
    pub fn from_glib_full(ptr: *mut ffi::GObject) -> Ref {
        Ref(ptr)
    }

    /// Returns a transfer: none raw pointer.
    #[inline]
    pub fn to_glib_none(&self) -> *mut ffi::GObject {
        self.0
    }

    /// Returns a transfer: full raw pointer.
    #[inline]
    pub fn to_glib_full(&self) -> *mut ffi::GObject {
        self.add_ref();
        self.0
    }
}

impl Clone for Ref {
    #[inline]
    fn clone(&self) -> Ref {
        self.add_ref();
        Ref(self.0)
    }
}

impl Drop for Ref {
    #[inline]
    fn drop(&mut self) {
        self.unref();
    }
}

/// A helper type holding a reference to a specific object or interface.
///
/// `T` is the foreign `struct` type corresponding to the object.
pub struct VirtualRef<'a, T> (&'a Ref, PhantomData<T>);

impl<'a, T> VirtualRef<'a, T> {
    #[inline]
    fn new(r: &'a Ref) -> VirtualRef<'a, T> { VirtualRef(r, PhantomData) }
}

impl<'a, T> ToGlibPtr<'a, *mut T> for VirtualRef<'a, T> {
    type Storage = &'a Ref;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut T, VirtualRef<'a, T>> {
        Stash(self.0.to_glib_none() as *mut _, self.0)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut T {
        self.0.to_glib_full() as *mut _
    }
}

impl<'a, T> ToGlibPtr<'a, *mut T> for Option<&'a VirtualRef<'a, T>> {
    type Storage = Option<&'a Ref>;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut T, Option<&'a VirtualRef<'a, T>>> {
        if let Some(ref s) = *self {
            Stash(s.0.to_glib_none() as *mut _, Some(s.0))
        }
        else {
            Stash(ptr::null_mut(), None)
        }
    }

    #[inline]
    fn to_glib_full(&self) -> *mut T {
        self.as_ref().map_or(ptr::null_mut(), |s| s.0.to_glib_full() as *mut _)
    }
}

/// A wrapper around the `Ref`.
pub trait Wrapper: StaticType {
    /// The foreign `struct` type corresponding to the object.
    type GlibType;
    /// Wraps a `Ref`.
    unsafe fn wrap(r: Ref) -> Self;
    /// Returns a reference to the inner `Ref`.
    fn as_ref(&self) -> &Ref;
    /// Transforms into the inner `Ref`.
    fn unwrap(self) -> Ref;
}

impl<'a, T, W: Wrapper<GlibType = T>> ToGlibPtr<'a, *mut T> for &'a W {
    type Storage = &'a Ref;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut T, &'a W> {
        Stash(self.as_ref().to_glib_none() as *mut _, self.as_ref())
    }

    #[inline]
    fn to_glib_full(&self) -> *mut T {
        self.as_ref().to_glib_full() as *mut _
    }
}

impl<'a, T, W: Wrapper<GlibType = T>> ToGlibPtr<'a, *mut T> for Option<&'a W> {
    type Storage = Option<&'a Ref>;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut T, Option<&'a W>> {
        if let Some(ref s) = *self {
            Stash(s.as_ref().to_glib_none() as *mut _, Some(s.as_ref()))
        }
        else {
            Stash(ptr::null_mut(), None)
        }
    }

    #[inline]
    fn to_glib_full(&self) -> *mut T {
        self.as_ref().map_or(ptr::null_mut(), |s| s.as_ref().to_glib_full() as *mut _)
    }
}

impl <T: Wrapper> FromGlibPtr<*mut <T as Wrapper>::GlibType> for T {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut <T as Wrapper>::GlibType) -> Self {
        assert!(!ptr.is_null());
        debug_assert!(types::instance_of::<T>(ptr as *const _));
        T::wrap(Ref::from_glib_none(ptr as *mut _))
    }

    #[inline]
    unsafe fn from_glib_full(ptr: *mut <T as Wrapper>::GlibType) -> Self {
        assert!(!ptr.is_null());
        debug_assert!(types::instance_of::<T>(ptr as *const _));
        T::wrap(Ref::from_glib_full(ptr as *mut _))
    }
}

/// Declares the "is a" relationship.
///
/// `Self` is said to implement `T` and can be `upcast` to a corresponding type.
///
/// `T` always implements `Upcast<T>`.
pub unsafe trait Upcast<T: Wrapper>: Wrapper {
    /// Upcasts to a helper type corresponding to `T`.
    #[inline]
    fn upcast(&self) -> VirtualRef<<T as Wrapper>::GlibType> {
        debug_assert!(types::instance_of::<T>(self.as_ref().to_glib_none() as *const _));
        VirtualRef::<<T as Wrapper>::GlibType>::new(self.as_ref())
    }
}

unsafe impl<T: Wrapper> Upcast<T> for T { }

/// A complement to `Upcast` that allows downcasting.
pub trait Downcast<T> {
    /// Tries to downcast to `T`.
    ///
    /// Returns `Ok(T)` if the instance implements `T` and `Err(Self)` otherwise.
    fn downcast(self) -> Result<T, Self>;
    /// Downcasts to `T` unconditionally.
    ///
    /// Panics if the instance doesn't implement `T`.
    fn downcast_unchecked(self) -> T;
}

impl <Super, Sub> Downcast<Sub> for Super
where Super: Wrapper, Sub: Wrapper + Upcast<Super> {
    #[inline]
    fn downcast(self) -> Result<Sub, Super> {
        if types::instance_of::<Sub>(self.as_ref().to_glib_none() as *const _) {
            unsafe { Ok(Sub::wrap(self.unwrap())) }
        }
        else {
            Err(self)
        }
    }

    #[inline]
    fn downcast_unchecked(self) -> Sub {
        assert!(types::instance_of::<Sub>(self.as_ref().to_glib_none() as *const _));
        unsafe { Sub::wrap(self.unwrap()) }
    }
}

#[derive(Clone)]
pub struct Object(Ref);

impl Wrapper for Object {
    type GlibType = ffi::GObject;
    #[inline]
    unsafe fn wrap(r: Ref) -> Object { Object(r) }
    #[inline]
    fn as_ref(&self) -> &Ref { &self.0 }
    #[inline]
    fn unwrap(self) -> Ref { self.0 }
}

impl StaticType for Object {
    fn static_type() -> Type { Type::BaseObject }
}

pub trait ObjectTrait { }

impl<T: Upcast<Object>> ObjectTrait for T { }

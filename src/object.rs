// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! `IMPL` Object wrapper implementation and `Object` binding.

use glib_sys;
use gobject_sys;
use std::cmp;
use std::fmt;
use std::hash;
use std::marker::PhantomData;
use std::mem;
use std::ops;
use std::ptr;
use translate::*;
use types::StaticType;

use value::ToValue;
use BoolError;
use Closure;
use SignalHandlerId;
use Type;
use Value;

use get_thread_id;

#[doc(hidden)]
pub use gobject_sys::GObject;

#[doc(hidden)]
pub use gobject_sys::GObjectClass;

/// Implemented by types representing `glib::Object` and subclasses of it.
pub unsafe trait ObjectType:
    UnsafeFrom<ObjectRef>
    + Into<ObjectRef>
    + StaticType
    + fmt::Debug
    + Clone
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + hash::Hash
    + for<'a> ToGlibPtr<'a, *mut <Self as ObjectType>::GlibType>
    + 'static
{
    /// type of the FFI Instance structure.
    type GlibType: 'static;
    /// type of the FFI Class structure.
    type GlibClassType: 'static;
    /// type of the Rust Class structure.
    type RustClassType: 'static;

    fn as_object_ref(&self) -> &ObjectRef;
    fn as_ptr(&self) -> *mut Self::GlibType;
}

/// Unsafe variant of the `From` trait.
pub trait UnsafeFrom<T> {
    unsafe fn unsafe_from(t: T) -> Self;
}

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
pub unsafe trait IsA<T: ObjectType>: ObjectType + AsRef<T> + 'static {}

/// Trait for mapping a class struct type to its corresponding instance type.
pub unsafe trait IsClassFor: Sized + 'static {
    /// Corresponding Rust instance type for this class.
    type Instance: ObjectType;

    /// Get the type id for this class.
    fn get_type(&self) -> Type {
        unsafe {
            let klass = self as *const _ as *const gobject_sys::GTypeClass;
            from_glib((*klass).g_type)
        }
    }

    /// Casts this class to a reference to a parent type's class.
    fn upcast_ref<U: IsClassFor>(&self) -> &U
    where
        Self::Instance: IsA<U::Instance>,
        U::Instance: ObjectType,
    {
        unsafe {
            let klass = self as *const _ as *const U;
            &*klass
        }
    }

    /// Casts this class to a mutable reference to a parent type's class.
    fn upcast_ref_mut<U: IsClassFor>(&mut self) -> &mut U
    where
        Self::Instance: IsA<U::Instance>,
        U::Instance: ObjectType,
    {
        unsafe {
            let klass = self as *mut _ as *mut U;
            &mut *klass
        }
    }

    /// Casts this class to a reference to a child type's class or
    /// fails if this class is not implementing the child class.
    fn downcast_ref<U: IsClassFor>(&self) -> Option<&U>
    where
        U::Instance: IsA<Self::Instance>,
        Self::Instance: ObjectType,
    {
        if !self.get_type().is_a(&U::Instance::static_type()) {
            return None;
        }

        unsafe {
            let klass = self as *const _ as *const U;
            Some(&*klass)
        }
    }

    /// Casts this class to a mutable reference to a child type's class or
    /// fails if this class is not implementing the child class.
    fn downcast_ref_mut<U: IsClassFor>(&mut self) -> Option<&mut U>
    where
        U::Instance: IsA<Self::Instance>,
        Self::Instance: ObjectType,
    {
        if !self.get_type().is_a(&U::Instance::static_type()) {
            return None;
        }

        unsafe {
            let klass = self as *mut _ as *mut U;
            Some(&mut *klass)
        }
    }

    /// Gets the class struct corresponding to `type_`.
    ///
    /// This will return `None` if `type_` is not a subclass of `Self`.
    fn from_type(type_: Type) -> Option<ClassRef<Self>> {
        if !type_.is_a(&Self::Instance::static_type()) {
            return None;
        }

        unsafe {
            let ptr = gobject_sys::g_type_class_ref(type_.to_glib());
            if ptr.is_null() {
                None
            } else {
                Some(ClassRef(ptr::NonNull::new_unchecked(ptr as *mut Self)))
            }
        }
    }
}

#[derive(Debug)]
pub struct ClassRef<T: IsClassFor>(ptr::NonNull<T>);

impl<T: IsClassFor> ops::Deref for ClassRef<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}

impl<T: IsClassFor> Drop for ClassRef<T> {
    fn drop(&mut self) {
        unsafe {
            gobject_sys::g_type_class_unref(self.0.as_ptr() as *mut _);
        }
    }
}

unsafe impl<T: IsClassFor> Send for ClassRef<T> {}
unsafe impl<T: IsClassFor> Sync for ClassRef<T> {}

/// Upcasting and downcasting support.
///
/// Provides conversions up and down the class hierarchy tree.
pub trait Cast: ObjectType {
    /// Upcasts an object to a superclass or interface `T`.
    ///
    /// *NOTE*: This statically checks at compile-time if casting is possible. It is not always
    /// known at compile-time, whether a specific object implements an interface or not, in which case
    /// `upcast` would fail to compile. `dynamic_cast` can be used in these circumstances, which
    /// is checking the types at runtime.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let button = gtk::Button::new();
    /// let widget = button.upcast::<gtk::Widget>();
    /// ```
    #[inline]
    fn upcast<T: ObjectType>(self) -> T
    where
        Self: IsA<T>,
    {
        unsafe { self.unsafe_cast() }
    }

    /// Upcasts an object to a reference of its superclass or interface `T`.
    ///
    /// *NOTE*: This statically checks at compile-time if casting is possible. It is not always
    /// known at compile-time, whether a specific object implements an interface or not, in which case
    /// `upcast` would fail to compile. `dynamic_cast` can be used in these circumstances, which
    /// is checking the types at runtime.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let button = gtk::Button::new();
    /// let widget = button.upcast_ref::<gtk::Widget>();
    /// ```
    #[inline]
    fn upcast_ref<T: ObjectType>(&self) -> &T
    where
        Self: IsA<T>,
    {
        unsafe { self.unsafe_cast_ref() }
    }

    /// Tries to downcast to a subclass or interface implementor `T`.
    ///
    /// Returns `Ok(T)` if the object is an instance of `T` and `Err(self)`
    /// otherwise.
    ///
    /// *NOTE*: This statically checks at compile-time if casting is possible. It is not always
    /// known at compile-time, whether a specific object implements an interface or not, in which case
    /// `upcast` would fail to compile. `dynamic_cast` can be used in these circumstances, which
    /// is checking the types at runtime.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let button = gtk::Button::new();
    /// let widget = button.upcast::<gtk::Widget>();
    /// assert!(widget.downcast::<gtk::Button>().is_ok());
    /// ```
    #[inline]
    fn downcast<T: ObjectType>(self) -> Result<T, Self>
    where
        Self: CanDowncast<T>,
    {
        if self.is::<T>() {
            Ok(unsafe { self.unsafe_cast() })
        } else {
            Err(self)
        }
    }

    /// Tries to downcast to a reference of its subclass or interface implementor `T`.
    ///
    /// Returns `Some(T)` if the object is an instance of `T` and `None`
    /// otherwise.
    ///
    /// *NOTE*: This statically checks at compile-time if casting is possible. It is not always
    /// known at compile-time, whether a specific object implements an interface or not, in which case
    /// `upcast` would fail to compile. `dynamic_cast` can be used in these circumstances, which
    /// is checking the types at runtime.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let button = gtk::Button::new();
    /// let widget = button.upcast::<gtk::Widget>();
    /// assert!(widget.downcast_ref::<gtk::Button>().is_some());
    /// ```
    #[inline]
    fn downcast_ref<T: ObjectType>(&self) -> Option<&T>
    where
        Self: CanDowncast<T>,
    {
        if self.is::<T>() {
            Some(unsafe { self.unsafe_cast_ref() })
        } else {
            None
        }
    }

    /// Tries to cast to an object of type `T`. This handles upcasting, downcasting
    /// and casting between interface and interface implementors. All checks are performed at
    /// runtime, while `downcast` and `upcast` will do many checks at compile-time already.
    ///
    /// It is not always known at compile-time, whether a specific object implements an interface or
    /// not, and checking as to be performed at runtime.
    ///
    /// Returns `Ok(T)` if the object is an instance of `T` and `Err(self)`
    /// otherwise.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let button = gtk::Button::new();
    /// let widget = button.dynamic_cast::<gtk::Widget>();
    /// assert!(widget.is_ok());
    /// let widget = widget.unwrap();
    /// assert!(widget.dynamic_cast::<gtk::Button>().is_ok());
    /// ```
    #[inline]
    fn dynamic_cast<T: ObjectType>(self) -> Result<T, Self> {
        if !self.is::<T>() {
            Err(self)
        } else {
            Ok(unsafe { self.unsafe_cast() })
        }
    }

    /// Tries to cast to reference to an object of type `T`. This handles upcasting, downcasting
    /// and casting between interface and interface implementors. All checks are performed at
    /// runtime, while `downcast` and `upcast` will do many checks at compile-time already.
    ///
    /// It is not always known at compile-time, whether a specific object implements an interface or
    /// not, and checking as to be performed at runtime.
    ///
    /// Returns `Some(T)` if the object is an instance of `T` and `None`
    /// otherwise.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let button = gtk::Button::new();
    /// let widget = button.dynamic_cast_ref::<gtk::Widget>();
    /// assert!(widget.is_some());
    /// let widget = widget.unwrap();
    /// assert!(widget.dynamic_cast_ref::<gtk::Button>().is_some());
    /// ```
    #[inline]
    fn dynamic_cast_ref<T: ObjectType>(&self) -> Option<&T> {
        if !self.is::<T>() {
            None
        } else {
            // This transmute is safe because all our wrapper types have the
            // same representation except for the name and the phantom data
            // type. IsA<> is an unsafe trait that must only be implemented
            // if this is a valid wrapper type
            Some(unsafe { self.unsafe_cast_ref() })
        }
    }

    /// Casts to `T` unconditionally.
    ///
    /// Panics if compiled with `debug_assertions` and the instance doesn't implement `T`.
    unsafe fn unsafe_cast<T: ObjectType>(self) -> T {
        debug_assert!(self.is::<T>());
        T::unsafe_from(self.into())
    }

    /// Casts to `&T` unconditionally.
    ///
    /// Panics if compiled with `debug_assertions` and the instance doesn't implement `T`.
    unsafe fn unsafe_cast_ref<T: ObjectType>(&self) -> &T {
        debug_assert!(self.is::<T>());
        // This transmute is safe because all our wrapper types have the
        // same representation except for the name and the phantom data
        // type. IsA<> is an unsafe trait that must only be implemented
        // if this is a valid wrapper type
        &*(self as *const Self as *const T)
    }
}

impl<T: ObjectType> Cast for T {}

/// Marker trait for the statically known possibility of downcasting from `Self` to `T`.
pub trait CanDowncast<T> {}

impl<Super: IsA<Super>, Sub: IsA<Super>> CanDowncast<Sub> for Super {}

// Manual implementation of glib_shared_wrapper! because of special cases
pub struct ObjectRef {
    inner: ptr::NonNull<GObject>,
    borrowed: bool,
}

impl Clone for ObjectRef {
    fn clone(&self) -> Self {
        unsafe {
            ObjectRef {
                inner: ptr::NonNull::new_unchecked(gobject_sys::g_object_ref(self.inner.as_ptr())),
                borrowed: false,
            }
        }
    }
}

impl Drop for ObjectRef {
    fn drop(&mut self) {
        unsafe {
            if !self.borrowed {
                gobject_sys::g_object_unref(self.inner.as_ptr());
            }
        }
    }
}

impl fmt::Debug for ObjectRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_ = unsafe {
            let klass = (*self.inner.as_ptr()).g_type_instance.g_class as *const ObjectClass;
            (&*klass).get_type()
        };

        f.debug_struct("ObjectRef")
            .field("inner", &self.inner)
            .field("type", &type_)
            .field("borrowed", &self.borrowed)
            .finish()
    }
}

impl PartialOrd for ObjectRef {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl Ord for ObjectRef {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl PartialEq for ObjectRef {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Eq for ObjectRef {}

impl hash::Hash for ObjectRef {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.inner.hash(state)
    }
}

#[doc(hidden)]
impl GlibPtrDefault for ObjectRef {
    type GlibType = *mut GObject;
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut GObject> for ObjectRef {
    type Storage = &'a ObjectRef;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut GObject, Self> {
        Stash(self.inner.as_ptr(), self)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut GObject {
        unsafe { gobject_sys::g_object_ref(self.inner.as_ptr()) }
    }
}

#[doc(hidden)]
impl<'a> ToGlibContainerFromSlice<'a, *mut *mut GObject> for ObjectRef {
    type Storage = (
        Vec<Stash<'a, *mut GObject, ObjectRef>>,
        Option<Vec<*mut GObject>>,
    );

    fn to_glib_none_from_slice(t: &'a [ObjectRef]) -> (*mut *mut GObject, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();
        let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
        v_ptr.push(ptr::null_mut() as *mut GObject);

        (v_ptr.as_ptr() as *mut *mut GObject, (v, Some(v_ptr)))
    }

    fn to_glib_container_from_slice(t: &'a [ObjectRef]) -> (*mut *mut GObject, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();

        let v_ptr = unsafe {
            let v_ptr = glib_sys::g_malloc0(mem::size_of::<*mut GObject>() * (t.len() + 1))
                as *mut *mut GObject;

            for (i, s) in v.iter().enumerate() {
                ptr::write(v_ptr.add(i), s.0);
            }

            v_ptr
        };

        (v_ptr, (v, None))
    }

    fn to_glib_full_from_slice(t: &[ObjectRef]) -> *mut *mut GObject {
        unsafe {
            let v_ptr = glib_sys::g_malloc0(std::mem::size_of::<*mut GObject>() * (t.len() + 1))
                as *mut *mut GObject;

            for (i, s) in t.iter().enumerate() {
                ptr::write(v_ptr.add(i), s.to_glib_full());
            }

            v_ptr
        }
    }
}

#[doc(hidden)]
impl<'a> ToGlibContainerFromSlice<'a, *const *mut GObject> for ObjectRef {
    type Storage = (
        Vec<Stash<'a, *mut GObject, ObjectRef>>,
        Option<Vec<*mut GObject>>,
    );

    fn to_glib_none_from_slice(t: &'a [ObjectRef]) -> (*const *mut GObject, Self::Storage) {
        let (ptr, stash) =
            ToGlibContainerFromSlice::<'a, *mut *mut GObject>::to_glib_none_from_slice(t);
        (ptr as *const *mut GObject, stash)
    }

    fn to_glib_container_from_slice(_: &'a [ObjectRef]) -> (*const *mut GObject, Self::Storage) {
        // Can't have consumer free a *const pointer
        unimplemented!()
    }

    fn to_glib_full_from_slice(_: &[ObjectRef]) -> *const *mut GObject {
        // Can't have consumer free a *const pointer
        unimplemented!()
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut GObject> for ObjectRef {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut GObject) -> Self {
        assert!(!ptr.is_null());

        // Attention: This takes ownership of floating references!
        ObjectRef {
            inner: ptr::NonNull::new_unchecked(gobject_sys::g_object_ref_sink(ptr)),
            borrowed: false,
        }
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const GObject> for ObjectRef {
    #[inline]
    unsafe fn from_glib_none(ptr: *const GObject) -> Self {
        // Attention: This takes ownership of floating references!
        from_glib_none(ptr as *mut GObject)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut GObject> for ObjectRef {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut GObject) -> Self {
        assert!(!ptr.is_null());

        ObjectRef {
            inner: ptr::NonNull::new_unchecked(ptr),
            borrowed: false,
        }
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut GObject> for ObjectRef {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut GObject) -> Self {
        assert!(!ptr.is_null());

        ObjectRef {
            inner: ptr::NonNull::new_unchecked(ptr),
            borrowed: true,
        }
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const GObject> for ObjectRef {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *const GObject) -> Self {
        from_glib_borrow(ptr as *mut GObject)
    }
}

#[doc(hidden)]
impl FromGlibContainerAsVec<*mut GObject, *mut *mut GObject> for ObjectRef {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut *mut GObject, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        // Attention: This takes ownership of floating references!
        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib_none(ptr::read(ptr.add(i))));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut *mut GObject, num: usize) -> Vec<Self> {
        // Attention: This takes ownership of floating references!
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib_sys::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut *mut GObject, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib_full(ptr::read(ptr.add(i))));
        }
        glib_sys::g_free(ptr as *mut _);
        res
    }
}

#[doc(hidden)]
impl FromGlibPtrArrayContainerAsVec<*mut GObject, *mut *mut GObject> for ObjectRef {
    unsafe fn from_glib_none_as_vec(ptr: *mut *mut GObject) -> Vec<Self> {
        // Attention: This takes ownership of floating references!
        FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, c_ptr_array_len(ptr))
    }

    unsafe fn from_glib_container_as_vec(ptr: *mut *mut GObject) -> Vec<Self> {
        // Attention: This takes ownership of floating references!
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, c_ptr_array_len(ptr))
    }

    unsafe fn from_glib_full_as_vec(ptr: *mut *mut GObject) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, c_ptr_array_len(ptr))
    }
}

#[doc(hidden)]
impl FromGlibContainerAsVec<*mut GObject, *const *mut GObject> for ObjectRef {
    unsafe fn from_glib_none_num_as_vec(ptr: *const *mut GObject, num: usize) -> Vec<Self> {
        // Attention: This takes ownership of floating references!
        FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *mut *mut _, num)
    }

    unsafe fn from_glib_container_num_as_vec(_: *const *mut GObject, _: usize) -> Vec<Self> {
        // Can't free a *const
        unimplemented!()
    }

    unsafe fn from_glib_full_num_as_vec(_: *const *mut GObject, _: usize) -> Vec<Self> {
        // Can't free a *const
        unimplemented!()
    }
}

#[doc(hidden)]
impl FromGlibPtrArrayContainerAsVec<*mut GObject, *const *mut GObject> for ObjectRef {
    unsafe fn from_glib_none_as_vec(ptr: *const *mut GObject) -> Vec<Self> {
        // Attention: This takes ownership of floating references!
        FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr as *mut *mut _)
    }

    unsafe fn from_glib_container_as_vec(_: *const *mut GObject) -> Vec<Self> {
        // Can't free a *const
        unimplemented!()
    }

    unsafe fn from_glib_full_as_vec(_: *const *mut GObject) -> Vec<Self> {
        // Can't free a *const
        unimplemented!()
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! glib_weak_impl {
    ($name:ident) => {
        #[doc(hidden)]
        impl $crate::clone::Downgrade for $name {
            type Weak = $crate::object::WeakRef<Self>;

            fn downgrade(&self) -> Self::Weak {
                <Self as $crate::object::ObjectExt>::downgrade(&self)
            }
        }
    };
}

/// ObjectType implementations for Object types. See `glib_wrapper!`.
#[macro_export]
macro_rules! glib_object_wrapper {
    (@generic_impl [$($attr:meta)*] $name:ident, $ffi_name:path, $ffi_class_name:path, $rust_class_name:path, @get_type $get_type_expr:expr) => {
        $(#[$attr])*
        // Always derive Hash/Ord (and below impl Debug, PartialEq, Eq, PartialOrd) for object
        // types. Due to inheritance and up/downcasting we must implement these by pointer or
        // otherwise they would potentially give differeny results for the same object depending on
        // the type we currently know for it
        #[derive(Clone, Hash, Ord)]
        pub struct $name($crate::object::ObjectRef, ::std::marker::PhantomData<$ffi_name>);

        #[doc(hidden)]
        impl Into<$crate::object::ObjectRef> for $name {
            fn into(self) -> $crate::object::ObjectRef {
                self.0
            }
        }

        #[doc(hidden)]
        impl $crate::object::UnsafeFrom<$crate::object::ObjectRef> for $name {
            #[allow(clippy::missing_safety_doc)]
            unsafe fn unsafe_from(t: $crate::object::ObjectRef) -> Self {
                $name(t, ::std::marker::PhantomData)
            }
        }

        #[doc(hidden)]
        impl $crate::translate::GlibPtrDefault for $name {
            type GlibType = *mut $ffi_name;
        }

        #[doc(hidden)]
        unsafe impl $crate::object::ObjectType for $name {
            type GlibType = $ffi_name;
            type GlibClassType = $ffi_class_name;
            type RustClassType = $rust_class_name;

            fn as_object_ref(&self) -> &$crate::object::ObjectRef {
                &self.0
            }

            fn as_ptr(&self) -> *mut Self::GlibType {
                self.0.to_glib_none().0 as *mut _
            }
        }

        #[doc(hidden)]
        impl AsRef<$crate::object::ObjectRef> for $name {
            fn as_ref(&self) -> &$crate::object::ObjectRef {
                &self.0
            }
        }

        #[doc(hidden)]
        impl AsRef<$name> for $name {
            fn as_ref(&self) -> &$name {
                self
            }
        }

        #[doc(hidden)]
        unsafe impl $crate::object::IsA<$name> for $name { }

        #[doc(hidden)]
        impl<'a> $crate::translate::ToGlibPtr<'a, *const $ffi_name> for $name {
            type Storage = <$crate::object::ObjectRef as
                $crate::translate::ToGlibPtr<'a, *mut $crate::object::GObject>>::Storage;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::translate::Stash<'a, *const $ffi_name, Self> {
                let stash = $crate::translate::ToGlibPtr::to_glib_none(&self.0);
                $crate::translate::Stash(stash.0 as *const _, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *const $ffi_name {
                $crate::translate::ToGlibPtr::to_glib_full(&self.0) as *const _
            }
        }

        #[doc(hidden)]
        impl<'a> $crate::translate::ToGlibPtr<'a, *mut $ffi_name> for $name {
            type Storage = <$crate::object::ObjectRef as
                $crate::translate::ToGlibPtr<'a, *mut $crate::object::GObject>>::Storage;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::translate::Stash<'a, *mut $ffi_name, Self> {
                let stash = $crate::translate::ToGlibPtr::to_glib_none(&self.0);
                $crate::translate::Stash(stash.0 as *mut _, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *mut $ffi_name {
                $crate::translate::ToGlibPtr::to_glib_full(&self.0) as *mut _
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
            #[allow(clippy::cast_ptr_alignment)]
            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_none(ptr: *mut $ffi_name) -> Self {
                debug_assert!($crate::types::instance_of::<Self>(ptr as *const _));
                $name($crate::translate::from_glib_none(ptr as *mut _), ::std::marker::PhantomData)
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibPtrNone<*const $ffi_name> for $name {
            #[inline]
            #[allow(clippy::cast_ptr_alignment)]
            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_none(ptr: *const $ffi_name) -> Self {
                debug_assert!($crate::types::instance_of::<Self>(ptr as *const _));
                $name($crate::translate::from_glib_none(ptr as *mut _), ::std::marker::PhantomData)
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibPtrFull<*mut $ffi_name> for $name {
            #[inline]
            #[allow(clippy::cast_ptr_alignment)]
            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_full(ptr: *mut $ffi_name) -> Self {
                debug_assert!($crate::types::instance_of::<Self>(ptr as *const _));
                $name($crate::translate::from_glib_full(ptr as *mut _), ::std::marker::PhantomData)
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibPtrBorrow<*mut $ffi_name> for $name {
            #[inline]
            #[allow(clippy::cast_ptr_alignment)]
            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_borrow(ptr: *mut $ffi_name) -> Self {
                debug_assert!($crate::types::instance_of::<Self>(ptr as *const _));
                $name($crate::translate::from_glib_borrow(ptr as *mut _),
                      ::std::marker::PhantomData)
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibPtrBorrow<*const $ffi_name> for $name {
            #[inline]
            #[allow(clippy::cast_ptr_alignment)]
            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_borrow(ptr: *const $ffi_name) -> Self {
                $crate::translate::from_glib_borrow(ptr as *mut $ffi_name)
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

        #[doc(hidden)]
        impl $crate::translate::FromGlibContainerAsVec<*mut $ffi_name, *const *mut $ffi_name> for $name {
            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_none_num_as_vec(ptr: *const *mut $ffi_name, num: usize) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *mut *mut _, num)
            }

            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_container_num_as_vec(_: *const *mut $ffi_name, _: usize) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }

            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_full_num_as_vec(_: *const *mut $ffi_name, _: usize) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }
        }

        #[doc(hidden)]
        impl $crate::translate::FromGlibPtrArrayContainerAsVec<*mut $ffi_name, *const *mut $ffi_name> for $name {
            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_none_as_vec(ptr: *const *mut $ffi_name) -> Vec<Self> {
                $crate::translate::FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr as *mut *mut _)
            }

            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_container_as_vec(_: *const *mut $ffi_name) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }

            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_glib_full_as_vec(_: *const *mut $ffi_name) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }
        }

        impl $crate::types::StaticType for $name {
            fn static_type() -> $crate::types::Type {
                #[allow(unused_unsafe)]
                unsafe { $crate::translate::from_glib($get_type_expr) }
            }
        }

        impl<T: $crate::object::ObjectType> ::std::cmp::PartialEq<T> for $name {
            #[inline]
            fn eq(&self, other: &T) -> bool {
                $crate::translate::ToGlibPtr::to_glib_none(&self.0).0 == $crate::translate::ToGlibPtr::to_glib_none($crate::object::ObjectType::as_object_ref(other)).0
            }
        }

        impl ::std::cmp::Eq for $name { }

        impl<T: $crate::object::ObjectType> ::std::cmp::PartialOrd<T> for $name {
            #[inline]
            fn partial_cmp(&self, other: &T) -> Option<::std::cmp::Ordering> {
                $crate::translate::ToGlibPtr::to_glib_none(&self.0).0.partial_cmp(&$crate::translate::ToGlibPtr::to_glib_none($crate::object::ObjectType::as_object_ref(other)).0)
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("inner", &self.0)
                    .finish()
            }
        }

        #[doc(hidden)]
        impl<'a> $crate::value::FromValueOptional<'a> for $name {
            #[allow(clippy::missing_safety_doc)]
            unsafe fn from_value_optional(value: &$crate::Value) -> Option<Self> {
                let obj = $crate::gobject_sys::g_value_get_object($crate::translate::ToGlibPtr::to_glib_none(value).0);

                // Attention: Don't use from_glib_none() here because we don't want to steal any
                // floating references that might be owned by someone else.
                if !obj.is_null() {
                    $crate::gobject_sys::g_object_ref(obj);
                }

                // And take the reference to the object from above to pass it to the caller
                Option::<$name>::from_glib_full(obj as *mut $ffi_name).map(|o| $crate::object::Cast::unsafe_cast(o))
            }
        }

        #[doc(hidden)]
        impl $crate::value::SetValue for $name {
            #[allow(clippy::cast_ptr_alignment)]
            #[allow(clippy::missing_safety_doc)]
            unsafe fn set_value(value: &mut $crate::Value, this: &Self) {
                $crate::gobject_sys::g_value_set_object($crate::translate::ToGlibPtrMut::to_glib_none_mut(value).0, $crate::translate::ToGlibPtr::<*mut $ffi_name>::to_glib_none(this).0 as *mut $crate::gobject_sys::GObject)
            }
        }

        #[doc(hidden)]
        impl $crate::value::SetValueOptional for $name {
            #[allow(clippy::cast_ptr_alignment)]
            #[allow(clippy::missing_safety_doc)]
            unsafe fn set_value_optional(value: &mut $crate::Value, this: Option<&Self>) {
                $crate::gobject_sys::g_value_set_object($crate::translate::ToGlibPtrMut::to_glib_none_mut(value).0, $crate::translate::ToGlibPtr::<*mut $ffi_name>::to_glib_none(&this).0 as *mut $crate::gobject_sys::GObject)
            }
        }

        $crate::glib_weak_impl!($name);
    };

    (@munch_impls $name:ident, ) => { };

    (@munch_impls $name:ident, $super_name:path) => {
        unsafe impl $crate::object::IsA<$super_name> for $name { }

        #[doc(hidden)]
        impl AsRef<$super_name> for $name {
            fn as_ref(&self) -> &$super_name {
                $crate::object::Cast::upcast_ref(self)
            }
        }
    };

    (@munch_impls $name:ident, $super_name:path, $($implements:tt)*) => {
        glib_object_wrapper!(@munch_impls $name, $super_name);
        glib_object_wrapper!(@munch_impls $name, $($implements)*);
    };

    // If there is no parent class, i.e. only glib::Object
    (@munch_first_impl $name:ident, $rust_class_name:ident, ) => {
        glib_object_wrapper!(@munch_impls $name, );

        impl ::std::ops::Deref for $rust_class_name {
            type Target = <$crate::object::Object as $crate::object::ObjectType>::RustClassType;

            fn deref(&self) -> &Self::Target {
                $crate::object::IsClassFor::upcast_ref(self)
            }
        }

        impl ::std::ops::DerefMut for $rust_class_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                $crate::object::IsClassFor::upcast_ref_mut(self)
            }
        }
    };

    // If there is only one parent class
    (@munch_first_impl $name:ident, $rust_class_name:ident, $super_name:path) => {
        glib_object_wrapper!(@munch_impls $name, $super_name);

        impl ::std::ops::Deref for $rust_class_name {
            type Target = <$super_name as $crate::object::ObjectType>::RustClassType;

            fn deref(&self) -> &Self::Target {
                $crate::object::IsClassFor::upcast_ref(self)
            }
        }

        impl ::std::ops::DerefMut for $rust_class_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                $crate::object::IsClassFor::upcast_ref_mut(self)
            }
        }
    };

    // If there is more than one parent class
    (@munch_first_impl $name:ident, $rust_class_name:ident, $super_name:path, $($implements:tt)*) => {
        glib_object_wrapper!(@munch_impls $name, $super_name);

        impl ::std::ops::Deref for $rust_class_name {
            type Target = <$super_name as $crate::object::ObjectType>::RustClassType;

            fn deref(&self) -> &Self::Target {
                $crate::object::IsClassFor::upcast_ref(self)
            }
        }

        impl ::std::ops::DerefMut for $rust_class_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                $crate::object::IsClassFor::upcast_ref_mut(self)
            }
        }

        glib_object_wrapper!(@munch_impls $name, $($implements)*);
    };

    (@class_impl $name:ident, $ffi_class_name:path, $rust_class_name:ident) => {
        #[repr(C)]
        #[derive(Debug)]
        pub struct $rust_class_name($ffi_class_name);

        unsafe impl $crate::object::IsClassFor for $rust_class_name {
            type Instance = $name;
        }

        unsafe impl Send for $rust_class_name { }
        unsafe impl Sync for $rust_class_name { }
    };

    // This case is only for glib::Object itself below. All other cases have glib::Object in its
    // parent class list
    (@object [$($attr:meta)*] $name:ident, $ffi_name:path, $ffi_class_name:path, $rust_class_name:ident, @get_type $get_type_expr:expr) => {
        glib_object_wrapper!(@generic_impl [$($attr)*] $name, $ffi_name, $ffi_class_name, $rust_class_name,
            @get_type $get_type_expr);
        glib_object_wrapper!(@class_impl $name, $ffi_class_name, $rust_class_name);
    };

    (@object [$($attr:meta)*] $name:ident, $ffi_name:path, $ffi_class_name:path, $rust_class_name:ident,
        @get_type $get_type_expr:expr, @extends [$($extends:tt)*], @implements [$($implements:tt)*]) => {
        glib_object_wrapper!(@generic_impl [$($attr)*] $name, $ffi_name, $ffi_class_name, $rust_class_name,
            @get_type $get_type_expr);
        glib_object_wrapper!(@munch_first_impl $name, $rust_class_name, $($extends)*);
        glib_object_wrapper!(@munch_impls $name, $($implements)*);
        glib_object_wrapper!(@class_impl $name, $ffi_class_name, $rust_class_name);

        #[doc(hidden)]
        impl AsRef<$crate::object::Object> for $name {
            fn as_ref(&self) -> &$crate::object::Object {
                $crate::object::Cast::upcast_ref(self)
            }
        }

        #[doc(hidden)]
        unsafe impl $crate::object::IsA<$crate::object::Object> for $name { }
    };

    (@interface [$($attr:meta)*] $name:ident, $ffi_name:path, @get_type $get_type_expr:expr, @requires [$($requires:tt)*]) => {
        glib_object_wrapper!(@generic_impl [$($attr)*] $name, $ffi_name, $crate::wrapper::Void, $crate::wrapper::Void,
            @get_type $get_type_expr);
        glib_object_wrapper!(@munch_impls $name, $($requires)*);

        #[doc(hidden)]
        impl AsRef<$crate::object::Object> for $name {
            fn as_ref(&self) -> &$crate::object::Object {
                $crate::object::Cast::upcast_ref(self)
            }
        }

        #[doc(hidden)]
        unsafe impl $crate::object::IsA<$crate::object::Object> for $name { }
    };
}

glib_object_wrapper!(@object
    [doc = "The base class in the object hierarchy."]
    Object, GObject, GObjectClass, ObjectClass, @get_type gobject_sys::g_object_get_type()
);

impl Object {
    pub fn new(type_: Type, properties: &[(&str, &dyn ToValue)]) -> Result<Object, BoolError> {
        use std::ffi::CString;

        if !type_.is_a(&Object::static_type()) {
            return Err(glib_bool_error!("Can't instantiate non-GObject objects"));
        }

        let params = properties
            .iter()
            .map(|&(name, value)| (CString::new(name).unwrap(), value.to_value()))
            .collect::<Vec<_>>();

        let params_c = params
            .iter()
            .map(|&(ref name, ref value)| gobject_sys::GParameter {
                name: name.as_ptr(),
                value: unsafe { *value.to_glib_none().0 },
            })
            .collect::<Vec<_>>();

        unsafe {
            let ptr = gobject_sys::g_object_newv(
                type_.to_glib(),
                params_c.len() as u32,
                mut_override(params_c.as_ptr()),
            );
            if ptr.is_null() {
                Err(glib_bool_error!("Can't instantiate object"))
            } else if type_.is_a(&InitiallyUnowned::static_type()) {
                // Attention: This takes ownership of the floating reference
                Ok(from_glib_none(ptr))
            } else {
                Ok(from_glib_full(ptr))
            }
        }
    }
}

pub trait ObjectExt: ObjectType {
    /// Returns `true` if the object is an instance of (can be cast to) `T`.
    fn is<T: StaticType>(&self) -> bool;

    fn get_type(&self) -> Type;
    fn get_object_class(&self) -> &ObjectClass;

    fn set_property<'a, N: Into<&'a str>>(
        &self,
        property_name: N,
        value: &dyn ToValue,
    ) -> Result<(), BoolError>;
    fn get_property<'a, N: Into<&'a str>>(&self, property_name: N) -> Result<Value, BoolError>;
    fn has_property<'a, N: Into<&'a str>>(
        &self,
        property_name: N,
        type_: Option<Type>,
    ) -> Result<(), BoolError>;
    fn get_property_type<'a, N: Into<&'a str>>(&self, property_name: N) -> Option<Type>;
    fn find_property<'a, N: Into<&'a str>>(&self, property_name: N) -> Option<::ParamSpec>;
    fn list_properties(&self) -> Vec<::ParamSpec>;

    fn block_signal(&self, handler_id: &SignalHandlerId);
    fn unblock_signal(&self, handler_id: &SignalHandlerId);
    fn stop_signal_emission(&self, signal_name: &str);

    fn connect<'a, N, F>(
        &self,
        signal_name: N,
        after: bool,
        callback: F,
    ) -> Result<SignalHandlerId, BoolError>
    where
        N: Into<&'a str>,
        F: Fn(&[Value]) -> Option<Value> + Send + Sync + 'static;
    fn connect_local<'a, N, F>(
        &self,
        signal_name: N,
        after: bool,
        callback: F,
    ) -> Result<SignalHandlerId, BoolError>
    where
        N: Into<&'a str>,
        F: Fn(&[Value]) -> Option<Value> + 'static;
    unsafe fn connect_unsafe<'a, N, F>(
        &self,
        signal_name: N,
        after: bool,
        callback: F,
    ) -> Result<SignalHandlerId, BoolError>
    where
        N: Into<&'a str>,
        F: Fn(&[Value]) -> Option<Value>;
    fn emit<'a, N: Into<&'a str>>(
        &self,
        signal_name: N,
        args: &[&dyn ToValue],
    ) -> Result<Option<Value>, BoolError>;
    fn disconnect(&self, handler_id: SignalHandlerId);

    fn connect_notify<F: Fn(&Self, &::ParamSpec) + Send + Sync + 'static>(
        &self,
        name: Option<&str>,
        f: F,
    ) -> SignalHandlerId;
    unsafe fn connect_notify_unsafe<F: Fn(&Self, &::ParamSpec)>(
        &self,
        name: Option<&str>,
        f: F,
    ) -> SignalHandlerId;
    fn notify<'a, N: Into<&'a str>>(&self, property_name: N);
    fn notify_by_pspec(&self, pspec: &::ParamSpec);

    fn downgrade(&self) -> WeakRef<Self>;

    fn bind_property<'a, O: ObjectType, N: Into<&'a str>, M: Into<&'a str>>(
        &'a self,
        source_property: N,
        target: &'a O,
        target_property: M,
    ) -> BindingBuilder<'a>;

    fn ref_count(&self) -> u32;
}

impl<T: ObjectType> ObjectExt for T {
    fn is<U: StaticType>(&self) -> bool {
        self.get_type().is_a(&U::static_type())
    }

    fn get_type(&self) -> Type {
        self.get_object_class().get_type()
    }

    fn get_object_class(&self) -> &ObjectClass {
        unsafe {
            let obj: *mut gobject_sys::GObject = self.as_object_ref().to_glib_none().0;
            let klass = (*obj).g_type_instance.g_class as *const ObjectClass;
            &*klass
        }
    }

    fn set_property<'a, N: Into<&'a str>>(
        &self,
        property_name: N,
        value: &dyn ToValue,
    ) -> Result<(), BoolError> {
        let property_name = property_name.into();
        let mut property_value = value.to_value();

        let pspec = match self.find_property(property_name) {
            Some(pspec) => pspec,
            None => {
                return Err(glib_bool_error!("property not found"));
            }
        };

        if !pspec.get_flags().contains(::ParamFlags::WRITABLE)
            || pspec.get_flags().contains(::ParamFlags::CONSTRUCT_ONLY)
        {
            return Err(glib_bool_error!("property is not writable"));
        }

        unsafe {
            // While GLib actually allows all types that can somehow be transformed
            // into the property type, we're more restrictive here to be consistent
            // with Rust's type rules. We only allow the exact same type, or if the
            // value type is a subtype of the property type
            let valid_type: bool = from_glib(gobject_sys::g_type_check_value_holds(
                mut_override(property_value.to_glib_none().0),
                pspec.get_value_type().to_glib(),
            ));

            // If it's not directly a valid type but an object type, we check if the
            // actual type of the contained object is compatible and if so create
            // a properly type Value. This can happen if the type field in the
            // Value is set to a more generic type than the contained value
            if !valid_type && property_value.type_().is_a(&Object::static_type()) {
                match property_value.get::<Object>() {
                    Ok(Some(obj)) => {
                        if obj.get_type().is_a(&pspec.get_value_type()) {
                            property_value.0.g_type = pspec.get_value_type().to_glib();
                        } else {
                            return Err(glib_bool_error!(format!(
                                concat!(
                                    "property can't be set from the given object type ",
                                    "(expected: {:?}, got: {:?})",
                                ),
                                pspec.get_value_type(),
                                obj.get_type(),
                            )));
                        }
                    }
                    Ok(None) => {
                        // If the value is None then the type is compatible too
                        property_value.0.g_type = pspec.get_value_type().to_glib();
                    }
                    Err(_) => unreachable!("property_value type conformity already checked"),
                }
            } else if !valid_type {
                return Err(glib_bool_error!(format!(
                    "property can't be set from the given type (expected: {:?}, got: {:?})",
                    pspec.get_value_type(),
                    property_value.type_(),
                )));
            }

            let changed: bool = from_glib(gobject_sys::g_param_value_validate(
                pspec.to_glib_none().0,
                property_value.to_glib_none_mut().0,
            ));
            let change_allowed = pspec.get_flags().contains(::ParamFlags::LAX_VALIDATION);
            if changed && !change_allowed {
                return Err(glib_bool_error!(
                    "property can't be set from given value, it is invalid or out of range"
                ));
            }

            gobject_sys::g_object_set_property(
                self.as_object_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                property_value.to_glib_none().0,
            );
        }

        Ok(())
    }

    fn get_property<'a, N: Into<&'a str>>(&self, property_name: N) -> Result<Value, BoolError> {
        let property_name = property_name.into();

        let pspec = match self.find_property(property_name) {
            Some(pspec) => pspec,
            None => {
                return Err(glib_bool_error!("property not found"));
            }
        };

        if !pspec.get_flags().contains(::ParamFlags::READABLE) {
            return Err(glib_bool_error!("property is not readable"));
        }

        unsafe {
            let mut value = Value::from_type(pspec.get_value_type());
            gobject_sys::g_object_get_property(
                self.as_object_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                value.to_glib_none_mut().0,
            );

            // This can't really happen unless something goes wrong inside GObject
            if value.type_() == ::Type::Invalid {
                Err(glib_bool_error!("Failed to get property value"))
            } else {
                Ok(value)
            }
        }
    }

    fn block_signal(&self, handler_id: &SignalHandlerId) {
        unsafe {
            gobject_sys::g_signal_handler_block(
                self.as_object_ref().to_glib_none().0,
                handler_id.to_glib(),
            );
        }
    }

    fn unblock_signal(&self, handler_id: &SignalHandlerId) {
        unsafe {
            gobject_sys::g_signal_handler_unblock(
                self.as_object_ref().to_glib_none().0,
                handler_id.to_glib(),
            );
        }
    }

    fn stop_signal_emission(&self, signal_name: &str) {
        unsafe {
            gobject_sys::g_signal_stop_emission_by_name(
                self.as_object_ref().to_glib_none().0,
                signal_name.to_glib_none().0,
            );
        }
    }

    fn disconnect(&self, handler_id: SignalHandlerId) {
        unsafe {
            gobject_sys::g_signal_handler_disconnect(
                self.as_object_ref().to_glib_none().0,
                handler_id.to_glib(),
            );
        }
    }

    fn connect_notify<F: Fn(&Self, &::ParamSpec) + Send + Sync + 'static>(
        &self,
        name: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe { self.connect_notify_unsafe(name, f) }
    }

    unsafe fn connect_notify_unsafe<F: Fn(&Self, &::ParamSpec)>(
        &self,
        name: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_trampoline<P, F: Fn(&P, &::ParamSpec)>(
            this: *mut gobject_sys::GObject,
            param_spec: *mut gobject_sys::GParamSpec,
            f: glib_sys::gpointer,
        ) where
            P: ObjectType,
        {
            let f: &F = &*(f as *const F);
            f(
                &Object::from_glib_borrow(this).unsafe_cast(),
                &from_glib_borrow(param_spec),
            )
        }

        let signal_name = if let Some(name) = name {
            format!("notify::{}\0", name)
        } else {
            "notify\0".into()
        };

        let f: Box<F> = Box::new(f);
        ::signal::connect_raw(
            self.as_object_ref().to_glib_none().0,
            signal_name.as_ptr() as *const _,
            Some(mem::transmute(notify_trampoline::<Self, F> as usize)),
            Box::into_raw(f),
        )
    }

    fn notify<'a, N: Into<&'a str>>(&self, property_name: N) {
        let property_name = property_name.into();

        unsafe {
            gobject_sys::g_object_notify(
                self.as_object_ref().to_glib_none().0,
                property_name.to_glib_none().0,
            );
        }
    }

    fn notify_by_pspec(&self, pspec: &::ParamSpec) {
        unsafe {
            gobject_sys::g_object_notify_by_pspec(
                self.as_object_ref().to_glib_none().0,
                pspec.to_glib_none().0,
            );
        }
    }

    fn has_property<'a, N: Into<&'a str>>(
        &self,
        property_name: N,
        type_: Option<Type>,
    ) -> Result<(), BoolError> {
        self.get_object_class().has_property(property_name, type_)
    }

    fn get_property_type<'a, N: Into<&'a str>>(&self, property_name: N) -> Option<Type> {
        self.get_object_class().get_property_type(property_name)
    }

    fn find_property<'a, N: Into<&'a str>>(&self, property_name: N) -> Option<::ParamSpec> {
        self.get_object_class().find_property(property_name)
    }

    fn list_properties(&self) -> Vec<::ParamSpec> {
        self.get_object_class().list_properties()
    }

    fn connect<'a, N, F>(
        &self,
        signal_name: N,
        after: bool,
        callback: F,
    ) -> Result<SignalHandlerId, BoolError>
    where
        N: Into<&'a str>,
        F: Fn(&[Value]) -> Option<Value> + Send + Sync + 'static,
    {
        unsafe { self.connect_unsafe(signal_name, after, callback) }
    }

    fn connect_local<'a, N, F>(
        &self,
        signal_name: N,
        after: bool,
        callback: F,
    ) -> Result<SignalHandlerId, BoolError>
    where
        N: Into<&'a str>,
        F: Fn(&[Value]) -> Option<Value> + 'static,
    {
        let callback = crate::ThreadGuard::new(callback);

        unsafe {
            self.connect_unsafe(signal_name, after, move |values| {
                (callback.get_ref())(values)
            })
        }
    }

    unsafe fn connect_unsafe<'a, N, F>(
        &self,
        signal_name: N,
        after: bool,
        callback: F,
    ) -> Result<SignalHandlerId, BoolError>
    where
        N: Into<&'a str>,
        F: Fn(&[Value]) -> Option<Value>,
    {
        let signal_name: &str = signal_name.into();

        let type_ = self.get_type();

        let mut signal_id = 0;
        let mut signal_detail = 0;

        let found: bool = from_glib(gobject_sys::g_signal_parse_name(
            signal_name.to_glib_none().0,
            type_.to_glib(),
            &mut signal_id,
            &mut signal_detail,
            true.to_glib(),
        ));

        if !found {
            return Err(glib_bool_error!("Signal not found"));
        }

        let mut details = mem::MaybeUninit::zeroed();
        gobject_sys::g_signal_query(signal_id, details.as_mut_ptr());
        let details = details.assume_init();
        if details.signal_id != signal_id {
            return Err(glib_bool_error!("Signal not found"));
        }

        // This is actually G_SIGNAL_TYPE_STATIC_SCOPE
        let return_type: Type =
            from_glib(details.return_type & (!gobject_sys::G_TYPE_FLAG_RESERVED_ID_BIT));
        let closure = Closure::new_unsafe(move |values| {
            let ret = callback(values);

            if return_type == Type::Unit {
                if let Some(ret) = ret {
                    panic!(
                        "Signal required no return value but got value of type {}",
                        ret.type_().name()
                    );
                }
                None
            } else {
                match ret {
                    Some(mut ret) => {
                        let valid_type: bool = from_glib(gobject_sys::g_type_check_value_holds(
                            mut_override(ret.to_glib_none().0),
                            return_type.to_glib(),
                        ));

                        // If it's not directly a valid type but an object type, we check if the
                        // actual typed of the contained object is compatible and if so create
                        // a properly typed Value. This can happen if the type field in the
                        // Value is set to a more generic type than the contained value
                        if !valid_type && ret.type_().is_a(&Object::static_type()) {
                            match ret.get::<Object>() {
                                Ok(Some(obj)) => {
                                    if obj.get_type().is_a(&return_type) {
                                        ret.0.g_type = return_type.to_glib();
                                    } else {
                                        panic!("Signal required return value of type {} but got {} (actual {})",
                                           return_type.name(), ret.type_().name(), obj.get_type().name());
                                    }
                                }
                                Ok(None) => {
                                    // If the value is None then the type is compatible too
                                    ret.0.g_type = return_type.to_glib();
                                }
                                Err(_) => unreachable!("ret type conformity already checked"),
                            }
                        } else if !valid_type {
                            panic!(
                                "Signal required return value of type {} but got {}",
                                return_type.name(),
                                ret.type_().name()
                            );
                        }
                        Some(ret)
                    }
                    None => {
                        panic!(
                            "Signal required return value of type {} but got None",
                            return_type.name()
                        );
                    }
                }
            }
        });
        let handler = gobject_sys::g_signal_connect_closure_by_id(
            self.as_object_ref().to_glib_none().0,
            signal_id,
            signal_detail,
            closure.to_glib_none().0,
            after.to_glib(),
        );

        if handler == 0 {
            Err(glib_bool_error!("Failed to connect to signal"))
        } else {
            Ok(from_glib(handler))
        }
    }

    fn emit<'a, N: Into<&'a str>>(
        &self,
        signal_name: N,
        args: &[&dyn ToValue],
    ) -> Result<Option<Value>, BoolError> {
        let signal_name: &str = signal_name.into();
        unsafe {
            let type_ = self.get_type();

            let mut signal_id = 0;
            let mut signal_detail = 0;

            let found: bool = from_glib(gobject_sys::g_signal_parse_name(
                signal_name.to_glib_none().0,
                type_.to_glib(),
                &mut signal_id,
                &mut signal_detail,
                true.to_glib(),
            ));

            if !found {
                return Err(glib_bool_error!("Signal not found"));
            }

            let mut details = mem::MaybeUninit::zeroed();
            gobject_sys::g_signal_query(signal_id, details.as_mut_ptr());
            let details = details.assume_init();
            if details.signal_id != signal_id {
                return Err(glib_bool_error!("Signal not found"));
            }

            if details.n_params != args.len() as u32 {
                return Err(glib_bool_error!("Incompatible number of arguments"));
            }

            for (i, item) in args.iter().enumerate() {
                let arg_type =
                    *(details.param_types.add(i)) & (!gobject_sys::G_TYPE_FLAG_RESERVED_ID_BIT);
                if arg_type != item.to_value_type().to_glib() {
                    return Err(glib_bool_error!("Incompatible argument types"));
                }
            }

            let mut v_args: Vec<Value>;
            let mut s_args: [Value; 10] = mem::zeroed();
            let self_v = {
                let mut v = Value::uninitialized();
                gobject_sys::g_value_init(v.to_glib_none_mut().0, self.get_type().to_glib());
                gobject_sys::g_value_set_object(
                    v.to_glib_none_mut().0,
                    self.as_object_ref().to_glib_none().0,
                );
                v
            };
            let args = if args.len() < 10 {
                s_args[0] = self_v;
                for (i, arg) in args.iter().enumerate() {
                    s_args[i + 1] = arg.to_value();
                }
                &s_args[0..=args.len()]
            } else {
                v_args = Vec::with_capacity(args.len() + 1);
                v_args.push(self_v);
                for arg in args {
                    v_args.push(arg.to_value());
                }
                v_args.as_slice()
            };

            let mut return_value = Value::uninitialized();
            if details.return_type != gobject_sys::G_TYPE_NONE {
                gobject_sys::g_value_init(return_value.to_glib_none_mut().0, details.return_type);
            }

            gobject_sys::g_signal_emitv(
                mut_override(args.as_ptr()) as *mut gobject_sys::GValue,
                signal_id,
                signal_detail,
                return_value.to_glib_none_mut().0,
            );

            if return_value.type_() != Type::Unit && return_value.type_() != Type::Invalid {
                Ok(Some(return_value))
            } else {
                Ok(None)
            }
        }
    }

    fn downgrade(&self) -> WeakRef<T> {
        unsafe {
            let w = WeakRef(Box::new(mem::zeroed()), PhantomData);
            gobject_sys::g_weak_ref_init(
                mut_override(&*w.0),
                self.as_object_ref().to_glib_none().0,
            );
            w
        }
    }

    fn bind_property<'a, O: ObjectType, N: Into<&'a str>, M: Into<&'a str>>(
        &'a self,
        source_property: N,
        target: &'a O,
        target_property: M,
    ) -> BindingBuilder<'a> {
        let source_property = source_property.into();
        let target_property = target_property.into();

        BindingBuilder::new(self, source_property, target, target_property)
    }

    fn ref_count(&self) -> u32 {
        let stash = self.as_object_ref().to_glib_none();
        let ptr: *mut gobject_sys::GObject = stash.0;

        unsafe { glib_sys::g_atomic_int_get(&(*ptr).ref_count as *const u32 as *const i32) as u32 }
    }
}

impl ObjectClass {
    pub fn has_property<'a, N: Into<&'a str>>(
        &self,
        property_name: N,
        type_: Option<Type>,
    ) -> Result<(), BoolError> {
        let property_name = property_name.into();
        let ptype = self.get_property_type(property_name);

        match (ptype, type_) {
            (None, _) => Err(glib_bool_error!("Invalid property name")),
            (Some(_), None) => Ok(()),
            (Some(ptype), Some(type_)) => {
                if ptype == type_ {
                    Ok(())
                } else {
                    Err(glib_bool_error!("Invalid property type"))
                }
            }
        }
    }

    pub fn get_property_type<'a, N: Into<&'a str>>(&self, property_name: N) -> Option<Type> {
        self.find_property(property_name)
            .map(|pspec| pspec.get_value_type())
    }

    pub fn find_property<'a, N: Into<&'a str>>(&self, property_name: N) -> Option<::ParamSpec> {
        let property_name = property_name.into();
        unsafe {
            let klass = self as *const _ as *const gobject_sys::GObjectClass;

            from_glib_none(gobject_sys::g_object_class_find_property(
                klass as *mut _,
                property_name.to_glib_none().0,
            ))
        }
    }

    pub fn list_properties(&self) -> Vec<::ParamSpec> {
        unsafe {
            let klass = self as *const _ as *const gobject_sys::GObjectClass;

            let mut n_properties = 0;

            let props =
                gobject_sys::g_object_class_list_properties(klass as *mut _, &mut n_properties);
            FromGlibContainer::from_glib_none_num(props, n_properties as usize)
        }
    }
}

glib_wrapper! {
    pub struct InitiallyUnowned(Object<gobject_sys::GInitiallyUnowned, gobject_sys::GInitiallyUnownedClass, InitiallyUnownedClass>);

    match fn {
        get_type => || gobject_sys::g_initially_unowned_get_type(),
    }
}

#[derive(Debug)]
pub struct WeakRef<T: ObjectType>(Box<gobject_sys::GWeakRef>, PhantomData<*const T>);

impl<T: ObjectType> WeakRef<T> {
    pub fn new() -> WeakRef<T> {
        unsafe {
            let w = WeakRef(Box::new(mem::zeroed()), PhantomData);
            gobject_sys::g_weak_ref_init(mut_override(&*w.0), ptr::null_mut());
            w
        }
    }

    pub fn upgrade(&self) -> Option<T> {
        unsafe {
            let ptr = gobject_sys::g_weak_ref_get(mut_override(&*self.0));
            if ptr.is_null() {
                None
            } else {
                let obj: Object = from_glib_full(ptr);
                Some(T::unsafe_from(obj.into()))
            }
        }
    }
}

impl<T: ObjectType> Drop for WeakRef<T> {
    fn drop(&mut self) {
        unsafe {
            gobject_sys::g_weak_ref_clear(mut_override(&*self.0));
        }
    }
}

impl<T: ObjectType> Clone for WeakRef<T> {
    fn clone(&self) -> Self {
        unsafe {
            let c = WeakRef(Box::new(mem::zeroed()), PhantomData);

            let o = gobject_sys::g_weak_ref_get(mut_override(&*self.0));
            gobject_sys::g_weak_ref_init(mut_override(&*c.0), o);
            if !o.is_null() {
                gobject_sys::g_object_unref(o);
            }

            c
        }
    }
}

impl<T: ObjectType> Default for WeakRef<T> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<T: ObjectType + Sync + Sync> Sync for WeakRef<T> {}
unsafe impl<T: ObjectType + Send + Sync> Send for WeakRef<T> {}

/// A weak reference to the object it was created for that can be sent to
/// different threads even for object types that don't implement `Send`.
///
/// Trying to upgrade the weak reference from another thread than the one
/// where it was created on will panic but dropping or cloning can be done
/// safely from any thread.
#[derive(Debug)]
pub struct SendWeakRef<T: ObjectType>(WeakRef<T>, Option<usize>);

impl<T: ObjectType> SendWeakRef<T> {
    pub fn new() -> SendWeakRef<T> {
        SendWeakRef(WeakRef::new(), None)
    }

    pub fn into_weak_ref(self) -> WeakRef<T> {
        if self.1.is_some() && self.1 != Some(get_thread_id()) {
            panic!("SendWeakRef dereferenced on a different thread");
        }

        self.0
    }
}

impl<T: ObjectType> ops::Deref for SendWeakRef<T> {
    type Target = WeakRef<T>;

    fn deref(&self) -> &WeakRef<T> {
        if self.1.is_some() && self.1 != Some(get_thread_id()) {
            panic!("SendWeakRef dereferenced on a different thread");
        }

        &self.0
    }
}

// Deriving this gives the wrong trait bounds
impl<T: ObjectType> Clone for SendWeakRef<T> {
    fn clone(&self) -> Self {
        SendWeakRef(self.0.clone(), self.1)
    }
}

impl<T: ObjectType> Default for SendWeakRef<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ObjectType> From<WeakRef<T>> for SendWeakRef<T> {
    fn from(v: WeakRef<T>) -> SendWeakRef<T> {
        SendWeakRef(v, Some(get_thread_id()))
    }
}

unsafe impl<T: ObjectType> Sync for SendWeakRef<T> {}
unsafe impl<T: ObjectType> Send for SendWeakRef<T> {}

#[derive(Debug)]
pub struct BindingBuilder<'a> {
    source: &'a ObjectRef,
    source_property: &'a str,
    target: &'a ObjectRef,
    target_property: &'a str,
    flags: ::BindingFlags,
    transform_to: Option<::Closure>,
    transform_from: Option<::Closure>,
}

impl<'a> BindingBuilder<'a> {
    fn new<S: ObjectType, T: ObjectType>(
        source: &'a S,
        source_property: &'a str,
        target: &'a T,
        target_property: &'a str,
    ) -> Self {
        Self {
            source: source.as_object_ref(),
            source_property,
            target: target.as_object_ref(),
            target_property,
            flags: ::BindingFlags::DEFAULT,
            transform_to: None,
            transform_from: None,
        }
    }

    fn transform_closure<F: Fn(&::Binding, &Value) -> Option<Value> + Send + Sync + 'static>(
        func: F,
    ) -> ::Closure {
        ::Closure::new(move |values| {
            assert_eq!(values.len(), 3);
            let binding = values[0].get::<::Binding>().unwrap_or_else(|_| {
                panic!(
                    "Type mismatch with the first argument in the closure: expected: `Binding`, got: {:?}",
                    values[0].type_(),
                )
            })
            .unwrap_or_else(|| {
                panic!("Found `None` for the first argument in the closure, expected `Some`")
            });
            let from = unsafe {
                let ptr = gobject_sys::g_value_get_boxed(mut_override(
                    &values[1] as *const Value as *const gobject_sys::GValue,
                ));
                assert!(!ptr.is_null());
                &*(ptr as *const gobject_sys::GValue as *const Value)
            };

            match func(&binding, &from) {
                None => Some(false.to_value()),
                Some(value) => {
                    unsafe {
                        gobject_sys::g_value_set_boxed(
                            mut_override(&values[2] as *const Value as *const gobject_sys::GValue),
                            &value as *const Value as *const _,
                        );
                    }

                    Some(true.to_value())
                }
            }
        })
    }

    pub fn transform_from<F: Fn(&::Binding, &Value) -> Option<Value> + Send + Sync + 'static>(
        self,
        func: F,
    ) -> Self {
        Self {
            transform_from: Some(Self::transform_closure(func)),
            ..self
        }
    }

    pub fn transform_to<F: Fn(&::Binding, &Value) -> Option<Value> + Send + Sync + 'static>(
        self,
        func: F,
    ) -> Self {
        Self {
            transform_to: Some(Self::transform_closure(func)),
            ..self
        }
    }

    pub fn flags(self, flags: ::BindingFlags) -> Self {
        Self { flags, ..self }
    }

    pub fn build(self) -> Option<::Binding> {
        unsafe {
            from_glib_none(gobject_sys::g_object_bind_property_with_closures(
                self.source.to_glib_none().0,
                self.source_property.to_glib_none().0,
                self.target.to_glib_none().0,
                self.target_property.to_glib_none().0,
                self.flags.to_glib(),
                self.transform_to.to_glib_none().0,
                self.transform_from.to_glib_none().0,
            ))
        }
    }
}

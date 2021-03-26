// Take a look at the license at the top of the repository in the LICENSE file.

//! Module that contains the basic infrastructure for subclassing `GObject`.

use crate::object::{Cast, ObjectSubclassIs, ObjectType};
use crate::translate::*;
use crate::{Closure, Object, StaticType, Type, Value};
use std::marker;
use std::mem;
use std::ptr;
use std::{any::Any, collections::HashMap};

use super::SignalId;

/// A newly registered `glib::Type` that is currently still being initialized.
///
/// This allows running additional type-setup functions.
#[derive(Debug, PartialEq, Eq)]
pub struct InitializingType<T>(pub(crate) Type, pub(crate) marker::PhantomData<*const T>);

impl<T> ToGlib for InitializingType<T> {
    type GlibType = ffi::GType;

    fn to_glib(&self) -> ffi::GType {
        self.0.to_glib()
    }
}

/// Struct used for the instance private data of the GObject.
struct PrivateStruct<T: ObjectSubclass> {
    imp: T,
    instance_data: Option<HashMap<Type, Box<dyn Any + Send + Sync>>>,
}

/// Trait implemented by structs that implement a `GObject` C instance struct.
///
/// The struct must be `#[repr(C)]` and have the parent type's instance struct
/// as the first field.
///
/// See [`basic::InstanceStruct`] for a basic implementation of this that can
/// be used most of the time and should only not be used if additional fields are
/// required in the instance struct.
///
/// [`basic::InstanceStruct`]: ../basic/struct.InstanceStruct.html
pub unsafe trait InstanceStruct: Sized + 'static {
    /// Corresponding object subclass type for this instance struct.
    type Type: ObjectSubclass;

    /// Returns the implementation for from this instance struct, that
    /// is the implementor of [`ObjectImpl`] or subtraits.
    ///
    /// [`ObjectImpl`]: ../object/trait.ObjectImpl.html
    fn get_impl(&self) -> &Self::Type {
        unsafe {
            let data = Self::Type::type_data();
            let private_offset = data.as_ref().get_impl_offset();
            let ptr: *const u8 = self as *const _ as *const u8;
            let imp_ptr = ptr.offset(private_offset);
            let imp = imp_ptr as *const Self::Type;

            &*imp
        }
    }

    /// Returns the class struct for this specific instance.
    fn get_class(&self) -> &<Self::Type as ObjectSubclass>::Class {
        unsafe { &**(self as *const _ as *const *const <Self::Type as ObjectSubclass>::Class) }
    }

    /// Instance specific initialization.
    ///
    /// This is automatically called during instance initialization and must call `instance_init()`
    /// of the parent class.
    fn instance_init(&mut self) {
        unsafe {
            let obj = from_glib_borrow::<_, Object>(self as *mut _ as *mut gobject_ffi::GObject);
            let obj = Borrowed::new(obj.into_inner().unsafe_cast());
            let mut obj = InitializingObject(obj);

            <<Self::Type as ObjectSubclass>::ParentType as IsSubclassable<Self::Type>>::instance_init(
                &mut obj,
            );
        }
    }
}

/// Trait implemented by structs that implement a `GObject` C class struct.
///
/// The struct must be `#[repr(C)]` and have the parent type's class struct
/// as the first field.
///
/// See [`basic::ClassStruct`] for a basic implementation of this that can
/// be used most of the time and should only not be used if additional fields are
/// required in the class struct, e.g. for declaring new virtual methods.
///
/// [`basic::ClassStruct`]: ../basic/struct.ClassStruct.html
pub unsafe trait ClassStruct: Sized + 'static {
    /// Corresponding object subclass type for this class struct.
    type Type: ObjectSubclass;

    /// Override the vfuncs of all parent types.
    ///
    /// This is automatically called during type initialization.
    fn class_init(&mut self) {
        unsafe {
            let base = &mut *(self as *mut _
                as *mut crate::Class<<Self::Type as ObjectSubclass>::ParentType>);
            <<Self::Type as ObjectSubclass>::ParentType as IsSubclassable<Self::Type>>::class_init(
                base,
            );
        }
    }
}

/// Trait for subclassable class structs.
pub unsafe trait IsSubclassable<T: ObjectSubclass>: crate::object::IsClass {
    /// Override the virtual methods of this class for the given subclass and do other class
    /// initialization.
    ///
    /// This is automatically called during type initialization and must call `class_init()` of the
    /// parent class.
    fn class_init(class: &mut crate::Class<Self>);

    /// Instance specific initialization.
    ///
    /// This is automatically called during instance initialization and must call `instance_init()`
    /// of the parent class.
    fn instance_init(instance: &mut InitializingObject<T>);
}

/// Trait for implementable interfaces.
pub unsafe trait IsImplementable<T: ObjectSubclass>: crate::object::IsInterface
where
    <Self as ObjectType>::GlibClassType: Copy,
{
    /// Override the virtual methods of this interface for the given subclass and do other
    /// interface initialization.
    ///
    /// This is automatically called during type initialization.
    fn interface_init(iface: &mut crate::Interface<Self>);

    /// Instance specific initialization.
    ///
    /// This is automatically called during instance initialization.
    fn instance_init(_instance: &mut InitializingObject<T>);
}

unsafe extern "C" fn interface_init<T: ObjectSubclass, A: IsImplementable<T>>(
    iface: ffi::gpointer,
    _iface_data: ffi::gpointer,
) where
    <A as ObjectType>::GlibClassType: Copy,
{
    let iface = &mut *(iface as *mut crate::Interface<A>);

    let mut data = T::type_data();
    if data.as_ref().parent_ifaces.is_none() {
        data.as_mut().parent_ifaces = Some(HashMap::new());
    }
    {
        let copy = Box::new(*iface.as_ref());
        data.as_mut()
            .parent_ifaces
            .as_mut()
            .unwrap()
            .insert(A::static_type(), Box::into_raw(copy) as ffi::gpointer);
    }

    A::interface_init(iface);
}

/// Trait for a type list of interfaces.
pub trait InterfaceList<T: ObjectSubclass> {
    /// Returns the list of types and corresponding interface infos for this list.
    fn iface_infos() -> Vec<(ffi::GType, gobject_ffi::GInterfaceInfo)>;

    /// Runs `instance_init` on each of the `IsImplementable` items.
    fn instance_init(_instance: &mut InitializingObject<T>);
}

impl<T: ObjectSubclass> InterfaceList<T> for () {
    fn iface_infos() -> Vec<(ffi::GType, gobject_ffi::GInterfaceInfo)> {
        vec![]
    }

    fn instance_init(_instance: &mut InitializingObject<T>) {}
}

impl<T: ObjectSubclass, A: IsImplementable<T>> InterfaceList<T> for (A,)
where
    <A as ObjectType>::GlibClassType: Copy,
{
    fn iface_infos() -> Vec<(ffi::GType, gobject_ffi::GInterfaceInfo)> {
        vec![(
            A::static_type().to_glib(),
            gobject_ffi::GInterfaceInfo {
                interface_init: Some(interface_init::<T, A>),
                interface_finalize: None,
                interface_data: ptr::null_mut(),
            },
        )]
    }

    fn instance_init(instance: &mut InitializingObject<T>) {
        A::instance_init(instance);
    }
}

// Generates all the InterfaceList impls for interface_lists of arbitrary sizes based on a list of type
// parameters like A B C. It would generate the impl then for (A, B) and (A, B, C).
macro_rules! interface_list_trait(
    ($name1:ident, $name2: ident, $($name:ident),*) => (
        interface_list_trait!(__impl $name1, $name2; $($name),*);
    );
    (__impl $($name:ident),+; $name1:ident, $($name2:ident),*) => (
        interface_list_trait_impl!($($name),+);
        interface_list_trait!(__impl $($name),+ , $name1; $($name2),*);
    );
    (__impl $($name:ident),+; $name1:ident) => (
        interface_list_trait_impl!($($name),+);
        interface_list_trait_impl!($($name),+, $name1);
    );
);

// Generates the impl block for InterfaceList on interface_lists or arbitrary sizes based on its
// arguments. Takes a list of type parameters as parameters, e.g. A B C
// and then implements the trait on (A, B, C).
macro_rules! interface_list_trait_impl(
    ($($name:ident),+) => (
        impl<T: ObjectSubclass, $($name: IsImplementable<T>),+> InterfaceList<T> for ( $($name),+ )
        where
            $(<$name as ObjectType>::GlibClassType: Copy),+
        {
            fn iface_infos() -> Vec<(ffi::GType, gobject_ffi::GInterfaceInfo)> {
                vec![
                    $(
                        (
                            $name::static_type().to_glib(),
                            gobject_ffi::GInterfaceInfo {
                                interface_init: Some(interface_init::<T, $name>),
                                interface_finalize: None,
                                interface_data: ptr::null_mut(),
                            },
                        )
                    ),+
                ]
            }

            fn instance_init(instance: &mut InitializingObject<T>) {
                $(
                    $name::instance_init(instance);
                )+
            }
        }
    );
);

interface_list_trait!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);

/// Type-specific data that is filled in during type creation.
// FIXME: Once trait bounds other than `Sized` on const fn parameters are stable
// the content of `TypeData` can be made private and we can add a `const fn new`
// for initialization by the `object_subclass_internal!` macro.
pub struct TypeData {
    #[doc(hidden)]
    pub type_: Type,
    #[doc(hidden)]
    pub parent_class: ffi::gpointer,
    #[doc(hidden)]
    pub parent_ifaces: Option<HashMap<Type, ffi::gpointer>>,
    #[doc(hidden)]
    pub class_data: Option<HashMap<Type, Box<dyn Any + Send + Sync>>>,
    #[doc(hidden)]
    pub private_offset: isize,
    #[doc(hidden)]
    pub private_imp_offset: isize,
}

unsafe impl Send for TypeData {}
unsafe impl Sync for TypeData {}

impl TypeData {
    /// Returns the type ID.
    pub fn get_type(&self) -> Type {
        self.type_
    }

    /// Returns a pointer to the native parent class.
    ///
    /// This is used for chaining up to the parent class' implementation
    /// of virtual methods.
    pub fn get_parent_class(&self) -> ffi::gpointer {
        debug_assert!(!self.parent_class.is_null());
        self.parent_class
    }

    /// Returns a pointer to the native parent interface struct for interface `type_`.
    ///
    /// This is used for chaining up to the parent interface's implementation
    /// of virtual methods.
    ///
    /// # Panics
    ///
    /// This function panics if the type to which the `TypeData` belongs does not implement the
    /// given interface or was not registered yet.
    pub fn get_parent_interface<I: crate::object::IsInterface>(&self) -> ffi::gpointer {
        match self.parent_ifaces {
            None => unreachable!("No parent interfaces"),
            Some(ref parent_ifaces) => *parent_ifaces
                .get(&I::static_type())
                .expect("Parent interface not found"),
        }
    }

    /// Returns a pointer to the class implementation specific data.
    ///
    /// This is used for class implementations to store additional data.
    pub fn get_class_data<T: Any + Send + Sync + 'static>(&self, type_: Type) -> Option<&T> {
        match self.class_data {
            None => None,
            Some(ref data) => data.get(&type_).and_then(|ptr| ptr.downcast_ref()),
        }
    }

    /// Gets a mutable reference of the class implementation specific data.
    ///
    /// # Safety
    ///
    /// This can only be used while the type is being initialized.
    pub unsafe fn get_class_data_mut<T: Any + Send + Sync + 'static>(
        &mut self,
        type_: Type,
    ) -> Option<&mut T> {
        match self.class_data {
            None => None,
            Some(ref mut data) => data.get_mut(&type_).and_then(|v| v.downcast_mut()),
        }
    }

    /// Sets class specific implementation data.
    ///
    /// # Safety
    ///
    /// This can only be used while the type is being initialized.
    ///
    /// # Panics
    ///
    /// If the class_data already contains a data for the specified `type_`.
    pub unsafe fn set_class_data<T: Any + Send + Sync + 'static>(&mut self, type_: Type, data: T) {
        if self.class_data.is_none() {
            self.class_data = Some(HashMap::new());
        }

        if let Some(ref mut class_data) = self.class_data {
            if class_data.get(&type_).is_some() {
                panic!("The class_data already contains a key for {}", type_);
            }

            class_data.insert(type_, Box::new(data));
        }
    }

    /// Returns the offset of the private implementation struct in bytes relative to the beginning
    /// of the instance struct.
    pub fn get_impl_offset(&self) -> isize {
        self.private_offset + self.private_imp_offset
    }
}

/// Type methods required for an [`ObjectSubclass`] implementation.
///
/// This is usually generated by the [`#[object_subclass]`](crate::object_subclass) attribute macro.
pub unsafe trait ObjectSubclassType {
    /// Storage for the type-specific data used during registration.
    fn type_data() -> ptr::NonNull<TypeData>;

    /// Returns the `glib::Type` ID of the subclass.
    ///
    /// This will register the type with the type system on the first call.
    fn get_type() -> Type;
}

/// The central trait for subclassing a `GObject` type.
///
/// Links together the type name, parent type and the instance and
/// class structs for type registration and allows subclasses to
/// hook into various steps of the type registration and initialization.
///
/// See [`register_type`] for registering an implementation of this trait
/// with the type system.
///
/// [`register_type`]: fn.register_type.html
pub trait ObjectSubclass: ObjectSubclassType + Sized + 'static {
    /// `GObject` type name.
    ///
    /// This must be unique in the whole process.
    const NAME: &'static str;

    /// If this subclass is an abstract class or not.
    ///
    /// By default all subclasses are non-abstract types but setting this to `true` will create an
    /// abstract class instead.
    ///
    /// Abstract classes can't be instantiated and require a non-abstract subclass.
    ///
    /// Optional.
    const ABSTRACT: bool = false;

    /// Wrapper around this subclass defined with `wrapper!`
    type Type: ObjectType
        + ObjectSubclassIs<Subclass = Self>
        + FromGlibPtrFull<*mut <Self::Type as ObjectType>::GlibType>
        + FromGlibPtrBorrow<*mut <Self::Type as ObjectType>::GlibType>
        + FromGlibPtrNone<*mut <Self::Type as ObjectType>::GlibType>;

    /// Parent Rust type to inherit from.
    type ParentType: IsSubclassable<Self>
        + FromGlibPtrFull<*mut <Self::ParentType as ObjectType>::GlibType>
        + FromGlibPtrBorrow<*mut <Self::ParentType as ObjectType>::GlibType>
        + FromGlibPtrNone<*mut <Self::ParentType as ObjectType>::GlibType>;

    /// List of interfaces implemented by this type.
    type Interfaces: InterfaceList<Self>;

    /// The C instance struct.
    ///
    /// See [`basic::InstanceStruct`] for an basic instance struct that should be
    /// used in most cases.
    ///
    /// [`basic::InstanceStruct`]: ../basic/struct.InstanceStruct.html
    // TODO: Should default to basic::InstanceStruct<Self> once associated
    // type defaults are stabilized https://github.com/rust-lang/rust/issues/29661
    type Instance: InstanceStruct<Type = Self>;

    /// The C class struct.
    ///
    /// See [`basic::ClassStruct`] for an basic instance struct that should be
    /// used in most cases.
    ///
    /// [`basic::ClassStruct`]: ../basic/struct.ClassStruct.html
    // TODO: Should default to basic::ClassStruct<Self> once associated
    // type defaults are stabilized https://github.com/rust-lang/rust/issues/29661
    type Class: ClassStruct<Type = Self>;

    /// Additional type initialization.
    ///
    /// This is called right after the type was registered and allows
    /// subclasses to do additional type-specific initialization, e.g.
    /// for implementing `GObject` interfaces.
    ///
    /// Optional
    fn type_init(_type_: &mut InitializingType<Self>) {}

    /// Class initialization.
    ///
    /// This is called after `type_init` and before the first instance
    /// of the subclass is created. Subclasses can use this to do class-
    /// specific initialization, e.g. for registering signals on the class
    /// or calling class methods.
    ///
    /// Optional
    fn class_init(_klass: &mut Self::Class) {}

    /// Constructor.
    ///
    /// This is called during object instantiation before further subclasses
    /// are initialized, and should return a new instance of the subclass
    /// private struct.
    ///
    /// Optional, either implement this or `with_class()`.
    fn new() -> Self {
        unimplemented!();
    }

    /// Constructor.
    ///
    /// This is called during object instantiation before further subclasses
    /// are initialized, and should return a new instance of the subclass
    /// private struct.
    ///
    /// Different to `new()` above it also gets the class of this type passed
    /// to itself for providing additional context.
    ///
    /// Optional, either implement this or `new()`.
    fn with_class(_klass: &Self::Class) -> Self {
        Self::new()
    }

    /// Performs additional instance initialization.
    ///
    /// Called just after `with_class()`. At this point the initialization has not completed yet, so
    /// only a limited set of operations is safe (see `InitializingObject`).
    fn instance_init(_obj: &InitializingObject<Self>) {}
}

/// Extension methods for all `ObjectSubclass` impls.
pub trait ObjectSubclassExt: ObjectSubclass {
    /// Returns the corresponding object instance.
    fn get_instance(&self) -> Self::Type;

    /// Returns the implementation from an instance.
    fn from_instance(obj: &Self::Type) -> &Self;

    /// Returns a pointer to the instance implementation specific data.
    ///
    /// This is used for the subclassing infrastructure to store additional instance data.
    fn get_instance_data<U: Any + Send + Sync + 'static>(&self, type_: Type) -> Option<&U>;
}

impl<T: ObjectSubclass> ObjectSubclassExt for T {
    fn get_instance(&self) -> Self::Type {
        unsafe {
            let data = Self::type_data();
            let type_ = data.as_ref().get_type();
            assert!(type_.is_valid());

            let offset = -data.as_ref().get_impl_offset();

            let ptr = self as *const Self as *const u8;
            let ptr = ptr.offset(offset);
            let ptr = ptr as *mut u8 as *mut <Self::Type as ObjectType>::GlibType;

            // The object might just be finalized, and in that case it's unsafe to access
            // it and use any API on it. This can only happen from inside the Drop impl
            // of Self.
            assert_ne!((*(ptr as *mut gobject_ffi::GObject)).ref_count, 0);

            // Don't steal floating reference here via from_glib_none() but
            // preserve it if needed by reffing manually.
            gobject_ffi::g_object_ref(ptr as *mut gobject_ffi::GObject);
            from_glib_full(ptr)
        }
    }

    fn from_instance(obj: &Self::Type) -> &Self {
        unsafe {
            let ptr = obj.as_ptr() as *const Self::Instance;
            (*ptr).get_impl()
        }
    }

    /// Returns a pointer to the instance implementation specific data.
    ///
    /// This is used for the subclassing infrastructure to store additional instance data.
    fn get_instance_data<U: Any + Send + Sync + 'static>(&self, type_: Type) -> Option<&U> {
        unsafe {
            let type_data = Self::type_data();
            let self_type_ = type_data.as_ref().get_type();
            assert!(self_type_.is_valid());

            let offset = -type_data.as_ref().private_imp_offset;

            let ptr = self as *const Self as *const u8;
            let ptr = ptr.offset(offset);
            let ptr = ptr as *const PrivateStruct<Self>;
            let priv_ = &*ptr;

            match priv_.instance_data {
                None => None,
                Some(ref data) => data.get(&type_).and_then(|ptr| ptr.downcast_ref()),
            }
        }
    }
}

/// An object that is currently being initialized.
///
/// Binding crates should use traits for adding methods to this struct. Only methods explicitly safe
/// to call during `instance_init()` should be added.
pub struct InitializingObject<T: ObjectSubclass>(Borrowed<T::Type>);

impl<T: ObjectSubclass> InitializingObject<T> {
    /// Returns a reference to the object.
    ///
    /// # Safety
    ///
    /// The returned object has not been completely initialized at this point. Use of the object
    /// should be restricted to methods that are explicitly documented to be safe to call during
    /// `instance_init()`.
    pub unsafe fn as_ref(&self) -> &T::Type {
        &self.0
    }

    /// Returns a pointer to the object.
    ///
    /// # Safety
    ///
    /// The returned object has not been completely initialized at this point. Use of the object
    /// should be restricted to methods that are explicitly documented to be safe to call during
    /// `instance_init()`.
    pub unsafe fn as_ptr(&self) -> *mut T::Type {
        self.0.as_ptr() as *const T::Type as *mut T::Type
    }

    /// Sets instance specific implementation data.
    ///
    /// # Panics
    ///
    /// If the instance_data already contains a data for the specified `type_`.
    pub fn set_instance_data<U: Any + Send + Sync + 'static>(&mut self, type_: Type, data: U) {
        unsafe {
            let type_data = T::type_data();
            let self_type_ = type_data.as_ref().get_type();
            assert!(self_type_.is_valid());

            let offset = type_data.as_ref().private_offset;

            let ptr = self.0.as_ptr() as *mut u8;
            let ptr = ptr.offset(offset);
            let ptr = ptr as *mut PrivateStruct<T>;
            let priv_ = &mut *ptr;

            if priv_.instance_data.is_none() {
                priv_.instance_data = Some(HashMap::new());
            }

            if let Some(ref mut instance_data) = priv_.instance_data {
                if instance_data.get(&type_).is_some() {
                    panic!("The class_data already contains a key for {}", type_);
                }

                instance_data.insert(type_, Box::new(data));
            }
        }
    }
}

unsafe extern "C" fn class_init<T: ObjectSubclass>(
    klass: ffi::gpointer,
    _klass_data: ffi::gpointer,
) {
    let mut data = T::type_data();

    // We have to update the private struct offset once the class is actually
    // being initialized.
    let mut private_offset = data.as_ref().private_offset as i32;
    gobject_ffi::g_type_class_adjust_private_offset(klass, &mut private_offset);
    (*data.as_mut()).private_offset = private_offset as isize;

    // Set trampolines for the basic GObject virtual methods.
    {
        let gobject_klass = &mut *(klass as *mut gobject_ffi::GObjectClass);

        gobject_klass.finalize = Some(finalize::<T>);
    }

    // And finally peek the parent class struct (containing the parent class'
    // implementations of virtual methods for chaining up), and call the subclass'
    // class initialization function.
    {
        let klass = &mut *(klass as *mut T::Class);
        let parent_class = gobject_ffi::g_type_class_peek_parent(klass as *mut _ as ffi::gpointer)
            as *mut <T::ParentType as ObjectType>::GlibClassType;
        assert!(!parent_class.is_null());

        (*data.as_mut()).parent_class = parent_class as ffi::gpointer;

        klass.class_init();
        T::class_init(klass);
    }
}

unsafe extern "C" fn instance_init<T: ObjectSubclass>(
    obj: *mut gobject_ffi::GTypeInstance,
    klass: ffi::gpointer,
) {
    // Get offset to the storage of our private struct, create it
    // and actually store it in that place.
    let mut data = T::type_data();
    let private_offset = (*data.as_mut()).private_offset;
    let ptr = obj as *mut u8;
    let priv_ptr = ptr.offset(private_offset);
    let priv_storage = priv_ptr as *mut PrivateStruct<T>;

    let klass = &*(klass as *const T::Class);

    let imp = T::with_class(klass);
    ptr::write(
        priv_storage,
        PrivateStruct {
            imp,
            instance_data: None,
        },
    );

    // Any additional instance initialization.
    T::Instance::instance_init(&mut *(obj as *mut _));

    let obj = from_glib_borrow::<_, Object>(obj.cast());
    let obj = Borrowed::new(obj.into_inner().unsafe_cast());
    let mut obj = InitializingObject(obj);

    T::Interfaces::instance_init(&mut obj);
    T::instance_init(&obj);
}

unsafe extern "C" fn finalize<T: ObjectSubclass>(obj: *mut gobject_ffi::GObject) {
    // Retrieve the private struct and drop it for freeing all associated memory.
    let mut data = T::type_data();
    let private_offset = (*data.as_mut()).private_offset;
    let ptr = obj as *mut u8;
    let priv_ptr = ptr.offset(private_offset);
    let priv_storage = &mut *(priv_ptr as *mut PrivateStruct<T>);
    ptr::drop_in_place(&mut priv_storage.imp);
    if let Some(instance_data) = priv_storage.instance_data.take() {
        drop(instance_data);
    }

    // Chain up to the parent class' finalize implementation, if any.
    let parent_class = &*(data.as_ref().get_parent_class() as *const gobject_ffi::GObjectClass);
    if let Some(ref func) = parent_class.finalize {
        func(obj);
    }
}

/// Register a `glib::Type` ID for `T`.
///
/// This must be called only once and will panic on a second call.
///
/// The [`object_subclass!`] macro will create a `get_type()` function around this, which will
/// ensure that it's only ever called once.
///
/// [`object_subclass!`]: ../../macro.object_subclass.html
pub fn register_type<T: ObjectSubclass>() -> Type {
    // GLib aligns the type private data to two gsizes so we can't safely store any type there that
    // requires a bigger alignment.
    if mem::align_of::<T>() > 2 * mem::size_of::<usize>() {
        panic!(
            "Alignment {} of type not supported, bigger than {}",
            mem::align_of::<T>(),
            2 * mem::size_of::<usize>(),
        );
    }

    unsafe {
        use std::ffi::CString;

        let type_name = CString::new(T::NAME).unwrap();
        if gobject_ffi::g_type_from_name(type_name.as_ptr()) != gobject_ffi::G_TYPE_INVALID {
            panic!(
                "Type {} has already been registered",
                type_name.to_str().unwrap()
            );
        }

        let type_ = from_glib(gobject_ffi::g_type_register_static_simple(
            <T::ParentType as StaticType>::static_type().to_glib(),
            type_name.as_ptr(),
            mem::size_of::<T::Class>() as u32,
            Some(class_init::<T>),
            mem::size_of::<T::Instance>() as u32,
            Some(instance_init::<T>),
            if T::ABSTRACT {
                gobject_ffi::G_TYPE_FLAG_ABSTRACT
            } else {
                0
            },
        ));

        let mut data = T::type_data();
        (*data.as_mut()).type_ = type_;

        let private_offset = gobject_ffi::g_type_add_instance_private(
            type_.to_glib(),
            mem::size_of::<PrivateStruct<T>>(),
        );
        (*data.as_mut()).private_offset = private_offset as isize;

        // Get the offset from PrivateStruct<T> to the imp field in it. This has to go through
        // some hoops because Rust doesn't have an offsetof operator yet.
        (*data.as_mut()).private_imp_offset = {
            // Must not be a dangling pointer so let's create some uninitialized memory
            let priv_ = std::mem::MaybeUninit::<PrivateStruct<T>>::uninit();
            let ptr = priv_.as_ptr();
            // FIXME: Technically UB but we'd need std::ptr::raw_const for this
            let imp_ptr = &(*ptr).imp as *const _ as *const u8;
            let ptr = ptr as *const u8;
            imp_ptr as isize - ptr as isize
        };

        let iface_types = T::Interfaces::iface_infos();
        for (iface_type, iface_info) in iface_types {
            gobject_ffi::g_type_add_interface_static(type_.to_glib(), iface_type, &iface_info);
        }

        T::type_init(&mut InitializingType::<T>(type_, marker::PhantomData));

        type_
    }
}

pub(crate) unsafe fn signal_override_class_handler<F>(
    name: &str,
    type_: ffi::GType,
    class_handler: F,
) where
    F: Fn(&super::SignalClassHandlerToken, &[Value]) -> Option<Value> + Send + Sync + 'static,
{
    let class_handler = Closure::new(move |values| {
        let instance = gobject_ffi::g_value_get_object(values[0].to_glib_none().0);
        class_handler(&super::SignalClassHandlerToken(instance as *mut _), values)
    });

    if let Some((signal_id, _)) = SignalId::parse_name(name, from_glib(type_), false) {
        gobject_ffi::g_signal_override_class_closure(
            signal_id.to_glib(),
            type_,
            class_handler.to_glib_none().0,
        );
    } else {
        panic!("Signal '{}' not found", name);
    }
}

pub(crate) unsafe fn signal_chain_from_overridden(
    instance: *mut gobject_ffi::GTypeInstance,
    token: &super::SignalClassHandlerToken,
    values: &[Value],
) -> Option<Value> {
    assert_eq!(instance, token.0);
    let mut result = Value::uninitialized();
    gobject_ffi::g_signal_chain_from_overridden(
        values.as_ptr() as *mut Value as *mut gobject_ffi::GValue,
        result.to_glib_none_mut().0,
    );
    Some(result).filter(|r| r.type_().is_valid() && r.type_() != Type::UNIT)
}

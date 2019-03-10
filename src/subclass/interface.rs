// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use super::{InitializingType, Property};
use glib_sys;
use gobject_sys;
use std::borrow::Borrow;
use std::marker;
use std::mem;
use std::ptr;
use translate::*;
use {IsA, Object, ObjectExt, SignalFlags, StaticType, Type, Value};

impl<T: ObjectInterface> InitializingType<T> {
    /// Adds an interface prerequisite for `I` to the type.
    ///
    /// All implementors of the interface must be a subclass of `I` or implement the interface `I`.
    pub fn add_prerequisite<I: StaticType>(&mut self) {
        unsafe {
            gobject_sys::g_type_interface_add_prerequisite(
                self.0.to_glib(),
                I::static_type().to_glib(),
            )
        }
    }
}

/// Macro for boilerplate of [`ObjectInterface`] implementations.
///
/// [`ObjectInterface`]: subclass/types/trait.ObjectInterface.html
#[macro_export]
macro_rules! glib_object_interface {
    () => {
        fn get_type() -> $crate::Type {
            static ONCE: ::std::sync::Once = ::std::sync::Once::new();
            static mut TYPE: $crate::Type = $crate::Type::Invalid;

            ONCE.call_once(|| {
                let type_ = $crate::subclass::register_interface::<Self>();
                unsafe {
                    TYPE = type_;
                }
            });

            unsafe {
                assert_ne!(TYPE, $crate::Type::Invalid);

                TYPE
            }
        }
    };
}

/// The central trait for defining a `GObject` interface.
///
/// Links together the type name and the interface struct for type registration and allows to hook
/// into various steps of the type registration and initialization.
///
/// This must only be implemented on `#[repr(C)]` structs and have `gobject_sys::GTypeInterface` as
/// the first field.
///
/// See [`register_interface`] for registering an implementation of this trait
/// with the type system.
///
/// [`register_interface`]: fn.register_interface.html
pub trait ObjectInterface: Sized + 'static {
    /// `GObject` type name.
    ///
    /// This must be unique in the whole process.
    const NAME: &'static str;

    /// Returns the `glib::Type` ID of the interface.
    ///
    /// This will register the type with the type system on the first call and is usually generated
    /// by the [`glib_object_interface!`] macro.
    ///
    /// [`glib_object_interface!`]: ../../macro.glib_object_interface.html
    fn get_type() -> Type;

    /// Additional type initialization.
    ///
    /// This is called right after the type was registered and allows
    /// interfaces to do additional type-specific initialization, e.g.
    /// for adding prerequisites.
    ///
    /// Optional
    fn type_init(_type_: &mut InitializingType<Self>) {}

    /// Interface initialization.
    ///
    /// This is called after `type_init` and before the first implementor
    /// of the interface is created. Interfaces can use this to do interface-
    /// specific initialization, e.g. for installing properties or signals
    /// on the interface, and for setting default implementations of interface
    /// functions.
    ///
    /// Optional
    fn interface_init(&mut self) {}
}

pub trait ObjectInterfaceExt: ObjectInterface {
    /// Get interface from an instance.
    ///
    /// This will panic if `obj` does not implement the interface.
    fn from_instance<T: IsA<Object>>(obj: &T) -> &Self {
        assert!(obj.as_ref().get_type().is_a(&Self::get_type()));

        unsafe {
            let klass = (*(obj.as_ptr() as *const gobject_sys::GTypeInstance)).g_class;
            let interface =
                gobject_sys::g_type_interface_peek(klass as *mut _, Self::get_type().to_glib());
            assert!(!interface.is_null());
            &*(interface as *const Self)
        }
    }

    /// Install properties on the interface.
    ///
    /// All implementors of the interface must provide these properties.
    fn install_properties<'a, T: Borrow<Property<'a>>>(&mut self, properties: &[T]) {
        if properties.is_empty() {
            return;
        }

        for property in properties {
            let property = property.borrow();
            let pspec = (property.1)(property.0);
            unsafe {
                gobject_sys::g_object_interface_install_property(
                    self as *mut Self as *mut _,
                    pspec.to_glib_none().0,
                );
            }
        }
    }

    /// Add a new signal to the interface.
    ///
    /// This can be emitted later by `glib::Object::emit` and external code
    /// can connect to the signal to get notified about emissions.
    fn add_signal(&mut self, name: &str, flags: SignalFlags, arg_types: &[Type], ret_type: Type) {
        unsafe {
            super::types::add_signal(
                *(self as *mut _ as *mut glib_sys::GType),
                name,
                flags,
                arg_types,
                ret_type,
            );
        }
    }

    /// Add a new signal with class handler to the interface.
    ///
    /// This can be emitted later by `glib::Object::emit` and external code
    /// can connect to the signal to get notified about emissions.
    ///
    /// The class handler will be called during the signal emission at the corresponding stage.
    fn add_signal_with_class_handler<F>(
        &mut self,
        name: &str,
        flags: SignalFlags,
        arg_types: &[Type],
        ret_type: Type,
        class_handler: F,
    ) where
        F: Fn(&super::SignalClassHandlerToken, &[Value]) -> Option<Value> + Send + Sync + 'static,
    {
        unsafe {
            super::types::add_signal_with_class_handler(
                *(self as *mut _ as *mut glib_sys::GType),
                name,
                flags,
                arg_types,
                ret_type,
                class_handler,
            );
        }
    }

    /// Add a new signal with accumulator to the interface.
    ///
    /// This can be emitted later by `glib::Object::emit` and external code
    /// can connect to the signal to get notified about emissions.
    ///
    /// The accumulator function is used for accumulating the return values of
    /// multiple signal handlers. The new value is passed as second argument and
    /// should be combined with the old value in the first argument. If no further
    /// signal handlers should be called, `false` should be returned.
    fn add_signal_with_accumulator<F>(
        &mut self,
        name: &str,
        flags: SignalFlags,
        arg_types: &[Type],
        ret_type: Type,
        accumulator: F,
    ) where
        F: Fn(&super::SignalInvocationHint, &mut Value, &Value) -> bool + Send + Sync + 'static,
    {
        unsafe {
            super::types::add_signal_with_accumulator(
                *(self as *mut _ as *mut glib_sys::GType),
                name,
                flags,
                arg_types,
                ret_type,
                accumulator,
            );
        }
    }

    /// Add a new signal with accumulator and class handler to the interface.
    ///
    /// This can be emitted later by `glib::Object::emit` and external code
    /// can connect to the signal to get notified about emissions.
    ///
    /// The accumulator function is used for accumulating the return values of
    /// multiple signal handlers. The new value is passed as second argument and
    /// should be combined with the old value in the first argument. If no further
    /// signal handlers should be called, `false` should be returned.
    ///
    /// The class handler will be called during the signal emission at the corresponding stage.
    fn add_signal_with_class_handler_and_accumulator<F, G>(
        &mut self,
        name: &str,
        flags: SignalFlags,
        arg_types: &[Type],
        ret_type: Type,
        class_handler: F,
        accumulator: G,
    ) where
        F: Fn(&super::SignalClassHandlerToken, &[Value]) -> Option<Value> + Send + Sync + 'static,
        G: Fn(&super::SignalInvocationHint, &mut Value, &Value) -> bool + Send + Sync + 'static,
    {
        unsafe {
            super::types::add_signal_with_class_handler_and_accumulator(
                *(self as *mut _ as *mut glib_sys::GType),
                name,
                flags,
                arg_types,
                ret_type,
                class_handler,
                accumulator,
            );
        }
    }
}

impl<T: ObjectInterface> ObjectInterfaceExt for T {}

unsafe extern "C" fn interface_init<T: ObjectInterface>(
    klass: glib_sys::gpointer,
    _klass_data: glib_sys::gpointer,
) {
    let iface = &mut *(klass as *mut T);
    iface.interface_init();
}

/// Register a `glib::Type` ID for `T`.
///
/// This must be called only once and will panic on a second call.
///
/// The [`glib_object_interface!`] macro will create a `get_type()` function around this, which will
/// ensure that it's only ever called once.
///
/// [`glib_object_interface!`]: ../../macro.glib_object_interface.html
pub fn register_interface<T: ObjectInterface>() -> Type {
    unsafe {
        use std::ffi::CString;

        let type_info = gobject_sys::GTypeInfo {
            class_size: mem::size_of::<T>() as u16,
            base_init: None,
            base_finalize: None,
            class_init: Some(interface_init::<T>),
            class_finalize: None,
            class_data: ptr::null_mut(),
            instance_size: 0,
            n_preallocs: 0,
            instance_init: None,
            value_table: ptr::null(),
        };

        let type_name = CString::new(T::NAME).unwrap();
        assert_eq!(
            gobject_sys::g_type_from_name(type_name.as_ptr()),
            gobject_sys::G_TYPE_INVALID
        );

        let type_ = from_glib(gobject_sys::g_type_register_static(
            Type::BaseInterface.to_glib(),
            type_name.as_ptr(),
            &type_info,
            0,
        ));

        T::type_init(&mut InitializingType::<T>(type_, marker::PhantomData));

        type_
    }
}

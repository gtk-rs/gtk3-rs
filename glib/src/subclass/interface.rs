// Take a look at the license at the top of the repository in the LICENSE file.

use super::{InitializingType, Signal};
use crate::translate::*;
use crate::{IsA, Object, ObjectExt, ParamSpec, StaticType, Type};
use std::marker;
use std::mem;

impl<T: ObjectInterface> InitializingType<T> {
    /// Adds an interface prerequisite for `I` to the type.
    ///
    /// All implementors of the interface must be a subclass of `I` or implement the interface `I`.
    pub fn add_prerequisite<I: StaticType>(&mut self) {
        unsafe {
            gobject_ffi::g_type_interface_add_prerequisite(
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
macro_rules! object_interface {
    () => {
        fn get_type() -> $crate::Type {
            static ONCE: std::sync::Once = std::sync::Once::new();
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
/// This must only be implemented on `#[repr(C)]` structs and have `gobject_ffi::GTypeInterface` as
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
    /// by the [`object_interface!`] macro.
    ///
    /// [`object_interface!`]: ../../macro.object_interface.html
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
    /// specific initialization, e.g. for installing signals on the interface,
    /// and for setting default implementations of interface functions.
    ///
    /// Optional
    fn interface_init(&mut self) {}

    /// Properties installed for this interface.
    ///
    /// All implementors of the interface must provide these properties.
    fn properties() -> &'static [ParamSpec] {
        &[]
    }

    /// Signals installed for this interface.
    fn signals() -> &'static [Signal] {
        &[]
    }
}

pub trait ObjectInterfaceExt: ObjectInterface {
    /// Get interface from an instance.
    ///
    /// This will panic if `obj` does not implement the interface.
    fn from_instance<T: IsA<Object>>(obj: &T) -> &Self {
        assert!(obj.as_ref().get_type().is_a(&Self::get_type()));

        unsafe {
            let klass = (*(obj.as_ptr() as *const gobject_ffi::GTypeInstance)).g_class;
            let interface =
                gobject_ffi::g_type_interface_peek(klass as *mut _, Self::get_type().to_glib());
            assert!(!interface.is_null());
            &*(interface as *const Self)
        }
    }
}

impl<T: ObjectInterface> ObjectInterfaceExt for T {}

unsafe extern "C" fn interface_init<T: ObjectInterface>(
    klass: ffi::gpointer,
    _klass_data: ffi::gpointer,
) {
    let iface = &mut *(klass as *mut T);

    let pspecs = <T as ObjectInterface>::properties();
    for pspec in pspecs {
        gobject_ffi::g_object_interface_install_property(
            iface as *mut T as *mut _,
            pspec.to_glib_none().0,
        );
    }

    let type_ = T::get_type();
    let signals = <T as ObjectInterface>::signals();
    for signal in signals {
        signal.register(type_);
    }

    iface.interface_init();
}

/// Register a `glib::Type` ID for `T`.
///
/// This must be called only once and will panic on a second call.
///
/// The [`object_interface!`] macro will create a `get_type()` function around this, which will
/// ensure that it's only ever called once.
///
/// [`object_interface!`]: ../../macro.object_interface.html
pub fn register_interface<T: ObjectInterface>() -> Type {
    unsafe {
        use std::ffi::CString;

        let type_name = CString::new(T::NAME).unwrap();
        assert_eq!(
            gobject_ffi::g_type_from_name(type_name.as_ptr()),
            gobject_ffi::G_TYPE_INVALID
        );

        let type_ = from_glib(gobject_ffi::g_type_register_static_simple(
            Type::Interface.to_glib(),
            type_name.as_ptr(),
            mem::size_of::<T>() as u32,
            Some(interface_init::<T>),
            0,
            None,
            0,
        ));

        T::type_init(&mut InitializingType::<T>(type_, marker::PhantomData));

        type_
    }
}

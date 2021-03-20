// Take a look at the license at the top of the repository in the LICENSE file.

//! Module that contains all types needed for creating a direct subclass of `GObject`
//! or implementing virtual methods of it.

use super::prelude::*;
use super::Signal;
use crate::translate::*;
use crate::{Cast, Object, ObjectType, ParamSpec, Value};
use std::mem;
use std::ptr;

/// Trait for implementors of `glib::Object` subclasses.
///
/// This allows overriding the virtual methods of `glib::Object`.
pub trait ObjectImpl: ObjectSubclass + ObjectImplExt {
    /// Properties installed for this type.
    fn properties() -> &'static [ParamSpec] {
        &[]
    }

    /// Signals installed for this type.
    fn signals() -> &'static [Signal] {
        &[]
    }

    /// Property setter.
    ///
    /// This is called whenever the property of this specific subclass with the
    /// given index is set. The new value is passed as `glib::Value`.
    fn set_property(&self, _obj: &Self::Type, _id: usize, _value: &Value, _pspec: &ParamSpec) {
        unimplemented!()
    }

    /// Property getter.
    ///
    /// This is called whenever the property value of the specific subclass with the
    /// given index should be returned.
    fn get_property(&self, _obj: &Self::Type, _id: usize, _pspec: &ParamSpec) -> Value {
        unimplemented!()
    }

    /// Constructed.
    ///
    /// This is called once construction of the instance is finished.
    ///
    /// Should chain up to the parent class' implementation.
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }

    /// Disposes of the object.
    ///
    /// When `dispose()` ends, the object should not hold any reference to any other member object.
    /// The object is also expected to be able to answer client method invocations (with possibly an
    /// error code but no memory violation) until it is dropped. `dispose()` can be executed more
    /// than once.
    fn dispose(&self, _obj: &Self::Type) {}
}

unsafe extern "C" fn get_property<T: ObjectImpl>(
    obj: *mut gobject_ffi::GObject,
    id: u32,
    value: *mut gobject_ffi::GValue,
    pspec: *mut gobject_ffi::GParamSpec,
) {
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.get_impl();

    let v = imp.get_property(
        &from_glib_borrow::<_, Object>(obj).unsafe_cast_ref(),
        id as usize,
        &from_glib_borrow(pspec),
    );

    // We first unset the value we get passed in, in case it contained
    // any previous data. Then we directly overwrite it with our new
    // value, and pass ownership of the contained data to the C GValue
    // by forgetting it on the Rust side.
    //
    // Without this, by using the GValue API, we would have to create
    // a copy of the value when setting it on the destination just to
    // immediately free the original value afterwards.
    gobject_ffi::g_value_unset(value);
    let v = mem::ManuallyDrop::new(v);
    ptr::write(value, ptr::read(v.to_glib_none().0));
}

unsafe extern "C" fn set_property<T: ObjectImpl>(
    obj: *mut gobject_ffi::GObject,
    id: u32,
    value: *mut gobject_ffi::GValue,
    pspec: *mut gobject_ffi::GParamSpec,
) {
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.get_impl();
    imp.set_property(
        &from_glib_borrow::<_, Object>(obj).unsafe_cast_ref(),
        id as usize,
        &*(value as *mut Value),
        &from_glib_borrow(pspec),
    );
}

unsafe extern "C" fn constructed<T: ObjectImpl>(obj: *mut gobject_ffi::GObject) {
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.get_impl();

    imp.constructed(&from_glib_borrow::<_, Object>(obj).unsafe_cast_ref());
}

unsafe extern "C" fn dispose<T: ObjectImpl>(obj: *mut gobject_ffi::GObject) {
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.get_impl();

    imp.dispose(&from_glib_borrow::<_, Object>(obj).unsafe_cast_ref());

    // Chain up to the parent's dispose.
    let data = T::type_data();
    let parent_class = data.as_ref().get_parent_class() as *mut gobject_ffi::GObjectClass;
    if let Some(ref func) = (*parent_class).dispose {
        func(obj);
    }
}

/// Extension trait for `glib::Object`'s class struct.
///
/// This contains various class methods and allows subclasses to override signal class handlers.
pub unsafe trait ObjectClassSubclassExt: Sized + 'static {
    fn override_signal_class_handler<F>(&mut self, name: &str, class_handler: F)
    where
        F: Fn(&super::SignalClassHandlerToken, &[Value]) -> Option<Value> + Send + Sync + 'static,
    {
        unsafe {
            super::types::signal_override_class_handler(
                name,
                *(self as *mut _ as *mut ffi::GType),
                class_handler,
            );
        }
    }
}

unsafe impl ObjectClassSubclassExt for crate::Class<Object> {}

unsafe impl<T: ObjectImpl> IsSubclassable<T> for Object {
    fn class_init(class: &mut crate::Class<Self>) {
        let klass = class.as_mut();
        klass.set_property = Some(set_property::<T>);
        klass.get_property = Some(get_property::<T>);
        klass.constructed = Some(constructed::<T>);
        klass.dispose = Some(dispose::<T>);

        let pspecs = <T as ObjectImpl>::properties();
        if !pspecs.is_empty() {
            unsafe {
                let mut pspecs_ptrs = Vec::with_capacity(pspecs.len() + 1);

                pspecs_ptrs.push(ptr::null_mut());

                for pspec in pspecs {
                    pspecs_ptrs.push(pspec.to_glib_none().0);
                }

                gobject_ffi::g_object_class_install_properties(
                    klass,
                    pspecs_ptrs.len() as u32,
                    pspecs_ptrs.as_mut_ptr(),
                );
            }
        }

        let type_ = T::get_type();
        let signals = <T as ObjectImpl>::signals();
        for signal in signals {
            signal.register(type_);
        }
    }

    fn instance_init(_instance: &mut super::InitializingObject<T>) {}
}

pub trait ObjectImplExt: ObjectSubclass {
    /// Chain up to the parent class' implementation of `glib::Object::constructed()`.
    fn parent_constructed(&self, obj: &Self::Type);

    /// Chain up to parent class signal handler.
    fn signal_chain_from_overridden(
        &self,
        token: &super::SignalClassHandlerToken,
        values: &[Value],
    ) -> Option<Value>;
}

impl<T: ObjectImpl> ObjectImplExt for T {
    fn parent_constructed(&self, obj: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gobject_ffi::GObjectClass;

            if let Some(ref func) = (*parent_class).constructed {
                func(obj.unsafe_cast_ref::<Object>().to_glib_none().0);
            }
        }
    }

    fn signal_chain_from_overridden(
        &self,
        token: &super::SignalClassHandlerToken,
        values: &[Value],
    ) -> Option<Value> {
        unsafe {
            super::types::signal_chain_from_overridden(
                self.get_instance().as_ptr() as *mut _,
                token,
                values,
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::super::object::ObjectExt;
    use super::super::super::value::{ToValue, Value};
    use super::*;
    use crate as glib;
    use crate::StaticType;

    use std::cell::RefCell;

    mod imp {
        use super::*;

        // A dummy `Object` to test setting an `Object` property and returning an `Object` in signals
        #[derive(Default)]
        pub struct ChildObject;

        #[glib::object_subclass]
        impl ObjectSubclass for ChildObject {
            const NAME: &'static str = "ChildObject";
            type Type = super::ChildObject;
            type ParentType = Object;
        }

        impl ObjectImpl for ChildObject {}

        #[derive(Default)]
        pub struct SimpleObject {
            name: RefCell<Option<String>>,
            construct_name: RefCell<Option<String>>,
            constructed: RefCell<bool>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for SimpleObject {
            const NAME: &'static str = "SimpleObject";
            type Type = super::SimpleObject;
            type ParentType = Object;
            type Interfaces = (super::Dummy,);
        }

        impl ObjectImpl for SimpleObject {
            fn signals() -> &'static [super::Signal] {
                use once_cell::sync::Lazy;
                static SIGNALS: Lazy<Vec<super::Signal>> = Lazy::new(|| {
                    vec![
                        super::Signal::builder(
                            "name-changed",
                            &[String::static_type().into()],
                            crate::Type::UNIT.into(),
                        )
                        .build(),
                        super::Signal::builder(
                            "change-name",
                            &[String::static_type().into()],
                            String::static_type().into(),
                        )
                        .action()
                        .class_handler(|_, args| {
                            let obj = args[0]
                                .get::<super::SimpleObject>()
                                .expect("Failed to get args[0]")
                                .expect("Failed to get Object from args[0]");
                            let new_name = args[1]
                                .get::<String>()
                                .expect("Failed to get args[1]")
                                .expect("Failed to get Object from args[1]");
                            let imp = SimpleObject::from_instance(&obj);

                            let old_name = imp.name.borrow_mut().take();
                            *imp.name.borrow_mut() = Some(new_name);

                            obj.emit_by_name("name-changed", &[&*imp.name.borrow()])
                                .expect("Failed to borrow name");

                            Some(old_name.to_value())
                        })
                        .build(),
                        super::Signal::builder("create-string", &[], String::static_type().into())
                            .build(),
                        super::Signal::builder(
                            "create-child-object",
                            &[],
                            ChildObject::get_type().into(),
                        )
                        .build(),
                    ]
                });

                SIGNALS.as_ref()
            }

            fn properties() -> &'static [ParamSpec] {
                use once_cell::sync::Lazy;
                static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                    vec![
                        crate::ParamSpec::string(
                            "name",
                            "Name",
                            "Name of this object",
                            None,
                            crate::ParamFlags::READWRITE,
                        ),
                        crate::ParamSpec::string(
                            "construct-name",
                            "Construct Name",
                            "Construct Name of this object",
                            None,
                            crate::ParamFlags::READWRITE | crate::ParamFlags::CONSTRUCT_ONLY,
                        ),
                        crate::ParamSpec::boolean(
                            "constructed",
                            "Constructed",
                            "True if the constructed() virtual method was called",
                            false,
                            crate::ParamFlags::READABLE,
                        ),
                        crate::ParamSpec::object(
                            "child",
                            "Child",
                            "Child object",
                            super::ChildObject::static_type(),
                            crate::ParamFlags::READWRITE,
                        ),
                    ]
                });

                PROPERTIES.as_ref()
            }

            fn set_property(
                &self,
                obj: &Self::Type,
                _id: usize,
                value: &Value,
                pspec: &crate::ParamSpec,
            ) {
                match pspec.get_name() {
                    "name" => {
                        let name = value
                            .get()
                            .expect("type conformity checked by 'Object::set_property'");
                        self.name.replace(name);
                        obj.emit_by_name("name-changed", &[&*self.name.borrow()])
                            .expect("Failed to borrow name");
                    }
                    "construct-name" => {
                        let name = value
                            .get()
                            .expect("type conformity checked by 'Object::set_property'");
                        self.construct_name.replace(name);
                    }
                    "child" => {
                        // not stored, only used to test `set_property` with `Objects`
                    }
                    _ => unimplemented!(),
                }
            }

            fn get_property(
                &self,
                _obj: &Self::Type,
                _id: usize,
                pspec: &crate::ParamSpec,
            ) -> Value {
                match pspec.get_name() {
                    "name" => self.name.borrow().to_value(),
                    "construct-name" => self.construct_name.borrow().to_value(),
                    "constructed" => self.constructed.borrow().to_value(),
                    _ => unimplemented!(),
                }
            }

            fn constructed(&self, obj: &Self::Type) {
                self.parent_constructed(obj);

                assert_eq!(obj, &self.get_instance());
                assert_eq!(self as *const _, Self::from_instance(obj) as *const _);

                *self.constructed.borrow_mut() = true;
            }
        }

        #[derive(Clone, Copy)]
        #[repr(C)]
        pub struct DummyInterface {
            parent: gobject_ffi::GTypeInterface,
        }

        #[glib::object_interface]
        unsafe impl ObjectInterface for DummyInterface {
            const NAME: &'static str = "Dummy";
        }
    }

    wrapper! {
        pub struct ChildObject(ObjectSubclass<imp::ChildObject>);
    }

    wrapper! {
        pub struct SimpleObject(ObjectSubclass<imp::SimpleObject>);
    }

    wrapper! {
        pub struct Dummy(ObjectInterface<imp::DummyInterface>);
    }

    unsafe impl<T: ObjectSubclass> IsImplementable<T> for Dummy {
        fn interface_init(_iface: &mut crate::Interface<Dummy>) {}
        fn instance_init(_instance: &mut super::super::InitializingObject<T>) {}
    }

    #[test]
    fn test_create() {
        let type_ = SimpleObject::static_type();
        let obj = Object::with_type(type_, &[]).expect("Object::new failed");

        assert!(obj.get_type().is_a(Dummy::static_type()));

        assert_eq!(
            obj.get_property("constructed")
                .expect("Failed to get 'constructed' property")
                .get_some::<bool>()
                .expect("Failed to get bool from 'constructed' property"),
            true
        );

        let weak = obj.downgrade();
        drop(obj);
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn test_create_child_object() {
        let obj: ChildObject = Object::new(&[]).expect("Object::new failed");

        let imp = imp::ChildObject::from_instance(&obj);
        assert_eq!(obj, imp.get_instance());
    }

    #[test]
    fn test_set_properties() {
        let obj = Object::with_type(
            SimpleObject::static_type(),
            &[("construct-name", &"meh"), ("name", &"initial")],
        )
        .expect("Object::new failed");

        assert_eq!(
            obj.get_property("construct-name")
                .expect("Failed to get 'construct-name' property")
                .get::<&str>()
                .expect("Failed to get str from 'construct-name' property"),
            Some("meh")
        );
        assert_eq!(
            obj.set_property("construct-name", &"test")
                .err()
                .expect("Failed to set 'construct-name' property")
                .to_string(),
            "property 'construct-name' of type 'SimpleObject' is not writable",
        );
        assert_eq!(
            obj.get_property("construct-name")
                .expect("Failed to get 'construct-name' property")
                .get::<&str>()
                .expect("Failed to get str from 'construct-name' property"),
            Some("meh")
        );

        assert_eq!(
            obj.get_property("name")
                .expect("Failed to get 'name' property")
                .get::<&str>()
                .expect("Failed to get str from 'name' property"),
            Some("initial")
        );
        assert!(obj.set_property("name", &"test").is_ok());
        assert_eq!(
            obj.get_property("name")
                .expect("Failed to get 'name' property")
                .get::<&str>()
                .expect("Failed to get str from 'name' property"),
            Some("test")
        );

        assert_eq!(
            obj.set_property("test", &true)
                .err()
                .expect("set_property failed")
                .to_string(),
            "property 'test' of type 'SimpleObject' not found",
        );

        assert_eq!(
            obj.set_property("constructed", &false)
                .err()
                .expect("Failed to set 'constructed' property")
                .to_string(),
            "property 'constructed' of type 'SimpleObject' is not writable",
        );

        assert_eq!(
            obj.set_property("name", &false)
                .err()
                .expect("Failed to set 'name' property")
                .to_string(),
            "property 'name' of type 'SimpleObject' can't be set from the given type (expected: 'gchararray', got: 'gboolean')",
        );

        let other_obj =
            Object::with_type(SimpleObject::static_type(), &[]).expect("Object::new failed");
        assert_eq!(
            obj.set_property("child", &other_obj)
                .err()
                .expect("Failed to set 'child' property")
                .to_string(),
            "property 'child' of type 'SimpleObject' can't be set from the given object type (expected: 'ChildObject', got: 'SimpleObject')",
        );

        let child = Object::with_type(ChildObject::static_type(), &[]).expect("Object::new failed");
        assert!(obj.set_property("child", &child).is_ok());
    }

    #[test]
    fn test_signals() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        let type_ = SimpleObject::static_type();
        let obj = Object::with_type(type_, &[("name", &"old-name")]).expect("Object::new failed");

        let name_changed_triggered = Arc::new(AtomicBool::new(false));
        let name_changed_clone = name_changed_triggered.clone();
        obj.connect("name-changed", false, move |args| {
            let _obj = args[0]
                .get::<Object>()
                .expect("Failed to get args[0]")
                .expect("Failed to get str from args[0]");
            let name = args[1]
                .get::<&str>()
                .expect("Failed to get args[1]")
                .expect("Failed to get str from args[1]");

            assert_eq!(name, "new-name");
            name_changed_clone.store(true, Ordering::Relaxed);

            None
        })
        .expect("Failed to connect on 'name-changed'");

        assert_eq!(
            obj.get_property("name")
                .expect("Failed to get 'name' property")
                .get::<&str>()
                .expect("Failed to get str from 'name' property"),
            Some("old-name")
        );
        assert!(!name_changed_triggered.load(Ordering::Relaxed));

        let old_name = obj
            .emit_by_name("change-name", &[&"new-name"])
            .expect("Failed to emit")
            .expect("Failed to get value from emit")
            .get::<String>()
            .expect("Failed to get str from emit");
        assert_eq!(old_name, Some("old-name".to_string()));
        assert!(name_changed_triggered.load(Ordering::Relaxed));
    }

    #[test]
    fn test_signal_return_expected_type() {
        let obj = Object::with_type(SimpleObject::static_type(), &[]).expect("Object::new failed");

        obj.connect("create-string", false, move |_args| {
            Some("return value".to_value())
        })
        .expect("Failed to connect on 'create-string'");

        let signal_id = imp::SimpleObject::signals()[2].signal_id();

        let value = obj
            .emit(signal_id, &[])
            .expect("Failed to emit")
            .expect("Failed to get value from emit");
        assert_eq!(value.get::<String>(), Ok(Some("return value".to_string())));
    }

    #[test]
    fn test_callback_validity() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        let type_ = SimpleObject::static_type();
        let obj = Object::with_type(type_, &[("name", &"old-name")]).expect("Object::new failed");

        let name_changed_triggered = Arc::new(AtomicBool::new(false));
        let name_changed_clone = name_changed_triggered.clone();

        obj.connect_notify(Some("name"), move |_, _| {
            name_changed_clone.store(true, Ordering::Relaxed);
        });
        obj.notify("name");
        assert!(name_changed_triggered.load(Ordering::Relaxed));
    }

    // Note: can't test type mismatch in signals since panics accross FFI boundaries
    // are UB. See https://github.com/gtk-rs/glib/issues/518

    #[test]
    fn test_signal_return_expected_object_type() {
        let obj = Object::with_type(SimpleObject::static_type(), &[]).expect("Object::new failed");

        obj.connect("create-child-object", false, move |_args| {
            Some(
                Object::with_type(ChildObject::static_type(), &[])
                    .expect("Object::new failed")
                    .to_value(),
            )
        })
        .expect("Failed to connect on 'create-child-object'");

        let value = obj
            .emit_by_name("create-child-object", &[])
            .expect("Failed to emit")
            .expect("Failed to get value from emit");
        assert!(value.type_().is_a(ChildObject::static_type()));
    }
}

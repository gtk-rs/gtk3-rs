// Take a look at the license at the top of the repository in the LICENSE file.

//! Module that contains all types needed for creating a direct subclass of `GObject`
//! or implementing virtual methods of it.

use super::prelude::*;
use crate::translate::*;
use crate::{Cast, Object, ObjectType, SignalFlags, Type, Value};
use std::borrow::Borrow;
use std::fmt;
use std::mem;
use std::ptr;

/// Trait for implementors of `glib::Object` subclasses.
///
/// This allows overriding the virtual methods of `glib::Object`.
pub trait ObjectImpl: ObjectSubclass + ObjectImplExt {
    /// Property setter.
    ///
    /// This is called whenever the property of this specific subclass with the
    /// given index is set. The new value is passed as `glib::Value`.
    fn set_property(&self, _obj: &Self::Type, _id: usize, _value: &Value) {
        unimplemented!()
    }

    /// Property getter.
    ///
    /// This is called whenever the property value of the specific subclass with the
    /// given index should be returned.
    fn get_property(&self, _obj: &Self::Type, _id: usize) -> Value {
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
    _pspec: *mut gobject_ffi::GParamSpec,
) {
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.get_impl();

    let v = imp.get_property(
        &from_glib_borrow::<_, Object>(obj).unsafe_cast_ref(),
        (id - 1) as usize,
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
    _pspec: *mut gobject_ffi::GParamSpec,
) {
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.get_impl();
    imp.set_property(
        &from_glib_borrow::<_, Object>(obj).unsafe_cast_ref(),
        (id - 1) as usize,
        &*(value as *mut Value),
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

/// Definition of a property.
#[derive(Clone)]
pub struct Property<'a>(pub &'a str, pub fn(&str) -> crate::ParamSpec);

impl<'a> fmt::Debug for Property<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_tuple("Property").field(&self.0).finish()
    }
}

/// Extension trait for `glib::Object`'s class struct.
///
/// This contains various class methods and allows subclasses to override the virtual methods.
pub unsafe trait ObjectClassSubclassExt: Sized + 'static {
    /// Install properties on the subclass.
    ///
    /// The index in the properties array is going to be the index passed to the
    /// property setters and getters.
    fn install_properties<'a, T: Borrow<Property<'a>>>(&mut self, properties: &[T]) {
        if properties.is_empty() {
            return;
        }

        let mut pspecs = Vec::with_capacity(properties.len());

        for property in properties {
            let property = property.borrow();
            let pspec = (property.1)(property.0);
            pspecs.push(pspec);
        }

        unsafe {
            let mut pspecs_ptrs = Vec::with_capacity(properties.len());

            pspecs_ptrs.push(ptr::null_mut());

            for pspec in &pspecs {
                pspecs_ptrs.push(pspec.to_glib_none().0);
            }

            gobject_ffi::g_object_class_install_properties(
                self as *mut _ as *mut gobject_ffi::GObjectClass,
                pspecs_ptrs.len() as u32,
                pspecs_ptrs.as_mut_ptr(),
            );
        }
    }

    /// Add a new signal to the subclass.
    ///
    /// This can be emitted later by `glib::Object::emit` and external code
    /// can connect to the signal to get notified about emissions.
    fn add_signal(&mut self, name: &str, flags: SignalFlags, arg_types: &[Type], ret_type: Type) {
        unsafe {
            super::types::add_signal(
                *(self as *mut _ as *mut ffi::GType),
                name,
                flags,
                arg_types,
                ret_type,
            );
        }
    }

    /// Add a new signal with class handler to the subclass.
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
                *(self as *mut _ as *mut ffi::GType),
                name,
                flags,
                arg_types,
                ret_type,
                class_handler,
            );
        }
    }

    /// Add a new signal with accumulator to the subclass.
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
                *(self as *mut _ as *mut ffi::GType),
                name,
                flags,
                arg_types,
                ret_type,
                accumulator,
            );
        }
    }

    /// Add a new signal with accumulator and class handler to the subclass.
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
                *(self as *mut _ as *mut ffi::GType),
                name,
                flags,
                arg_types,
                ret_type,
                class_handler,
                accumulator,
            );
        }
    }

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
    fn override_vfuncs(class: &mut crate::Class<Self>) {
        let klass = class.as_mut();
        klass.set_property = Some(set_property::<T>);
        klass.get_property = Some(get_property::<T>);
        klass.constructed = Some(constructed::<T>);
        klass.dispose = Some(dispose::<T>);
    }
}

pub trait ObjectImplExt: ObjectSubclass {
    /// Chain up to the parent class' implementation of `glib::Object::constructed()`.
    fn parent_constructed(&self, obj: &Self::Type);

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
    use super::super::super::subclass;
    use super::super::super::value::{ToValue, Value};
    use super::*;
    use crate::prelude::*;

    use std::cell::RefCell;

    mod imp {
        use super::*;

        // A dummy `Object` to test setting an `Object` property and returning an `Object` in signals
        pub struct ChildObject;
        impl ObjectSubclass for ChildObject {
            const NAME: &'static str = "ChildObject";
            type Type = super::ChildObject;
            type ParentType = Object;
            type Instance = subclass::simple::InstanceStruct<Self>;
            type Class = subclass::simple::ClassStruct<Self>;

            object_subclass!();

            fn new() -> Self {
                ChildObject
            }
        }

        impl ObjectImpl for ChildObject {}

        pub struct SimpleObject {
            name: RefCell<Option<String>>,
            construct_name: RefCell<Option<String>>,
            constructed: RefCell<bool>,
        }

        impl ObjectSubclass for SimpleObject {
            const NAME: &'static str = "SimpleObject";
            type Type = super::SimpleObject;
            type ParentType = Object;
            type Instance = subclass::simple::InstanceStruct<Self>;
            type Class = subclass::simple::ClassStruct<Self>;

            object_subclass!();

            fn type_init(type_: &mut subclass::InitializingType<Self>) {
                type_.add_interface::<DummyInterface>();
            }

            fn class_init(klass: &mut Self::Class) {
                klass.install_properties(&PROPERTIES);

                klass.add_signal(
                    "name-changed",
                    SignalFlags::RUN_LAST,
                    &[String::static_type()],
                    crate::Type::Unit,
                );

                klass.add_signal_with_class_handler(
                    "change-name",
                    SignalFlags::RUN_LAST | SignalFlags::ACTION,
                    &[String::static_type()],
                    String::static_type(),
                    |_, args| {
                        let obj = args[0]
                            .get::<Self::Type>()
                            .expect("Failed to get args[0]")
                            .expect("Failed to get Object from args[0]");
                        let new_name = args[1]
                            .get::<String>()
                            .expect("Failed to get args[1]")
                            .expect("Failed to get Object from args[1]");
                        let imp = Self::from_instance(&obj);

                        let old_name = imp.name.borrow_mut().take();
                        *imp.name.borrow_mut() = Some(new_name);

                        obj.emit("name-changed", &[&*imp.name.borrow()])
                            .expect("Failed to borrow name");

                        Some(old_name.to_value())
                    },
                );

                klass.add_signal(
                    "create-string",
                    SignalFlags::RUN_LAST,
                    &[],
                    String::static_type(),
                );

                klass.add_signal(
                    "create-child-object",
                    SignalFlags::RUN_LAST,
                    &[],
                    ChildObject::get_type(),
                );
            }

            fn new() -> Self {
                Self {
                    name: RefCell::new(None),
                    construct_name: RefCell::new(None),
                    constructed: RefCell::new(false),
                }
            }
        }

        impl ObjectImpl for SimpleObject {
            fn set_property(&self, obj: &Self::Type, id: usize, value: &Value) {
                let prop = &PROPERTIES[id];

                match *prop {
                    Property("name", ..) => {
                        let name = value
                            .get()
                            .expect("type conformity checked by 'Object::set_property'");
                        self.name.replace(name);
                        obj.emit("name-changed", &[&*self.name.borrow()])
                            .expect("Failed to borrow name");
                    }
                    Property("construct-name", ..) => {
                        let name = value
                            .get()
                            .expect("type conformity checked by 'Object::set_property'");
                        self.construct_name.replace(name);
                    }
                    Property("child", ..) => {
                        // not stored, only used to test `set_property` with `Objects`
                    }
                    _ => unimplemented!(),
                }
            }

            fn get_property(&self, _obj: &Self::Type, id: usize) -> Value {
                let prop = &PROPERTIES[id];

                match *prop {
                    Property("name", ..) => self.name.borrow().to_value(),
                    Property("construct-name", ..) => self.construct_name.borrow().to_value(),
                    Property("constructed", ..) => self.constructed.borrow().to_value(),
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
    }

    wrapper! {
        pub struct ChildObject(ObjectSubclass<imp::ChildObject>);
    }

    static PROPERTIES: [Property; 4] = [
        Property("name", |name| {
            crate::ParamSpec::string(
                name,
                "Name",
                "Name of this object",
                None,
                crate::ParamFlags::READWRITE,
            )
        }),
        Property("construct-name", |name| {
            crate::ParamSpec::string(
                name,
                "Construct Name",
                "Construct Name of this object",
                None,
                crate::ParamFlags::READWRITE | crate::ParamFlags::CONSTRUCT_ONLY,
            )
        }),
        Property("constructed", |name| {
            crate::ParamSpec::boolean(
                name,
                "Constructed",
                "True if the constructed() virtual method was called",
                false,
                crate::ParamFlags::READABLE,
            )
        }),
        Property("child", |name| {
            crate::ParamSpec::object(
                name,
                "Child",
                "Child object",
                ChildObject::static_type(),
                crate::ParamFlags::READWRITE,
            )
        }),
    ];

    wrapper! {
        pub struct SimpleObject(ObjectSubclass<imp::SimpleObject>);
    }

    #[repr(C)]
    pub struct DummyInterface {
        parent: gobject_ffi::GTypeInterface,
    }

    impl ObjectInterface for DummyInterface {
        const NAME: &'static str = "DummyInterface";

        object_interface!();

        fn type_init(type_: &mut subclass::InitializingType<Self>) {
            type_.add_prerequisite::<Object>();
        }
    }

    // Usually this would be implemented on a Rust wrapper type defined
    // with wrapper!() but for the test the following is susyscient
    impl StaticType for DummyInterface {
        fn static_type() -> Type {
            DummyInterface::get_type()
        }
    }

    // Usually this would be implemented on a Rust wrapper type defined
    // with wrapper!() but for the test the following is susyscient
    unsafe impl<T: ObjectImpl> IsImplementable<T> for DummyInterface {
        unsafe extern "C" fn interface_init(_iface: ffi::gpointer, _iface_data: ffi::gpointer) {}
    }

    #[test]
    fn test_create() {
        let type_ = SimpleObject::static_type();
        let obj = Object::with_type(type_, &[]).expect("Object::new failed");

        assert!(obj.get_type().is_a(&DummyInterface::static_type()));

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

        // ChildObject is a zero-sized type and we map that to the same pointer as the object
        // itself. No private/impl data is allocated for zero-sized types.
        let imp = imp::ChildObject::from_instance(&obj);
        assert_eq!(imp as *const _ as *const (), obj.as_ptr() as *const _);
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
            .emit("change-name", &[&"new-name"])
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

        let value = obj
            .emit("create-string", &[])
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
            .emit("create-child-object", &[])
            .expect("Failed to emit")
            .expect("Failed to get value from emit");
        assert!(value.type_().is_a(&ChildObject::static_type()));
    }
}

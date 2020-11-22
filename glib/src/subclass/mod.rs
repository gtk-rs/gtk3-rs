// Copyright 2017-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

#![allow(clippy::needless_doctest_main)]

//! Module containing infrastructure for subclassing `GObject`s and registering boxed types.
//!
//! # Example for registering a `glib::Object` subclass
//!
//! The following code implements a subclass of `glib::Object` with a
//! string-typed "name" property.
//!
//! ```rust
//! use glib::prelude::*;
//! use glib::subclass;
//! use glib::subclass::prelude::*;
//!
//! use std::cell::{Cell, RefCell};
//!
//! #[derive(Debug, Eq, PartialEq, Clone, Copy, glib::GEnum)]
//! #[repr(u32)]
//! // type_name: GType name of the GEnum (mandatory)
//! #[genum(type_name = "SimpleObjectAnimal")]
//! enum Animal {
//!     Goat = 0,
//!     #[genum(name = "The Dog")]
//!     Dog = 1,
//!     // name: the name of the GEnumValue (optional, default to the enum name in CamelCase
//!     // nick: the nick of the GEnumValue (optional, default to the enum name in kebab-case
//!     #[genum(name = "The Cat", nick = "chat")]
//!     Cat = 2,
//! }
//!
//! impl Default for Animal {
//!     fn default() -> Self {
//!         Animal::Goat
//!     }
//! }
//!
//! #[glib::gflags("MyFlags")]
//! enum MyFlags {
//!     #[glib::gflags(name = "Flag A", nick = "nick-a")]
//!     A = 0b00000001,
//!     #[glib::gflags(name = "Flag B")]
//!     B = 0b00000010,
//!     #[glib::gflags(skip)]
//!     AB = Self::A.bits() | Self::B.bits(),
//!     C = 0b00000100,
//! }
//!
//! impl Default for MyFlags {
//!     fn default() -> Self {
//!         MyFlags::A
//!     }
//! }
//!
//! mod imp {
//!     use super::*;
//!
//!     // Static array for defining the properties of the new type.
//!     static PROPERTIES: [subclass::Property; 3] = [
//!         subclass::Property("name", |name| {
//!             glib::ParamSpec::string(
//!                 name,
//!                 "Name",
//!                 "Name of this object",
//!                 None,
//!                 glib::ParamFlags::READWRITE,
//!             )
//!         }),
//!         subclass::Property("animal", |name| {
//!             glib::ParamSpec::enum_(
//!                 name,
//!                 "Animal",
//!                 "Animal",
//!                 Animal::static_type(),
//!                 Animal::default() as i32,
//!                 glib::ParamFlags::READWRITE,
//!             )
//!         }),
//!         subclass::Property("flags", |name| {
//!             glib::ParamSpec::flags(
//!                 name,
//!                 "Flags",
//!                 "Flags",
//!                 MyFlags::static_type(),
//!                 MyFlags::default().bits(),
//!                 glib::ParamFlags::READWRITE,
//!             )
//!         }),
//!     ];
//!
//!     // This is the struct containing all state carried with
//!     // the new type. Generally this has to make use of
//!     // interior mutability.
//!     pub struct SimpleObject {
//!         name: RefCell<Option<String>>,
//!         animal: Cell<Animal>,
//!         flags: Cell<MyFlags>,
//!     }
//!
//!     // ObjectSubclass is the trait that defines the new type and
//!     // contains all information needed by the GObject type system,
//!     // including the new type's name, parent type, etc.
//!     impl ObjectSubclass for SimpleObject {
//!         // This type name must be unique per process.
//!         const NAME: &'static str = "SimpleObject";
//!
//!         // The parent type this one is inheriting from.
//!         type Type = super::SimpleObject;
//!         type ParentType = glib::Object;
//!
//!         // The C/FFI instance and class structs. The simple ones
//!         // are enough in most cases and more is only needed to
//!         // expose public instance fields to C APIs or to provide
//!         // new virtual methods for subclasses of this type.
//!         type Instance = subclass::simple::InstanceStruct<Self>;
//!         type Class = subclass::simple::ClassStruct<Self>;
//!
//!         // This macro defines some boilerplate.
//!         glib::glib_object_subclass!();
//!
//!         // Called right before the first time an instance of the new
//!         // type is created. Here class specific settings can be performed,
//!         // including installation of properties and registration of signals
//!         // for the new type.
//!         fn class_init(klass: &mut subclass::simple::ClassStruct<Self>) {
//!             klass.install_properties(&PROPERTIES);
//!         }
//!
//!         // Called every time a new instance is created. This should return
//!         // a new instance of our type with its basic values.
//!         fn new() -> Self {
//!             Self {
//!                 name: RefCell::new(None),
//!                 animal: Cell::new(Animal::default()),
//!                 flags: Cell::new(MyFlags::default()),
//!             }
//!         }
//!     }
//!
//!     // Trait that is used to override virtual methods of glib::Object.
//!     impl ObjectImpl for SimpleObject {
//!         // Called whenever a property is set on this instance. The id
//!         // is the same as the index of the property in the PROPERTIES array.
//!         fn set_property(&self, _obj: &Self::Type, id: usize, value: &glib::Value) {
//!             let prop = &PROPERTIES[id];
//!
//!             match *prop {
//!                 subclass::Property("name", ..) => {
//!                 let name = value
//!                         .get()
//!                         .expect("type conformity checked by `Object::set_property`");
//!                     self.name.replace(name);
//!                 },
//!                 subclass::Property("animal", ..) => {
//!                     let animal = value
//!                         .get()
//!                         .expect("type conformity checked by `Object::set_property`");
//!                     self.animal.replace(animal.unwrap());
//!                 },
//!                 subclass::Property("flags", ..) => {
//!                     let flags = value
//!                         .get()
//!                         .expect("type conformity checked by `Object::set_property`");
//!                     self.flags.replace(flags.unwrap());
//!                 },
//!                 _ => unimplemented!(),
//!             }
//!         }
//!
//!         // Called whenever a property is retrieved from this instance. The id
//!         // is the same as the index of the property in the PROPERTIES array.
//!         fn get_property(&self, _obj: &Self::Type, id: usize) -> glib::Value {
//!             let prop = &PROPERTIES[id];
//!
//!             match *prop {
//!                 subclass::Property("name", ..) => self.name.borrow().to_value(),
//!                 subclass::Property("animal", ..) => self.animal.get().to_value(),
//!                 subclass::Property("flags", ..) => self.flags.get().to_value(),
//!                 _ => unimplemented!(),
//!             }
//!         }
//!
//!         // Called right after construction of the instance.
//!         fn constructed(&self, obj: &Self::Type) {
//!                 // Chain up to the parent type's implementation of this virtual
//!             // method.
//!             self.parent_constructed(obj);
//!
//!             // And here we could do our own initialization.
//!         }
//!     }
//! }
//!
//! // Optionally, define a wrapper type to make it more ergonomic to use from Rust
//! glib::glib_wrapper! {
//!     pub struct SimpleObject(ObjectSubclass<imp::SimpleObject>);
//! }
//!
//! impl SimpleObject {
//!     // Create an object instance of the new type.
//!     pub fn new() -> Self {
//!         glib::Object::new(Self::static_type(), &[])
//!             .unwrap()
//!             .downcast()
//!             .unwrap()
//!     }
//! }
//!
//! pub fn main() {
//!     let obj = SimpleObject::new();
//!
//!     // Get the name property and change its value.
//!     assert_eq!(obj.get_property("name").unwrap().get::<&str>(), Ok(None));
//!     obj.set_property("name", &"test").unwrap();
//!     assert_eq!(
//!         obj.get_property("name").unwrap().get::<&str>(),
//!         Ok(Some("test"))
//!     );
//!
//!     assert_eq!(obj.get_property("animal").unwrap().get::<Animal>(), Ok(Some(Animal::Goat)));
//!     obj.set_property("animal", &Animal::Cat).unwrap();
//!     assert_eq!(obj.get_property("animal").unwrap().get::<Animal>(), Ok(Some(Animal::Cat)));
//!
//!     assert_eq!(obj.get_property("flags").unwrap().get::<MyFlags>(), Ok(Some(MyFlags::A)));
//!     obj.set_property("flags", &MyFlags::B).unwrap();
//!     assert_eq!(obj.get_property("flags").unwrap().get::<MyFlags>(), Ok(Some(MyFlags::B)));
//! }
//! ```
//!
//! # Example for registering a boxed type for a Rust struct
//!
//! The following code boxed type for a tuple struct around `String` and uses it in combination
//! with `glib::Value`.
//!
//! ```rust
//! use glib::prelude::*;
//! use glib::subclass;
//! use glib::subclass::prelude::*;
//!
//! #[derive(Clone, Debug, PartialEq, Eq, glib::GBoxed)]
//! #[gboxed(type_name = "MyBoxed")]
//! struct MyBoxed(String);
//!
//! pub fn main() {
//!     assert_ne!(glib::Type::Invalid, MyBoxed::get_type());
//!
//!     let b = MyBoxed(String::from("abc"));
//!     let v = b.to_value();
//!     let b2 = v.get::<&MyBoxed>().unwrap().unwrap();
//!     assert_eq!(&b, b2);
//! }
//! ```

pub mod simple;
#[macro_use]
pub mod types;

#[macro_use]
pub mod interface;

#[macro_use]
pub mod object;

#[macro_use]
pub mod boxed;

pub mod prelude {
    //! Prelude that re-exports all important traits from this crate.
    pub use super::boxed::BoxedType;
    pub use super::interface::{ObjectInterface, ObjectInterfaceExt};
    pub use super::object::{ObjectClassSubclassExt, ObjectImpl, ObjectImplExt};
    pub use super::types::{
        ClassStruct, InstanceStruct, IsImplementable, IsSubclassable, ObjectSubclass,
    };
}

pub use self::boxed::register_boxed_type;
pub use self::interface::register_interface;
pub use self::object::Property;
pub use self::types::{
    register_type, InitializingType, SignalClassHandlerToken, SignalInvocationHint, TypeData,
};

// Take a look at the license at the top of the repository in the LICENSE file.

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
//!     #[glib::object_subclass]
//!     impl ObjectSubclass for SimpleObject {
//!         // This type name must be unique per process.
//!         const NAME: &'static str = "SimpleObject";
//!
//!         // The parent type this one is inheriting from.
//!         type Type = super::SimpleObject;
//!         type ParentType = glib::Object;
//!
//!         // Interfaces this type implements
//!         type Interfaces = ();
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
//!         // Called once in the very beginning to list all properties of this class.
//!         fn properties() -> &'static [glib::ParamSpec] {
//!             use once_cell::sync::Lazy;
//!             static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
//!                 vec![
//!                     glib::ParamSpec::string(
//!                         "name",
//!                         "Name",
//!                         "Name of this object",
//!                         None,
//!                         glib::ParamFlags::READWRITE,
//!                     ),
//!                     glib::ParamSpec::enum_(
//!                         "animal",
//!                         "Animal",
//!                         "Animal",
//!                         Animal::static_type(),
//!                         Animal::default() as i32,
//!                         glib::ParamFlags::READWRITE,
//!                     ),
//!                     glib::ParamSpec::flags(
//!                         "flags",
//!                         "Flags",
//!                         "Flags",
//!                         MyFlags::static_type(),
//!                         MyFlags::default().bits(),
//!                         glib::ParamFlags::READWRITE,
//!                     ),
//!                 ]
//!             });
//!
//!             PROPERTIES.as_ref()
//!         }
//!
//!         // Called whenever a property is set on this instance. The id
//!         // is the same as the index of the property in the PROPERTIES array.
//!         fn set_property(&self, _obj: &Self::Type, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
//!             match pspec.get_name() {
//!                 "name" => {
//!                     let name = value
//!                         .get()
//!                         .expect("type conformity checked by `Object::set_property`");
//!                     self.name.replace(name);
//!                 },
//!                 "animal" => {
//!                     let animal = value
//!                         .get()
//!                         .expect("type conformity checked by `Object::set_property`");
//!                     self.animal.replace(animal.unwrap());
//!                 },
//!                 "flags" => {
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
//!         fn get_property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
//!             match pspec.get_name() {
//!                 "name" => self.name.borrow().to_value(),
//!                 "animal" => self.animal.get().to_value(),
//!                 "flags" => self.flags.get().to_value(),
//!                 _ => unimplemented!(),
//!             }
//!         }
//!
//!         // Called right after construction of the instance.
//!         fn constructed(&self, obj: &Self::Type) {
//!             // Chain up to the parent type's implementation of this virtual
//!             // method.
//!             self.parent_constructed(obj);
//!
//!             // And here we could do our own initialization.
//!         }
//!     }
//! }
//!
//! // Optionally, define a wrapper type to make it more ergonomic to use from Rust
//! glib::wrapper! {
//!     pub struct SimpleObject(ObjectSubclass<imp::SimpleObject>);
//! }
//!
//! impl SimpleObject {
//!     // Create an object instance of the new type.
//!     pub fn new() -> Self {
//!         glib::Object::new(&[]).unwrap()
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
//!     assert!(MyBoxed::get_type().is_valid());
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

pub mod signal;

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
pub use self::signal::{
    Signal, SignalClassHandlerToken, SignalId, SignalInvocationHint, SignalQuery, SignalType,
};
pub use self::types::{register_type, InitializingObject, InitializingType, TypeData};

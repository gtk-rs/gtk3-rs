// Copyright 2017-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Module containing infrastructure for subclassing `GObject`s.
//!
//! # Example
//!
//! The following code implements a subclass of `glib::Object` with a
//! string-typed "name" property.
//!
//! ```rust
//! #[macro_use]
//! extern crate glib;
//! use glib::prelude::*;
//! use glib::subclass;
//! use glib::subclass::prelude::*;
//!
//! use std::cell::RefCell;
//!
//! // Static array for defining the properties of the new type.
//! static PROPERTIES: [subclass::Property; 1] = [subclass::Property("name", || {
//!     glib::ParamSpec::string(
//!         "name",
//!         "Name",
//!         "Name of this object",
//!         None,
//!         glib::ParamFlags::READWRITE,
//!     )
//! })];
//!
//! // This is the struct containing all state carried with
//! // the new type. Generally this has to make use of
//! // interior mutability.
//! pub struct SimpleObject {
//!     name: RefCell<Option<String>>,
//! }
//!
//! // impl block for the new type.
//! impl SimpleObject {
//!     // This macro defines a
//!     //   fn get_type() -> glib::Type
//!     // that registers the new type with the type system
//!     // on the first call and returns its type id.
//!     glib_object_get_type!();
//! }
//!
//! // ObjectSubclass is the trait that defines the new type and
//! // contains all information needed by the GObject type system,
//! // including the new type's name, parent type, etc.
//! impl ObjectSubclass for SimpleObject {
//!     // This type name must be unique per process.
//!     const NAME: &'static str = "SimpleObject";
//!
//!     // The parent type this one is inheriting from.
//!     type ParentType = glib::Object;
//!
//!     // The C/FFI instance and class structs. The simple ones
//!     // are enough in most cases and more is only needed to
//!     // expose public instance fields to C APIs or to provide
//!     // new virtual methods for subclasses of this type.
//!     type Instance = subclass::simple::InstanceStruct<Self>;
//!     type Class = subclass::simple::ClassStruct<Self>;
//!
//!     // This macro defines some boilerplate.
//!     glib_object_subclass!();
//!
//!     // Called right before the first time an instance of the new
//!     // type is created. Here class specific settings can be performed,
//!     // including installation of properties and registration of signals
//!     // for the new type.
//!     fn class_init(klass: &mut subclass::simple::ClassStruct<Self>) {
//!         klass.install_properties(&PROPERTIES);
//!     }
//!
//!     // Called every time a new instance is created. This should return
//!     // a new instance of our type with its basic values.
//!     fn new() -> Self {
//!         Self {
//!             name: RefCell::new(None),
//!         }
//!     }
//! }
//!
//! // Trait that is used to override virtual methods of glib::Object.
//! impl ObjectImpl for SimpleObject {
//!     // This macro defines some boilerplate.
//!     glib_object_impl!();
//!
//!     // Called whenever a property is set on this instance. The id
//!     // is the same as the index of the property in the PROPERTIES array.
//!     fn set_property(&self, _obj: &glib::Object, id: usize, value: &glib::Value) {
//!         let prop = &PROPERTIES[id];
//!
//!         match *prop {
//!             subclass::Property("name", ..) => {
//!                 let name = value.get();
//!                 self.name.replace(name);
//!             }
//!             _ => unimplemented!(),
//!         }
//!     }
//!
//!     // Called whenever a property is retrieved from this instance. The id
//!     // is the same as the index of the property in the PROPERTIES array.
//!     fn get_property(&self, _obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
//!         let prop = &PROPERTIES[id];
//!
//!         match *prop {
//!             subclass::Property("name", ..) => Ok(self.name.borrow().to_value()),
//!             _ => unimplemented!(),
//!         }
//!     }
//!
//!     // Called right after construction of the instance.
//!     fn constructed(&self, obj: &glib::Object) {
//!         // Chain up to the parent type's implementation of this virtual
//!         // method.
//!         self.parent_constructed(obj);
//!
//!         // And here we could do our own initialization.
//!     }
//! }
//!
//! pub fn main() {
//!     // Create an object instance of the new type.
//!     let obj = glib::Object::new(SimpleObject::get_type(), &[]).unwrap();
//!
//!     // Get the name property and change its value.
//!     assert_eq!(obj.get_property("name").unwrap().get::<&str>(), None);
//!     obj.set_property("name", &"test").unwrap();
//!     assert_eq!(
//!         obj.get_property("name").unwrap().get::<&str>(),
//!         Some("test")
//!     );
//! }
//! ```

#[macro_use]
#[doc(hidden)]
pub mod guard;

pub mod simple;
#[macro_use]
pub mod types;

#[macro_use]
pub mod object;

pub mod prelude {
    //! Prelude that re-exports all important traits from this crate.
    pub use super::object::{ObjectClassSubclassExt, ObjectImpl};
    pub use super::types::{ClassStruct, InstanceStruct, IsSubclassable, ObjectSubclass};
}

pub use self::object::Property;
pub use self::types::{register_type, InitializingType, TypeData};

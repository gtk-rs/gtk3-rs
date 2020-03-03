// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ::glib_macros::{GBoxed, GEnum};
use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::{FromGlib, ToGlib};

#[test]
fn derive_genum() {
    #[derive(Debug, Eq, PartialEq, Clone, Copy, GEnum)]
    #[repr(u32)]
    #[genum(type_name = "TestAnimalType")]
    enum Animal {
        Goat,
        #[genum(name = "The Dog")]
        Dog,
        #[genum(name = "The Cat", nick = "chat")]
        Cat = 5,
        Badger,
    }

    assert_eq!(Animal::Goat.to_glib(), 0);
    assert_eq!(Animal::Dog.to_glib(), 1);
    assert_eq!(Animal::Cat.to_glib(), 5);

    assert_eq!(Animal::from_glib(0), Animal::Goat);
    assert_eq!(Animal::from_glib(1), Animal::Dog);
    assert_eq!(Animal::from_glib(5), Animal::Cat);

    assert_eq!(
        Animal::Goat.to_value().get::<Animal>(),
        Ok(Some(Animal::Goat))
    );
    assert_eq!(
        Animal::Dog.to_value().get::<Animal>(),
        Ok(Some(Animal::Dog))
    );
    assert_eq!(
        Animal::Cat.to_value().get::<Animal>(),
        Ok(Some(Animal::Cat))
    );

    let t = Animal::static_type();
    assert!(t.is_a(&glib::Type::BaseEnum));
    assert_eq!(t.name(), "TestAnimalType");

    let e = glib::EnumClass::new(t).expect("EnumClass::new failed");
    let v = e.get_value(0).expect("EnumClass::get_value(0) failed");
    assert_eq!(v.get_name(), "Goat");
    assert_eq!(v.get_nick(), "goat");
    let v = e.get_value(1).expect("EnumClass::get_value(1) failed");
    assert_eq!(v.get_name(), "The Dog");
    assert_eq!(v.get_nick(), "dog");
    let v = e.get_value(5).expect("EnumClass::get_value(5) failed");
    assert_eq!(v.get_name(), "The Cat");
    assert_eq!(v.get_nick(), "chat");
    assert_eq!(e.get_value(2), None);
}

#[test]
fn derive_gboxed() {
    #[derive(Clone, Debug, PartialEq, Eq, GBoxed)]
    #[gboxed(type_name = "MyBoxed")]
    struct MyBoxed(String);

    assert_eq!(MyBoxed::get_type().name(), "MyBoxed");

    let b = MyBoxed(String::from("abc"));
    let v = b.to_value();
    assert_eq!(&b, v.get::<&MyBoxed>().unwrap().unwrap());
    assert_eq!(&b, v.get_some::<&MyBoxed>().unwrap());

    let b = Some(MyBoxed(String::from("def")));
    let v = b.to_value();
    let b = b.unwrap();
    assert_eq!(&b, v.get::<&MyBoxed>().unwrap().unwrap());
    assert_eq!(&b, v.get_some::<&MyBoxed>().unwrap());

    let b: Option<MyBoxed> = None;
    let result = std::panic::catch_unwind(|| b.to_value());
    assert!(result.is_err());
}

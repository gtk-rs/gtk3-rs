// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::{FromGlib, ToGlib};
use glib::{gflags, GBoxed, GEnum};

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

    assert_eq!(unsafe { Animal::from_glib(0) }, Animal::Goat);
    assert_eq!(unsafe { Animal::from_glib(1) }, Animal::Dog);
    assert_eq!(unsafe { Animal::from_glib(5) }, Animal::Cat);

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
    assert!(t.is_a(glib::Type::ENUM));
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
}

#[test]
fn derive_gboxed_nullable() {
    #[derive(Clone, Debug, PartialEq, Eq, GBoxed)]
    #[gboxed(type_name = "MyNullableBoxed", nullable)]
    struct MyNullableBoxed(String);

    assert_eq!(MyNullableBoxed::get_type().name(), "MyNullableBoxed");

    let b = MyNullableBoxed(String::from("abc"));
    let v = b.to_value();
    assert_eq!(&b, v.get::<&MyNullableBoxed>().unwrap().unwrap());

    let b = Some(MyNullableBoxed(String::from("def")));
    let v = b.to_value();
    let b = b.unwrap();
    assert_eq!(&b, v.get::<&MyNullableBoxed>().unwrap().unwrap());

    let b: Option<MyNullableBoxed> = None;
    let v = b.to_value();
    assert_eq!(None, v.get::<&MyNullableBoxed>().unwrap());
}

#[test]
fn attr_gflags() {
    #[gflags("MyFlags")]
    enum MyFlags {
        #[gflags(name = "Flag A", nick = "nick-a")]
        A = 0b00000001,
        #[gflags(name = "Flag B")]
        B = 0b00000010,
        #[gflags(skip)]
        AB = Self::A.bits() | Self::B.bits(),
        C = 0b00000100,
    }

    assert_eq!(MyFlags::A.bits(), 1);
    assert_eq!(MyFlags::B.bits(), 2);
    assert_eq!(MyFlags::AB.bits(), 3);

    assert_eq!(MyFlags::empty().to_glib(), 0);
    assert_eq!(MyFlags::A.to_glib(), 1);
    assert_eq!(MyFlags::B.to_glib(), 2);
    assert_eq!(MyFlags::AB.to_glib(), 3);

    assert_eq!(unsafe { MyFlags::from_glib(0) }, MyFlags::empty());
    assert_eq!(unsafe { MyFlags::from_glib(1) }, MyFlags::A);
    assert_eq!(unsafe { MyFlags::from_glib(2) }, MyFlags::B);
    assert_eq!(unsafe { MyFlags::from_glib(3) }, MyFlags::AB);

    assert_eq!(
        MyFlags::empty().to_value().get::<MyFlags>(),
        Ok(Some(MyFlags::empty()))
    );
    assert_eq!(MyFlags::A.to_value().get::<MyFlags>(), Ok(Some(MyFlags::A)));
    assert_eq!(MyFlags::B.to_value().get::<MyFlags>(), Ok(Some(MyFlags::B)));
    assert_eq!(
        MyFlags::AB.to_value().get::<MyFlags>(),
        Ok(Some(MyFlags::AB))
    );

    let t = MyFlags::static_type();
    assert!(t.is_a(glib::Type::FLAGS));
    assert_eq!(t.name(), "MyFlags");

    let e = glib::FlagsClass::new(t).expect("FlagsClass::new failed");
    let v = e.get_value(1).expect("FlagsClass::get_value(1) failed");
    assert_eq!(v.get_name(), "Flag A");
    assert_eq!(v.get_nick(), "nick-a");
    let v = e.get_value(2).expect("FlagsClass::get_value(2) failed");
    assert_eq!(v.get_name(), "Flag B");
    assert_eq!(v.get_nick(), "b");
    let v = e.get_value(4).expect("FlagsClass::get_value(4) failed");
    assert_eq!(v.get_name(), "C");
    assert_eq!(v.get_nick(), "c");

    assert!(e.get_value_by_name("Flag A").is_some());
    assert!(e.get_value_by_name("Flag B").is_some());
    assert!(e.get_value_by_name("AB").is_none());
    assert!(e.get_value_by_name("C").is_some());

    assert!(e.get_value_by_nick("nick-a").is_some());
    assert!(e.get_value_by_nick("b").is_some());
    assert!(e.get_value_by_nick("ab").is_none());
    assert!(e.get_value_by_nick("c").is_some());
}

// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! # **glib**, **gobject** and **gio** bindings for Rust
//!
//! This library contains
//!
//! - bindings to some essential GLib, GObject, GIO types and APIs,
//!
//! - common building blocks used in both handmade and machine generated
//! bindings to GTK+ and other GLib-based libraries.
//!
//! It is the foundation for higher level libraries with uniform Rusty (safe
//! and strongly typed) APIs. It avoids exposing GLib-specific data types where
//! possible and is not meant to provide comprehensive GLib bindings, which
//! would often amount to duplicating the Rust Standard Library or other utility
//! crates.
//!
//! The library is a work in progress: expect missing functionality and breaking
//! changes.
//!
//! # Objects
//!
//! Each class and interface has a corresponding smart pointer struct
//! representing an instance of that type (e.g. `Object` for `GObject`,
//! `gtk::Widget` for `GtkWidget`). They are reference counted and feature
//! interior mutability similarly to Rust's `Rc<RefCell<T>>` idiom.
//! Consequently, cloning objects is cheap and their methods never require
//! mutable borrows. Two smart pointers are equal iff they point to the same
//! object.
//!
//! The root of the object hierarchy is [`Object`](object/struct.Object.html).
//! Inheritance and subtyping is denoted with the [`IsA`](object/trait.IsA.html)
//! marker trait. The [`Cast`](object/trait.Cast.html) trait enables upcasting
//! and downcasting.
//!
//! Interfaces and non-leaf classes also have corresponding traits (e.g.
//! `ObjectExt` and `gtk::WidgetExt`), which are blanketly implemented for all
//! their subtypes.

extern crate libc;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gio_sys as gio_ffi;

use libc::c_char;

pub use self::app_info::AppInfo;
pub use self::glib_container::GlibContainer;
pub use self::error::{Error};
pub use self::object::{
    Cast,
    IsA,
    Object,
};
pub use self::permission::Permission;
pub use self::source::{Continue, idle_add, timeout_add, timeout_add_seconds};
pub use self::traits::FFIGObject;
pub use self::value::{Value, ValuePublic};
pub use types::Type;
pub use self::date::{TimeVal, Time, Date, Year, Month, Weekday, Day};

#[macro_use]
pub mod wrapper;
#[macro_use]
pub mod boxed;
#[macro_use]
pub mod shared;
#[macro_use]
pub mod object;

mod app_info;
pub mod glib_container;
pub mod error;
mod file_error;
mod permission;
pub mod signal;
pub mod source;
pub mod traits;
pub mod translate;
mod value;

pub mod types;
pub mod date;

pub fn to_gboolean(b: bool) -> glib_ffi::gboolean {
    match b {
        true => glib_ffi::GTRUE,
        false => glib_ffi::GFALSE
    }
}

pub fn to_bool(b: glib_ffi::gboolean) -> bool {
    b != glib_ffi::GFALSE
}

// An opaque structure used as the base of all interface types.
pub struct TypeInterface;

// An opaque structure used as the base of all type instances.
pub struct TypeInstance;

// An opaque structure used as the base of all classes.
pub struct TypeClass;

//FIXME: Check if this is actually correct (maybe not since ParamFlags is deprecated)
#[derive(Clone, Copy)]
pub enum ParamFlags{
    Readable,
    Writable,
    ReadWrite,
    Construct,
    ConstructOnly,
    LaxValidation,
    StaticName,
    Private,
    StaticNick,
    StaticBlurb,
    Deprecated
}

#[repr(C)]
pub struct ParamSpec {
    g_type_instance: TypeInstance,
    name: *mut c_char,
    flags: ParamFlags,
    value_type: glib_ffi::GType,
    owner_type: glib_ffi::GType,
}

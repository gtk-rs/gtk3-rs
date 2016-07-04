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
//! It is the foundation for higher level libraries with uniform Rusty (safe and
//! strongly typed) APIs. It avoids exposing GLib-specific data types where
//! possible and is not meant to provide comprehensive GLib bindings, which
//! would often amount to duplicating the Rust Standard Library or other utility
//! crates.
//!
//! The library is a work in progress: expect missing functionality and breaking
//! changes.
//!
//! # Dynamic typing
//!
//! Most types in the GLib family have type identifiers
//! ([`Type`](types/enum.Type.html)). Their corresponding Rust types implement
//! the [`StaticType`](types/trait.StaticType.html) trait.
//!
//! Dynamically typed [`Value`](value/index.html) can carry values of any `T:
//! StaticType`.
//!
//! [`Variant`](variant/index.html) can carry values of `T: StaticVariantType`.
//!
//! # Errors
//!
//! Errors are represented by [`Error`](error/struct.Error.html), which can
//! carry values from various [error
//! domains](error/trait.ErrorDomain.html#implementors) (such as
//! [`FileError`](enum.FileError.html)).
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
//!
//! # Under the hood
//!
//! GLib-based libraries largely operate on pointers to various boxed or
//! reference counted structures so the bindings have to implement corresponding
//! smart pointers (wrappers), which encapsulate resource management and safety
//! checks. Such wrappers are defined via the
//! [`glib_wrapper!`](macro.glib_wrapper!.html) macro, which uses abstractions
//! defined in the [`wrapper`](wrapper/index.html), [`boxed`](boxed/index.html),
//! [`shared`](shared/index.html) and [`object`](object/index.html) modules.
//!
//! The [`translate`](translate/index.html) module defines and partly implements
//! conversions between high level Rust types (including the aforementioned
//! wrappers) and their FFI counterparts.

extern crate libc;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gio_sys as gio_ffi;

pub use bytes::Bytes;
pub use error::Error;
pub use file_error::FileError;
pub use object::{
    Cast,
    IsA,
    Object,
    ObjectExt,
};
pub use signal::{
    signal_handler_block,
    signal_handler_unblock,
    signal_stop_emission,
    signal_stop_emission_by_name
};
pub use source::{
    CallbackGuard,
    Continue,
    idle_add,
    timeout_add,
    timeout_add_seconds,
};
pub use types::{
    StaticType,
    Type,
};
pub use value::{
    ToValue,
    TypedValue,
    Value,
};
pub use variant::{
    StaticVariantType,
    ToVariant,
    Variant,
};
pub use variant_type::{
    VariantTy,
    VariantType,
};
pub use time_val::{
    TimeVal,
    get_current_time,
};

#[macro_use]
pub mod wrapper;
#[macro_use]
pub mod boxed;
#[macro_use]
pub mod shared;
#[macro_use]
pub mod object;

mod bytes;
pub mod error;
mod file_error;
pub mod prelude;
pub mod signal;
mod source;
mod time_val;
pub mod translate;
pub mod types;
pub mod value;
pub mod variant;
mod variant_type;

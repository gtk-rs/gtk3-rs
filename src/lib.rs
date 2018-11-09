// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! # ATK bindings
//!
//! This library contains safe Rust bindings for [ATK](https://developer.gnome.org/atk/). It's
//! a part of [Gtk-rs](http://gtk-rs.org/).

#![cfg_attr(feature = "cargo-clippy", allow(let_unit_value))]
#![cfg_attr(feature = "cargo-clippy", allow(new_without_default))]
#![cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
#![cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
#![cfg_attr(feature = "cargo-clippy", allow(trivially_copy_pass_by_ref))]
#![cfg_attr(feature = "cargo-clippy", allow(derive_hash_xor_eq))]
#![allow(deprecated)]

extern crate libc;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;

extern crate atk_sys as ffi;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
#[macro_use]
extern crate glib;

pub use glib::{
    Cast,
    Continue,
    Error,
    IsA,
    Object,
    StaticType,
    ToValue,
    Type,
    TypedValue,
    Value,
};

#[macro_use]
mod rt;

#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
#[cfg_attr(feature = "cargo-clippy", allow(let_and_return))]
#[cfg_attr(feature = "cargo-clippy", allow(many_single_char_names))]
#[cfg_attr(feature = "cargo-clippy", allow(wrong_self_convention))]
mod auto;

pub use auto::*;

pub mod prelude;

pub use prelude::*;

mod editable_text;
mod selection;

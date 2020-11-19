// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

//! # ATK bindings
//!
//! This library contains safe Rust bindings for [ATK](https://developer.gnome.org/atk/). It's
//! a part of [Gtk-rs](https://gtk-rs.org/).

#![cfg_attr(feature = "dox", feature(doc_cfg))]

extern crate libc;
#[macro_use]
extern crate bitflags;

#[doc(hidden)]
pub extern crate atk_sys;
pub use atk_sys as ffi;
extern crate glib_sys;
extern crate gobject_sys;
#[macro_use]
extern crate glib;

#[macro_use]
mod rt;

#[allow(unused_imports)]
mod auto;

pub use auto::*;

pub mod prelude;

pub use attribute::Attribute;
pub use attribute_set::AttributeSet;
pub use text_rectangle::TextRectangle;

mod attribute;
mod attribute_set;
mod editable_text;
mod table;
mod text_rectangle;

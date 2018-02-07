// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(deprecated)]

extern crate pango_sys as ffi;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
#[macro_use]
extern crate glib;
#[macro_use]
extern crate bitflags;
extern crate libc;

pub use glib::Error;

#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(useless_transmute))]
mod auto;
pub use auto::*;
pub use auto::functions::*;

pub use ffi::PANGO_SCALE as SCALE;

pub mod prelude;

pub mod attr_class;
pub use attr_class::AttrClass;
pub mod attr_list;
pub mod attribute;
pub mod font_description;
pub mod item;
pub use item::Item;
pub mod language;
pub use language::Language;
pub mod rectangle;
pub use rectangle::Rectangle;

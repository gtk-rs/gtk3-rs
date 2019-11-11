// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(deprecated)]

extern crate glib_sys;
extern crate gobject_sys;
extern crate pango_sys;
#[macro_use]
extern crate glib;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
extern crate libc;

#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(useless_transmute))]
#[cfg_attr(feature = "cargo-clippy", allow(should_implement_trait))]
mod auto;
pub use auto::functions::*;
pub use auto::*;
pub use functions::*;

pub use pango_sys::PANGO_SCALE as SCALE;

pub mod prelude;

pub mod analysis;
pub use analysis::Analysis;
pub mod attr_class;
pub use attr_class::AttrClass;
pub mod attr_iterator;
pub mod attr_list;
pub mod attribute;
pub mod font_description;
mod functions;
pub mod gravity;
pub mod item;
pub mod language;
pub use language::Language;
pub mod rectangle;
pub use rectangle::Rectangle;
pub mod glyph;

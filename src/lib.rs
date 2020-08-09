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
extern crate libc;
extern crate once_cell;

#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(should_implement_trait))]
mod auto;
pub use auto::functions::*;
pub use auto::*;
pub use functions::*;

pub use pango_sys::PANGO_SCALE as SCALE;

/// The scale factor for three shrinking steps (1 / (1.2 * 1.2 * 1.2)).
pub const SCALE_XX_SMALL: f64 = 0.5787037037037;

/// The scale factor for two shrinking steps (1 / (1.2 * 1.2)).
pub const SCALE_X_SMALL: f64 = 0.6944444444444;

/// The scale factor for one shrinking step (1 / 1.2).
pub const SCALE_SMALL: f64 = 0.8333333333333;

/// The scale factor for normal size (1.0).
pub const SCALE_MEDIUM: f64 = 1.0;

/// The scale factor for one magnification step (1.2).
pub const SCALE_LARGE: f64 = 1.2;

/// The scale factor for two magnification steps (1.2 * 1.2).
pub const SCALE_X_LARGE: f64 = 1.44;

/// The scale factor for three magnification steps (1.2 * 1.2 * 1.2).
pub const SCALE_XX_LARGE: f64 = 1.728;

pub mod prelude;

pub mod analysis;
pub use analysis::Analysis;
pub mod attr_class;
pub use attr_class::AttrClass;
pub mod attr_iterator;
pub mod attr_list;
pub mod attribute;
mod functions;
pub mod gravity;
pub mod item;
pub mod language;
pub use language::Language;
pub mod rectangle;
pub use rectangle::Rectangle;
pub mod glyph;

mod coverage;
pub use coverage::*;

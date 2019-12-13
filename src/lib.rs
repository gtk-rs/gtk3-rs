// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(deprecated)]

extern crate cairo;
extern crate cairo_sys;
extern crate glib_sys;
extern crate gobject_sys;
extern crate pango;
extern crate pango_cairo_sys;
extern crate pango_sys;
#[macro_use]
extern crate glib;
extern crate bitflags;
extern crate libc;

#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(useless_transmute))]
mod auto;

pub use auto::functions::*;
pub use auto::*;
pub mod prelude;

mod font_map;

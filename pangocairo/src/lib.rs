// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

#![allow(deprecated)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

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
#[allow(unused_imports)]
mod auto;

pub use auto::functions::*;
pub use auto::*;
pub mod prelude;

mod font_map;

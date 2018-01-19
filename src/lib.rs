// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(deprecated)]

extern crate cairo_sys as cairo_ffi;
extern crate pango_sys as pango_ffi;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate pango_cairo_sys as ffi;
extern crate cairo;
extern crate pango;
#[macro_use]
extern crate glib;
extern crate bitflags;
extern crate libc;

pub use glib::Error;

#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(useless_transmute))]
mod auto;
pub use auto::*;

mod font_map;
pub use font_map::*;
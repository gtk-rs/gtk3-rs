// Copyright 2013-2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

#![allow(deprecated)]
#![cfg_attr(feature = "cargo-clippy", allow(type_complexity))]

#[macro_use]
extern crate glib;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate gdk_sys;
extern crate gdk_x11_sys;
extern crate gio;
extern crate gio_sys;
extern crate glib_sys;
extern crate gobject_sys;
extern crate pango;

extern crate libc;
extern crate x11;
#[macro_use]
extern crate bitflags;

pub(crate) use x11::xlib;

#[macro_use]
mod rt;
mod auto;

pub use auto::*;

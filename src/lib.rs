// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate pango_sys as ffi;
#[macro_use]
extern crate glib;
#[macro_use]
extern crate bitflags;
extern crate libc;

mod auto;
pub use auto::*;

pub use self::widgets::{
    Item,
    Rectangle,
    Matrix,
    GlyphString,
    FontDescription,
};

pub use ffi::PANGO_SCALE as SCALE;

pub mod widgets;

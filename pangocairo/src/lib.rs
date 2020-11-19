// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

#![allow(deprecated)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use pango_cairo_sys as ffi;

#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[allow(unused_imports)]
mod auto;

pub use self::auto::functions::*;
pub use self::auto::*;
pub mod prelude;

mod font_map;

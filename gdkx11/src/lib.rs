// Copyright 2013-2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

#![allow(deprecated)]
#![cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use gdk_x11_sys as ffi;

#[macro_use]
mod rt;
#[allow(unused_imports)]
mod auto;

pub use self::auto::*;

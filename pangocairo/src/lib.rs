// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

#![allow(deprecated)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;

#[allow(clippy::too_many_arguments)]
#[allow(unused_imports)]
mod auto;

pub use crate::auto::functions::*;
pub use crate::auto::*;
pub mod prelude;

mod font_map;

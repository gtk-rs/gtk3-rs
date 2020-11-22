// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

//! # ATK bindings
//!
//! This library contains safe Rust bindings for [ATK](https://developer.gnome.org/atk/). It's
//! a part of [Gtk-rs](https://gtk-rs.org/).

#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;

#[macro_use]
mod rt;

#[allow(unused_imports)]
mod auto;

pub use crate::auto::*;

pub mod prelude;

pub use crate::attribute::Attribute;
pub use crate::attribute_set::AttributeSet;
pub use crate::text_rectangle::TextRectangle;

mod attribute;
mod attribute_set;
mod editable_text;
mod table;
mod text_rectangle;

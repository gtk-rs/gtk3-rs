// Copyright 2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

/*!
Bindings and wrappers for __PANGO__
*/

extern crate pango_sys as pango_ffi;
extern crate libc;

pub use pango_ffi as ffi;
pub use ffi::enums;

pub use self::widgets::{
    Item,
    Rectangle,
    Matrix,
    GlyphString
};

pub use ffi::enums::{
    Gravity,
    GravityHint,
    Script,
    Direction,
    BidiType,
    Style,
    Weight,
    Variant,
    Stretch,
    FontMask
};

pub mod widgets;

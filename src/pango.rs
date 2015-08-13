// Copyright 2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

/*!
Bindings and wrappers for __PANGO__
*/

extern crate pango_sys as pango_ffi;
extern crate glib;
extern crate libc;

pub use pango_ffi as ffi;

pub use self::widgets::{
    Item,
    Rectangle,
    Matrix,
    GlyphString,
    FontDescription,
};

pub use ffi::PangoGravity as Gravity;
pub use ffi::PangoGravityHint as GravityHint;
pub use ffi::PangoScript as Script;
pub use ffi::PangoDirection as Direction;
pub use ffi::PangoBidiType as BidiType;
pub use ffi::PangoStyle as Style;
pub use ffi::PangoWeight as Weight;
pub use ffi::PangoVariant as Variant;
pub use ffi::PangoStretch as Stretch;
pub use ffi::PangoFontMask as FontMask;

pub mod widgets;

// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate pango_sys as ffi;
extern crate glib_sys as glib_ffi;
#[macro_use]
extern crate glib;
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

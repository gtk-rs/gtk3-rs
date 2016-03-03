// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate gdk_pixbuf_sys as gdk_pixbuf_ffi;
#[macro_use]
extern crate glib;
extern crate libc;

mod animation;
mod format;
mod loader;
mod pixbuf;
pub mod prelude;

pub use self::animation::{
    PixbufAnimation,
    PixbufAnimationIter,
    PixbufSimpleAnim,
    PixbufAnimationExt,
};
pub use self::format::PixbufFormat;
pub use self::loader::PixbufLoader;
pub use self::pixbuf::Pixbuf;

pub use gdk_pixbuf_ffi::GdkColorspace as Colorspace;
pub use gdk_pixbuf_ffi::GdkInterpType as InterpType;
pub use gdk_pixbuf_ffi::GdkPixbufAlphaMode as PixbufAlphaMode;
pub use gdk_pixbuf_ffi::GdkPixbufError as PixbufError;
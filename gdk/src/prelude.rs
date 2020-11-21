// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

//! Traits intended for blanket imports.

pub use crate::auto::traits::*;
pub use crate::cairo_interaction::{GdkContextExt, GdkPixbufExt, GdkSurfaceExt};
pub use crate::window::WindowExtManual;
#[doc(hidden)]
pub use glib::prelude::*;

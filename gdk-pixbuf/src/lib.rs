// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>
#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;

#[allow(clippy::too_many_arguments)]
#[allow(unused_imports)]
mod auto;

mod pixbuf;
mod pixbuf_animation;
mod pixbuf_animation_iter;
pub mod prelude;

pub use crate::auto::*;

pub use self::pixbuf_animation_iter::PixbufAnimationIter;

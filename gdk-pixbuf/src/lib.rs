// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;
pub use gio;
pub use glib;

#[allow(clippy::too_many_arguments)]
#[allow(unused_imports)]
mod auto;

mod pixbuf;
mod pixbuf_animation;
mod pixbuf_animation_iter;
pub mod prelude;

pub use crate::auto::*;

pub use self::pixbuf_animation_iter::PixbufAnimationIter;

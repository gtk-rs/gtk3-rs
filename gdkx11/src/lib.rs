// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(deprecated)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]

pub use ffi;
pub use gdk;
pub use gio;
pub use glib;
pub use x11;

#[macro_use]
mod rt;
#[allow(clippy::let_and_return)]
#[allow(unused_doc_comments)]
#[allow(unused_imports)]
mod auto;

pub use crate::auto::*;

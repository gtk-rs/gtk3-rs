// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(deprecated)]
#![allow(clippy::type_complexity)]

pub use ffi;

#[macro_use]
mod rt;
#[allow(clippy::let_and_return)]
#[allow(unused_doc_comments)]
#[allow(unused_imports)]
mod auto;

pub use crate::auto::*;

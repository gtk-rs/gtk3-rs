// Copyright 2013-2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

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

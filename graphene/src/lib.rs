// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>
#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(deprecated)]

pub use ffi;

// Graphene has no runtime to initialize
macro_rules! assert_initialized_main_thread {
    () => {};
}

// No-op
macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::transmute_ptr_to_ref)]
#[allow(clippy::type_complexity)]
#[allow(clippy::unreadable_literal)]
#[allow(clippy::derive_hash_xor_eq)]
#[allow(unused_imports)]
mod auto;

pub mod prelude;

pub use crate::auto::*;

mod box_;
mod euler;
mod frustum;
mod matrix;
mod plane;
mod point;
mod point3_d;
mod quad;
mod quaternion;
mod ray;
mod rect;
mod size;
mod sphere;
mod triangle;
mod vec2;
mod vec3;
mod vec4;

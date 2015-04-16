// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Points — Simple graphical data type

use libc::{c_int};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Point {
    pub x: c_int,
    pub y: c_int
}
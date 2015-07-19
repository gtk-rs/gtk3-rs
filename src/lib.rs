// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

/*!
    Bindings and wrappers for __Cairo__
*/

extern crate cairo_sys as cairo_ffi;
extern crate libc;
extern crate glib as glib_main;
extern crate c_vec;

pub use glib_main as glib;

pub use cairo_ffi as ffi;
pub use ffi::enums;

pub use ffi::cairo_rectangle_t as Rectangle;
pub use ffi::cairo_rectangle_int_t as RectangleInt;

pub use self::context::{
    Context,
    RectangleVec,
};

pub use self::paths::{
    Path,
    PathSegments,
    PathSegment
};

pub use self::enums::{
    Status,
    Antialias,
    FillRule,
    LineCap,
    LineJoin,
    Operator,
    PathDataType
};

pub use self::patterns::{
    //Traits
    Pattern,
    Gradient,

    //Structs
    LinearGradient,
    RadialGradient,
    SolidPattern,
    SurfacePattern,
};

#[cfg(cairo_1_12)]
pub use self::patterns::{
    Mesh,
    MeshCorner,
};

pub use self::fonts::{
    FontFace,
    ScaledFont,
    FontOptions,

    Glyph,
    FontExtents,
    TextExtents,
    TextCluster,
};

pub use self::matrices::{
    Matrix,
    MatrixTrait,
};

mod fonts;
mod context;
mod paths;
mod patterns;
mod matrices;

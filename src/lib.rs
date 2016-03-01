// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate cairo_sys as ffi;
extern crate libc;
extern crate glib;
extern crate c_vec;

pub use ffi::enums;
pub use ffi::cairo_rectangle_t as Rectangle;
pub use ffi::cairo_rectangle_int_t as RectangleInt;

pub use context::{
    Context,
    RectangleVec,
};

pub use paths::{
    Path,
    PathSegments,
    PathSegment
};

pub use enums::{
    Status,
    Antialias,
    FillRule,
    LineCap,
    LineJoin,
    Operator,
    PathDataType,
    Format,
    SurfaceType,
};

pub use error::IoError;

pub use patterns::{
    //Traits
    Pattern,
    Gradient,

    //Structs
    LinearGradient,
    RadialGradient,
    SolidPattern,
    SurfacePattern,
};

#[cfg(feature = "1.12")]
pub use patterns::{
    Mesh,
    MeshCorner,
};

pub use fonts::{
    FontFace,
    ScaledFont,
    FontOptions,

    Glyph,
    FontExtents,
    TextExtents,
    TextCluster,
};

pub use matrices::{
    Matrix,
    MatrixTrait,
};

pub use image_surface::ImageSurface;
pub use surface::Surface;

pub mod prelude;

mod fonts;
mod context;
mod error;
mod image_surface;
#[cfg(feature = "png")]
mod image_surface_png;
mod paths;
mod patterns;
mod surface;
mod matrices;

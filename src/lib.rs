// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate cairo_sys as ffi;
extern crate libc;
extern crate glib;
extern crate c_vec;

pub use ffi::enums;
pub use ffi::cairo_rectangle_t as Rectangle;

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
    Content,
    Extend,
    FillRule,
    Filter,
    LineCap,
    LineJoin,
    Operator,
    PathDataType,
    Format,
    SurfaceType,
};

pub use error::{
    BorrowError,
    IoError,
};

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

#[cfg(feature = "v1_12")]
pub use patterns::{
    Mesh,
    MeshCorner,
};

pub use fonts::{
    FontFace,
    FontType,
    FontSlant,
    FontWeight,
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

pub use rectangle::RectangleInt;

pub use surface::Surface;

pub use image_surface::{
    ImageSurface,
    ImageSurfaceData,
};

#[cfg(feature = "xcb")]
pub use xcb::{
    XCBConnection,
    XCBSurface,
    Device,
    XCBDrawable,
    XCBPixmap,
    XCBRenderPictFormInfo,
    XCBScreen,
    XCBVisualType,
};

pub mod prelude;

mod fonts;
mod context;
mod error;
mod image_surface;
#[cfg(feature = "png")]
mod image_surface_png;
mod paths;
mod patterns;
mod rectangle;
mod surface;
mod matrices;
#[cfg(feature = "xcb")]
mod xcb;

#[cfg(windows)]
mod win32_surface;

#[cfg(windows)]
pub use win32_surface::Win32Surface;

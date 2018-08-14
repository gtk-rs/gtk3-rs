// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate cairo_sys as ffi;
extern crate libc;
extern crate c_vec;

#[cfg(feature = "use_glib")]
#[macro_use]
extern crate glib;

#[cfg(feature = "use_glib")]
extern crate glib_sys as glib_ffi;

#[cfg(feature = "use_glib")]
extern crate gobject_sys as gobject_ffi;


// Helper macro for our GValue related trait impls
#[cfg(feature = "use_glib")]
macro_rules! gvalue_impl {
    ($name:ty, $ffi_name:ty, $get_type:expr) => {
        use glib;
        #[allow(unused_imports)]
        use glib::translate::*;
        use glib_ffi;
        use gobject_ffi;
        use std::ptr;

        impl glib::types::StaticType for $name {
            fn static_type() -> glib::types::Type {
                unsafe { from_glib($get_type()) }
            }
        }

        impl<'a> glib::value::FromValueOptional<'a> for $name {
            unsafe fn from_value_optional(v: &'a glib::value::Value) -> Option<Self> {
                let ptr = gobject_ffi::g_value_get_boxed(v.to_glib_none().0);
                assert!(!ptr.is_null());
                from_glib_none(ptr as *mut $ffi_name)
            }
        }

        impl glib::value::SetValue for $name {
            unsafe fn set_value(v: &mut glib::value::Value, s: &Self) {
                gobject_ffi::g_value_set_boxed(v.to_glib_none_mut().0, s.to_glib_none().0 as glib_ffi::gpointer);
            }
        }

        impl glib::value::SetValueOptional for $name {
            unsafe fn set_value_optional(v: &mut glib::value::Value, s: Option<&Self>) {
                if let Some(s) = s {
                    gobject_ffi::g_value_set_boxed(v.to_glib_none_mut().0, s.to_glib_none().0 as glib_ffi::gpointer);
                } else {
                    gobject_ffi::g_value_set_boxed(v.to_glib_none_mut().0, ptr::null_mut());
                }
            }
        }
    }
}

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
    RegionOverlap,
    SurfaceType,
};

pub use error::{
    BorrowError,
    IoError,
};

pub use patterns::{
    // Enums
    Pattern,
    // Traits
    PatternTrait,
    Gradient,

    // Structs
    LinearGradient,
    RadialGradient,
    SolidPattern,
    SurfacePattern,
};

#[cfg(any(feature = "v1_12", feature = "dox"))]
pub use patterns::{
    Mesh,
    MeshCorner,
};

pub use font::{
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

pub use region::Region;

pub use surface::Surface;

pub use image_surface::{
    ImageSurface,
    ImageSurfaceData,
};

pub use pdf_surface::PDFSurface;

#[cfg(any(feature = "xcb", feature = "dox"))]
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

mod font;
mod context;
mod error;
mod pdf_surface;
mod image_surface;
#[cfg(any(feature = "png", feature = "dox"))]
mod image_surface_png;
mod paths;
mod patterns;
mod rectangle;
mod region;
mod surface;
mod matrices;
#[cfg(any(feature = "xcb", feature = "dox"))]
mod xcb;

#[cfg(any(windows, feature = "dox"))]
mod win32_surface;

#[cfg(any(windows, feature = "dox"))]
pub use win32_surface::Win32Surface;


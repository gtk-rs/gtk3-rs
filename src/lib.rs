// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! # Cairo bindings
//!
//! This library contains safe Rust bindings for [Cairo](https://www.cairographics.org/).
//! It is a part of [Gtk-rs](http://gtk-rs.org/).
//!
//! ## Crate features
//!
//! ### Default-on features
//!
//! * **use_glib** - Use with [glib](https://gtk-rs.org/docs/glib/)
//!
//! ### Fileformat features
//!
//! * **png** - Reading and writing PNG images
//! * **pdf** - Rendering PDF documents
//! * **svg** - Rendering SVG documents
//! * **ps** - Rendering PostScript documents
//!
//! ### Cairo API version features
//!
//! * **v1_14** - Use Cairo 1.14 APIs
//! * **v1_16** - Use Cairo 1.16 APIs
//!
//! ### Documentation features
//!
//! * **embed-lgpl-docs** - Embed API docs locally
//! * **purge-lgpl-docs** - Remove API docs again (counterpart to `embed-lgpl-docs`)
//! * **dox** - Used to keep system dependent items in documentation
//!
//! ### X Window features
//!
//! * **xcb** - X Window System rendering using the XCB library
//! * **xlib** - X Window System rendering using XLib
//!
//! ### Windows API features
//!
//! * **win32-surface** - Microsoft Windows surface support

extern crate cairo_sys as ffi;
extern crate libc;
extern crate thiserror;

#[macro_use]
extern crate bitflags;

#[cfg(feature = "use_glib")]
#[macro_use]
extern crate glib;

#[cfg(feature = "use_glib")]
extern crate glib_sys as glib_ffi;

#[cfg(feature = "use_glib")]
extern crate gobject_sys as gobject_ffi;

#[cfg(test)]
extern crate tempfile;

// Helper macro for our GValue related trait impls
#[cfg(feature = "use_glib")]
macro_rules! gvalue_impl {
    ($name:ty, $ffi_name:ty, $get_type:expr) => {
        use glib;
        #[allow(unused_imports)]
        use glib::translate::*;
        use glib_ffi;
        use gobject_ffi;

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
                gobject_ffi::g_value_set_boxed(
                    v.to_glib_none_mut().0,
                    s.to_glib_none().0 as glib_ffi::gpointer,
                );
            }
        }

        impl glib::value::SetValueOptional for $name {
            unsafe fn set_value_optional(v: &mut glib::value::Value, s: Option<&Self>) {
                if let Some(s) = s {
                    gobject_ffi::g_value_set_boxed(
                        v.to_glib_none_mut().0,
                        s.to_glib_none().0 as glib_ffi::gpointer,
                    );
                } else {
                    gobject_ffi::g_value_set_boxed(v.to_glib_none_mut().0, ::std::ptr::null_mut());
                }
            }
        }
    };
}

pub use user_data::UserDataKey;

pub use context::{Context, RectangleList};

pub use paths::{Path, PathSegment, PathSegments};

pub use device::Device;

pub use enums::*;

pub use error::{BorrowError, Error, IoError};

pub use patterns::{
    Gradient, LinearGradient, Mesh, Pattern, RadialGradient, SolidPattern, SurfacePattern,
};

pub use font::{
    FontExtents, FontFace, FontOptions, FontSlant, FontType, FontWeight, Glyph, ScaledFont,
    TextCluster, TextExtents,
};

pub use matrices::Matrix;

pub use recording_surface::RecordingSurface;
pub use rectangle::Rectangle;
pub use rectangle_int::RectangleInt;

pub use region::Region;

pub use surface::{MappedImageSurface, Surface};

pub use image_surface::{ImageSurface, ImageSurfaceData};

#[cfg(any(feature = "pdf", feature = "svg", feature = "ps", feature = "dox"))]
pub use stream::StreamWithError;

#[cfg(any(feature = "pdf", feature = "dox"))]
pub use pdf::PdfSurface;

#[cfg(any(feature = "ps", feature = "dox"))]
pub use ps::PsSurface;

#[cfg(any(feature = "svg", feature = "dox"))]
pub use svg::SvgSurface;

#[cfg(any(feature = "xcb", feature = "dox"))]
pub use xcb::{
    XCBConnection, XCBDrawable, XCBPixmap, XCBRenderPictFormInfo, XCBScreen, XCBSurface,
    XCBVisualType,
};

#[macro_use]
mod surface_macros;
#[macro_use]
mod user_data;
mod constants;
pub use constants::*;
mod utils;
pub use utils::{debug_reset_static_data, get_version_string, Version};
mod context;
mod device;
mod enums;
mod error;
mod font;
mod image_surface;
#[cfg(any(feature = "png", feature = "dox"))]
mod image_surface_png;
mod matrices;
mod paths;
mod patterns;
mod recording_surface;
mod rectangle;
mod rectangle_int;
mod region;
mod surface;
#[cfg(any(feature = "xcb", feature = "dox"))]
mod xcb;

#[cfg(any(feature = "pdf", feature = "svg", feature = "ps", feature = "dox"))]
#[macro_use]
mod stream;
#[cfg(any(feature = "pdf", feature = "dox"))]
mod pdf;
#[cfg(any(feature = "ps", feature = "dox"))]
mod ps;
#[cfg(any(feature = "svg", feature = "dox"))]
mod svg;

#[cfg(any(target_os = "macos", target_os = "ios", feature = "dox"))]
mod quartz_surface;
#[cfg(any(target_os = "macos", target_os = "ios", feature = "dox"))]
pub use quartz_surface::QuartzSurface;

#[cfg(any(all(windows, feature = "win32-surface"), feature = "dox"))]
mod win32_surface;

#[cfg(any(all(windows, feature = "win32-surface"), feature = "dox"))]
pub use win32_surface::Win32Surface;

#[cfg(not(feature = "use_glib"))]
mod borrowed {
    use std::mem;

    /// Wrapper around values representing borrowed C memory.
    ///
    /// This is returned by `from_glib_borrow()` and ensures that the wrapped value
    /// is never dropped when going out of scope.
    ///
    /// Borrowed values must never be passed by value or mutable reference to safe Rust code and must
    /// not leave the C scope in which they are valid.
    #[derive(Debug)]
    pub struct Borrowed<T>(mem::ManuallyDrop<T>);

    impl<T> Borrowed<T> {
        /// Creates a new borrowed value.
        pub fn new(val: T) -> Self {
            Self(mem::ManuallyDrop::new(val))
        }

        /// Extracts the contained value.
        ///
        /// The returned value must never be dropped and instead has to be passed to `mem::forget()` or
        /// be directly wrapped in `mem::ManuallyDrop` or another `Borrowed` wrapper.
        pub unsafe fn into_inner(self) -> T {
            mem::ManuallyDrop::into_inner(self.0)
        }
    }

    impl<T> AsRef<T> for Borrowed<T> {
        fn as_ref(&self) -> &T {
            &*self.0
        }
    }

    impl<T> std::ops::Deref for Borrowed<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &*self.0
        }
    }
}

#[cfg(not(feature = "use_glib"))]
pub use borrowed::Borrowed;

#[cfg(feature = "use_glib")]
pub(crate) use glib::translate::Borrowed;

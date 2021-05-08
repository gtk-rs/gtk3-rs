// Take a look at the license at the top of the repository in the LICENSE file.

//! # Cairo bindings
//!
//! This library contains safe Rust bindings for [Cairo](https://www.cairographics.org/).
//! It is a part of [Gtk-rs](https://gtk-rs.org/).
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
//! * **v1_16** - Use Cairo 1.16 APIs
//!
//! ### Documentation features
//!
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

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]

pub use ffi;
#[cfg(feature = "freetype")]
pub use freetype_crate as freetype;
#[cfg(feature = "use_glib")]
pub use glib;

// Helper macro for our GValue related trait impls
#[cfg(feature = "use_glib")]
macro_rules! gvalue_impl {
    ($name:ty, $ffi_name:ty, $get_type:expr) => {
        #[allow(unused_imports)]
        use glib::translate::*;

        impl glib::types::StaticType for $name {
            fn static_type() -> glib::types::Type {
                unsafe { from_glib($get_type()) }
            }
        }

        impl glib::value::ValueType for $name {
            type Type = Self;
        }

        unsafe impl<'a> glib::value::FromValue<'a> for $name {
            type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

            unsafe fn from_value(value: &'a glib::Value) -> Self {
                let ptr = glib::gobject_ffi::g_value_get_boxed(
                    glib::translate::ToGlibPtr::to_glib_none(value).0,
                );
                assert!(!ptr.is_null());
                <$name as glib::translate::FromGlibPtrNone<*mut $ffi_name>>::from_glib_none(
                    ptr as *mut $ffi_name,
                )
            }
        }

        impl glib::value::ToValue for $name {
            fn to_value(&self) -> glib::Value {
                unsafe {
                    let mut value =
                        glib::Value::from_type(<$name as glib::StaticType>::static_type());
                    glib::gobject_ffi::g_value_set_boxed(
                        value.to_glib_none_mut().0,
                        self.to_glib_none().0 as *mut _,
                    );
                    value
                }
            }

            fn value_type(&self) -> glib::Type {
                <$name as glib::StaticType>::static_type()
            }
        }

        impl glib::value::ToValueOptional for $name {
            fn to_value_optional(s: Option<&Self>) -> glib::Value {
                let mut value = glib::Value::for_value_type::<Self>();
                unsafe {
                    glib::gobject_ffi::g_value_take_boxed(
                        value.to_glib_none_mut().0,
                        glib::translate::ToGlibPtr::to_glib_full(&s) as *mut _,
                    );
                }

                value
            }
        }
    };
}

pub use crate::user_data::UserDataKey;

pub use crate::context::{Context, RectangleList};

pub use crate::paths::{Path, PathSegment, PathSegments};

pub use crate::device::Device;

pub use crate::enums::*;

pub use crate::error::{BorrowError, Error, IoError};

pub use crate::patterns::{
    Gradient, LinearGradient, Mesh, Pattern, RadialGradient, SolidPattern, SurfacePattern,
};

pub use crate::font::{
    FontExtents, FontFace, FontOptions, FontSlant, FontType, FontWeight, Glyph, ScaledFont,
    TextCluster, TextExtents,
};

pub use crate::matrices::Matrix;

pub use crate::recording_surface::RecordingSurface;
pub use crate::rectangle::Rectangle;
pub use crate::rectangle_int::RectangleInt;

pub use crate::region::Region;

pub use crate::surface::{MappedImageSurface, Surface};

pub use crate::image_surface::{ImageSurface, ImageSurfaceData};

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
pub use crate::constants::*;
mod utils;
pub use crate::utils::{debug_reset_static_data, version_string, Version};
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

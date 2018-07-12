// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::fmt::{Error, Debug};
use std::ffi::CStr;
use std::i32;
use std::u32;

#[cfg(feature = "use_glib")]
use glib;
#[cfg(feature = "use_glib")]
use glib::translate::*;
#[cfg(feature = "use_glib")]
use gobject_ffi;
#[cfg(feature = "use_glib")]
use std::mem;

// Helper macro for our GValue related trait impls
#[cfg(feature = "use_glib")]
macro_rules! gvalue_impl {
    ($name:ty, $get_type:expr) => {
        impl glib::types::StaticType for $name {
            fn static_type() -> glib::Type {
                unsafe { from_glib($get_type()) }
            }
        }

        impl<'a> glib::value::FromValueOptional<'a> for $name {
            unsafe fn from_value_optional(value: &glib::value::Value) -> Option<Self> {
                Some(glib::value::FromValue::from_value(value))
            }
        }

        impl<'a> glib::value::FromValue<'a> for $name {
            unsafe fn from_value(value: &glib::value::Value) -> Self {
                mem::transmute::<i32, $name>(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
            }
        }

        impl glib::value::SetValue for $name {
            unsafe fn set_value(value: &mut glib::value::Value, this: &Self) {
                gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, *this as i32)
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Success = 0,

    NoMemory,
    InvalidRestore,
    InvalidPopGroup,
    NoCurrentPoint,
    InvalidMatrix,
    InvalidStatus,
    NullPointer,
    InvalidString,
    InvalidPathData,
    ReadError,
    WriteError,
    SurfaceFinished,
    SurfaceTypeMismatch,
    PatternTypeMismatch,
    InvalidContent,
    InvalidFormat,
    InvalidVisual,
    FileNotFound,
    InvalidDash,
    InvalidDscComment,
    InvalidIndex,
    ClipNotRepresentable,
    TempFileError,
    InvalidStride,
    FontTypeMismatch,
    UserFontImmutable,
    UserFontError,
    NegativeCount,
    InvalidClusters,
    InvalidSlant,
    InvalidWeight,
    InvalidSize,
    UserFontNotImplemented,
    DeviceTypeMismatch,
    DeviceError,
    InvalidMeshConstruction,
    DeviceFinished,
    // CAIRO_MIME_TYPE_JBIG2_GLOBAL_ID has been used on at least one image but no
    // image provided `JBig2Global` (Since 1.14)
    JBig2GlobalMissing,
    PngError,
    FreetypeError,
    Win32GdiError,
    LastStatus
}

impl Debug for Status {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> Result<(), Error> {
        unsafe {
            let char_ptr = super::cairo_status_to_string(*self);
            let tmp = String::from_utf8_lossy(CStr::from_ptr(char_ptr).to_bytes()).into_owned();

            tmp.fmt(formatter)
        }
    }
}

impl Status {
    pub fn ensure_valid(self) {
        if self != Status::Success {
            panic!("Cairo error {:?}", self)
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Status, ::gobject::cairo_gobject_status_get_type);

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Antialias {
    Default,

    /* method */
    None,
    Gray,
    Subpixel,

    /* hints */
    Fast,
    Good,
    Best
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Antialias, ::gobject::cairo_gobject_antialias_get_type);

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FillRule {
    Winding,
    EvenOdd
}

#[cfg(feature = "use_glib")]
gvalue_impl!(FillRule, ::gobject::cairo_gobject_fill_rule_get_type);

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum LineCap {
    Butt,
    Round,
    Square
}

#[cfg(feature = "use_glib")]
gvalue_impl!(LineCap, ::gobject::cairo_gobject_line_cap_get_type);

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel
}

#[cfg(feature = "use_glib")]
gvalue_impl!(LineJoin, ::gobject::cairo_gobject_line_join_get_type);

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Operator {
    Clear,

    Source,
    Over,
    In,
    Out,
    Atop,

    Dest,
    DestOver,
    DestIn,
    DestOut,
    DestAtop,

    Xor,
    Add,
    Saturate,

    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    HslHue,
    HslSaturation,
    HslColor,
    HslLuminosity
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Operator, ::gobject::cairo_gobject_operator_get_type);

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum PathDataType {
    MoveTo,
    LineTo,
    CurveTo,
    ClosePath
}

#[cfg(feature = "use_glib")]
gvalue_impl!(PathDataType, ::gobject::cairo_gobject_path_data_type_get_type);

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Content {
    Color      = 0x1000,
    Alpha      = 0x2000,
    ColorAlpha = 0x3000
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Content, ::gobject::cairo_gobject_content_get_type);

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Extend {
    None,
    Repeat,
    Reflect,
    Pad
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Extend, ::gobject::cairo_gobject_extend_get_type);

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Filter {
    Fast,
    Good,
    Best,
    Nearest,
    Bilinear,
    Gaussian
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Filter, ::gobject::cairo_gobject_filter_get_type);

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum PatternType {
    Solid,
    Surface,
    LinearGradient,
    RadialGradient,
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    Mesh,
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    RasterSource
}

#[cfg(feature = "use_glib")]
gvalue_impl!(PatternType, ::gobject::cairo_gobject_pattern_type_get_type);

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FontSlant {
    Normal,
    Italic,
    Oblique
}

#[cfg(feature = "use_glib")]
gvalue_impl!(FontSlant, ::gobject::cairo_gobject_font_slant_get_type);

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FontWeight {
    Normal,
    Bold
}

#[cfg(feature = "use_glib")]
gvalue_impl!(FontWeight, ::gobject::cairo_gobject_font_weight_get_type);

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum TextClusterFlags {
    None     = 0x00000000,
    Backward = 0x00000001
}

#[cfg(feature = "use_glib")]
gvalue_impl!(TextClusterFlags, ::gobject::cairo_gobject_text_cluster_flags_get_type);

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FontType {
    FontTypeToy,
    FontTypeFt,
    FontTypeWin32,
    FontTypeQuartz,
    FontTypeUser
}

#[cfg(feature = "use_glib")]
gvalue_impl!(FontType, ::gobject::cairo_gobject_font_type_get_type);

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum SubpixelOrder {
    Default,
    Rgb,
    Bgr,
    Vrgb,
    Vbgr
}

#[cfg(feature = "use_glib")]
gvalue_impl!(SubpixelOrder, ::gobject::cairo_gobject_subpixel_order_get_type);

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum HintStyle {
    Default,
    None,
    Slight,
    Medium,
    Full
}

#[cfg(feature = "use_glib")]
gvalue_impl!(HintStyle, ::gobject::cairo_gobject_hint_style_get_type);

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum HintMetrics {
    Default,
    Off,
    On
}

#[cfg(feature = "use_glib")]
gvalue_impl!(HintMetrics, ::gobject::cairo_gobject_hint_metrics_get_type);

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceType {
    Image,
    Pdf,
    Ps,
    Xlib,
    Xcb,
    Glitz,
    Quartz,
    Win32,
    BeOs,
    DirectFb,
    Svg,
    Os2,
    Win32Printing,
    QuartzImage,
    Script,
    Qt,
    Recording,
    Vg,
    Gl,
    Drm,
    Tee,
    Xml,
    Skia,
    Subsurface,
    Cogl,
}

#[cfg(feature = "use_glib")]
gvalue_impl!(SurfaceType, ::gobject::cairo_gobject_surface_type_get_type);

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SvgUnit {
    User = 0,
    Em,
    Ex,
    Px,
    In,
    Cm,
    Mm,
    Pt,
    Pc,
    Percent,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Invalid = -1,
    ARgb32 = 0,
    Rgb24 = 1,
    A8 = 2,
    A1 = 3,
    Rgb16_565 = 4,
    Rgb30 = 5,
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Format, ::gobject::cairo_gobject_format_get_type);

impl Format {
    pub fn stride_for_width(self, width: u32) -> Result<i32, ()> {
        assert!(width <= i32::MAX as u32);
        let width = width as i32;

        let stride = unsafe { super::cairo_format_stride_for_width(self, width) };
        if stride == -1 {
            Err(())
        } else {
            Ok(stride)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionOverlap {
    In,
    Out,
    Part,
}

#[cfg(feature = "use_glib")]
gvalue_impl!(RegionOverlap, ::gobject::cairo_gobject_region_overlap_get_type);

#[cfg(any(feature = "pdf", feature = "dox"))]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdfVersion {
    _1_4,
    _1_5,
}
#[cfg(any(feature = "svg", feature = "dox"))]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SvgVersion {
    _1_1,
    _1_2,
}
#[cfg(any(feature = "ps", feature = "dox"))]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PsLevel {
    _2,
    _3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn stride_panics_on_bad_value() {
        let _ = Format::Rgb24.stride_for_width(u32::MAX);
    }

    #[test]
    fn stride_errors_on_large_width() {
        assert!(Format::Rgb24.stride_for_width(i32::MAX as u32).is_err());
    }

    #[test]
    fn stride_works() {
        assert!(Format::Rgb24.stride_for_width(1).unwrap() == 4);
    }
}

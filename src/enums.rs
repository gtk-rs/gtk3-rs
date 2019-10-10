// Copyright 2013-2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ffi::CStr;
use std::fmt::{self, Debug, Error};
use std::i32;
use std::u32;

use ffi;

#[cfg(feature = "use_glib")]
use glib;
#[cfg(feature = "use_glib")]
use glib::translate::*;
#[cfg(feature = "use_glib")]
use gobject_ffi;

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
                Self::from(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
            }
        }

        impl glib::value::SetValue for $name {
            unsafe fn set_value(value: &mut glib::value::Value, this: &Self) {
                gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, (*this).into())
            }
        }
    };
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Success,

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
    LastStatus,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_status_t> for Status {
    fn into(self) -> ffi::cairo_status_t {
        match self {
            Status::Success => ffi::STATUS_SUCCESS,
            Status::NoMemory => ffi::STATUS_NO_MEMORY,
            Status::InvalidRestore => ffi::STATUS_INVALID_RESTORE,
            Status::InvalidPopGroup => ffi::STATUS_INVALID_POP_GROUP,
            Status::NoCurrentPoint => ffi::STATUS_NO_CURRENT_POINT,
            Status::InvalidMatrix => ffi::STATUS_INVALID_MATRIX,
            Status::InvalidStatus => ffi::STATUS_INVALID_STATUS,
            Status::NullPointer => ffi::STATUS_NULL_POINTER,
            Status::InvalidString => ffi::STATUS_INVALID_STRING,
            Status::InvalidPathData => ffi::STATUS_INVALID_PATH_DATA,
            Status::ReadError => ffi::STATUS_READ_ERROR,
            Status::WriteError => ffi::STATUS_WRITE_ERROR,
            Status::SurfaceFinished => ffi::STATUS_SURFACE_FINISHED,
            Status::SurfaceTypeMismatch => ffi::STATUS_SURFACE_TYPE_MISMATCH,
            Status::PatternTypeMismatch => ffi::STATUS_PATTERN_TYPE_MISMATCH,
            Status::InvalidContent => ffi::STATUS_INVALID_CONTENT,
            Status::InvalidFormat => ffi::STATUS_INVALID_FORMAT,
            Status::InvalidVisual => ffi::STATUS_INVALID_VISUAL,
            Status::FileNotFound => ffi::STATUS_FILE_NOT_FOUND,
            Status::InvalidDash => ffi::STATUS_INVALID_DASH,
            Status::InvalidDscComment => ffi::STATUS_INVALID_DSC_COMMENT,
            Status::InvalidIndex => ffi::STATUS_INVALID_INDEX,
            Status::ClipNotRepresentable => ffi::STATUS_CLIP_NOT_REPRESENTABLE,
            Status::TempFileError => ffi::STATUS_TEMP_FILE_ERROR,
            Status::InvalidStride => ffi::STATUS_INVALID_STRIDE,
            Status::FontTypeMismatch => ffi::STATUS_FONT_TYPE_MISMATCH,
            Status::UserFontImmutable => ffi::STATUS_USER_FONT_IMMUTABLE,
            Status::UserFontError => ffi::STATUS_USER_FONT_ERROR,
            Status::NegativeCount => ffi::STATUS_NEGATIVE_COUNT,
            Status::InvalidClusters => ffi::STATUS_INVALID_CLUSTERS,
            Status::InvalidSlant => ffi::STATUS_INVALID_SLANT,
            Status::InvalidWeight => ffi::STATUS_INVALID_WEIGHT,
            Status::InvalidSize => ffi::STATUS_INVALID_SIZE,
            Status::UserFontNotImplemented => ffi::STATUS_USER_FONT_NOT_IMPLEMENTED,
            Status::DeviceTypeMismatch => ffi::STATUS_DEVICE_TYPE_MISMATCH,
            Status::DeviceError => ffi::STATUS_DEVICE_ERROR,
            Status::InvalidMeshConstruction => ffi::STATUS_INVALID_MESH_CONSTRUCTION,
            Status::DeviceFinished => ffi::STATUS_DEVICE_FINISHED,
            Status::JBig2GlobalMissing => ffi::STATUS_J_BIG2_GLOBAL_MISSING,
            Status::PngError => ffi::STATUS_PNG_ERROR,
            Status::FreetypeError => ffi::STATUS_FREETYPE_ERROR,
            Status::Win32GdiError => ffi::STATUS_WIN32_GDI_ERROR,
            Status::LastStatus => ffi::STATUS_LAST_STATUS,
            Status::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_status_t> for Status {
    fn from(value: ffi::cairo_status_t) -> Self {
        match value {
            ffi::STATUS_SUCCESS => Status::Success,
            ffi::STATUS_NO_MEMORY => Status::NoMemory,
            ffi::STATUS_INVALID_RESTORE => Status::InvalidRestore,
            ffi::STATUS_INVALID_POP_GROUP => Status::InvalidPopGroup,
            ffi::STATUS_NO_CURRENT_POINT => Status::NoCurrentPoint,
            ffi::STATUS_INVALID_MATRIX => Status::InvalidMatrix,
            ffi::STATUS_INVALID_STATUS => Status::InvalidStatus,
            ffi::STATUS_NULL_POINTER => Status::NullPointer,
            ffi::STATUS_INVALID_STRING => Status::InvalidString,
            ffi::STATUS_INVALID_PATH_DATA => Status::InvalidPathData,
            ffi::STATUS_READ_ERROR => Status::ReadError,
            ffi::STATUS_WRITE_ERROR => Status::WriteError,
            ffi::STATUS_SURFACE_FINISHED => Status::SurfaceFinished,
            ffi::STATUS_SURFACE_TYPE_MISMATCH => Status::SurfaceTypeMismatch,
            ffi::STATUS_PATTERN_TYPE_MISMATCH => Status::PatternTypeMismatch,
            ffi::STATUS_INVALID_CONTENT => Status::InvalidContent,
            ffi::STATUS_INVALID_FORMAT => Status::InvalidFormat,
            ffi::STATUS_INVALID_VISUAL => Status::InvalidVisual,
            ffi::STATUS_FILE_NOT_FOUND => Status::FileNotFound,
            ffi::STATUS_INVALID_DASH => Status::InvalidDash,
            ffi::STATUS_INVALID_DSC_COMMENT => Status::InvalidDscComment,
            ffi::STATUS_INVALID_INDEX => Status::InvalidIndex,
            ffi::STATUS_CLIP_NOT_REPRESENTABLE => Status::ClipNotRepresentable,
            ffi::STATUS_TEMP_FILE_ERROR => Status::TempFileError,
            ffi::STATUS_INVALID_STRIDE => Status::InvalidStride,
            ffi::STATUS_FONT_TYPE_MISMATCH => Status::FontTypeMismatch,
            ffi::STATUS_USER_FONT_IMMUTABLE => Status::UserFontImmutable,
            ffi::STATUS_USER_FONT_ERROR => Status::UserFontError,
            ffi::STATUS_NEGATIVE_COUNT => Status::NegativeCount,
            ffi::STATUS_INVALID_CLUSTERS => Status::InvalidClusters,
            ffi::STATUS_INVALID_SLANT => Status::InvalidSlant,
            ffi::STATUS_INVALID_WEIGHT => Status::InvalidWeight,
            ffi::STATUS_INVALID_SIZE => Status::InvalidSize,
            ffi::STATUS_USER_FONT_NOT_IMPLEMENTED => Status::UserFontNotImplemented,
            ffi::STATUS_DEVICE_TYPE_MISMATCH => Status::DeviceTypeMismatch,
            ffi::STATUS_DEVICE_ERROR => Status::DeviceError,
            ffi::STATUS_INVALID_MESH_CONSTRUCTION => Status::InvalidMeshConstruction,
            ffi::STATUS_DEVICE_FINISHED => Status::DeviceFinished,
            ffi::STATUS_J_BIG2_GLOBAL_MISSING => Status::JBig2GlobalMissing,
            ffi::STATUS_PNG_ERROR => Status::PngError,
            ffi::STATUS_FREETYPE_ERROR => Status::FreetypeError,
            ffi::STATUS_WIN32_GDI_ERROR => Status::Win32GdiError,
            ffi::STATUS_LAST_STATUS => Status::LastStatus,
            value => Status::__Unknown(value),
        }
    }
}

impl Debug for Status {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> Result<(), Error> {
        unsafe {
            let char_ptr = ffi::cairo_status_to_string((*self).into());
            let tmp = String::from_utf8_lossy(CStr::from_ptr(char_ptr).to_bytes()).into_owned();

            tmp.fmt(formatter)
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Status::{}",
            match *self {
                Status::Success => "Success",
                Status::NoMemory => "NoMemory",
                Status::InvalidRestore => "InvalidRestore",
                Status::InvalidPopGroup => "InvalidPopGroup",
                Status::NoCurrentPoint => "NoCurrentPoint",
                Status::InvalidMatrix => "InvalidMatrix",
                Status::InvalidStatus => "InvalidStatus",
                Status::NullPointer => "NullPointer",
                Status::InvalidString => "InvalidString",
                Status::InvalidPathData => "InvalidPathData",
                Status::ReadError => "ReadError",
                Status::WriteError => "WriteError",
                Status::SurfaceFinished => "SurfaceFinished",
                Status::SurfaceTypeMismatch => "SurfaceTypeMismatch",
                Status::PatternTypeMismatch => "PatternTypeMismatch",
                Status::InvalidContent => "InvalidContent",
                Status::InvalidFormat => "InvalidFormat",
                Status::InvalidVisual => "InvalidVisual",
                Status::FileNotFound => "FileNotFound",
                Status::InvalidDash => "InvalidDash",
                Status::InvalidDscComment => "InvalidDscComment",
                Status::InvalidIndex => "InvalidIndex",
                Status::ClipNotRepresentable => "ClipNotRepresentable",
                Status::TempFileError => "TempFileError",
                Status::InvalidStride => "InvalidStride",
                Status::FontTypeMismatch => "FontTypeMismatch",
                Status::UserFontImmutable => "UserFontImmutable",
                Status::UserFontError => "UserFontError",
                Status::NegativeCount => "NegativeCount",
                Status::InvalidClusters => "InvalidClusters",
                Status::InvalidSlant => "InvalidSlant",
                Status::InvalidWeight => "InvalidWeight",
                Status::InvalidSize => "InvalidSize",
                Status::UserFontNotImplemented => "UserFontNotImplemented",
                Status::DeviceTypeMismatch => "DeviceTypeMismatch",
                Status::DeviceError => "DeviceError",
                Status::InvalidMeshConstruction => "InvalidMeshConstruction",
                Status::DeviceFinished => "DeviceFinished",
                Status::JBig2GlobalMissing => "JBig2GlobalMissing",
                Status::PngError => "PngError",
                Status::FreetypeError => "FreetypeError",
                Status::Win32GdiError => "Win32GdiError",
                Status::LastStatus => "LastStatus",
                _ => "Unknown",
            }
        )
    }
}

impl Status {
    pub fn ensure_valid(self) {
        if self != Status::Success {
            panic!("Cairo error {:?}", self)
        }
    }

    pub(crate) fn to_result<T>(self, obj: T) -> Result<T, Self> {
        if self == Status::Success {
            Ok(obj)
        } else {
            Err(self)
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Status, ffi::gobject::cairo_gobject_status_get_type);

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
    Best,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_antialias_t> for Antialias {
    fn into(self) -> ffi::cairo_antialias_t {
        match self {
            Antialias::Default => ffi::ANTIALIAS_DEFAULT,
            Antialias::None => ffi::ANTIALIAS_NONE,
            Antialias::Gray => ffi::ANTIALIAS_GRAY,
            Antialias::Subpixel => ffi::ANTIALIAS_SUBPIXEL,
            Antialias::Fast => ffi::ANTIALIAS_FAST,
            Antialias::Good => ffi::ANTIALIAS_GOOD,
            Antialias::Best => ffi::ANTIALIAS_BEST,
            Antialias::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_antialias_t> for Antialias {
    fn from(value: ffi::cairo_antialias_t) -> Self {
        match value {
            ffi::ANTIALIAS_DEFAULT => Antialias::Default,
            ffi::ANTIALIAS_NONE => Antialias::None,
            ffi::ANTIALIAS_GRAY => Antialias::Gray,
            ffi::ANTIALIAS_SUBPIXEL => Antialias::Subpixel,
            ffi::ANTIALIAS_FAST => Antialias::Fast,
            ffi::ANTIALIAS_GOOD => Antialias::Good,
            ffi::ANTIALIAS_BEST => Antialias::Best,
            value => Antialias::__Unknown(value),
        }
    }
}

impl fmt::Display for Antialias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Antialias::{}",
            match *self {
                Antialias::Default => "Default",
                Antialias::None => "None",
                Antialias::Gray => "Gray",
                Antialias::Subpixel => "Subpixel",
                Antialias::Fast => "Fast",
                Antialias::Good => "Good",
                Antialias::Best => "Best",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Antialias, ffi::gobject::cairo_gobject_antialias_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FillRule {
    Winding,
    EvenOdd,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_fill_rule_t> for FillRule {
    fn into(self) -> ffi::cairo_fill_rule_t {
        match self {
            FillRule::Winding => ffi::FILL_RULE_WINDING,
            FillRule::EvenOdd => ffi::FILL_RULE_EVEN_ODD,
            FillRule::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_fill_rule_t> for FillRule {
    fn from(value: ffi::cairo_fill_rule_t) -> Self {
        match value {
            ffi::FILL_RULE_WINDING => FillRule::Winding,
            ffi::FILL_RULE_EVEN_ODD => FillRule::EvenOdd,
            value => FillRule::__Unknown(value),
        }
    }
}

impl fmt::Display for FillRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FillRule::{}",
            match *self {
                FillRule::Winding => "Winding",
                FillRule::EvenOdd => "EvenOdd",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(FillRule, ffi::gobject::cairo_gobject_fill_rule_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum LineCap {
    Butt,
    Round,
    Square,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_line_cap_t> for LineCap {
    fn into(self) -> ffi::cairo_line_cap_t {
        match self {
            LineCap::Butt => ffi::LINE_CAP_BUTT,
            LineCap::Round => ffi::LINE_CAP_ROUND,
            LineCap::Square => ffi::LINE_CAP_SQUARE,
            LineCap::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_line_cap_t> for LineCap {
    fn from(value: ffi::cairo_line_cap_t) -> Self {
        match value {
            ffi::LINE_CAP_BUTT => LineCap::Butt,
            ffi::LINE_CAP_ROUND => LineCap::Round,
            ffi::LINE_CAP_SQUARE => LineCap::Square,
            value => LineCap::__Unknown(value),
        }
    }
}

impl fmt::Display for LineCap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "LineCap::{}",
            match *self {
                LineCap::Butt => "Butt",
                LineCap::Round => "Round",
                LineCap::Square => "Square",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(LineCap, ffi::gobject::cairo_gobject_line_cap_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_line_join_t> for LineJoin {
    fn into(self) -> ffi::cairo_line_join_t {
        match self {
            LineJoin::Miter => ffi::LINE_JOIN_MITER,
            LineJoin::Round => ffi::LINE_JOIN_ROUND,
            LineJoin::Bevel => ffi::LINE_JOIN_BEVEL,
            LineJoin::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_line_join_t> for LineJoin {
    fn from(value: ffi::cairo_line_join_t) -> Self {
        match value {
            ffi::LINE_JOIN_MITER => LineJoin::Miter,
            ffi::LINE_JOIN_ROUND => LineJoin::Round,
            ffi::LINE_JOIN_BEVEL => LineJoin::Bevel,
            value => LineJoin::__Unknown(value),
        }
    }
}

impl fmt::Display for LineJoin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "LineJoin::{}",
            match *self {
                LineJoin::Miter => "Miter",
                LineJoin::Round => "Round",
                LineJoin::Bevel => "Bevel",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(LineJoin, ffi::gobject::cairo_gobject_line_join_get_type);

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
    HslLuminosity,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_operator_t> for Operator {
    fn into(self) -> ffi::cairo_operator_t {
        match self {
            Operator::Clear => ffi::OPERATOR_CLEAR,
            Operator::Source => ffi::OPERATOR_SOURCE,
            Operator::Over => ffi::OPERATOR_OVER,
            Operator::In => ffi::OPERATOR_IN,
            Operator::Out => ffi::OPERATOR_OUT,
            Operator::Atop => ffi::OPERATOR_ATOP,
            Operator::Dest => ffi::OPERATOR_DEST,
            Operator::DestOver => ffi::OPERATOR_DEST_OVER,
            Operator::DestIn => ffi::OPERATOR_DEST_IN,
            Operator::DestOut => ffi::OPERATOR_DEST_OUT,
            Operator::DestAtop => ffi::OPERATOR_DEST_ATOP,
            Operator::Xor => ffi::OPERATOR_XOR,
            Operator::Add => ffi::OPERATOR_ADD,
            Operator::Saturate => ffi::OPERATOR_SATURATE,
            Operator::Multiply => ffi::OPERATOR_MULTIPLY,
            Operator::Screen => ffi::OPERATOR_SCREEN,
            Operator::Overlay => ffi::OPERATOR_OVERLAY,
            Operator::Darken => ffi::OPERATOR_DARKEN,
            Operator::Lighten => ffi::OPERATOR_LIGHTEN,
            Operator::ColorDodge => ffi::OPERATOR_COLOR_DODGE,
            Operator::ColorBurn => ffi::OPERATOR_COLOR_BURN,
            Operator::HardLight => ffi::OPERATOR_HARD_LIGHT,
            Operator::SoftLight => ffi::OPERATOR_SOFT_LIGHT,
            Operator::Difference => ffi::OPERATOR_DIFFERENCE,
            Operator::Exclusion => ffi::OPERATOR_EXCLUSION,
            Operator::HslHue => ffi::OPERATOR_HSL_HUE,
            Operator::HslSaturation => ffi::OPERATOR_HSL_SATURATION,
            Operator::HslColor => ffi::OPERATOR_HSL_COLOR,
            Operator::HslLuminosity => ffi::OPERATOR_HSL_LUMINOSITY,
            Operator::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_operator_t> for Operator {
    fn from(value: ffi::cairo_operator_t) -> Self {
        match value {
            ffi::OPERATOR_CLEAR => Operator::Clear,
            ffi::OPERATOR_SOURCE => Operator::Source,
            ffi::OPERATOR_OVER => Operator::Over,
            ffi::OPERATOR_IN => Operator::In,
            ffi::OPERATOR_OUT => Operator::Out,
            ffi::OPERATOR_ATOP => Operator::Atop,
            ffi::OPERATOR_DEST => Operator::Dest,
            ffi::OPERATOR_DEST_OVER => Operator::DestOver,
            ffi::OPERATOR_DEST_IN => Operator::DestIn,
            ffi::OPERATOR_DEST_OUT => Operator::DestOut,
            ffi::OPERATOR_DEST_ATOP => Operator::DestAtop,
            ffi::OPERATOR_XOR => Operator::Xor,
            ffi::OPERATOR_ADD => Operator::Add,
            ffi::OPERATOR_SATURATE => Operator::Saturate,
            ffi::OPERATOR_MULTIPLY => Operator::Multiply,
            ffi::OPERATOR_SCREEN => Operator::Screen,
            ffi::OPERATOR_OVERLAY => Operator::Overlay,
            ffi::OPERATOR_DARKEN => Operator::Darken,
            ffi::OPERATOR_LIGHTEN => Operator::Lighten,
            ffi::OPERATOR_COLOR_DODGE => Operator::ColorDodge,
            ffi::OPERATOR_COLOR_BURN => Operator::ColorBurn,
            ffi::OPERATOR_HARD_LIGHT => Operator::HardLight,
            ffi::OPERATOR_SOFT_LIGHT => Operator::SoftLight,
            ffi::OPERATOR_DIFFERENCE => Operator::Difference,
            ffi::OPERATOR_EXCLUSION => Operator::Exclusion,
            ffi::OPERATOR_HSL_HUE => Operator::HslHue,
            ffi::OPERATOR_HSL_SATURATION => Operator::HslSaturation,
            ffi::OPERATOR_HSL_COLOR => Operator::HslColor,
            ffi::OPERATOR_HSL_LUMINOSITY => Operator::HslLuminosity,
            value => Operator::__Unknown(value),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Operator::{}",
            match *self {
                Operator::Clear => "Clear",
                Operator::Source => "Source",
                Operator::Over => "Over",
                Operator::In => "In",
                Operator::Out => "Out",
                Operator::Atop => "Atop",
                Operator::Dest => "Dest",
                Operator::DestOver => "DestOver",
                Operator::DestIn => "DestIn",
                Operator::DestOut => "DestOut",
                Operator::DestAtop => "DestAtop",
                Operator::Xor => "Xor",
                Operator::Add => "Add",
                Operator::Saturate => "Saturate",
                Operator::Multiply => "Multiply",
                Operator::Screen => "Screen",
                Operator::Overlay => "Overlay",
                Operator::Darken => "Darken",
                Operator::Lighten => "Lighten",
                Operator::ColorDodge => "ColorDodge",
                Operator::ColorBurn => "ColorBurn",
                Operator::HardLight => "HardLight",
                Operator::SoftLight => "SoftLight",
                Operator::Difference => "Difference",
                Operator::Exclusion => "Exclusion",
                Operator::HslHue => "HslHue",
                Operator::HslSaturation => "HslSaturation",
                Operator::HslColor => "HslColor",
                Operator::HslLuminosity => "HslLuminosity",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Operator, ffi::gobject::cairo_gobject_operator_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum PathDataType {
    MoveTo,
    LineTo,
    CurveTo,
    ClosePath,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_path_data_type_t> for PathDataType {
    fn into(self) -> ffi::cairo_path_data_type_t {
        match self {
            PathDataType::MoveTo => ffi::PATH_DATA_TYPE_MOVE_TO,
            PathDataType::LineTo => ffi::PATH_DATA_TYPE_LINE_TO,
            PathDataType::CurveTo => ffi::PATH_DATA_TYPE_CURVE_TO,
            PathDataType::ClosePath => ffi::PATH_DATA_TYPE_CLOSE_PATH,
            PathDataType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_path_data_type_t> for PathDataType {
    fn from(value: ffi::cairo_path_data_type_t) -> Self {
        match value {
            ffi::PATH_DATA_TYPE_MOVE_TO => PathDataType::MoveTo,
            ffi::PATH_DATA_TYPE_LINE_TO => PathDataType::LineTo,
            ffi::PATH_DATA_TYPE_CURVE_TO => PathDataType::CurveTo,
            ffi::PATH_DATA_TYPE_CLOSE_PATH => PathDataType::ClosePath,
            value => PathDataType::__Unknown(value),
        }
    }
}

impl fmt::Display for PathDataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PathDataType::{}",
            match *self {
                PathDataType::MoveTo => "MoveTo",
                PathDataType::LineTo => "LineTo",
                PathDataType::CurveTo => "CurveTo",
                PathDataType::ClosePath => "ClosePath",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(
    PathDataType,
    ffi::gobject::cairo_gobject_path_data_type_get_type
);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Content {
    Color,
    Alpha,
    ColorAlpha,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_content_t> for Content {
    fn into(self) -> ffi::cairo_content_t {
        match self {
            Content::Color => ffi::CONTENT_COLOR,
            Content::Alpha => ffi::CONTENT_ALPHA,
            Content::ColorAlpha => ffi::CONTENT_COLOR_ALPHA,
            Content::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_content_t> for Content {
    fn from(value: ffi::cairo_content_t) -> Self {
        match value {
            ffi::CONTENT_COLOR => Content::Color,
            ffi::CONTENT_ALPHA => Content::Alpha,
            ffi::CONTENT_COLOR_ALPHA => Content::ColorAlpha,
            value => Content::__Unknown(value),
        }
    }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Content::{}",
            match *self {
                Content::Color => "Color",
                Content::Alpha => "Alpha",
                Content::ColorAlpha => "ColorAlpha",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Content, ffi::gobject::cairo_gobject_content_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Extend {
    None,
    Repeat,
    Reflect,
    Pad,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_extend_t> for Extend {
    fn into(self) -> ffi::cairo_extend_t {
        match self {
            Extend::None => ffi::EXTEND_NONE,
            Extend::Repeat => ffi::EXTEND_REPEAT,
            Extend::Reflect => ffi::EXTEND_REFLECT,
            Extend::Pad => ffi::EXTEND_PAD,
            Extend::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_extend_t> for Extend {
    fn from(value: ffi::cairo_extend_t) -> Self {
        match value {
            ffi::EXTEND_NONE => Extend::None,
            ffi::EXTEND_REPEAT => Extend::Repeat,
            ffi::EXTEND_REFLECT => Extend::Reflect,
            ffi::EXTEND_PAD => Extend::Pad,
            value => Extend::__Unknown(value),
        }
    }
}

impl fmt::Display for Extend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Extend::{}",
            match *self {
                Extend::None => "None",
                Extend::Repeat => "Repeat",
                Extend::Reflect => "Reflect",
                Extend::Pad => "Pad",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Extend, ffi::gobject::cairo_gobject_extend_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Filter {
    Fast,
    Good,
    Best,
    Nearest,
    Bilinear,
    Gaussian,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_filter_t> for Filter {
    fn into(self) -> ffi::cairo_filter_t {
        match self {
            Filter::Fast => ffi::FILTER_FAST,
            Filter::Good => ffi::FILTER_GOOD,
            Filter::Best => ffi::FILTER_BEST,
            Filter::Nearest => ffi::FILTER_NEAREST,
            Filter::Bilinear => ffi::FILTER_BILINEAR,
            Filter::Gaussian => ffi::FILTER_GAUSSIAN,
            Filter::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_filter_t> for Filter {
    fn from(value: ffi::cairo_filter_t) -> Self {
        match value {
            ffi::FILTER_FAST => Filter::Fast,
            ffi::FILTER_GOOD => Filter::Good,
            ffi::FILTER_BEST => Filter::Best,
            ffi::FILTER_NEAREST => Filter::Nearest,
            ffi::FILTER_BILINEAR => Filter::Bilinear,
            ffi::FILTER_GAUSSIAN => Filter::Gaussian,
            value => Filter::__Unknown(value),
        }
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Filter::{}",
            match *self {
                Filter::Fast => "Fast",
                Filter::Good => "Good",
                Filter::Best => "Best",
                Filter::Nearest => "Nearest",
                Filter::Bilinear => "Bilinear",
                Filter::Gaussian => "Gaussian",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Filter, ffi::gobject::cairo_gobject_filter_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum PatternType {
    Solid,
    Surface,
    LinearGradient,
    RadialGradient,
    Mesh,
    RasterSource,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_pattern_type_t> for PatternType {
    fn into(self) -> ffi::cairo_pattern_type_t {
        match self {
            PatternType::Solid => ffi::PATTERN_TYPE_SOLID,
            PatternType::Surface => ffi::PATTERN_TYPE_SURFACE,
            PatternType::LinearGradient => ffi::PATTERN_TYPE_LINEAR_GRADIENT,
            PatternType::RadialGradient => ffi::PATTERN_TYPE_RADIAL_GRADIENT,
            PatternType::Mesh => ffi::PATTERN_TYPE_MESH,
            PatternType::RasterSource => ffi::PATTERN_TYPE_RASTER_SOURCE,
            PatternType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_pattern_type_t> for PatternType {
    fn from(value: ffi::cairo_pattern_type_t) -> Self {
        match value {
            ffi::PATTERN_TYPE_SOLID => PatternType::Solid,
            ffi::PATTERN_TYPE_SURFACE => PatternType::Surface,
            ffi::PATTERN_TYPE_LINEAR_GRADIENT => PatternType::LinearGradient,
            ffi::PATTERN_TYPE_RADIAL_GRADIENT => PatternType::RadialGradient,
            ffi::PATTERN_TYPE_MESH => PatternType::Mesh,
            ffi::PATTERN_TYPE_RASTER_SOURCE => PatternType::RasterSource,
            value => PatternType::__Unknown(value),
        }
    }
}

impl fmt::Display for PatternType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PatternType::{}",
            match *self {
                PatternType::Solid => "Solid",
                PatternType::Surface => "Surface",
                PatternType::LinearGradient => "LinearGradient",
                PatternType::RadialGradient => "RadialGradient",
                PatternType::Mesh => "Mesh",
                PatternType::RasterSource => "RasterSource",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(
    PatternType,
    ffi::gobject::cairo_gobject_pattern_type_get_type
);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FontSlant {
    Normal,
    Italic,
    Oblique,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_font_slant_t> for FontSlant {
    fn into(self) -> ffi::cairo_font_slant_t {
        match self {
            FontSlant::Normal => ffi::FONT_SLANT_NORMAL,
            FontSlant::Italic => ffi::FONT_SLANT_ITALIC,
            FontSlant::Oblique => ffi::FONT_SLANT_OBLIQUE,
            FontSlant::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_font_slant_t> for FontSlant {
    fn from(value: ffi::cairo_font_slant_t) -> Self {
        match value {
            ffi::FONT_SLANT_NORMAL => FontSlant::Normal,
            ffi::FONT_SLANT_ITALIC => FontSlant::Italic,
            ffi::FONT_SLANT_OBLIQUE => FontSlant::Oblique,
            value => FontSlant::__Unknown(value),
        }
    }
}

impl fmt::Display for FontSlant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FontSlant::{}",
            match *self {
                FontSlant::Normal => "Normal",
                FontSlant::Italic => "Italic",
                FontSlant::Oblique => "Oblique",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(FontSlant, ffi::gobject::cairo_gobject_font_slant_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FontWeight {
    Normal,
    Bold,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_font_weight_t> for FontWeight {
    fn into(self) -> ffi::cairo_font_weight_t {
        match self {
            FontWeight::Normal => ffi::FONT_WEIGHT_NORMAL,
            FontWeight::Bold => ffi::FONT_WEIGHT_BOLD,
            FontWeight::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_font_weight_t> for FontWeight {
    fn from(value: ffi::cairo_font_weight_t) -> Self {
        match value {
            ffi::FONT_WEIGHT_NORMAL => FontWeight::Normal,
            ffi::FONT_WEIGHT_BOLD => FontWeight::Bold,
            value => FontWeight::__Unknown(value),
        }
    }
}

impl fmt::Display for FontWeight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FontWeight::{}",
            match *self {
                FontWeight::Normal => "Normal",
                FontWeight::Bold => "Bold",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(FontWeight, ffi::gobject::cairo_gobject_font_weight_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum TextClusterFlags {
    None,
    Backward,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_text_cluster_flags_t> for TextClusterFlags {
    fn into(self) -> ffi::cairo_text_cluster_flags_t {
        match self {
            TextClusterFlags::None => ffi::TEXT_CLUSTER_FLAGS_NONE,
            TextClusterFlags::Backward => ffi::TEXT_CLUSTER_FLAGS_BACKWARD,
            TextClusterFlags::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_text_cluster_flags_t> for TextClusterFlags {
    fn from(value: ffi::cairo_text_cluster_flags_t) -> Self {
        match value {
            ffi::TEXT_CLUSTER_FLAGS_NONE => TextClusterFlags::None,
            ffi::TEXT_CLUSTER_FLAGS_BACKWARD => TextClusterFlags::Backward,
            value => TextClusterFlags::__Unknown(value),
        }
    }
}

impl fmt::Display for TextClusterFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TextClusterFlags::{}",
            match *self {
                TextClusterFlags::None => "None",
                TextClusterFlags::Backward => "Backward",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(
    TextClusterFlags,
    ffi::gobject::cairo_gobject_text_cluster_flags_get_type
);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FontType {
    FontTypeToy,
    FontTypeFt,
    FontTypeWin32,
    FontTypeQuartz,
    FontTypeUser,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_font_type_t> for FontType {
    fn into(self) -> ffi::cairo_font_type_t {
        match self {
            FontType::FontTypeToy => ffi::FONT_TYPE_FONT_TYPE_TOY,
            FontType::FontTypeFt => ffi::FONT_TYPE_FONT_TYPE_FT,
            FontType::FontTypeWin32 => ffi::FONT_TYPE_FONT_TYPE_WIN32,
            FontType::FontTypeQuartz => ffi::FONT_TYPE_FONT_TYPE_QUARTZ,
            FontType::FontTypeUser => ffi::FONT_TYPE_FONT_TYPE_USER,
            FontType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_font_type_t> for FontType {
    fn from(value: ffi::cairo_font_type_t) -> Self {
        match value {
            ffi::FONT_TYPE_FONT_TYPE_TOY => FontType::FontTypeToy,
            ffi::FONT_TYPE_FONT_TYPE_FT => FontType::FontTypeFt,
            ffi::FONT_TYPE_FONT_TYPE_WIN32 => FontType::FontTypeWin32,
            ffi::FONT_TYPE_FONT_TYPE_QUARTZ => FontType::FontTypeQuartz,
            ffi::FONT_TYPE_FONT_TYPE_USER => FontType::FontTypeUser,
            value => FontType::__Unknown(value),
        }
    }
}

impl fmt::Display for FontType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FontType::{}",
            match *self {
                FontType::FontTypeToy => "FontTypeToy",
                FontType::FontTypeFt => "FontTypeFt",
                FontType::FontTypeWin32 => "FontTypeWin32",
                FontType::FontTypeQuartz => "FontTypeQuartz",
                FontType::FontTypeUser => "FontTypeUser",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(FontType, ffi::gobject::cairo_gobject_font_type_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum SubpixelOrder {
    Default,
    Rgb,
    Bgr,
    Vrgb,
    Vbgr,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_subpixel_order_t> for SubpixelOrder {
    fn into(self) -> ffi::cairo_subpixel_order_t {
        match self {
            SubpixelOrder::Default => ffi::SUBPIXEL_ORDER_DEFAULT,
            SubpixelOrder::Rgb => ffi::SUBPIXEL_ORDER_RGB,
            SubpixelOrder::Bgr => ffi::SUBPIXEL_ORDER_BGR,
            SubpixelOrder::Vrgb => ffi::SUBPIXEL_ORDER_VRGB,
            SubpixelOrder::Vbgr => ffi::SUBPIXEL_ORDER_VBGR,
            SubpixelOrder::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_subpixel_order_t> for SubpixelOrder {
    fn from(value: ffi::cairo_subpixel_order_t) -> Self {
        match value {
            ffi::SUBPIXEL_ORDER_DEFAULT => SubpixelOrder::Default,
            ffi::SUBPIXEL_ORDER_RGB => SubpixelOrder::Rgb,
            ffi::SUBPIXEL_ORDER_BGR => SubpixelOrder::Bgr,
            ffi::SUBPIXEL_ORDER_VRGB => SubpixelOrder::Vrgb,
            ffi::SUBPIXEL_ORDER_VBGR => SubpixelOrder::Vbgr,
            value => SubpixelOrder::__Unknown(value),
        }
    }
}

impl fmt::Display for SubpixelOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SubpixelOrder::{}",
            match *self {
                SubpixelOrder::Default => "Default",
                SubpixelOrder::Rgb => "Rgb",
                SubpixelOrder::Bgr => "Bgr",
                SubpixelOrder::Vrgb => "Vrgb",
                SubpixelOrder::Vbgr => "Vbgr",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(
    SubpixelOrder,
    ffi::gobject::cairo_gobject_subpixel_order_get_type
);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum HintStyle {
    Default,
    None,
    Slight,
    Medium,
    Full,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_hint_style_t> for HintStyle {
    fn into(self) -> ffi::cairo_hint_style_t {
        match self {
            HintStyle::Default => ffi::HINT_STYLE_DEFAULT,
            HintStyle::None => ffi::HINT_STYLE_NONE,
            HintStyle::Slight => ffi::HINT_STYLE_SLIGHT,
            HintStyle::Medium => ffi::HINT_STYLE_MEDIUM,
            HintStyle::Full => ffi::HINT_STYLE_FULL,
            HintStyle::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_hint_style_t> for HintStyle {
    fn from(value: ffi::cairo_hint_style_t) -> Self {
        match value {
            ffi::HINT_STYLE_DEFAULT => HintStyle::Default,
            ffi::HINT_STYLE_NONE => HintStyle::None,
            ffi::HINT_STYLE_SLIGHT => HintStyle::Slight,
            ffi::HINT_STYLE_MEDIUM => HintStyle::Medium,
            ffi::HINT_STYLE_FULL => HintStyle::Full,
            value => HintStyle::__Unknown(value),
        }
    }
}

impl fmt::Display for HintStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "HintStyle::{}",
            match *self {
                HintStyle::Default => "Default",
                HintStyle::None => "None",
                HintStyle::Slight => "Slight",
                HintStyle::Medium => "Medium",
                HintStyle::Full => "Full",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(HintStyle, ffi::gobject::cairo_gobject_hint_style_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum HintMetrics {
    Default,
    Off,
    On,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_hint_metrics_t> for HintMetrics {
    fn into(self) -> ffi::cairo_hint_metrics_t {
        match self {
            HintMetrics::Default => ffi::HINT_METRICS_DEFAULT,
            HintMetrics::Off => ffi::HINT_METRICS_OFF,
            HintMetrics::On => ffi::HINT_METRICS_ON,
            HintMetrics::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_hint_metrics_t> for HintMetrics {
    fn from(value: ffi::cairo_hint_metrics_t) -> Self {
        match value {
            ffi::HINT_METRICS_DEFAULT => HintMetrics::Default,
            ffi::HINT_METRICS_OFF => HintMetrics::Off,
            ffi::HINT_METRICS_ON => HintMetrics::On,
            value => HintMetrics::__Unknown(value),
        }
    }
}

impl fmt::Display for HintMetrics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "HintMetrics::{}",
            match *self {
                HintMetrics::Default => "Default",
                HintMetrics::Off => "Off",
                HintMetrics::On => "On",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(
    HintMetrics,
    ffi::gobject::cairo_gobject_hint_metrics_get_type
);

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
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_surface_type_t> for SurfaceType {
    fn into(self) -> ffi::cairo_surface_type_t {
        match self {
            SurfaceType::Image => ffi::SURFACE_TYPE_IMAGE,
            SurfaceType::Pdf => ffi::SURFACE_TYPE_PDF,
            SurfaceType::Ps => ffi::SURFACE_TYPE_PS,
            SurfaceType::Xlib => ffi::SURFACE_TYPE_XLIB,
            SurfaceType::Xcb => ffi::SURFACE_TYPE_XCB,
            SurfaceType::Glitz => ffi::SURFACE_TYPE_GLITZ,
            SurfaceType::Quartz => ffi::SURFACE_TYPE_QUARTZ,
            SurfaceType::Win32 => ffi::SURFACE_TYPE_WIN32,
            SurfaceType::BeOs => ffi::SURFACE_TYPE_BE_OS,
            SurfaceType::DirectFb => ffi::SURFACE_TYPE_DIRECT_FB,
            SurfaceType::Svg => ffi::SURFACE_TYPE_SVG,
            SurfaceType::Os2 => ffi::SURFACE_TYPE_OS2,
            SurfaceType::Win32Printing => ffi::SURFACE_TYPE_WIN32_PRINTING,
            SurfaceType::QuartzImage => ffi::SURFACE_TYPE_QUARTZ_IMAGE,
            SurfaceType::Script => ffi::SURFACE_TYPE_SCRIPT,
            SurfaceType::Qt => ffi::SURFACE_TYPE_QT,
            SurfaceType::Recording => ffi::SURFACE_TYPE_RECORDING,
            SurfaceType::Vg => ffi::SURFACE_TYPE_VG,
            SurfaceType::Gl => ffi::SURFACE_TYPE_GL,
            SurfaceType::Drm => ffi::SURFACE_TYPE_DRM,
            SurfaceType::Tee => ffi::SURFACE_TYPE_TEE,
            SurfaceType::Xml => ffi::SURFACE_TYPE_XML,
            SurfaceType::Skia => ffi::SURFACE_TYPE_SKIA,
            SurfaceType::Subsurface => ffi::SURFACE_TYPE_SUBSURFACE,
            SurfaceType::Cogl => ffi::SURFACE_TYPE_COGL,
            SurfaceType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_surface_type_t> for SurfaceType {
    fn from(value: ffi::cairo_surface_type_t) -> Self {
        match value {
            ffi::SURFACE_TYPE_IMAGE => SurfaceType::Image,
            ffi::SURFACE_TYPE_PDF => SurfaceType::Pdf,
            ffi::SURFACE_TYPE_PS => SurfaceType::Ps,
            ffi::SURFACE_TYPE_XLIB => SurfaceType::Xlib,
            ffi::SURFACE_TYPE_XCB => SurfaceType::Xcb,
            ffi::SURFACE_TYPE_GLITZ => SurfaceType::Glitz,
            ffi::SURFACE_TYPE_QUARTZ => SurfaceType::Quartz,
            ffi::SURFACE_TYPE_WIN32 => SurfaceType::Win32,
            ffi::SURFACE_TYPE_BE_OS => SurfaceType::BeOs,
            ffi::SURFACE_TYPE_DIRECT_FB => SurfaceType::DirectFb,
            ffi::SURFACE_TYPE_SVG => SurfaceType::Svg,
            ffi::SURFACE_TYPE_OS2 => SurfaceType::Os2,
            ffi::SURFACE_TYPE_WIN32_PRINTING => SurfaceType::Win32Printing,
            ffi::SURFACE_TYPE_QUARTZ_IMAGE => SurfaceType::QuartzImage,
            ffi::SURFACE_TYPE_SCRIPT => SurfaceType::Script,
            ffi::SURFACE_TYPE_QT => SurfaceType::Qt,
            ffi::SURFACE_TYPE_RECORDING => SurfaceType::Recording,
            ffi::SURFACE_TYPE_VG => SurfaceType::Vg,
            ffi::SURFACE_TYPE_GL => SurfaceType::Gl,
            ffi::SURFACE_TYPE_DRM => SurfaceType::Drm,
            ffi::SURFACE_TYPE_TEE => SurfaceType::Tee,
            ffi::SURFACE_TYPE_XML => SurfaceType::Xml,
            ffi::SURFACE_TYPE_SKIA => SurfaceType::Skia,
            ffi::SURFACE_TYPE_SUBSURFACE => SurfaceType::Subsurface,
            ffi::SURFACE_TYPE_COGL => SurfaceType::Cogl,
            value => SurfaceType::__Unknown(value),
        }
    }
}

impl fmt::Display for SurfaceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SurfaceType::{}",
            match *self {
                SurfaceType::Image => "Image",
                SurfaceType::Pdf => "Pdf",
                SurfaceType::Ps => "Ps",
                SurfaceType::Xlib => "Xlib",
                SurfaceType::Xcb => "Xcb",
                SurfaceType::Glitz => "Glitz",
                SurfaceType::Quartz => "Quartz",
                SurfaceType::Win32 => "Win32",
                SurfaceType::BeOs => "BeOs",
                SurfaceType::DirectFb => "DirectFb",
                SurfaceType::Svg => "Svg",
                SurfaceType::Os2 => "Os2",
                SurfaceType::Win32Printing => "Win32Printing",
                SurfaceType::QuartzImage => "QuartzImage",
                SurfaceType::Script => "Script",
                SurfaceType::Qt => "Qt",
                SurfaceType::Recording => "Recording",
                SurfaceType::Vg => "Vg",
                SurfaceType::Gl => "Gl",
                SurfaceType::Drm => "Drm",
                SurfaceType::Tee => "Tee",
                SurfaceType::Xml => "Xml",
                SurfaceType::Skia => "Skia",
                SurfaceType::Subsurface => "Subsurface",
                SurfaceType::Cogl => "Cogl",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(
    SurfaceType,
    ffi::gobject::cairo_gobject_surface_type_get_type
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg(any(all(feature = "svg", feature = "v1_16"), feature = "dox"))]
pub enum SvgUnit {
    User,
    Em,
    Ex,
    Px,
    In,
    Cm,
    Mm,
    Pt,
    Pc,
    Percent,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
#[cfg(any(all(feature = "svg", feature = "v1_16"), feature = "dox"))]
impl Into<ffi::cairo_svg_unit_t> for SvgUnit {
    fn into(self) -> ffi::cairo_svg_unit_t {
        match self {
            SvgUnit::User => ffi::SVG_UNIT_USER,
            SvgUnit::Em => ffi::SVG_UNIT_EM,
            SvgUnit::Ex => ffi::SVG_UNIT_EX,
            SvgUnit::Px => ffi::SVG_UNIT_PX,
            SvgUnit::In => ffi::SVG_UNIT_IN,
            SvgUnit::Cm => ffi::SVG_UNIT_CM,
            SvgUnit::Mm => ffi::SVG_UNIT_MM,
            SvgUnit::Pt => ffi::SVG_UNIT_PT,
            SvgUnit::Pc => ffi::SVG_UNIT_PC,
            SvgUnit::Percent => ffi::SVG_UNIT_PERCENT,
            SvgUnit::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
#[cfg(any(all(feature = "svg", feature = "v1_16"), feature = "dox"))]
impl From<ffi::cairo_svg_unit_t> for SvgUnit {
    fn from(value: ffi::cairo_svg_unit_t) -> Self {
        match value {
            ffi::SVG_UNIT_USER => SvgUnit::User,
            ffi::SVG_UNIT_EM => SvgUnit::Em,
            ffi::SVG_UNIT_EX => SvgUnit::Ex,
            ffi::SVG_UNIT_PX => SvgUnit::Px,
            ffi::SVG_UNIT_IN => SvgUnit::In,
            ffi::SVG_UNIT_CM => SvgUnit::Cm,
            ffi::SVG_UNIT_MM => SvgUnit::Mm,
            ffi::SVG_UNIT_PT => SvgUnit::Pt,
            ffi::SVG_UNIT_PC => SvgUnit::Pc,
            ffi::SVG_UNIT_PERCENT => SvgUnit::Percent,
            value => SvgUnit::__Unknown(value),
        }
    }
}

#[cfg(any(all(feature = "svg", feature = "v1_16"), feature = "dox"))]
impl fmt::Display for SvgUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SvgUnit::{}",
            match *self {
                SvgUnit::User => "User",
                SvgUnit::Em => "Em",
                SvgUnit::Ex => "Ex",
                SvgUnit::Px => "Px",
                SvgUnit::In => "In",
                SvgUnit::Cm => "Cm",
                SvgUnit::Mm => "Mm",
                SvgUnit::Pt => "Pt",
                SvgUnit::Pc => "Pc",
                SvgUnit::Percent => "Percent",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Invalid,
    ARgb32,
    Rgb24,
    A8,
    A1,
    Rgb16_565,
    Rgb30,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_format_t> for Format {
    fn into(self) -> ffi::cairo_format_t {
        match self {
            Format::Invalid => ffi::FORMAT_INVALID,
            Format::ARgb32 => ffi::FORMAT_A_RGB32,
            Format::Rgb24 => ffi::FORMAT_RGB24,
            Format::A8 => ffi::FORMAT_A8,
            Format::A1 => ffi::FORMAT_A1,
            Format::Rgb16_565 => ffi::FORMAT_RGB16_565,
            Format::Rgb30 => ffi::FORMAT_RGB30,
            Format::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_format_t> for Format {
    fn from(value: ffi::cairo_format_t) -> Self {
        match value {
            ffi::FORMAT_INVALID => Format::Invalid,
            ffi::FORMAT_A_RGB32 => Format::ARgb32,
            ffi::FORMAT_RGB24 => Format::Rgb24,
            ffi::FORMAT_A8 => Format::A8,
            ffi::FORMAT_A1 => Format::A1,
            ffi::FORMAT_RGB16_565 => Format::Rgb16_565,
            ffi::FORMAT_RGB30 => Format::Rgb30,
            value => Format::__Unknown(value),
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Format::{}",
            match *self {
                Format::Invalid => "Invalid",
                Format::ARgb32 => "ARgb32",
                Format::Rgb24 => "Rgb24",
                Format::A8 => "A8",
                Format::A1 => "A1",
                Format::Rgb16_565 => "Rgb16_565",
                Format::Rgb30 => "Rgb30",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Format, ffi::gobject::cairo_gobject_format_get_type);

impl Format {
    pub fn stride_for_width(self, width: u32) -> Result<i32, ()> {
        assert!(width <= i32::MAX as u32);
        let width = width as i32;

        let stride = unsafe { ffi::cairo_format_stride_for_width(self.into(), width) };
        if stride == -1 {
            Err(())
        } else {
            Ok(stride)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionOverlap {
    In,
    Out,
    Part,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_region_overlap_t> for RegionOverlap {
    fn into(self) -> ffi::cairo_region_overlap_t {
        match self {
            RegionOverlap::In => ffi::REGION_OVERLAP_IN,
            RegionOverlap::Out => ffi::REGION_OVERLAP_OUT,
            RegionOverlap::Part => ffi::REGION_OVERLAP_PART,
            RegionOverlap::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_region_overlap_t> for RegionOverlap {
    fn from(value: ffi::cairo_region_overlap_t) -> Self {
        match value {
            ffi::REGION_OVERLAP_IN => RegionOverlap::In,
            ffi::REGION_OVERLAP_OUT => RegionOverlap::Out,
            ffi::REGION_OVERLAP_PART => RegionOverlap::Part,
            value => RegionOverlap::__Unknown(value),
        }
    }
}

impl fmt::Display for RegionOverlap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RegionOverlap::{}",
            match *self {
                RegionOverlap::In => "In",
                RegionOverlap::Out => "Out",
                RegionOverlap::Part => "Part",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(
    RegionOverlap,
    ffi::gobject::cairo_gobject_region_overlap_get_type
);

bitflags! {
    pub struct PdfOutline: i32 {
        const OPEN = ffi::PDF_OUTLINE_FLAG_OPEN;
        const BOLD = ffi::PDF_OUTLINE_FLAG_BOLD;
        const ITALIC = ffi::PDF_OUTLINE_FLAG_ITALIC;
    }
}

#[cfg(any(feature = "pdf", feature = "dox"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdfMetadata {
    Title,
    Author,
    Subject,
    Keywords,
    Creator,
    CreateDate,
    ModDate,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(all(feature = "pdf", feature = "v1_16"), feature = "dox"))]
#[doc(hidden)]
impl Into<ffi::cairo_pdf_metadata_t> for PdfMetadata {
    fn into(self) -> ffi::cairo_pdf_metadata_t {
        match self {
            PdfMetadata::Title => ffi::PDF_METADATA_TITLE,
            PdfMetadata::Author => ffi::PDF_METADATA_AUTHOR,
            PdfMetadata::Subject => ffi::PDF_METADATA_SUBJECT,
            PdfMetadata::Keywords => ffi::PDF_METADATA_KEYWORDS,
            PdfMetadata::Creator => ffi::PDF_METADATA_CREATOR,
            PdfMetadata::CreateDate => ffi::PDF_METADATA_CREATE_DATE,
            PdfMetadata::ModDate => ffi::PDF_METADATA_MOD_DATE,
            PdfMetadata::__Unknown(value) => value,
        }
    }
}

#[cfg(any(all(feature = "pdf", feature = "v1_16"), feature = "dox"))]
#[doc(hidden)]
impl From<ffi::cairo_pdf_metadata_t> for PdfMetadata {
    fn from(value: ffi::cairo_pdf_metadata_t) -> Self {
        match value {
            ffi::PDF_METADATA_TITLE => PdfMetadata::Title,
            ffi::PDF_METADATA_AUTHOR => PdfMetadata::Author,
            ffi::PDF_METADATA_SUBJECT => PdfMetadata::Subject,
            ffi::PDF_METADATA_KEYWORDS => PdfMetadata::Keywords,
            ffi::PDF_METADATA_CREATOR => PdfMetadata::Creator,
            ffi::PDF_METADATA_CREATE_DATE => PdfMetadata::CreateDate,
            ffi::PDF_METADATA_MOD_DATE => PdfMetadata::ModDate,
            value => PdfMetadata::__Unknown(value),
        }
    }
}

#[cfg(any(all(feature = "pdf", feature = "v1_16"), feature = "dox"))]
impl fmt::Display for PdfMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PdfMetadata::{}",
            match *self {
                PdfMetadata::Title => "Title",
                PdfMetadata::Author => "Author",
                PdfMetadata::Subject => "Subject",
                PdfMetadata::Keywords => "Keywords",
                PdfMetadata::Creator => "Creator",
                PdfMetadata::CreateDate => "CreateDate",
                PdfMetadata::ModDate => "ModDate",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(any(feature = "pdf", feature = "dox"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdfVersion {
    _1_4,
    _1_5,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "pdf", feature = "dox"))]
#[doc(hidden)]
impl Into<ffi::cairo_pdf_version_t> for PdfVersion {
    fn into(self) -> ffi::cairo_pdf_version_t {
        match self {
            PdfVersion::_1_4 => ffi::PDF_VERSION__1_4,
            PdfVersion::_1_5 => ffi::PDF_VERSION__1_5,
            PdfVersion::__Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "pdf", feature = "dox"))]
#[doc(hidden)]
impl From<ffi::cairo_pdf_version_t> for PdfVersion {
    fn from(value: ffi::cairo_pdf_version_t) -> Self {
        match value {
            ffi::PDF_VERSION__1_4 => PdfVersion::_1_4,
            ffi::PDF_VERSION__1_5 => PdfVersion::_1_5,
            value => PdfVersion::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "pdf", feature = "dox"))]
impl fmt::Display for PdfVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PdfVersion::{}",
            match *self {
                PdfVersion::_1_4 => "1_4",
                PdfVersion::_1_5 => "1_5",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(any(feature = "svg", feature = "dox"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SvgVersion {
    _1_1,
    _1_2,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "svg", feature = "dox"))]
#[doc(hidden)]
impl Into<ffi::cairo_svg_version_t> for SvgVersion {
    fn into(self) -> ffi::cairo_svg_version_t {
        match self {
            SvgVersion::_1_1 => ffi::SVG_VERSION__1_1,
            SvgVersion::_1_2 => ffi::SVG_VERSION__1_2,
            SvgVersion::__Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "svg", feature = "dox"))]
#[doc(hidden)]
impl From<ffi::cairo_svg_version_t> for SvgVersion {
    fn from(value: ffi::cairo_svg_version_t) -> Self {
        match value {
            ffi::SVG_VERSION__1_1 => SvgVersion::_1_1,
            ffi::SVG_VERSION__1_2 => SvgVersion::_1_2,
            value => SvgVersion::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "svg", feature = "dox"))]
impl fmt::Display for SvgVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SvgVersion::{}",
            match *self {
                SvgVersion::_1_1 => "1_1",
                SvgVersion::_1_2 => "1_2",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(any(feature = "ps", feature = "dox"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PsLevel {
    _2,
    _3,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "ps", feature = "dox"))]
#[doc(hidden)]
impl Into<ffi::cairo_ps_level_t> for PsLevel {
    fn into(self) -> ffi::cairo_ps_level_t {
        match self {
            PsLevel::_2 => ffi::PS_LEVEL__2,
            PsLevel::_3 => ffi::PS_LEVEL__3,
            PsLevel::__Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "ps", feature = "dox"))]
#[doc(hidden)]
impl From<ffi::cairo_ps_level_t> for PsLevel {
    fn from(value: ffi::cairo_ps_level_t) -> Self {
        match value {
            ffi::PS_LEVEL__2 => PsLevel::_2,
            ffi::PS_LEVEL__3 => PsLevel::_3,
            value => PsLevel::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "ps", feature = "dox"))]
impl fmt::Display for PsLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PsLevel::{}",
            match *self {
                PsLevel::_2 => "_2",
                PsLevel::_3 => "_3",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Clone, PartialEq, PartialOrd, Copy, Debug)]
pub enum MeshCorner {
    MeshCorner0,
    MeshCorner1,
    MeshCorner2,
    MeshCorner3,
    #[doc(hidden)]
    __Unknown(u32),
}

#[doc(hidden)]
impl Into<ffi::cairo_mesh_corner_t> for MeshCorner {
    fn into(self) -> ffi::cairo_mesh_corner_t {
        match self {
            MeshCorner::MeshCorner0 => ffi::MESH_CORNER_MESH_CORNER0,
            MeshCorner::MeshCorner1 => ffi::MESH_CORNER_MESH_CORNER1,
            MeshCorner::MeshCorner2 => ffi::MESH_CORNER_MESH_CORNER2,
            MeshCorner::MeshCorner3 => ffi::MESH_CORNER_MESH_CORNER3,
            MeshCorner::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_mesh_corner_t> for MeshCorner {
    fn from(value: ffi::cairo_mesh_corner_t) -> Self {
        match value {
            ffi::MESH_CORNER_MESH_CORNER0 => MeshCorner::MeshCorner0,
            ffi::MESH_CORNER_MESH_CORNER1 => MeshCorner::MeshCorner1,
            ffi::MESH_CORNER_MESH_CORNER2 => MeshCorner::MeshCorner2,
            ffi::MESH_CORNER_MESH_CORNER3 => MeshCorner::MeshCorner3,
            value => MeshCorner::__Unknown(value),
        }
    }
}

impl fmt::Display for MeshCorner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MeshCorner::{}",
            match *self {
                MeshCorner::MeshCorner0 => "MeshCorner0",
                MeshCorner::MeshCorner1 => "MeshCorner1",
                MeshCorner::MeshCorner2 => "MeshCorner2",
                MeshCorner::MeshCorner3 => "MeshCorner3",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FtSynthesize {
    Bold,
    Oblique,
    #[doc(hidden)]
    __Unknown(u32),
}

#[doc(hidden)]
impl Into<ffi::cairo_ft_synthesize_t> for FtSynthesize {
    fn into(self) -> ffi::cairo_ft_synthesize_t {
        match self {
            FtSynthesize::Bold => ffi::CAIRO_FT_SYNTHESIZE_BOLD,
            FtSynthesize::Oblique => ffi::CAIRO_FT_SYNTHESIZE_OBLIQUE,
            FtSynthesize::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_ft_synthesize_t> for FtSynthesize {
    fn from(value: ffi::cairo_ft_synthesize_t) -> Self {
        match value {
            ffi::CAIRO_FT_SYNTHESIZE_BOLD => FtSynthesize::Bold,
            ffi::CAIRO_FT_SYNTHESIZE_OBLIQUE => FtSynthesize::Oblique,
            value => FtSynthesize::__Unknown(value),
        }
    }
}

impl fmt::Display for FtSynthesize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FtSynthesize::{}",
            match *self {
                FtSynthesize::Bold => "Bold",
                FtSynthesize::Oblique => "Oblique",
                FtSynthesize::__Unknown(_) => "Unknown",
            }
        )
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum ScriptMode {
    Ascii,
    Binary,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_script_mode_t> for ScriptMode {
    fn into(self) -> ffi::cairo_script_mode_t {
        match self {
            ScriptMode::Ascii => ffi::CAIRO_SCRIPT_MODE_ASCII,
            ScriptMode::Binary => ffi::CAIRO_SCRIPT_MODE_BINARY,
            ScriptMode::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_script_mode_t> for ScriptMode {
    fn from(value: ffi::cairo_script_mode_t) -> Self {
        match value {
            ffi::CAIRO_SCRIPT_MODE_ASCII => ScriptMode::Ascii,
            ffi::CAIRO_SCRIPT_MODE_BINARY => ScriptMode::Binary,
            value => ScriptMode::__Unknown(value),
        }
    }
}

impl fmt::Display for ScriptMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ScriptMode::{}",
            match *self {
                ScriptMode::Ascii => "Ascii",
                ScriptMode::Binary => "Binary",
                ScriptMode::__Unknown(_) => "Unknown",
            }
        )
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum DeviceType {
    Ascii,
    Binary,
    Script,
    Xcb,
    Xlib,
    Xml,
    Cogl,
    Win32,
    Invalid,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::cairo_device_type_t> for DeviceType {
    fn into(self) -> ffi::cairo_device_type_t {
        match self {
            DeviceType::Ascii => ffi::CAIRO_DEVICE_TYPE_DRM,
            DeviceType::Binary => ffi::CAIRO_DEVICE_TYPE_GL,
            DeviceType::Script => ffi::CAIRO_DEVICE_TYPE_SCRIPT,
            DeviceType::Xcb => ffi::CAIRO_DEVICE_TYPE_XCB,
            DeviceType::Xlib => ffi::CAIRO_DEVICE_TYPE_XLIB,
            DeviceType::Xml => ffi::CAIRO_DEVICE_TYPE_XML,
            DeviceType::Cogl => ffi::CAIRO_DEVICE_TYPE_COGL,
            DeviceType::Win32 => ffi::CAIRO_DEVICE_TYPE_WIN32,
            DeviceType::Invalid => ffi::CAIRO_DEVICE_TYPE_INVALID,
            DeviceType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_device_type_t> for DeviceType {
    fn from(value: ffi::cairo_device_type_t) -> Self {
        match value {
            ffi::CAIRO_DEVICE_TYPE_DRM => DeviceType::Ascii,
            ffi::CAIRO_DEVICE_TYPE_GL => DeviceType::Binary,
            ffi::CAIRO_DEVICE_TYPE_SCRIPT => DeviceType::Script,
            ffi::CAIRO_DEVICE_TYPE_XCB => DeviceType::Xcb,
            ffi::CAIRO_DEVICE_TYPE_XLIB => DeviceType::Xlib,
            ffi::CAIRO_DEVICE_TYPE_XML => DeviceType::Xml,
            ffi::CAIRO_DEVICE_TYPE_COGL => DeviceType::Cogl,
            ffi::CAIRO_DEVICE_TYPE_WIN32 => DeviceType::Win32,
            ffi::CAIRO_DEVICE_TYPE_INVALID => DeviceType::Invalid,
            value => DeviceType::__Unknown(value),
        }
    }
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DeviceType::{}",
            match *self {
                DeviceType::Ascii => "Ascii",
                DeviceType::Binary => "Binary",
                DeviceType::Script => "Script",
                DeviceType::Xcb => "Xcb",
                DeviceType::Xlib => "Xlib",
                DeviceType::Xml => "Xml",
                DeviceType::Cogl => "Cogl",
                DeviceType::Win32 => "Win32",
                DeviceType::Invalid => "Invalid",
                DeviceType::__Unknown(_) => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(DeviceType, ffi::gobject::cairo_gobject_device_type_get_type);

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

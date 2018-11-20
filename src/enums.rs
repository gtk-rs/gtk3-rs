// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::fmt::{Error, Debug};
use std::ffi::CStr;
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
    }
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
impl Into<ffi::CairoStatus> for Status {
    fn into(self) -> ffi::CairoStatus {
        match self {
            Status::Success => 0,
            Status::NoMemory => 1,
            Status::InvalidRestore => 2,
            Status::InvalidPopGroup => 3,
            Status::NoCurrentPoint => 4,
            Status::InvalidMatrix => 5,
            Status::InvalidStatus => 6,
            Status::NullPointer => 7,
            Status::InvalidString => 8,
            Status::InvalidPathData => 9,
            Status::ReadError => 10,
            Status::WriteError => 11,
            Status::SurfaceFinished => 12,
            Status::SurfaceTypeMismatch => 13,
            Status::PatternTypeMismatch => 14,
            Status::InvalidContent => 15,
            Status::InvalidFormat => 16,
            Status::InvalidVisual => 17,
            Status::FileNotFound => 18,
            Status::InvalidDash => 19,
            Status::InvalidDscComment => 20,
            Status::InvalidIndex => 21,
            Status::ClipNotRepresentable => 22,
            Status::TempFileError => 23,
            Status::InvalidStride => 24,
            Status::FontTypeMismatch => 25,
            Status::UserFontImmutable => 26,
            Status::UserFontError => 27,
            Status::NegativeCount => 28,
            Status::InvalidClusters => 29,
            Status::InvalidSlant => 30,
            Status::InvalidWeight => 31,
            Status::InvalidSize => 32,
            Status::UserFontNotImplemented => 33,
            Status::DeviceTypeMismatch => 34,
            Status::DeviceError => 35,
            Status::InvalidMeshConstruction => 36,
            Status::DeviceFinished => 37,
            Status::JBig2GlobalMissing => 38,
            Status::PngError => 39,
            Status::FreetypeError => 40,
            Status::Win32GdiError => 41,
            Status::LastStatus => 42,
            Status::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoStatus> for Status {
    fn from(value: ffi::CairoStatus) -> Self {
        match value {
            0 => Status::Success,
            1 => Status::NoMemory,
            2 => Status::InvalidRestore,
            3 => Status::InvalidPopGroup,
            4 => Status::NoCurrentPoint,
            5 => Status::InvalidMatrix,
            6 => Status::InvalidStatus,
            7 => Status::NullPointer,
            8 => Status::InvalidString,
            9 => Status::InvalidPathData,
            10 => Status::ReadError,
            11 => Status::WriteError,
            12 => Status::SurfaceFinished,
            13 => Status::SurfaceTypeMismatch,
            14 => Status::PatternTypeMismatch,
            15 => Status::InvalidContent,
            16 => Status::InvalidFormat,
            17 => Status::InvalidVisual,
            18 => Status::FileNotFound,
            19 => Status::InvalidDash,
            20 => Status::InvalidDscComment,
            21 => Status::InvalidIndex,
            22 => Status::ClipNotRepresentable,
            23 => Status::TempFileError,
            24 => Status::InvalidStride,
            25 => Status::FontTypeMismatch,
            26 => Status::UserFontImmutable,
            27 => Status::UserFontError,
            28 => Status::NegativeCount,
            29 => Status::InvalidClusters,
            30 => Status::InvalidSlant,
            31 => Status::InvalidWeight,
            32 => Status::InvalidSize,
            33 => Status::UserFontNotImplemented,
            34 => Status::DeviceTypeMismatch,
            35 => Status::DeviceError,
            36 => Status::InvalidMeshConstruction,
            37 => Status::DeviceFinished,
            38 => Status::JBig2GlobalMissing,
            39 => Status::PngError,
            40 => Status::FreetypeError,
            41 => Status::Win32GdiError,
            42 => Status::LastStatus,
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

impl Status {
    pub fn ensure_valid(self) {
        if self != Status::Success {
            panic!("Cairo error {:?}", self)
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
impl Into<ffi::CairoAntialias> for Antialias {
    fn into(self) -> ffi::CairoAntialias {
        match self {
            Antialias::Default => 0,
            Antialias::None => 1,
            Antialias::Gray => 2,
            Antialias::Subpixel => 3,
            Antialias::Fast => 4,
            Antialias::Good => 5,
            Antialias::Best => 6,
            Antialias::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoAntialias> for Antialias {
    fn from(value: ffi::CairoAntialias) -> Self {
        match value {
            0 => Antialias::Default,
            1 => Antialias::None,
            2 => Antialias::Gray,
            3 => Antialias::Subpixel,
            4 => Antialias::Fast,
            5 => Antialias::Good,
            6 => Antialias::Best,
            value => Antialias::__Unknown(value),
        }
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
impl Into<ffi::CairoFillRule> for FillRule {
    fn into(self) -> ffi::CairoFillRule {
        match self {
            FillRule::Winding => 0,
            FillRule::EvenOdd => 1,
            FillRule::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoFillRule> for FillRule {
    fn from(value: ffi::CairoFillRule) -> Self {
        match value {
            0 => FillRule::Winding,
            1 => FillRule::EvenOdd,
            value => FillRule::__Unknown(value),
        }
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
impl Into<ffi::CairoLineCap> for LineCap {
    fn into(self) -> ffi::CairoLineCap {
        match self {
            LineCap::Butt => 0,
            LineCap::Round => 1,
            LineCap::Square => 2,
            LineCap::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoLineCap> for LineCap {
    fn from(value: ffi::CairoLineCap) -> Self {
        match value {
            0 => LineCap::Butt,
            1 => LineCap::Round,
            2 => LineCap::Square,
            value => LineCap::__Unknown(value),
        }
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
impl Into<ffi::CairoLineJoin> for LineJoin {
    fn into(self) -> ffi::CairoLineJoin {
        match self {
            LineJoin::Miter => 0,
            LineJoin::Round => 1,
            LineJoin::Bevel => 2,
            LineJoin::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoLineJoin> for LineJoin {
    fn from(value: ffi::CairoLineJoin) -> Self {
        match value {
            0 => LineJoin::Miter,
            1 => LineJoin::Round,
            2 => LineJoin::Bevel,
            value => LineJoin::__Unknown(value),
        }
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
impl Into<ffi::CairoOperator> for Operator {
    fn into(self) -> ffi::CairoOperator {
        match self {
            Operator::Clear => 0,
            Operator::Source => 1,
            Operator::Over => 2,
            Operator::In => 3,
            Operator::Out => 4,
            Operator::Atop => 5,
            Operator::Dest => 6,
            Operator::DestOver => 7,
            Operator::DestIn => 8,
            Operator::DestOut => 9,
            Operator::DestAtop => 10,
            Operator::Xor => 11,
            Operator::Add => 12,
            Operator::Saturate => 13,
            Operator::Multiply => 14,
            Operator::Screen => 15,
            Operator::Overlay => 16,
            Operator::Darken => 17,
            Operator::Lighten => 18,
            Operator::ColorDodge => 19,
            Operator::ColorBurn => 20,
            Operator::HardLight => 21,
            Operator::SoftLight => 22,
            Operator::Difference => 23,
            Operator::Exclusion => 24,
            Operator::HslHue => 25,
            Operator::HslSaturation => 26,
            Operator::HslColor => 27,
            Operator::HslLuminosity => 28,
            Operator::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoOperator> for Operator {
    fn from(value: ffi::CairoOperator) -> Self {
        match value {
            0 => Operator::Clear,
            1 => Operator::Source,
            2 => Operator::Over,
            3 => Operator::In,
            4 => Operator::Out,
            5 => Operator::Atop,
            6 => Operator::Dest,
            7 => Operator::DestOver,
            8 => Operator::DestIn,
            9 => Operator::DestOut,
            10 => Operator::DestAtop,
            11 => Operator::Xor,
            12 => Operator::Add,
            13 => Operator::Saturate,
            14 => Operator::Multiply,
            15 => Operator::Screen,
            16 => Operator::Overlay,
            17 => Operator::Darken,
            18 => Operator::Lighten,
            19 => Operator::ColorDodge,
            20 => Operator::ColorBurn,
            21 => Operator::HardLight,
            22 => Operator::SoftLight,
            23 => Operator::Difference,
            24 => Operator::Exclusion,
            25 => Operator::HslHue,
            26 => Operator::HslSaturation,
            27 => Operator::HslColor,
            28 => Operator::HslLuminosity,
            value => Operator::__Unknown(value),
        }
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
impl Into<ffi::CairoPathDataType> for PathDataType {
    fn into(self) -> ffi::CairoPathDataType {
        match self {
            PathDataType::MoveTo => 0,
            PathDataType::LineTo => 1,
            PathDataType::CurveTo => 2,
            PathDataType::ClosePath => 3,
            PathDataType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoPathDataType> for PathDataType {
    fn from(value: ffi::CairoPathDataType) -> Self {
        match value {
            0 => PathDataType::MoveTo,
            1 => PathDataType::LineTo,
            2 => PathDataType::CurveTo,
            3 => PathDataType::ClosePath,
            value => PathDataType::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(PathDataType, ffi::gobject::cairo_gobject_path_data_type_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Content {
    Color,
    Alpha,
    ColorAlpha,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::CairoContent> for Content {
    fn into(self) -> ffi::CairoContent {
        match self {
            Content::Color      => 0x1000,
            Content::Alpha      => 0x2000,
            Content::ColorAlpha => 0x3000,
            Content::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoContent> for Content {
    fn from(value: ffi::CairoContent) -> Self {
        match value {
            0x1000 => Content::Color,
            0x2000 => Content::Alpha,
            0x3000 => Content::ColorAlpha,
            value => Content::__Unknown(value),
        }
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
impl Into<ffi::CairoExtend> for Extend {
    fn into(self) -> ffi::CairoExtend {
        match self {
            Extend::None => 0,
            Extend::Repeat => 1,
            Extend::Reflect => 2,
            Extend::Pad => 3,
            Extend::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoExtend> for Extend {
    fn from(value: ffi::CairoExtend) -> Self {
        match value {
            0 => Extend::None,
            1 => Extend::Repeat,
            2 => Extend::Reflect,
            3 => Extend::Pad,
            value => Extend::__Unknown(value),
        }
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
impl Into<ffi::CairoFilter> for Filter {
    fn into(self) -> ffi::CairoFilter {
        match self {
            Filter::Fast => 0,
            Filter::Good => 1,
            Filter::Best => 2,
            Filter::Nearest => 3,
            Filter::Bilinear => 4,
            Filter::Gaussian => 5,
            Filter::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoFilter> for Filter {
    fn from(value: ffi::CairoFilter) -> Self {
        match value {
            0 => Filter::Fast,
            1 => Filter::Good,
            2 => Filter::Best,
            3 => Filter::Nearest,
            4 => Filter::Bilinear,
            5 => Filter::Gaussian,
            value => Filter::__Unknown(value),
        }
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
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    Mesh,
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    RasterSource,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::CairoPatternType> for PatternType {
    fn into(self) -> ffi::CairoPatternType {
        match self {
            PatternType::Solid => 0,
            PatternType::Surface => 1,
            PatternType::LinearGradient => 2,
            PatternType::RadialGradient => 3,
            #[cfg(any(feature = "v1_12", feature = "dox"))]
            PatternType::Mesh => 4,
            #[cfg(any(feature = "v1_12", feature = "dox"))]
            PatternType::RasterSource => 5,
            PatternType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoPatternType> for PatternType {
    fn from(value: ffi::CairoPatternType) -> Self {
        match value {
            0 => PatternType::Solid,
            1 => PatternType::Surface,
            2 => PatternType::LinearGradient,
            3 => PatternType::RadialGradient,
            #[cfg(any(feature = "v1_12", feature = "dox"))]
            4 => PatternType::Mesh,
            #[cfg(any(feature = "v1_12", feature = "dox"))]
            5 => PatternType::RasterSource,
            value => PatternType::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(PatternType, ffi::gobject::cairo_gobject_pattern_type_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FontSlant {
    Normal,
    Italic,
    Oblique,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl Into<ffi::CairoFontSlant> for FontSlant {
    fn into(self) -> ffi::CairoFontSlant {
        match self {
            FontSlant::Normal => 0,
            FontSlant::Italic => 1,
            FontSlant::Oblique => 2,
            FontSlant::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoFontSlant> for FontSlant {
    fn from(value: ffi::CairoFontSlant) -> Self {
        match value {
            0 => FontSlant::Normal,
            1 => FontSlant::Italic,
            2 => FontSlant::Oblique,
            value => FontSlant::__Unknown(value),
        }
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
impl Into<ffi::CairoFontWeight> for FontWeight {
    fn into(self) -> ffi::CairoFontWeight {
        match self {
            FontWeight::Normal => 0,
            FontWeight::Bold => 1,
            FontWeight::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoFontWeight> for FontWeight {
    fn from(value: ffi::CairoFontWeight) -> Self {
        match value {
            0 => FontWeight::Normal,
            1 => FontWeight::Bold,
            value => FontWeight::__Unknown(value),
        }
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
impl Into<ffi::CairoTextClusterFlags> for TextClusterFlags {
    fn into(self) -> ffi::CairoTextClusterFlags {
        match self {
            TextClusterFlags::None     => 0x00000000,
            TextClusterFlags::Backward => 0x00000001,
            TextClusterFlags::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoTextClusterFlags> for TextClusterFlags {
    fn from(value: ffi::CairoTextClusterFlags) -> Self {
        match value {
            0x00000000 => TextClusterFlags::None,
            0x00000001 => TextClusterFlags::Backward,
            value => TextClusterFlags::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(TextClusterFlags, ffi::gobject::cairo_gobject_text_cluster_flags_get_type);

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
impl Into<ffi::CairoFontType> for FontType {
    fn into(self) -> ffi::CairoFontType {
        match self {
            FontType::FontTypeToy => 0,
            FontType::FontTypeFt => 1,
            FontType::FontTypeWin32 => 2,
            FontType::FontTypeQuartz => 3,
            FontType::FontTypeUser => 4,
            FontType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoFontType> for FontType {
    fn from(value: ffi::CairoFontType) -> Self {
        match value {
            0 => FontType::FontTypeToy,
            1 => FontType::FontTypeFt,
            2 => FontType::FontTypeWin32,
            3 => FontType::FontTypeQuartz,
            4 => FontType::FontTypeUser,
            value => FontType::__Unknown(value),
        }
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
impl Into<ffi::CairoSubpixelOrder> for SubpixelOrder {
    fn into(self) -> ffi::CairoSubpixelOrder {
        match self {
            SubpixelOrder::Default => 0,
            SubpixelOrder::Rgb => 1,
            SubpixelOrder::Bgr => 2,
            SubpixelOrder::Vrgb => 3,
            SubpixelOrder::Vbgr => 4,
            SubpixelOrder::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoSubpixelOrder> for SubpixelOrder {
    fn from(value: ffi::CairoSubpixelOrder) -> Self {
        match value {
            0 => SubpixelOrder::Default,
            1 => SubpixelOrder::Rgb,
            2 => SubpixelOrder::Bgr,
            3 => SubpixelOrder::Vrgb,
            4 => SubpixelOrder::Vbgr,
            value => SubpixelOrder::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(SubpixelOrder, ffi::gobject::cairo_gobject_subpixel_order_get_type);

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
impl Into<ffi::CairoHintStyle> for HintStyle {
    fn into(self) -> ffi::CairoHintStyle {
        match self {
            HintStyle::Default => 0,
            HintStyle::None => 1,
            HintStyle::Slight => 2,
            HintStyle::Medium => 3,
            HintStyle::Full => 4,
            HintStyle::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoHintStyle> for HintStyle {
    fn from(value: ffi::CairoHintStyle) -> Self {
        match value {
            0 => HintStyle::Default,
            1 => HintStyle::None,
            2 => HintStyle::Slight,
            3 => HintStyle::Medium,
            4 => HintStyle::Full,
            value => HintStyle::__Unknown(value),
        }
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
impl Into<ffi::CairoHintMetrics> for HintMetrics {
    fn into(self) -> ffi::CairoHintMetrics {
        match self {
            HintMetrics::Default => 0,
            HintMetrics::Off => 1,
            HintMetrics::On => 2,
            HintMetrics::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoHintMetrics> for HintMetrics {
    fn from(value: ffi::CairoHintMetrics) -> Self {
        match value {
            0 => HintMetrics::Default,
            1 => HintMetrics::Off,
            2 => HintMetrics::On,
            value => HintMetrics::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(HintMetrics, ffi::gobject::cairo_gobject_hint_metrics_get_type);

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
impl Into<ffi::CairoSurfaceType> for SurfaceType {
    fn into(self) -> ffi::CairoSurfaceType {
        match self {
            SurfaceType::Image => 0,
            SurfaceType::Pdf => 1,
            SurfaceType::Ps => 2,
            SurfaceType::Xlib => 3,
            SurfaceType::Xcb => 4,
            SurfaceType::Glitz => 5,
            SurfaceType::Quartz => 6,
            SurfaceType::Win32 => 7,
            SurfaceType::BeOs => 8,
            SurfaceType::DirectFb => 9,
            SurfaceType::Svg => 10,
            SurfaceType::Os2 => 11,
            SurfaceType::Win32Printing => 12,
            SurfaceType::QuartzImage => 13,
            SurfaceType::Script => 14,
            SurfaceType::Qt => 15,
            SurfaceType::Recording => 16,
            SurfaceType::Vg => 17,
            SurfaceType::Gl => 18,
            SurfaceType::Drm => 19,
            SurfaceType::Tee => 20,
            SurfaceType::Xml => 21,
            SurfaceType::Skia => 22,
            SurfaceType::Subsurface => 23,
            SurfaceType::Cogl => 24,
            SurfaceType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoSurfaceType> for SurfaceType {
    fn from(value: ffi::CairoSurfaceType) -> Self {
        match value {
            0 => SurfaceType::Image,
            1 => SurfaceType::Pdf,
            2 => SurfaceType::Ps,
            3 => SurfaceType::Xlib,
            4 => SurfaceType::Xcb,
            5 => SurfaceType::Glitz,
            6 => SurfaceType::Quartz,
            7 => SurfaceType::Win32,
            8 => SurfaceType::BeOs,
            9 => SurfaceType::DirectFb,
            10 => SurfaceType::Svg,
            11 => SurfaceType::Os2,
            12 => SurfaceType::Win32Printing,
            13 => SurfaceType::QuartzImage,
            14 => SurfaceType::Script,
            15 => SurfaceType::Qt,
            16 => SurfaceType::Recording,
            17 => SurfaceType::Vg,
            18 => SurfaceType::Gl,
            19 => SurfaceType::Drm,
            20 => SurfaceType::Tee,
            21 => SurfaceType::Xml,
            22 => SurfaceType::Skia,
            23 => SurfaceType::Subsurface,
            24 => SurfaceType::Cogl,
            value => SurfaceType::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(SurfaceType, ffi::gobject::cairo_gobject_surface_type_get_type);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl Into<ffi::CairoSvgUnit> for SvgUnit {
    fn into(self) -> ffi::CairoSvgUnit {
        match self {
            SvgUnit::User => 0,
            SvgUnit::Em => 1,
            SvgUnit::Ex => 2,
            SvgUnit::Px => 3,
            SvgUnit::In => 4,
            SvgUnit::Cm => 5,
            SvgUnit::Mm => 6,
            SvgUnit::Pt => 7,
            SvgUnit::Pc => 8,
            SvgUnit::Percent => 9,
            SvgUnit::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoSvgUnit> for SvgUnit {
    fn from(value: ffi::CairoSvgUnit) -> Self {
        match value {
            0 => SvgUnit::User,
            1 => SvgUnit::Em,
            2 => SvgUnit::Ex,
            3 => SvgUnit::Px,
            4 => SvgUnit::In,
            5 => SvgUnit::Cm,
            6 => SvgUnit::Mm,
            7 => SvgUnit::Pt,
            8 => SvgUnit::Pc,
            9 => SvgUnit::Percent,
            value => SvgUnit::__Unknown(value),
        }
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
impl Into<ffi::CairoFormat> for Format {
    fn into(self) -> ffi::CairoFormat {
        match self {
            Format::Invalid => -1,
            Format::ARgb32 => 0,
            Format::Rgb24 => 1,
            Format::A8 => 2,
            Format::A1 => 3,
            Format::Rgb16_565 => 4,
            Format::Rgb30 => 5,
            Format::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoFormat> for Format {
    fn from(value: ffi::CairoFormat) -> Self {
        match value {
            -1 => Format::Invalid,
            0 => Format::ARgb32,
            1 => Format::Rgb24,
            2 => Format::A8,
            3 => Format::A1,
            4 => Format::Rgb16_565,
            5 => Format::Rgb30,
            value => Format::__Unknown(value),
        }
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
impl Into<ffi::CairoRegionOverlap> for RegionOverlap {
    fn into(self) -> ffi::CairoRegionOverlap {
        match self {
            RegionOverlap::In => 0,
            RegionOverlap::Out => 1,
            RegionOverlap::Part => 2,
            RegionOverlap::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::CairoRegionOverlap> for RegionOverlap {
    fn from(value: ffi::CairoRegionOverlap) -> Self {
        match value {
            0 => RegionOverlap::In,
            1 => RegionOverlap::Out,
            2 => RegionOverlap::Part,
            value => RegionOverlap::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(RegionOverlap, ffi::gobject::cairo_gobject_region_overlap_get_type);

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
impl Into<ffi::CairoPdfVersion> for PdfVersion {
    fn into(self) -> ffi::CairoPdfVersion {
        match self {
            PdfVersion::_1_4 => 0,
            PdfVersion::_1_5 => 1,
            PdfVersion::__Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "pdf", feature = "dox"))]
#[doc(hidden)]
impl From<ffi::CairoPdfVersion> for PdfVersion {
    fn from(value: ffi::CairoPdfVersion) -> Self {
        match value {
            0 => PdfVersion::_1_4,
            1 => PdfVersion::_1_5,
            value => PdfVersion::__Unknown(value),
        }
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
impl Into<ffi::CairoSvgVersion> for SvgVersion {
    fn into(self) -> ffi::CairoSvgVersion {
        match self {
            SvgVersion::_1_1 => 0,
            SvgVersion::_1_2 => 1,
            SvgVersion::__Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "svg", feature = "dox"))]
#[doc(hidden)]
impl From<ffi::CairoSvgVersion> for SvgVersion {
    fn from(value: ffi::CairoSvgVersion) -> Self {
        match value {
            0 => SvgVersion::_1_1,
            1 => SvgVersion::_1_2,
            value => SvgVersion::__Unknown(value),
        }
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
impl Into<ffi::CairoPsLevel> for PsLevel {
    fn into(self) -> ffi::CairoPsLevel {
        match self {
            PsLevel::_2 => 0,
            PsLevel::_3 => 1,
            PsLevel::__Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "ps", feature = "dox"))]
#[doc(hidden)]
impl From<ffi::CairoPsLevel> for PsLevel {
    fn from(value: ffi::CairoPsLevel) -> Self {
        match value {
            0 => PsLevel::_2,
            1 => PsLevel::_3,
            value => PsLevel::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[derive(Clone, PartialEq, PartialOrd, Copy)]
pub enum MeshCorner {
    MeshCorner0,
    MeshCorner1,
    MeshCorner2,
    MeshCorner3,
    #[doc(hidden)]
    __Unknown(u32),
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[doc(hidden)]
impl Into<ffi::CairoMeshCorner> for MeshCorner {
    fn into(self) -> ffi::CairoMeshCorner {
        match self {
            MeshCorner::MeshCorner0 => 0,
            MeshCorner::MeshCorner1 => 1,
            MeshCorner::MeshCorner2 => 2,
            MeshCorner::MeshCorner3 => 3,
            MeshCorner::__Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[doc(hidden)]
impl From<ffi::CairoMeshCorner> for MeshCorner {
    fn from(value: ffi::CairoMeshCorner) -> Self {
        match value {
            0 => MeshCorner::MeshCorner0,
            1 => MeshCorner::MeshCorner1,
            2 => MeshCorner::MeshCorner2,
            3 => MeshCorner::MeshCorner3,
            value => MeshCorner::__Unknown(value),
        }
    }
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

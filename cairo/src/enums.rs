// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt::{self, Debug};
use std::i32;
use std::u32;

use crate::error::Error;

#[cfg(feature = "use_glib")]
use glib::translate::*;

// Helper macro for our GValue related trait impls
#[cfg(feature = "use_glib")]
macro_rules! gvalue_impl {
    ($name:ty, $get_type:expr) => {
        impl glib::types::StaticType for $name {
            fn static_type() -> glib::Type {
                unsafe { from_glib($get_type()) }
            }
        }

        impl glib::value::ValueType for $name {
            type Type = Self;
        }

        unsafe impl<'a> glib::value::FromValue<'a> for $name {
            type Checker = glib::value::GenericValueTypeChecker<Self>;

            unsafe fn from_value(value: &'a glib::Value) -> Self {
                Self::from(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
            }
        }

        impl glib::value::ToValue for $name {
            fn to_value(&self) -> glib::Value {
                let mut value = glib::Value::for_value_type::<Self>();
                unsafe {
                    glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, (*self).into());
                }
                value
            }

            fn value_type(&self) -> glib::Type {
                <Self as glib::StaticType>::static_type()
            }
        }
    };
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
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
impl From<Antialias> for ffi::cairo_antialias_t {
    fn from(val: Antialias) -> ffi::cairo_antialias_t {
        match val {
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
            ffi::ANTIALIAS_DEFAULT => Self::Default,
            ffi::ANTIALIAS_NONE => Self::None,
            ffi::ANTIALIAS_GRAY => Self::Gray,
            ffi::ANTIALIAS_SUBPIXEL => Self::Subpixel,
            ffi::ANTIALIAS_FAST => Self::Fast,
            ffi::ANTIALIAS_GOOD => Self::Good,
            ffi::ANTIALIAS_BEST => Self::Best,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for Antialias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Default => "Default",
                Self::None => "None",
                Self::Gray => "Gray",
                Self::Subpixel => "Subpixel",
                Self::Fast => "Fast",
                Self::Good => "Good",
                Self::Best => "Best",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Antialias, ffi::gobject::cairo_gobject_antialias_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
pub enum FillRule {
    Winding,
    EvenOdd,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<FillRule> for ffi::cairo_fill_rule_t {
    fn from(val: FillRule) -> ffi::cairo_fill_rule_t {
        match val {
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
            ffi::FILL_RULE_WINDING => Self::Winding,
            ffi::FILL_RULE_EVEN_ODD => Self::EvenOdd,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for FillRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Winding => "Winding",
                Self::EvenOdd => "EvenOdd",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(FillRule, ffi::gobject::cairo_gobject_fill_rule_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
pub enum LineCap {
    Butt,
    Round,
    Square,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<LineCap> for ffi::cairo_line_cap_t {
    fn from(val: LineCap) -> ffi::cairo_line_cap_t {
        match val {
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
            ffi::LINE_CAP_BUTT => Self::Butt,
            ffi::LINE_CAP_ROUND => Self::Round,
            ffi::LINE_CAP_SQUARE => Self::Square,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for LineCap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Butt => "Butt",
                Self::Round => "Round",
                Self::Square => "Square",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(LineCap, ffi::gobject::cairo_gobject_line_cap_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<LineJoin> for ffi::cairo_line_join_t {
    fn from(val: LineJoin) -> ffi::cairo_line_join_t {
        match val {
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
            ffi::LINE_JOIN_MITER => Self::Miter,
            ffi::LINE_JOIN_ROUND => Self::Round,
            ffi::LINE_JOIN_BEVEL => Self::Bevel,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for LineJoin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Miter => "Miter",
                Self::Round => "Round",
                Self::Bevel => "Bevel",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(LineJoin, ffi::gobject::cairo_gobject_line_join_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
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
impl From<Operator> for ffi::cairo_operator_t {
    fn from(val: Operator) -> ffi::cairo_operator_t {
        match val {
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
            ffi::OPERATOR_CLEAR => Self::Clear,
            ffi::OPERATOR_SOURCE => Self::Source,
            ffi::OPERATOR_OVER => Self::Over,
            ffi::OPERATOR_IN => Self::In,
            ffi::OPERATOR_OUT => Self::Out,
            ffi::OPERATOR_ATOP => Self::Atop,
            ffi::OPERATOR_DEST => Self::Dest,
            ffi::OPERATOR_DEST_OVER => Self::DestOver,
            ffi::OPERATOR_DEST_IN => Self::DestIn,
            ffi::OPERATOR_DEST_OUT => Self::DestOut,
            ffi::OPERATOR_DEST_ATOP => Self::DestAtop,
            ffi::OPERATOR_XOR => Self::Xor,
            ffi::OPERATOR_ADD => Self::Add,
            ffi::OPERATOR_SATURATE => Self::Saturate,
            ffi::OPERATOR_MULTIPLY => Self::Multiply,
            ffi::OPERATOR_SCREEN => Self::Screen,
            ffi::OPERATOR_OVERLAY => Self::Overlay,
            ffi::OPERATOR_DARKEN => Self::Darken,
            ffi::OPERATOR_LIGHTEN => Self::Lighten,
            ffi::OPERATOR_COLOR_DODGE => Self::ColorDodge,
            ffi::OPERATOR_COLOR_BURN => Self::ColorBurn,
            ffi::OPERATOR_HARD_LIGHT => Self::HardLight,
            ffi::OPERATOR_SOFT_LIGHT => Self::SoftLight,
            ffi::OPERATOR_DIFFERENCE => Self::Difference,
            ffi::OPERATOR_EXCLUSION => Self::Exclusion,
            ffi::OPERATOR_HSL_HUE => Self::HslHue,
            ffi::OPERATOR_HSL_SATURATION => Self::HslSaturation,
            ffi::OPERATOR_HSL_COLOR => Self::HslColor,
            ffi::OPERATOR_HSL_LUMINOSITY => Self::HslLuminosity,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Clear => "Clear",
                Self::Source => "Source",
                Self::Over => "Over",
                Self::In => "In",
                Self::Out => "Out",
                Self::Atop => "Atop",
                Self::Dest => "Dest",
                Self::DestOver => "DestOver",
                Self::DestIn => "DestIn",
                Self::DestOut => "DestOut",
                Self::DestAtop => "DestAtop",
                Self::Xor => "Xor",
                Self::Add => "Add",
                Self::Saturate => "Saturate",
                Self::Multiply => "Multiply",
                Self::Screen => "Screen",
                Self::Overlay => "Overlay",
                Self::Darken => "Darken",
                Self::Lighten => "Lighten",
                Self::ColorDodge => "ColorDodge",
                Self::ColorBurn => "ColorBurn",
                Self::HardLight => "HardLight",
                Self::SoftLight => "SoftLight",
                Self::Difference => "Difference",
                Self::Exclusion => "Exclusion",
                Self::HslHue => "HslHue",
                Self::HslSaturation => "HslSaturation",
                Self::HslColor => "HslColor",
                Self::HslLuminosity => "HslLuminosity",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Operator, ffi::gobject::cairo_gobject_operator_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
pub enum PathDataType {
    MoveTo,
    LineTo,
    CurveTo,
    ClosePath,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<PathDataType> for ffi::cairo_path_data_type_t {
    fn from(val: PathDataType) -> ffi::cairo_path_data_type_t {
        match val {
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
            ffi::PATH_DATA_TYPE_MOVE_TO => Self::MoveTo,
            ffi::PATH_DATA_TYPE_LINE_TO => Self::LineTo,
            ffi::PATH_DATA_TYPE_CURVE_TO => Self::CurveTo,
            ffi::PATH_DATA_TYPE_CLOSE_PATH => Self::ClosePath,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for PathDataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::MoveTo => "MoveTo",
                Self::LineTo => "LineTo",
                Self::CurveTo => "CurveTo",
                Self::ClosePath => "ClosePath",
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
#[non_exhaustive]
pub enum Content {
    Color,
    Alpha,
    ColorAlpha,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<Content> for ffi::cairo_content_t {
    fn from(val: Content) -> ffi::cairo_content_t {
        match val {
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
            ffi::CONTENT_COLOR => Self::Color,
            ffi::CONTENT_ALPHA => Self::Alpha,
            ffi::CONTENT_COLOR_ALPHA => Self::ColorAlpha,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Color => "Color",
                Self::Alpha => "Alpha",
                Self::ColorAlpha => "ColorAlpha",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Content, ffi::gobject::cairo_gobject_content_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
pub enum Extend {
    None,
    Repeat,
    Reflect,
    Pad,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<Extend> for ffi::cairo_extend_t {
    fn from(val: Extend) -> ffi::cairo_extend_t {
        match val {
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
            ffi::EXTEND_NONE => Self::None,
            ffi::EXTEND_REPEAT => Self::Repeat,
            ffi::EXTEND_REFLECT => Self::Reflect,
            ffi::EXTEND_PAD => Self::Pad,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for Extend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::None => "None",
                Self::Repeat => "Repeat",
                Self::Reflect => "Reflect",
                Self::Pad => "Pad",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Extend, ffi::gobject::cairo_gobject_extend_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
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
impl From<Filter> for ffi::cairo_filter_t {
    fn from(val: Filter) -> ffi::cairo_filter_t {
        match val {
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
            ffi::FILTER_FAST => Self::Fast,
            ffi::FILTER_GOOD => Self::Good,
            ffi::FILTER_BEST => Self::Best,
            ffi::FILTER_NEAREST => Self::Nearest,
            ffi::FILTER_BILINEAR => Self::Bilinear,
            ffi::FILTER_GAUSSIAN => Self::Gaussian,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Fast => "Fast",
                Self::Good => "Good",
                Self::Best => "Best",
                Self::Nearest => "Nearest",
                Self::Bilinear => "Bilinear",
                Self::Gaussian => "Gaussian",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Filter, ffi::gobject::cairo_gobject_filter_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
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
impl From<PatternType> for ffi::cairo_pattern_type_t {
    fn from(val: PatternType) -> ffi::cairo_pattern_type_t {
        match val {
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
            ffi::PATTERN_TYPE_SOLID => Self::Solid,
            ffi::PATTERN_TYPE_SURFACE => Self::Surface,
            ffi::PATTERN_TYPE_LINEAR_GRADIENT => Self::LinearGradient,
            ffi::PATTERN_TYPE_RADIAL_GRADIENT => Self::RadialGradient,
            ffi::PATTERN_TYPE_MESH => Self::Mesh,
            ffi::PATTERN_TYPE_RASTER_SOURCE => Self::RasterSource,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for PatternType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Solid => "Solid",
                Self::Surface => "Surface",
                Self::LinearGradient => "LinearGradient",
                Self::RadialGradient => "RadialGradient",
                Self::Mesh => "Mesh",
                Self::RasterSource => "RasterSource",
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
#[non_exhaustive]
pub enum FontSlant {
    Normal,
    Italic,
    Oblique,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<FontSlant> for ffi::cairo_font_slant_t {
    fn from(val: FontSlant) -> ffi::cairo_font_slant_t {
        match val {
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
            ffi::FONT_SLANT_NORMAL => Self::Normal,
            ffi::FONT_SLANT_ITALIC => Self::Italic,
            ffi::FONT_SLANT_OBLIQUE => Self::Oblique,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for FontSlant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Normal => "Normal",
                Self::Italic => "Italic",
                Self::Oblique => "Oblique",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(FontSlant, ffi::gobject::cairo_gobject_font_slant_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
pub enum FontWeight {
    Normal,
    Bold,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<FontWeight> for ffi::cairo_font_weight_t {
    fn from(val: FontWeight) -> ffi::cairo_font_weight_t {
        match val {
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
            ffi::FONT_WEIGHT_NORMAL => Self::Normal,
            ffi::FONT_WEIGHT_BOLD => Self::Bold,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for FontWeight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Normal => "Normal",
                Self::Bold => "Bold",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(FontWeight, ffi::gobject::cairo_gobject_font_weight_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
pub enum TextClusterFlags {
    None,
    Backward,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<TextClusterFlags> for ffi::cairo_text_cluster_flags_t {
    fn from(val: TextClusterFlags) -> ffi::cairo_text_cluster_flags_t {
        match val {
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
            ffi::TEXT_CLUSTER_FLAGS_NONE => Self::None,
            ffi::TEXT_CLUSTER_FLAGS_BACKWARD => Self::Backward,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for TextClusterFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::None => "None",
                Self::Backward => "Backward",
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
#[non_exhaustive]
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
impl From<FontType> for ffi::cairo_font_type_t {
    fn from(val: FontType) -> ffi::cairo_font_type_t {
        match val {
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
            ffi::FONT_TYPE_FONT_TYPE_TOY => Self::FontTypeToy,
            ffi::FONT_TYPE_FONT_TYPE_FT => Self::FontTypeFt,
            ffi::FONT_TYPE_FONT_TYPE_WIN32 => Self::FontTypeWin32,
            ffi::FONT_TYPE_FONT_TYPE_QUARTZ => Self::FontTypeQuartz,
            ffi::FONT_TYPE_FONT_TYPE_USER => Self::FontTypeUser,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for FontType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::FontTypeToy => "FontTypeToy",
                Self::FontTypeFt => "FontTypeFt",
                Self::FontTypeWin32 => "FontTypeWin32",
                Self::FontTypeQuartz => "FontTypeQuartz",
                Self::FontTypeUser => "FontTypeUser",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(FontType, ffi::gobject::cairo_gobject_font_type_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
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
impl From<SubpixelOrder> for ffi::cairo_subpixel_order_t {
    fn from(val: SubpixelOrder) -> ffi::cairo_subpixel_order_t {
        match val {
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
            ffi::SUBPIXEL_ORDER_DEFAULT => Self::Default,
            ffi::SUBPIXEL_ORDER_RGB => Self::Rgb,
            ffi::SUBPIXEL_ORDER_BGR => Self::Bgr,
            ffi::SUBPIXEL_ORDER_VRGB => Self::Vrgb,
            ffi::SUBPIXEL_ORDER_VBGR => Self::Vbgr,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for SubpixelOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Default => "Default",
                Self::Rgb => "Rgb",
                Self::Bgr => "Bgr",
                Self::Vrgb => "Vrgb",
                Self::Vbgr => "Vbgr",
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
#[non_exhaustive]
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
impl From<HintStyle> for ffi::cairo_hint_style_t {
    fn from(val: HintStyle) -> ffi::cairo_hint_style_t {
        match val {
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
            ffi::HINT_STYLE_DEFAULT => Self::Default,
            ffi::HINT_STYLE_NONE => Self::None,
            ffi::HINT_STYLE_SLIGHT => Self::Slight,
            ffi::HINT_STYLE_MEDIUM => Self::Medium,
            ffi::HINT_STYLE_FULL => Self::Full,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for HintStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Default => "Default",
                Self::None => "None",
                Self::Slight => "Slight",
                Self::Medium => "Medium",
                Self::Full => "Full",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(HintStyle, ffi::gobject::cairo_gobject_hint_style_get_type);

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
pub enum HintMetrics {
    Default,
    Off,
    On,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<HintMetrics> for ffi::cairo_hint_metrics_t {
    fn from(val: HintMetrics) -> ffi::cairo_hint_metrics_t {
        match val {
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
            ffi::HINT_METRICS_DEFAULT => Self::Default,
            ffi::HINT_METRICS_OFF => Self::Off,
            ffi::HINT_METRICS_ON => Self::On,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for HintMetrics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Default => "Default",
                Self::Off => "Off",
                Self::On => "On",
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
#[non_exhaustive]
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
impl From<SurfaceType> for ffi::cairo_surface_type_t {
    fn from(val: SurfaceType) -> ffi::cairo_surface_type_t {
        match val {
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
            ffi::SURFACE_TYPE_IMAGE => Self::Image,
            ffi::SURFACE_TYPE_PDF => Self::Pdf,
            ffi::SURFACE_TYPE_PS => Self::Ps,
            ffi::SURFACE_TYPE_XLIB => Self::Xlib,
            ffi::SURFACE_TYPE_XCB => Self::Xcb,
            ffi::SURFACE_TYPE_GLITZ => Self::Glitz,
            ffi::SURFACE_TYPE_QUARTZ => Self::Quartz,
            ffi::SURFACE_TYPE_WIN32 => Self::Win32,
            ffi::SURFACE_TYPE_BE_OS => Self::BeOs,
            ffi::SURFACE_TYPE_DIRECT_FB => Self::DirectFb,
            ffi::SURFACE_TYPE_SVG => Self::Svg,
            ffi::SURFACE_TYPE_OS2 => Self::Os2,
            ffi::SURFACE_TYPE_WIN32_PRINTING => Self::Win32Printing,
            ffi::SURFACE_TYPE_QUARTZ_IMAGE => Self::QuartzImage,
            ffi::SURFACE_TYPE_SCRIPT => Self::Script,
            ffi::SURFACE_TYPE_QT => Self::Qt,
            ffi::SURFACE_TYPE_RECORDING => Self::Recording,
            ffi::SURFACE_TYPE_VG => Self::Vg,
            ffi::SURFACE_TYPE_GL => Self::Gl,
            ffi::SURFACE_TYPE_DRM => Self::Drm,
            ffi::SURFACE_TYPE_TEE => Self::Tee,
            ffi::SURFACE_TYPE_XML => Self::Xml,
            ffi::SURFACE_TYPE_SKIA => Self::Skia,
            ffi::SURFACE_TYPE_SUBSURFACE => Self::Subsurface,
            ffi::SURFACE_TYPE_COGL => Self::Cogl,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for SurfaceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Image => "Image",
                Self::Pdf => "Pdf",
                Self::Ps => "Ps",
                Self::Xlib => "Xlib",
                Self::Xcb => "Xcb",
                Self::Glitz => "Glitz",
                Self::Quartz => "Quartz",
                Self::Win32 => "Win32",
                Self::BeOs => "BeOs",
                Self::DirectFb => "DirectFb",
                Self::Svg => "Svg",
                Self::Os2 => "Os2",
                Self::Win32Printing => "Win32Printing",
                Self::QuartzImage => "QuartzImage",
                Self::Script => "Script",
                Self::Qt => "Qt",
                Self::Recording => "Recording",
                Self::Vg => "Vg",
                Self::Gl => "Gl",
                Self::Drm => "Drm",
                Self::Tee => "Tee",
                Self::Xml => "Xml",
                Self::Skia => "Skia",
                Self::Subsurface => "Subsurface",
                Self::Cogl => "Cogl",
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
#[non_exhaustive]
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
impl From<SvgUnit> for ffi::cairo_svg_unit_t {
    fn from(val: SvgUnit) -> ffi::cairo_svg_unit_t {
        match val {
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
            ffi::SVG_UNIT_USER => Self::User,
            ffi::SVG_UNIT_EM => Self::Em,
            ffi::SVG_UNIT_EX => Self::Ex,
            ffi::SVG_UNIT_PX => Self::Px,
            ffi::SVG_UNIT_IN => Self::In,
            ffi::SVG_UNIT_CM => Self::Cm,
            ffi::SVG_UNIT_MM => Self::Mm,
            ffi::SVG_UNIT_PT => Self::Pt,
            ffi::SVG_UNIT_PC => Self::Pc,
            ffi::SVG_UNIT_PERCENT => Self::Percent,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(any(all(feature = "svg", feature = "v1_16"), feature = "dox"))]
impl fmt::Display for SvgUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::User => "User",
                Self::Em => "Em",
                Self::Ex => "Ex",
                Self::Px => "Px",
                Self::In => "In",
                Self::Cm => "Cm",
                Self::Mm => "Mm",
                Self::Pt => "Pt",
                Self::Pc => "Pc",
                Self::Percent => "Percent",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
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
impl From<Format> for ffi::cairo_format_t {
    fn from(val: Format) -> ffi::cairo_format_t {
        match val {
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
            ffi::FORMAT_INVALID => Self::Invalid,
            ffi::FORMAT_A_RGB32 => Self::ARgb32,
            ffi::FORMAT_RGB24 => Self::Rgb24,
            ffi::FORMAT_A8 => Self::A8,
            ffi::FORMAT_A1 => Self::A1,
            ffi::FORMAT_RGB16_565 => Self::Rgb16_565,
            ffi::FORMAT_RGB30 => Self::Rgb30,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Invalid => "Invalid",
                Self::ARgb32 => "ARgb32",
                Self::Rgb24 => "Rgb24",
                Self::A8 => "A8",
                Self::A1 => "A1",
                Self::Rgb16_565 => "Rgb16_565",
                Self::Rgb30 => "Rgb30",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Format, ffi::gobject::cairo_gobject_format_get_type);

impl Format {
    pub fn stride_for_width(self, width: u32) -> Result<i32, Error> {
        assert!(width <= i32::MAX as u32);
        let width = width as i32;

        let stride = unsafe { ffi::cairo_format_stride_for_width(self.into(), width) };
        if stride == -1 {
            Err(Error::InvalidFormat)
        } else {
            Ok(stride)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum RegionOverlap {
    In,
    Out,
    Part,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<RegionOverlap> for ffi::cairo_region_overlap_t {
    fn from(val: RegionOverlap) -> ffi::cairo_region_overlap_t {
        match val {
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
            ffi::REGION_OVERLAP_IN => Self::In,
            ffi::REGION_OVERLAP_OUT => Self::Out,
            ffi::REGION_OVERLAP_PART => Self::Part,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for RegionOverlap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::In => "In",
                Self::Out => "Out",
                Self::Part => "Part",
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

bitflags::bitflags! {
    pub struct PdfOutline: i32 {
        const OPEN = ffi::PDF_OUTLINE_FLAG_OPEN;
        const BOLD = ffi::PDF_OUTLINE_FLAG_BOLD;
        const ITALIC = ffi::PDF_OUTLINE_FLAG_ITALIC;
    }
}

#[cfg(any(feature = "pdf", feature = "dox"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
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
impl From<PdfMetadata> for ffi::cairo_pdf_metadata_t {
    fn from(val: PdfMetadata) -> ffi::cairo_pdf_metadata_t {
        match val {
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
            ffi::PDF_METADATA_TITLE => Self::Title,
            ffi::PDF_METADATA_AUTHOR => Self::Author,
            ffi::PDF_METADATA_SUBJECT => Self::Subject,
            ffi::PDF_METADATA_KEYWORDS => Self::Keywords,
            ffi::PDF_METADATA_CREATOR => Self::Creator,
            ffi::PDF_METADATA_CREATE_DATE => Self::CreateDate,
            ffi::PDF_METADATA_MOD_DATE => Self::ModDate,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(any(all(feature = "pdf", feature = "v1_16"), feature = "dox"))]
impl fmt::Display for PdfMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Title => "Title",
                Self::Author => "Author",
                Self::Subject => "Subject",
                Self::Keywords => "Keywords",
                Self::Creator => "Creator",
                Self::CreateDate => "CreateDate",
                Self::ModDate => "ModDate",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(any(feature = "pdf", feature = "dox"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PdfVersion {
    _1_4,
    _1_5,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "pdf", feature = "dox"))]
#[doc(hidden)]
impl From<PdfVersion> for ffi::cairo_pdf_version_t {
    fn from(val: PdfVersion) -> ffi::cairo_pdf_version_t {
        match val {
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
            ffi::PDF_VERSION__1_4 => Self::_1_4,
            ffi::PDF_VERSION__1_5 => Self::_1_5,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "pdf", feature = "dox"))]
impl fmt::Display for PdfVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::_1_4 => "1_4",
                Self::_1_5 => "1_5",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(any(feature = "svg", feature = "dox"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SvgVersion {
    _1_1,
    _1_2,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "svg", feature = "dox"))]
#[doc(hidden)]
impl From<SvgVersion> for ffi::cairo_svg_version_t {
    fn from(val: SvgVersion) -> ffi::cairo_svg_version_t {
        match val {
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
            ffi::SVG_VERSION__1_1 => Self::_1_1,
            ffi::SVG_VERSION__1_2 => Self::_1_2,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "svg", feature = "dox"))]
impl fmt::Display for SvgVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::_1_1 => "1_1",
                Self::_1_2 => "1_2",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(any(feature = "ps", feature = "dox"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PsLevel {
    _2,
    _3,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "ps", feature = "dox"))]
#[doc(hidden)]
impl From<PsLevel> for ffi::cairo_ps_level_t {
    fn from(val: PsLevel) -> ffi::cairo_ps_level_t {
        match val {
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
            ffi::PS_LEVEL__2 => Self::_2,
            ffi::PS_LEVEL__3 => Self::_3,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "ps", feature = "dox"))]
impl fmt::Display for PsLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::_2 => "_2",
                Self::_3 => "_3",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Clone, PartialEq, PartialOrd, Copy, Debug)]
#[non_exhaustive]
pub enum MeshCorner {
    MeshCorner0,
    MeshCorner1,
    MeshCorner2,
    MeshCorner3,
    #[doc(hidden)]
    __Unknown(u32),
}

#[doc(hidden)]
impl From<MeshCorner> for ffi::cairo_mesh_corner_t {
    fn from(val: MeshCorner) -> ffi::cairo_mesh_corner_t {
        match val {
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
            ffi::MESH_CORNER_MESH_CORNER0 => Self::MeshCorner0,
            ffi::MESH_CORNER_MESH_CORNER1 => Self::MeshCorner1,
            ffi::MESH_CORNER_MESH_CORNER2 => Self::MeshCorner2,
            ffi::MESH_CORNER_MESH_CORNER3 => Self::MeshCorner3,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for MeshCorner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::MeshCorner0 => "MeshCorner0",
                Self::MeshCorner1 => "MeshCorner1",
                Self::MeshCorner2 => "MeshCorner2",
                Self::MeshCorner3 => "MeshCorner3",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(any(feature = "freetype", feature = "dox"))]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
pub enum FtSynthesize {
    Bold,
    Oblique,
    #[doc(hidden)]
    __Unknown(u32),
}

#[cfg(any(feature = "freetype", feature = "dox"))]
#[doc(hidden)]
impl From<FtSynthesize> for ffi::cairo_ft_synthesize_t {
    fn from(val: FtSynthesize) -> ffi::cairo_ft_synthesize_t {
        match val {
            FtSynthesize::Bold => ffi::CAIRO_FT_SYNTHESIZE_BOLD,
            FtSynthesize::Oblique => ffi::CAIRO_FT_SYNTHESIZE_OBLIQUE,
            FtSynthesize::__Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "freetype", feature = "dox"))]
#[doc(hidden)]
impl From<ffi::cairo_ft_synthesize_t> for FtSynthesize {
    fn from(value: ffi::cairo_ft_synthesize_t) -> Self {
        match value {
            ffi::CAIRO_FT_SYNTHESIZE_BOLD => Self::Bold,
            ffi::CAIRO_FT_SYNTHESIZE_OBLIQUE => Self::Oblique,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "freetype", feature = "dox"))]
impl fmt::Display for FtSynthesize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Bold => "Bold",
                Self::Oblique => "Oblique",
                Self::__Unknown(_) => "Unknown",
            }
        )
    }
}

#[cfg(any(feature = "script", feature = "dox"))]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
pub enum ScriptMode {
    Ascii,
    Binary,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "script", feature = "dox"))]
#[doc(hidden)]
impl From<ScriptMode> for ffi::cairo_script_mode_t {
    fn from(val: ScriptMode) -> ffi::cairo_script_mode_t {
        match val {
            ScriptMode::Ascii => ffi::CAIRO_SCRIPT_MODE_ASCII,
            ScriptMode::Binary => ffi::CAIRO_SCRIPT_MODE_BINARY,
            ScriptMode::__Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "script", feature = "dox"))]
#[doc(hidden)]
impl From<ffi::cairo_script_mode_t> for ScriptMode {
    fn from(value: ffi::cairo_script_mode_t) -> Self {
        match value {
            ffi::CAIRO_SCRIPT_MODE_ASCII => Self::Ascii,
            ffi::CAIRO_SCRIPT_MODE_BINARY => Self::Binary,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "script", feature = "dox"))]
impl fmt::Display for ScriptMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Ascii => "Ascii",
                Self::Binary => "Binary",
                Self::__Unknown(_) => "Unknown",
            }
        )
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
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
impl From<DeviceType> for ffi::cairo_device_type_t {
    fn from(val: DeviceType) -> ffi::cairo_device_type_t {
        match val {
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
            ffi::CAIRO_DEVICE_TYPE_DRM => Self::Ascii,
            ffi::CAIRO_DEVICE_TYPE_GL => Self::Binary,
            ffi::CAIRO_DEVICE_TYPE_SCRIPT => Self::Script,
            ffi::CAIRO_DEVICE_TYPE_XCB => Self::Xcb,
            ffi::CAIRO_DEVICE_TYPE_XLIB => Self::Xlib,
            ffi::CAIRO_DEVICE_TYPE_XML => Self::Xml,
            ffi::CAIRO_DEVICE_TYPE_COGL => Self::Cogl,
            ffi::CAIRO_DEVICE_TYPE_WIN32 => Self::Win32,
            ffi::CAIRO_DEVICE_TYPE_INVALID => Self::Invalid,
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Self::{}",
            match *self {
                Self::Ascii => "Ascii",
                Self::Binary => "Binary",
                Self::Script => "Script",
                Self::Xcb => "Xcb",
                Self::Xlib => "Xlib",
                Self::Xml => "Xml",
                Self::Cogl => "Cogl",
                Self::Win32 => "Win32",
                Self::Invalid => "Invalid",
                Self::__Unknown(_) => "Unknown",
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

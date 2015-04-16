// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::fmt::{Error, Debug};
use std::ffi::CStr;

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Copy)]
pub enum Status {
    StatusSuccess = 0,

    StatusNoMemory,
    StatusInvalidRestore,
    StatusInvalidPopGroup,
    StatusNoCurrentPoint,
    StatusInvalidMatrix,
    StatusInvalidStatus,
    StatusNullPointer,
    StatusInvalidString,
    StatusInvalidPathData,
    StatusReadError,
    StatusWriteError,
    StatusSurfaceFinished,
    StatusSurfaceTypeMismatch,
    StatusPatternTypeMismatch,
    StatusInvalidContent,
    StatusInvalidFormat,
    StatusInvalidVisual,
    StatusFileNotFound,
    StatusInvalidDash,
    StatusInvalidDscComment,
    StatusInvalidIndex,
    StatusClipNotRepresentable,
    StatusTempFileError,
    StatusInvalidStride,
    StatusFontTypeMismatch,
    StatusUserFontImmutable,
    StatusUserFontError,
    StatusNegativeCount,
    StatusInvalidClusters,
    StatusInvalidSlant,
    StatusInvalidWeight,
    StatusInvalidSize,
    StatusUserFontNotImplemented,
    StatusDeviceTypeMismatch,
    StatusDeviceError,
    StatusInvalidMeshConstruction,
    StatusDeviceFinished,

    StatusLastStatus
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
    pub fn ensure_valid(&self) {
        if *self != Status::StatusSuccess {
            panic!("Cairo error {:?}", *self)
        }
    }
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Antialias {
    AntialiasDefault,

    /* method */
    AntialiasNone,
    AntialiasGray,
    AntialiasSubpixel,

    /* hints */
    AntialiasFast,
    AntialiasGood,
    AntialiasBest
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FillRule {
    FillRuleWinding,
    FillRuleEvenOdd
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum LineCap {
    LineCapButt,
    LineCapRound,
    LineCapSquare
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum LineJoin {
    LineJoinMiter,
    LineJoinRound,
    LineJoinBevel
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Operator {
    OperatorClear,

    OperatorSource,
    OperatorOver,
    OperatorIn,
    OperatorOut,
    OperatorAtop,

    OperatorDest,
    OperatorDestOver,
    OperatorDestIn,
    OperatorDestOut,
    OperatorDestAtop,

    OperatorXor,
    OperatorAdd,
    OperatorSaturate,

    OperatorMultiply,
    OperatorScreen,
    OperatorOverlay,
    OperatorDarken,
    OperatorLighten,
    OperatorColorDodge,
    OperatorColorBurn,
    OperatorHardLight,
    OperatorSoftLight,
    OperatorDifference,
    OperatorExclusion,
    OperatorHslHue,
    OperatorHslSaturation,
    OperatorHslColor,
    OperatorHslLuminosity
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum PathDataType {
    PathMoveTo,
    PathLineTo,
    PathCurveTo,
    PathClosePath
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Content {
    ContentColor      = 0x1000,
    ContentAlpha      = 0x2000,
    ContentColorAlpha = 0x3000
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Extend {
    ExtendNone,
    ExtendRepeat,
    ExtendReflect,
    ExtendPad
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Filter {
    FilterFast,
    FilterGood,
    FilterBest,
    FilterNearest,
    FilterBilinear,
    FilterGaussian
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum PatternType {
    PatternTypeSolid,
    PatternTypeSurface,
    PatternTypeLinearGradient,
    PatternTypeRadialGradient,
    #[cfg(feature = "cairo_1_12")]
    PatternTypeMesh,
    #[cfg(feature = "cairo_1_12")]
    PatternTypeRasterSource
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FontSlant {
    FontSlantNormal,
    FontSlantItalic,
    FontSlantOblique
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FontWeight {
    FontWeightNormal,
    FontWeightBold
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum TextClusterFlags {
    TextClusterFlagNone     = 0x00000000,
    TextClusterFlagBackward = 0x00000001
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FontType {
    FontTypeToy,
    FontTypeFt,
    FontTypeWin32,
    FontTypeQuartz,
    FontTypeUser
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum SubpixelOrder {
    SubpixelOrderDefault,
    SubpixelOrderRgb,
    SubpixelOrderBgr,
    SubpixelOrderVrgb,
    SubpixelOrderVbgr
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum HintStyle {
    HintStyleDefault,
    HintStyleNone,
    HintStyleSlight,
    HintStyleMedium,
    HintStyleFull
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum HintMetrics {
    HintMetricsDefault,
    HintMetricsOff,
    HintMetricsOn
}

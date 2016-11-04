// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::fmt::{Error, Debug};
use std::ffi::CStr;

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
    // JBig2GlobalMissing,
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
    pub fn ensure_valid(&self) {
        if *self != Status::Success {
            panic!("Cairo error {:?}", *self)
        }
    }
}

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

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FillRule {
    Winding,
    EvenOdd
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum LineCap {
    Butt,
    Round,
    Square
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel
}

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

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum PathDataType {
    MoveTo,
    LineTo,
    CurveTo,
    ClosePath
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Content {
    Color      = 0x1000,
    Alpha      = 0x2000,
    ColorAlpha = 0x3000
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Extend {
    None,
    Repeat,
    Reflect,
    Pad
}

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

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum PatternType {
    Solid,
    Surface,
    LinearGradient,
    RadialGradient,
    #[cfg(feature = "v1_12")]
    Mesh,
    #[cfg(feature = "v1_12")]
    RasterSource
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FontSlant {
    Normal,
    Italic,
    Oblique
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FontWeight {
    Normal,
    Bold
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum TextClusterFlags {
    None     = 0x00000000,
    Backward = 0x00000001
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
    Default,
    Rgb,
    Bgr,
    Vrgb,
    Vbgr
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum HintStyle {
    Default,
    None,
    Slight,
    Medium,
    Full
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum HintMetrics {
    Default,
    Off,
    On
}

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

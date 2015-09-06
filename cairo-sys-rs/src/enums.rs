// Copyright 2013-2015, The RGtk Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::fmt::{Error, Debug};
use std::ffi::CStr;

/// Status is used to indicate errors that can occur when using Cairo. In some cases it is
/// returned directly by functions, but when using `Context`, the last error, if any, is
/// stored in the context and can be retrieved with `Context::status()`.
///
/// New entries may be added in future versions. Use `Context::status_to_string()` to get a
/// human-readable representation of an error message.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// no error has occurred (Since 1.0)
    Success = 0,

    /// out of memory (Since 1.0)
    NoMemory,
    /// `Context::restore()` called without matching `Context::save()` (Since 1.0)
    InvalidRestore,
    /// no saved group to pop, i.e. `Context::pop_group()` without matching
    /// `Context::push_group()` (Since 1.0)
    InvalidPopGroup,
    /// no current point defined (Since 1.0)
    NoCurrentPoint,
    /// invalid matrix (not invertible) (Since 1.0)
    InvalidMatrix,
    /// invalid value for an input `Status` (Since 1.0)
    InvalidStatus,
    /// NULL pointer (Since 1.0)
    NullPointer,
    /// input string not valid UTF-8 (Since 1.0)
    InvalidString,
    /// input path data not valid (Since 1.0)
    InvalidPathData,
    /// error while reading from input stream (Since 1.0)
    ReadError,
    /// error while writing to output stream (Since 1.0)
    WriteError,
    /// target surface has been finished (Since 1.0)
    SurfaceFinished,
    /// the surface type is not appropriate for the operation (Since 1.0)
    SurfaceTypeMismatch,
    /// the pattern type is not appropriate for the operation (Since 1.0)
    PatternTypeMismatch,
    /// invalid value for an input `Content` (Since 1.0)
    InvalidContent,
    /// invalid value for an input `Format` (Since 1.0)
    InvalidFormat,
    /// invalid value for an input Visual* (Since 1.0)
    InvalidVisual,
    /// file not found (Since 1.0)
    FileNotFound,
    /// invalid value for a dash setting (Since 1.0)
    InvalidDash,
    /// invalid value for a DSC comment (Since 1.2)
    InvalidDscComment,
    /// invalid index passed to getter (Since 1.4)
    InvalidIndex,
    /// clip region not representable in desired format (Since 1.4)
    ClipNotRepresentable,
    /// error creating or writing to a temporary file (Since 1.6)
    TempFileError,
    /// invalid value for stride (Since 1.6)
    InvalidStride,
    /// the font type is not appropriate for the operation (Since 1.8)
    FontTypeMismatch,
    /// the user-font is immutable (Since 1.8)
    UserFontImmutable,
    /// error occurred in a user-font callback function (Since 1.8)
    UserFontError,
    /// negative number used where it is not allowed (Since 1.8)
    NegativeCount,
    /// input clusters do not represent the accompanying text and glyph array (Since 1.8)
    InvalidClusters,
    /// invalid value for an input `FontSlant` (Since 1.8)
    InvalidSlant,
    /// invalid value for an input `FontWeight` (Since 1.8)
    InvalidWeight,
    /// invalid value (typically too big) for the size of the input (surface, pattern,
    /// etc.) (Since 1.10)
    InvalidSize,
    /// user-font method not implemented (Since 1.10)
    UserFontNotImplemented,
    /// the device type is not appropriate for the operation (Since 1.10)
    DeviceTypeMismatch,
    /// an operation to the device caused an unspecified error (Since 1.10)
    DeviceError,
    /// a mesh pattern construction operation was used outside of a
    /// `Context::mesh_pattern_begin_patch()`/`Context::mesh_pattern_end_patch()`
    /// pair (Since 1.12)
    InvalidMeshConstruction,
    /// target device has been finished (Since 1.12)
    DeviceFinished,
    // CAIRO_MIME_TYPE_JBIG2_GLOBAL_ID has been used on at least one image but no
    // image provided `JBig2Global` (Since 1.14)
    // JBig2GlobalMissing,
    /// this is a special value indicating the number of status values defined in this
    /// enumeration. When using this value, note that the version of cairo at run-time
    /// may have additional status values defined than the value of this symbol at
    /// compile-time. (Since 1.10)
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

/// Specifies the type of antialiasing to do when rendering text or shapes.
///
/// As it is not necessarily clear from the above what advantages a particular antialias method
/// provides, since 1.12, there is also a set of hints:
/// `Fast`: Allow the backend to degrade raster quality for speed.
/// `Good`: A balance between speed and quality.
/// `Best`: A high-fidelity, but potentially slow, raster mode.
///
/// These make no guarantee on how the backend will perform its rasterisation (if it even
/// rasterises!), nor that they have any differing effect other than to enable some form of
/// antialiasing. In the case of glyph rendering, `Fast` and `Good` will be mapped to `Gray`, with
/// `Best` being equivalent to `Subpixel`.
///
/// The interpretation of `Default` is left entirely up to the backend, typically this will be
/// similar to `Good`.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Antialias {
    /// Use the default antialiasing for the subsystem and target device, since 1.0
    Default,

    /* method */
    /// Use a bilevel alpha mask, since 1.0
    None,
    /// Perform single-color antialiasing (using shades of gray for black text on a white
    /// background, for example), since 1.0
    Gray,
    /// Perform antialiasing by taking advantage of the order of subpixel elements on devices
    /// such as LCD panels, since 1.0
    Subpixel,

    /* hints */
    /// Hint that the backend should perform some antialiasing but prefer speed over quality,
    /// since 1.12
    Fast,
    /// The backend should balance quality against performance, since 1.12
    Good,
    /// Hint that the backend should render at the highest quality, sacrificing speed if
    /// necessary, since 1.12
    Best
}

/// `FillRule` is used to select how paths are filled. For both fill rules, whether or not
/// a point is included in the fill is determined by taking a ray from that point to infinity
/// and looking at intersections with the path. The ray can be in any direction, as long as
/// it doesn't pass through the end point of a segment or have a tricky intersection such as
/// intersecting tangent to the path. (Note that filling is not actually implemented in this
/// way. This is just a description of the rule that is applied.)
///
/// The default fill rule is `Winding`.
///
/// New entries may be added in future versions.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FillRule {
    /// If the path crosses the ray from left-to-right, counts +1. If the path crosses the ray
    /// from right to left, counts -1. (Left and right are determined from the perspective of
    /// looking along the ray from the starting point.) If the total count is non-zero, the point
    /// will be filled. (Since 1.0)
    Winding,
    /// Counts the total number of intersections, without regard to the orientation of the contour.
    /// If the total number of intersections is odd, the point will be filled. (Since 1.0)
    EvenOdd
}

/// Specifies how to render the endpoints of the path when stroking.
///
/// The default line cap style is `Butt`.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum LineCap {
    /// start(stop) the line exactly at the start(end) point (Since 1.0)
    Butt,
    /// use a round ending, the center of the circle is the end point (Since 1.0)
    Round,
    /// use squared ending, the center of the square is the end point (Since 1.0)
    Square
}

/// Specifies how to render the junction of two lines when stroking.
///
/// The default line join style is `Miter`.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum LineJoin {
    /// use a sharp (angled) corner, see `Context::set_miter_limit()` (Since 1.0)
    Miter,
    /// use a rounded join, the center of the circle is the joint point (Since 1.0)
    Round,
    /// use a cut-off join, the join is cut off at half the line width from the
    /// joint point (Since 1.0)
    nBevel
}

/// `Operator` is used to set the compositing operator for all cairo drawing operations.
///
/// The default operator is `Over`.
///
/// The operators marked as unbounded modify their destination even outside of the mask layer
/// (that is, their effect is not bound by the mask layer). However, their effect can still be
/// limited by way of clipping.
///
/// To keep things simple, the operator descriptions here document the behavior for when both
/// source and destination are either fully transparent or fully opaque. The actual implementation
/// works for translucent layers too. For a more detailed explanation of the effects of each
/// operator, including the mathematical definitions, see http://cairographics.org/operators/.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Operator {
    /// clear destination layer (bounded) (Since 1.0)
    Clear,

    /// replace destination layer (bounded) (Since 1.0)
    Source,
    /// draw source layer on top of destination layer (bounded) (Since 1.0)
    Over,
    /// draw source where there was destination content (unbounded) (Since 1.0)
    In,
    /// draw source where there was no destination content (unbounded) (Since 1.0)
    Out,
    /// draw source on top of destination content and only there (Since 1.0)
    Atop,

    /// ignore the source (Since 1.0)
    Dest,
    /// draw destination on top of source (Since 1.0)
    DestOver,
    /// leave destination only where there was source content (unbounded) (Since 1.0)
    DestIn,
    /// leave destination only where there was no source content (Since 1.0)
    DestOut,
    /// leave destination on top of source content and only there (unbounded) (Since 1.0)
    DestAtop,

    /// source and destination are shown where there is only one of them (Since 1.0)
    Xor,
    /// source and destination layers are accumulated (Since 1.0)
    Add,
    /// like over, but assuming source and dest are disjoint geometries (Since 1.0)
    Saturate,

    /// source and destination layers are multiplied. This causes the result to be at
    /// least as dark as the darker inputs. (Since 1.10)
    Multiply,
    /// source and destination are complemented and multiplied. This causes the result
    /// to be at least as light as the lighter inputs. (Since 1.10)
    Screen,
    /// multiplies or screens, depending on the lightness of the destination color.
    /// (Since 1.10)
    Overlay,
    /// replaces the destination with the source if it is darker, otherwise keeps the
    /// source. (Since 1.10)
    Darken,
    /// replaces the destination with the source if it is lighter, otherwise keeps the
    /// source. (Since 1.10)
    Lighten,
    /// brightens the destination color to reflect the source color. (Since 1.10)
    ColorDodge,
    /// darkens the destination color to reflect the source color. (Since 1.10)
    ColorBurn,
    /// Multiplies or screens, dependent on source color. (Since 1.10)
    HardLight,
    /// Darkens or lightens, dependent on source color. (Since 1.10)
    SoftLight,
    /// Takes the difference of the source and destination color. (Since 1.10)
    Difference,
    /// Produces an effect similar to difference, but with lower contrast. (Since 1.10)
    Exclusion,
    /// Creates a color with the hue of the source and the saturation and luminosity of
    /// the target. (Since 1.10)
    HslHue,
    /// Creates a color with the saturation of the source and the hue and luminosity of
    /// the target. Painting with this mode onto a gray area produces no change. (Since 1.10)
    HslSaturation,
    /// Creates a color with the hue and saturation of the source and the luminosity of the
    /// target. This preserves the gray levels of the target and is useful for coloring
    /// monochrome images or tinting color images. (Since 1.10)
    HslColor,
    /// Creates a color with the luminosity of the source and the hue and saturation of the
    /// target. This produces an inverse effect to `HslColor`. (Since 1.10)
    HslLuminosity
}

/// `PathData` is used to describe the type of one portion of a path when represented as a `Path`.
/// See `PathData` for details.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum PathDataType {
    /// A move-to operation, since 1.0
    MoveTo,
    /// A line-to operation, since 1.0
    LineTo,
    /// A curve-to operation, since 1.0
    CurveTo,
    /// A close-path operation, since 1.0
    ClosePath
}

/// `Content` is used to describe the content that a surface will contain, whether color
/// information, alpha information (translucence vs. opacity), or both.
///
/// Note: The large values here are designed to keep `Content` values distinct from `Format`
/// values so that the implementation can detect the error if users confuse the two types.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Content {
    /// The surface will hold color content only. (Since 1.0)
    Color      = 0x1000,
    /// The surface will hold alpha content only. (Since 1.0)
    Alpha      = 0x2000,
    /// The surface will hold color and alpha content. (Since 1.0)
    ColorAlpha = 0x3000
}

/// `Extend` is used to describe how pattern color/alpha will be determined for areas
/// "outside" the pattern's natural area, (for example, outside the surface bounds or
/// outside the gradient geometry).
///
/// Mesh patterns are not affected by the extend mode.
///
/// The default extend mode is `None` for surface patterns and `Pad` for gradient
/// patterns.
///
/// New entries may be added in future versions.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Extend {
    /// pixels outside of the source pattern are fully transparent (Since 1.0)
    None,
    /// the pattern is tiled by repeating (Since 1.0)
    Repeat,
    /// the pattern is tiled by reflecting at the edges (Since 1.0; but only implemented
    /// for surface patterns since 1.6)
    Reflect,
    /// pixels outside of the pattern copy the closest pixel from the source (Since 1.2;
    /// but only implemented for surface patterns since 1.6)
    Pad
}

/// `Filter` is used to indicate what filtering should be applied when reading pixel values
/// from patterns. See `Pattern::set_filter()` for indicating the desired filter to be used
/// with a particular pattern.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Filter {
    /// A high-performance filter, with quality similar to `Nearest` (Since 1.0)
    Fast,
    /// A reasonable-performance filter, with quality similar to `Bilinear` (Since 1.0)
    Good,
    /// The highest-quality available, performance may not be suitable for interactive
    /// use. (Since 1.0)
    Best,
    /// Nearest-neighbor filtering (Since 1.0)
    Nearest,
    /// Linear interpolation in two dimensions (Since 1.0)
    Bilinear,
    /// This filter value is currently unimplemented, and should not be used in current
    /// code. (Since 1.0)
    Gaussian
}

/// `PatternType` is used to describe the type of a given pattern.
///
/// The type of a pattern is determined by the function used to create it. The
/// `Pattern::create_rgb()` and `Pattern::create_rgba()` functions create `Solid` patterns.
/// The remaining cairo_pattern_create functions map to pattern types in obvious ways.
///
/// The pattern type can be queried with `Pattern::get_type()`.
///
/// Most `Pattern` functions can be called with a pattern of any type, (though trying to
/// change the extend or filter for a solid pattern will have no effect). A notable exception
/// is `Pattern::add_color_stop_rgb()` and `Pattern::add_color_stop_rgba()` which must only be
/// called with gradient patterns (either `Linear` or `Radial`). Otherwise the pattern will be
/// shutdown and put into an error state.
///
/// New entries may be added in future versions.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum PatternType {
    /// The pattern is a solid (uniform) color. It may be opaque or translucent, since 1.2.
    Solid,
    /// The pattern is a based on a surface (an image), since 1.2.
    Surface,
    /// The pattern is a linear gradient, since 1.2.
    LinearGradient,
    /// The pattern is a radial gradient, since 1.2.
    RadialGradient,
    /// The pattern is a mesh, since 1.12.
    #[cfg(cairo_1_12)]
    Mesh,
    /// The pattern is a user pattern providing raster data, since 1.12.
    #[cfg(cairo_1_12)]
    RasterSource
}

/// Specifies variants of a font face based on their slant.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FontSlant {
    /// Upright font style, since 1.0
    Normal,
    /// Italic font style, since 1.0
    Italic,
    /// Oblique font style, since 1.0
    Oblique
}

/// Specifies variants of a font face based on their weight.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FontWeight {
    /// Normal font weight, since 1.0
    Normal,
    /// Bold font weight, since 1.0
    Bold
}

/// Specifies properties of a text cluster mapping.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum TextClusterFlags {
    None     = 0x00000000,
    /// The clusters in the cluster array map to glyphs in the glyph array from end
    /// to start. (Since 1.8)
    Backward = 0x00000001
}

/// `FontType` is used to describe the type of a given font face or scaled font. The
/// font types are also known as "font backends" within cairo.
///
/// The type of a font face is determined by the function used to create it, which will
/// generally be of the form `Context::type_font_face_create()`. The font face type can
/// be queried with `Context::font_face_get_type()`.
///
/// The various cairo_font_face_t functions can be used with a font face of any type.
///
/// The type of a scaled font is determined by the type of the font face passed to
/// `Context::scaled_font_create()`. The scaled font type can be queried with
/// `Context::scaled_font_get_type()`.
///
/// The various `ScaledFont` functions can be used with scaled fonts of any type, but some
/// font backends also provide type-specific functions that must only be called with a
/// scaled font of the appropriate type. These functions have names that begin with
/// `Context::type_scaled_font()` such as `Context::ft_scaled_font_lock_face()`.
///
/// The behavior of calling a type-specific function with a scaled font of the wrong type
/// is undefined.
///
/// New entries may be added in future versions.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FontType {
    /// The font was created using cairo's toy font api (Since: 1.2)
    FontTypeToy,
    /// The font is of type FreeType (Since: 1.2)
    FontTypeFt,
    /// The font is of type Win32 (Since: 1.2)
    FontTypeWin32,
    /// The font is of type Quartz (Since: 1.6, in 1.2 and 1.4 it was named
    /// CAIRO_FONT_TYPE_ATSUI)
    FontTypeQuartz,
    /// The font was create using cairo's user font api (Since: 1.8)
    FontTypeUser
}

/// The subpixel order specifies the order of color elements within each pixel on the display
/// device when rendering with an antialiasing mode of `Antialias::Subpixel`.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum SubpixelOrder {
    /// Use the default subpixel order for for the target device, since 1.0
    Default,
    /// Subpixel elements are arranged horizontally with red at the left, since 1.0
    Rgb,
    /// Subpixel elements are arranged horizontally with blue at the left, since 1.0
    Bgr,
    /// Subpixel elements are arranged vertically with red at the top, since 1.0
    Vrgb,
    /// Subpixel elements are arranged vertically with blue at the top, since 1.0
    Vbgr
}

/// Specifies the type of hinting to do on font outlines. Hinting is the process of fitting
/// outlines to the pixel grid in order to improve the appearance of the result. Since hinting
/// outlines involves distorting them, it also reduces the faithfulness to the original outline
/// shapes. Not all of the outline hinting styles are supported by all font backends.
///
/// New entries may be added in future versions.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum HintStyle {
    /// Use the default hint style for font backend and target device, since 1.0
    Default,
    /// Do not hint outlines, since 1.0
    None,
    /// Hint outlines slightly to improve contrast while retaining good fidelity to the
    /// original shapes, since 1.0
    Slight,
    /// Hint outlines with medium strength giving a compromise between fidelity to the
    /// original shapes and contrast, since 1.0
    Medium,
    /// Hint outlines to maximize contrast, since 1.0
    Full
}

/// Specifies whether to hint font metrics; hinting font metrics means quantizing them so
/// that they are integer values in device space. Doing this improves the consistency of
/// letter and line spacing, however it also means that text will be laid out differently
/// at different zoom factors
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum HintMetrics {
    /// Hint metrics in the default manner for the font backend and target device, since 1.0
    Default,
    /// Do not hint font metrics, since 1.0
    Off,
    /// Hint font metrics, since 1.0
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

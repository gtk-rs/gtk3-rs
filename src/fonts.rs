// Copyright 2013-2015, The RGtk Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! FontOptions: How a font should be rendered.
//!
//! FontFace: Base class for font faces.
//!
//! ScaledFont: Font face at particular size and options.

use glib::translate::*;
use std::clone::Clone;
use std::cmp::PartialEq;
use std::ops::Drop;
use ffi;

use ffi::enums::{
    Antialias,
    SubpixelOrder,
    HintStyle,
    HintMetrics,

    FontType,
    FontWeight,
    FontSlant,
};
use ::matrices::Matrix;
use ffi::{
    cairo_font_options_t,
    cairo_font_face_t,
    cairo_scaled_font_t
};

pub use ffi::{
    FontExtents,
    Glyph,
    TextCluster,
    TextExtents
};

/* TODO
 Allocates an array of cairo_glyph_t's. This function is only useful in
 implementations of cairo_user_scaled_font_text_to_glyphs_func_t where the user
 needs to allocate an array of glyphs that cairo will free. For all other uses,
 user can use their own allocation method for glyphs.


impl Glyph {

    //pub fn cairo_glyph_allocate(num_glyphs: c_int) -> *Glyph;

    //pub fn cairo_glyph_free(glyphs: *Glyph);
}

 Allocates an array of cairo_glyph_t's. This function is only useful in
 implementations of cairo_user_scaled_font_text_to_glyphs_func_t where the user
 needs to allocate an array of glyphs that cairo will free. For all other uses,
 user can use their own allocation method for glyphs.

impl TextCluster {
    //pub fn cairo_text_cluster_allocate(num_clusters: c_int) -> *TextCluster;

    //pub fn cairo_text_cluster_free(clusters: *TextCluster);
}
*/


pub struct FontOptions(*mut cairo_font_options_t);

/// The font options specify how fonts should be rendered. Most of the time the font options
/// implied by a surface are just right and do not need any changes, but for pixel-based targets
/// tweaking font options may result in superior output on a particular display.
impl FontOptions {
    /// Allocates a new font options object with all options initialized to default values.
    pub fn new() -> FontOptions {
        let font_options = unsafe {
            FontOptions(ffi::cairo_font_options_create())
        };
        font_options.ensure_status();
        font_options
    }

    #[doc(hidden)]
    pub fn get_ptr(&self) -> *mut cairo_font_options_t {
        let FontOptions(ptr) = *self;
        ptr
    }

    /// Checks whether an error has previously occurred for this font options object.
    pub fn ensure_status(&self) {
        let status = unsafe {
            ffi::cairo_font_options_status(self.get_ptr())
        };
        status.ensure_valid()
    }

    /// Merges non-default options from other into self, replacing existing values.
    /// This operation can be thought of as somewhat similar to compositing other onto
    /// self with the operation of Operator::Over.
    pub fn merge(&mut self, other: &mut FontOptions) {
        unsafe {
            ffi::cairo_font_options_merge(self.get_ptr(), other.get_ptr())
        }
    }

    /// Compute a hash for the font options object; this value will be useful when
    /// storing an object containing a FontOptions in a hash table.
    pub fn hash(&self) -> u64{
        unsafe {
            ffi::cairo_font_options_hash(self.get_ptr()) as u64
        }
    }

    /// Sets the antialiasing mode for the font options object. This specifies the type
    /// of antialiasing to do when rendering text.
    pub fn set_antialias(&self, antialias: Antialias) {
        unsafe {
            ffi::cairo_font_options_set_antialias(self.get_ptr(), antialias)
        }
    }

    /// Gets the antialiasing mode for the font options object.
    pub fn get_antialias(&self) -> Antialias {
        unsafe {
            ffi::cairo_font_options_get_antialias(self.get_ptr())
        }
    }

    /// Sets the subpixel order for the font options object. The subpixel order specifies
    /// the order of color elements within each pixel on the display device when rendering
    /// with an antialiasing mode of Antialias::Subpixel. See the documentation for
    /// SubpixelOrder for full details.
    pub fn set_subpixel_order(&self, order: SubpixelOrder) {
        unsafe {
            ffi::cairo_font_options_set_subpixel_order(self.get_ptr(), order)
        }
    }

    /// Gets the subpixel order for the font options object. See the documentation for
    /// SubpixelOrder for full details.
    pub fn get_subpixel_order(&self) -> SubpixelOrder {
        unsafe {
            ffi::cairo_font_options_get_subpixel_order(self.get_ptr())
        }
    }

    /// Sets the hint style for font outlines for the font options object. This controls
    /// whether to fit font outlines to the pixel grid, and if so, whether to optimize
    /// for fidelity or contrast. See the documentation for HintStyle for full
    /// details.
    pub fn set_hint_style(&self, hint_style: HintStyle) {
        unsafe {
            ffi::cairo_font_options_set_hint_style(self.get_ptr(), hint_style)
        }
    }

    /// Gets the hint style for font outlines for the font options object. See the
    /// documentation for HintStyle for full details.
    pub fn get_hint_style(&self) -> HintStyle {
        unsafe {
            ffi::cairo_font_options_get_hint_style(self.get_ptr())
        }
    }

    /// Sets the metrics hinting mode for the font options object. This controls
    /// whether metrics are quantized to integer values in device units. See the
    /// documentation for HintMetrics for full details.
    pub fn set_hint_metrics(&self, hint_metrics: HintMetrics) {
        unsafe {
            ffi::cairo_font_options_set_hint_metrics(self.get_ptr(), hint_metrics)
        }
    }

    /// Gets the metrics hinting mode for the font options object. See the documentation
    /// for HintMetrics for full details.
    pub fn get_hint_metrics(&self) -> HintMetrics {
        unsafe {
            ffi::cairo_font_options_get_hint_metrics(self.get_ptr())
        }
    }
}

impl PartialEq for FontOptions {
    /// Compares two font options objects for equality.
    fn eq(&self, other: &FontOptions) -> bool {
        unsafe {
            ffi::cairo_font_options_equal(self.get_ptr(), other.get_ptr()).as_bool()
        }
    }
}

impl Clone for FontOptions {
    fn clone(&self) -> FontOptions {
        unsafe {
            FontOptions(ffi::cairo_font_options_copy(self.get_ptr()))
        }
    }
}

impl Drop for FontOptions {
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_font_options_destroy(self.get_ptr())
        }
    }
}

/// FontFace represents a particular font at a particular weight, slant, and other
/// characteristic but no size, transformation, or size.
///
/// Font faces are created using font-backend-specific constructors, typically of the
/// form Context::backend_font_face_create(), or implicitly using the toy text API by
/// way of Context::select_font_face(). The resulting face can be accessed using
/// Context::get_font_face().
pub struct FontFace(pub *mut cairo_font_face_t);

impl FontFace {
    #[doc(hidden)]
    pub fn get_ptr(&self) -> *mut cairo_font_face_t {
        let FontFace(ptr) = *self;
        ptr
    }

    /// Creates a font face from a triplet of family, slant, and weight. These font faces
    /// are used in implementation of the the cairo "toy" font API.
    ///
    /// If family is the zero-length string "", the platform-specific default family is assumed.
    /// The default family then can be queried using FontFace::toy_get_family().
    ///
    /// The Context::select_font_face() function uses this to create font faces. See that
    /// function for limitations and other details of toy font faces.
    pub fn toy_create(family: &str, slant: FontSlant, weight: FontWeight) -> FontFace {
        let font_face = FontFace(
            unsafe {
                ffi::cairo_toy_font_face_create(family.to_glib_none().0, slant, weight)
            }
        );
        font_face.ensure_status();
        font_face
    }

    /// Gets the familly name of a toy font.
    pub fn toy_get_family(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::cairo_toy_font_face_get_family(self.get_ptr()))
        }
    }

    /// Gets the slant a toy font.
    pub fn toy_get_slant(&self) -> FontSlant {
        unsafe {
            ffi::cairo_toy_font_face_get_slant(self.get_ptr())
        }
    }

    /// Gets the weight a toy font.
    pub fn toy_get_weight(&self) -> FontWeight {
        unsafe {
            ffi::cairo_toy_font_face_get_weight(self.get_ptr())
        }
    }

    /// Checks whether an error has previously occurred for this font face.
    pub fn ensure_status(&self) {
        let status = unsafe {
            ffi::cairo_font_face_status(self.get_ptr())
        };
        status.ensure_valid()
    }

    /// This function returns the type of the backend used to create a font face. See
    /// FontType for available types.
    pub fn get_type(&self) -> FontType {
        unsafe {
            ffi::cairo_font_face_get_type(self.get_ptr())
        }
    }

    /// Returns the current reference count of self.
    pub fn get_reference_count(&self) -> usize {
        unsafe {
            ffi::cairo_font_face_get_reference_count(self.get_ptr()) as usize
        }
    }

    /// Increases the reference count on self by one. This prevents self from being
    /// destroyed until a matching call to FontFace drop trait is made.
    ///
    /// The number of references to a FontFace can be get using
    /// FontFace::get_reference_count().
    pub fn reference(&self) -> FontFace {
        unsafe {
            FontFace(ffi::cairo_font_face_reference(self.get_ptr()))
        }
    }
}

impl Drop for FontFace {
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_font_face_destroy(self.get_ptr())
        }
    }
}

/// ScaledFont represents a realization of a font face at a particular size and
/// transformation and a certain set of font options.
pub struct ScaledFont(pub *mut cairo_scaled_font_t);

impl ScaledFont {
    #[doc(hidden)]
    pub fn get_ptr(&self) -> *mut cairo_scaled_font_t {
        let ScaledFont(ptr) = *self;
        ptr
    }

    /// Creates a ScaledFont object from a font face and matrices that describe the
    /// size of the font and the environment in which it will be used.
    pub fn new(font_face: FontFace, font_matrix: &mut Matrix, ctm: &mut Matrix, options: FontOptions) -> ScaledFont {
        let scaled_font = unsafe {
            ScaledFont(ffi::cairo_scaled_font_create(font_face.get_ptr(), font_matrix, ctm, options.get_ptr()))
        };
        scaled_font.ensure_status();
        scaled_font
    }

    /// Checks whether an error has previously occurred for this ScaledFont.
    pub fn ensure_status(&self) {
        let status = unsafe {
            ffi::cairo_scaled_font_status(self.get_ptr())
        };
        status.ensure_valid()
    }

    /// This function returns the type of the backend used to create a scaled font.
    /// See FontType for available types. However, this function never returns
    /// FontType::Toy.
    pub fn get_type(&self) -> FontType {
        unsafe {
            ffi::cairo_scaled_font_get_type(self.get_ptr())
        }
    }

    /// Returns the current reference count of self.
    pub fn get_reference_count(&self) -> usize {
        unsafe {
            ffi::cairo_scaled_font_get_reference_count(self.get_ptr()) as usize
        }
    }

    //pub fn cairo_scaled_font_extents(scaled_font: *mut cairo_scaled_font_t, extents: *mut cairo_font_extents_t);

    //                    cairo_text_extents_t;
    //pub fn cairo_scaled_font_text_extents(scaled_font: *mut cairo_scaled_font_t, utf8: *mut char, extents: *mut cairo_text_extents_t);

    //pub fn cairo_scaled_font_glyph_extents(scaled_font: *mut cairo_scaled_font_t, glyphs: *mut Glyph, num_glyphs: c_int, extents: *mut cairo_text_extents_t);

    //pub fn cairo_scaled_font_text_to_glyphs(scaled_font: *mut cairo_scaled_font_t, x: c_double, y: c_double, utf8: *mut char, utf8_len: c_int, glyphs: **mut Glyph, num_glyphs: *mut c_int, clusters: **mut TextCluster, num_clusters: *mut c_int, cluster_flags: *mut TextClusterFlags) -> Status;

    //pub fn cairo_scaled_font_get_font_face(scaled_font: *mut cairo_scaled_font_t) -> *mut cairo_font_face_t;

    //pub fn cairo_scaled_font_get_font_options(scaled_font: *mut cairo_scaled_font_t, options: *mut cairo_font_options_t);

    //pub fn cairo_scaled_font_get_font_matrix(scaled_font: *mut cairo_scaled_font_t, font_matrix: *mut cairo_matrix_t);

    //pub fn cairo_scaled_font_get_ctm(scaled_font: *mut cairo_scaled_font_t, ctm: *mut cairo_matrix_t);

    //pub fn cairo_scaled_font_get_scale_matrix(scaled_font: *mut cairo_scaled_font_t, scale_matrix: *cairo_matrix_t);

    /// Increases the reference count on self by one. This prevents self from being
    /// destroyed until a matching call to ScaledFont drop trait is made.
    ///
    /// The number of references to a cairo_scaled_font_t can be get using
    /// ScaledFont::get_reference_count().
    pub fn reference(&self) -> ScaledFont {
        unsafe {
            ScaledFont(ffi::cairo_scaled_font_reference(self.get_ptr()))
        }
    }
}

impl Drop for ScaledFont {
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_scaled_font_destroy(self.get_ptr())
        }
    }
}

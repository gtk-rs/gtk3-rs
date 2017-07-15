#[cfg(feature = "use_glib")]
use glib::translate::*;
#[cfg(feature = "use_glib")]
use glib_ffi;
use std::ptr;
#[cfg(feature = "use_glib")]
use std::mem;
use ffi;
use std::ffi::CString;

use ffi::enums::{
    FontType,
    TextClusterFlags,
};
use ::matrices::{
    Matrix,
    MatrixTrait
};
use ffi::{
    FontExtents,
    Glyph,
    TextCluster,
    TextExtents
};

use super::{FontFace, FontOptions};

#[cfg(feature = "use_glib")]
glib_wrapper! {
    pub struct ScaledFont(Shared<ffi::cairo_scaled_font_t>);

    match fn {
        ref => |ptr| ffi::cairo_scaled_font_reference(ptr),
        unref => |ptr| ffi::cairo_scaled_font_destroy(ptr),
    }
}

#[cfg(not(feature = "use_glib"))]
pub struct ScaledFont(*mut ffi::cairo_scaled_font_t);

impl ScaledFont {
    pub fn new(font_face: FontFace, font_matrix: &Matrix, ctm: &Matrix, options: &FontOptions) -> ScaledFont {
        let scaled_font: ScaledFont = unsafe {
            ScaledFont::from_raw_full(ffi::cairo_scaled_font_create(font_face.to_raw_none(), font_matrix, ctm, options.to_raw_none()))
        };
        scaled_font.ensure_status();
        scaled_font
    }

    #[cfg(feature = "use_glib")]
    #[doc(hidden)]
    pub fn to_raw_none(&self) -> *mut ffi::cairo_scaled_font_t {
        self.to_glib_none().0
    }

    #[cfg(not(feature = "use_glib"))]
    #[doc(hidden)]
    pub fn to_raw_none(&self) -> *mut ffi::cairo_scaled_font_t {
        self.0
    }

    #[cfg(not(feature = "use_glib"))]
    #[doc(hidden)]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_scaled_font_t) -> ScaledFont {
        assert!(!ptr.is_null());
        ScaledFont(ptr)
    }

    #[cfg(feature = "use_glib")]
    #[doc(hidden)]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_scaled_font_t) -> ScaledFont {
        from_glib_full(ptr)
    }

    #[cfg(feature = "use_glib")]
    #[doc(hidden)]
    pub unsafe fn from_raw_none(ptr: *mut ffi::cairo_scaled_font_t) -> ScaledFont {
        from_glib_none(ptr)
    }

    #[cfg(not(feature = "use_glib"))]
    #[doc(hidden)]
    pub unsafe fn from_raw_none(ptr: *mut ffi::cairo_scaled_font_t) -> ScaledFont {
        assert!(!ptr.is_null());
        ffi::cairo_scaled_font_reference(ptr);
        ScaledFont(ptr)
    }

    pub fn ensure_status(&self) {
        let status = unsafe {
            ffi::cairo_scaled_font_status(self.to_raw_none())
        };
        status.ensure_valid()
    }

    pub fn get_type(&self) -> FontType {
        unsafe {
            ffi::cairo_scaled_font_get_type(self.to_raw_none())
        }
    }

    pub fn get_reference_count(&self) -> usize {
        unsafe {
            ffi::cairo_scaled_font_get_reference_count(self.to_raw_none()) as usize
        }
    }

    pub fn extents(&self) -> FontExtents {
        let mut extents = FontExtents {
            ascent: 0.0,
            descent: 0.0,
            height: 0.0,
            max_x_advance: 0.0,
            max_y_advance: 0.0,
        };

        unsafe {
            ffi::cairo_scaled_font_extents(self.to_raw_none(), &mut extents)
        }

        extents
    }

    pub fn text_extents(&self, text: &str) -> TextExtents {
        let mut extents = TextExtents {
            x_bearing: 0.0,
            y_bearing: 0.0,
            width: 0.0,
            height: 0.0,
            x_advance: 0.0,
            y_advance: 0.0,
        };

        let text = CString::new(text).unwrap();
        unsafe {
            ffi::cairo_scaled_font_text_extents(self.to_raw_none(), text.as_ptr(), &mut extents)
        }

        extents
    }

    pub fn glyph_extents(&self, glyphs: &[Glyph]) -> TextExtents {
        let mut extents = TextExtents {
            x_bearing: 0.0,
            y_bearing: 0.0,
            width: 0.0,
            height: 0.0,
            x_advance: 0.0,
            y_advance: 0.0,
        };

        unsafe {
            ffi::cairo_scaled_font_glyph_extents(self.to_raw_none(), glyphs.as_ptr(), glyphs.len() as i32, &mut extents)
        }

        extents
    }

    pub fn text_to_glyphs(&self, x: f64, y: f64, text: &str) -> (Vec<Glyph>, Vec<TextCluster>) {
        // This large unsafe block is due to the FFI function returning two specially allocated
        // (cairo_{glyph,text_cluster}_allocate) pointers that need to be copied into Vec<T>
        // types before they're of any use to Rust code.

        unsafe {
            let mut glyphs_ptr: *mut Glyph = ptr::null_mut();
            let mut glyph_count = 0i32;
            let mut clusters_ptr: *mut TextCluster = ptr::null_mut();
            let mut cluster_count = 0i32;
            let mut cluster_flags = TextClusterFlags::None;
            let text_length = text.len() as i32;
            let text = CString::new(text).unwrap();

            let status = ffi::cairo_scaled_font_text_to_glyphs(
                self.to_raw_none(),
                x,
                y,
                text.as_ptr(),
                text_length,
                &mut glyphs_ptr,
                &mut glyph_count,
                &mut clusters_ptr,
                &mut cluster_count,
                &mut cluster_flags);

            status.ensure_valid();

            let glyph_count = glyph_count as usize;
            let glyphs: Vec<Glyph> = {
                let mut glyphs: Vec<Glyph> = Vec::with_capacity(glyph_count);

                glyphs.set_len(glyph_count);
                ptr::copy(glyphs_ptr, glyphs.as_mut_ptr(), glyph_count);

                glyphs
            };

            let cluster_count = cluster_count as usize;
            let clusters: Vec<TextCluster> = {
                let mut clusters = Vec::with_capacity(cluster_count);

                clusters.set_len(cluster_count);
                ptr::copy(clusters_ptr, clusters.as_mut_ptr(), cluster_count);

                clusters
            };

            ffi::cairo_glyph_free(glyphs_ptr);
            ffi::cairo_text_cluster_free(clusters_ptr);

            (glyphs, clusters)
        }
    }

    pub fn get_font_face(&self) -> FontFace {
        unsafe {
            FontFace::from_raw_none(ffi::cairo_scaled_font_get_font_face(self.to_raw_none()))
        }
    }

    pub fn get_font_options(&self) -> FontOptions {
        let options = FontOptions::new();

        unsafe {
            ffi::cairo_scaled_font_get_font_options(self.to_raw_none(), options.to_raw_none())
        }

        options
    }

    pub fn get_font_matrix(&self) -> Matrix {
        let mut matrix = Matrix::null();

        unsafe {
            ffi::cairo_scaled_font_get_font_matrix(self.to_raw_none(), &mut matrix)
        }

        matrix
    }

    pub fn get_ctm(&self) -> Matrix {
        let mut matrix = Matrix::null();

        unsafe {
            ffi::cairo_scaled_font_get_ctm(self.to_raw_none(), &mut matrix)
        }

        matrix
    }

    pub fn get_scale_matrix(&self) -> Matrix {
        let mut matrix = Matrix::null();

        unsafe {
            ffi::cairo_scaled_font_get_scale_matrix(self.to_raw_none(), &mut matrix)
        }

        matrix
    }
}

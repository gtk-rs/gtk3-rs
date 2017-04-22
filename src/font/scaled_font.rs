use glib::translate::*;
use std::ops::Drop;
use std::ptr;
use ffi;

use ffi::enums::{
    FontType,
    TextClusterFlags,
};
use ::matrices::{
    Matrix,
    MatrixTrait
};
use ffi::cairo_scaled_font_t;
use ffi::{
    FontExtents,
    Glyph,
    TextCluster,
    TextExtents
};

use super::{FontFace, FontOptions};

pub struct ScaledFont(*mut cairo_scaled_font_t);

impl ScaledFont {
    #[doc(hidden)]
    pub fn get_ptr(&self) -> *mut cairo_scaled_font_t {
        let ScaledFont(ptr) = *self;
        ptr
    }

    pub fn new(font_face: FontFace, font_matrix: &mut Matrix, ctm: &mut Matrix, options: FontOptions) -> ScaledFont {
        let scaled_font = unsafe {
            ScaledFont(ffi::cairo_scaled_font_create(font_face.get_ptr(), font_matrix, ctm, options.get_ptr()))
        };
        scaled_font.ensure_status();
        scaled_font
    }

    pub fn ensure_status(&self) {
        let status = unsafe {
            ffi::cairo_scaled_font_status(self.get_ptr())
        };
        status.ensure_valid()
    }

    pub fn get_type(&self) -> FontType {
        unsafe {
            ffi::cairo_scaled_font_get_type(self.get_ptr())
        }
    }

    pub fn get_reference_count(&self) -> usize {
        unsafe {
            ffi::cairo_scaled_font_get_reference_count(self.get_ptr()) as usize
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
            ffi::cairo_scaled_font_extents(self.get_ptr(), &mut extents)
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

        unsafe {
            ffi::cairo_scaled_font_text_extents(self.get_ptr(), text.to_glib_none().0, &mut extents)
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
            ffi::cairo_scaled_font_glyph_extents(self.get_ptr(), glyphs.as_ptr(), glyphs.len() as i32, &mut extents)
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

            let status = ffi::cairo_scaled_font_text_to_glyphs(
                self.get_ptr(),
                x,
                y,
                text.to_glib_none().0,
                text.len() as i32,
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
            from_glib_none(ffi::cairo_scaled_font_get_font_face(self.get_ptr()))
        }
    }

    pub fn get_font_options(&self) -> FontOptions {
        let options = FontOptions::new();

        unsafe {
            ffi::cairo_scaled_font_get_font_options(self.get_ptr(), options.get_ptr())
        }

        options
    }

    pub fn get_font_matrix(&self) -> Matrix {
        let mut matrix = Matrix::null();

        unsafe {
            ffi::cairo_scaled_font_get_font_matrix(self.get_ptr(), &mut matrix)
        }

        matrix
    }

    pub fn get_ctm(&self) -> Matrix {
        let mut matrix = Matrix::null();

        unsafe {
            ffi::cairo_scaled_font_get_ctm(self.get_ptr(), &mut matrix)
        }

        matrix
    }

    pub fn get_scale_matrix(&self) -> Matrix {
        let mut matrix = Matrix::null();

        unsafe {
            ffi::cairo_scaled_font_get_scale_matrix(self.get_ptr(), &mut matrix)
        }

        matrix
    }

    pub fn reference(&self) -> ScaledFont {
        unsafe {
            ScaledFont(ffi::cairo_scaled_font_reference(self.get_ptr()))
        }
    }
}

impl<'a> ToGlibPtr<'a, *const cairo_scaled_font_t> for &'a ScaledFont {
    type Storage = &'a ScaledFont;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *const cairo_scaled_font_t, &'a ScaledFont> {
        Stash(self.0, *self)
    }
}

impl FromGlibPtrNone<*mut cairo_scaled_font_t> for ScaledFont {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut cairo_scaled_font_t) -> Self {
        let ptr = ffi::cairo_scaled_font_reference(ptr);
        assert!(!ptr.is_null());
        let tmp = ScaledFont(ptr);
        tmp.ensure_status();
        tmp
    }
}

impl FromGlibPtrFull<*mut cairo_scaled_font_t> for ScaledFont {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut cairo_scaled_font_t) -> Self {
        assert!(!ptr.is_null());
        let tmp = ScaledFont(ptr);
        tmp.ensure_status();
        tmp
    }
}

impl Drop for ScaledFont {
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_scaled_font_destroy(self.get_ptr())
        }
    }
}

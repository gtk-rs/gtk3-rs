// Take a look at the license at the top of the repository in the LICENSE file.

use crate::font::{
    FontExtents, FontFace, FontOptions, Glyph, ScaledFont, TextCluster, TextExtents,
};
use crate::matrices::Matrix;
use crate::paths::Path;
use crate::Rectangle;
use crate::{
    Antialias, Content, FillRule, FontSlant, FontWeight, LineCap, LineJoin, Operator,
    TextClusterFlags,
};
#[cfg(feature = "use_glib")]
use glib::translate::*;
use libc::c_int;
use std::ffi::CString;
use std::fmt;
use std::ops;
use std::ptr;
use std::slice;

use crate::error::Error;
use crate::ffi::{cairo_rectangle_list_t, cairo_t};
use crate::patterns::Pattern;
use crate::surface::Surface;
use crate::utils::status_to_result;

pub struct RectangleList {
    ptr: *mut cairo_rectangle_list_t,
}

impl ops::Deref for RectangleList {
    type Target = [Rectangle];

    fn deref(&self) -> &[Rectangle] {
        unsafe {
            let ptr = (*self.ptr).rectangles as *mut Rectangle;
            let len = (*self.ptr).num_rectangles;

            if ptr.is_null() || len == 0 {
                &[]
            } else {
                slice::from_raw_parts(ptr, len as usize)
            }
        }
    }
}

impl Drop for RectangleList {
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_rectangle_list_destroy(self.ptr);
        }
    }
}

impl fmt::Debug for RectangleList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::ops::Deref;
        f.debug_tuple("RectangleList").field(&self.deref()).finish()
    }
}

impl fmt::Display for RectangleList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RectangleList")
    }
}

#[derive(Debug)]
pub struct Context(ptr::NonNull<cairo_t>);

#[cfg(feature = "use_glib")]
impl<'a> ToGlibPtr<'a, *mut ffi::cairo_t> for &'a Context {
    type Storage = &'a Context;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut ffi::cairo_t, &'a Context> {
        Stash(self.0.as_ptr(), *self)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut ffi::cairo_t {
        unsafe { ffi::cairo_reference(self.0.as_ptr()) }
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrNone<*mut ffi::cairo_t> for Context {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::cairo_t) -> Context {
        Self::from_raw_none(ptr)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrBorrow<*mut ffi::cairo_t> for Context {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::cairo_t) -> crate::Borrowed<Context> {
        Self::from_raw_borrow(ptr)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrFull<*mut ffi::cairo_t> for Context {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::cairo_t) -> Context {
        Self::from_raw_full(ptr)
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(
    Context,
    cairo_t,
    ffi::gobject::cairo_gobject_context_get_type
);

impl Clone for Context {
    fn clone(&self) -> Context {
        unsafe { Self::from_raw_none(self.to_raw_none()) }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_destroy(self.0.as_ptr());
        }
    }
}

impl Context {
    #[inline]
    pub unsafe fn from_raw_none(ptr: *mut ffi::cairo_t) -> Context {
        assert!(!ptr.is_null());
        ffi::cairo_reference(ptr);
        Context(ptr::NonNull::new_unchecked(ptr))
    }

    #[inline]
    pub unsafe fn from_raw_borrow(ptr: *mut ffi::cairo_t) -> crate::Borrowed<Context> {
        assert!(!ptr.is_null());
        crate::Borrowed::new(Context(ptr::NonNull::new_unchecked(ptr)))
    }

    #[inline]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_t) -> Context {
        assert!(!ptr.is_null());
        Context(ptr::NonNull::new_unchecked(ptr))
    }

    pub fn to_raw_none(&self) -> *mut ffi::cairo_t {
        self.0.as_ptr()
    }

    pub fn status(&self) -> Result<(), Error> {
        let status = unsafe { ffi::cairo_status(self.0.as_ptr()) };
        status_to_result(status)
    }

    pub fn new(target: &Surface) -> Context {
        unsafe { Self::from_raw_full(ffi::cairo_create(target.to_raw_none())) }
    }

    pub fn save(&self) -> Result<(), Error> {
        unsafe { ffi::cairo_save(self.0.as_ptr()) }
        self.status()
    }

    pub fn restore(&self) -> Result<(), Error> {
        unsafe { ffi::cairo_restore(self.0.as_ptr()) }
        self.status()
    }

    pub fn get_target(&self) -> Surface {
        unsafe { Surface::from_raw_none(ffi::cairo_get_target(self.0.as_ptr())) }
    }

    pub fn push_group(&self) {
        unsafe { ffi::cairo_push_group(self.0.as_ptr()) }
    }

    pub fn push_group_with_content(&self, content: Content) {
        unsafe { ffi::cairo_push_group_with_content(self.0.as_ptr(), content.into()) }
    }

    pub fn pop_group(&self) -> Pattern {
        unsafe { Pattern::from_raw_full(ffi::cairo_pop_group(self.0.as_ptr())) }
    }

    pub fn pop_group_to_source(&self) {
        unsafe { ffi::cairo_pop_group_to_source(self.0.as_ptr()) }
    }

    pub fn get_group_target(&self) -> Surface {
        unsafe { Surface::from_raw_none(ffi::cairo_get_group_target(self.0.as_ptr())) }
    }

    pub fn set_source_rgb(&self, red: f64, green: f64, blue: f64) {
        unsafe { ffi::cairo_set_source_rgb(self.0.as_ptr(), red, green, blue) }
    }

    pub fn set_source_rgba(&self, red: f64, green: f64, blue: f64, alpha: f64) {
        unsafe { ffi::cairo_set_source_rgba(self.0.as_ptr(), red, green, blue, alpha) }
    }

    pub fn set_source(&self, source: &Pattern) {
        unsafe {
            ffi::cairo_set_source(self.0.as_ptr(), source.to_raw_none());
        }
        self.status().expect("Failed to set source");
    }

    pub fn get_source(&self) -> Pattern {
        unsafe { Pattern::from_raw_none(ffi::cairo_get_source(self.0.as_ptr())) }
    }

    pub fn set_source_surface(&self, surface: &Surface, x: f64, y: f64) {
        unsafe {
            ffi::cairo_set_source_surface(self.0.as_ptr(), surface.to_raw_none(), x, y);
        }
    }

    pub fn set_antialias(&self, antialias: Antialias) {
        unsafe { ffi::cairo_set_antialias(self.0.as_ptr(), antialias.into()) }
        self.status().expect("Failed to set antialias");
    }

    pub fn get_antialias(&self) -> Antialias {
        unsafe { Antialias::from(ffi::cairo_get_antialias(self.0.as_ptr())) }
    }

    pub fn set_dash(&self, dashes: &[f64], offset: f64) {
        unsafe {
            ffi::cairo_set_dash(
                self.0.as_ptr(),
                dashes.as_ptr(),
                dashes.len() as i32,
                offset,
            )
        }
        self.status().expect("Failed to set a dash"); //Possible invalid dashes value
    }

    pub fn get_dash_count(&self) -> i32 {
        unsafe { ffi::cairo_get_dash_count(self.0.as_ptr()) }
    }

    pub fn get_dash(&self) -> (Vec<f64>, f64) {
        let dash_count = self.get_dash_count() as usize;
        let mut dashes: Vec<f64> = Vec::with_capacity(dash_count);
        let mut offset: f64 = 0.0;

        unsafe {
            ffi::cairo_get_dash(self.0.as_ptr(), dashes.as_mut_ptr(), &mut offset);
            dashes.set_len(dash_count);
            (dashes, offset)
        }
    }

    pub fn get_dash_dashes(&self) -> Vec<f64> {
        let (dashes, _) = self.get_dash();
        dashes
    }

    pub fn get_dash_offset(&self) -> f64 {
        let (_, offset) = self.get_dash();
        offset
    }

    pub fn set_fill_rule(&self, fill_rule: FillRule) {
        unsafe {
            ffi::cairo_set_fill_rule(self.0.as_ptr(), fill_rule.into());
        }
        self.status().expect("Failed to set fill rule");
    }

    pub fn get_fill_rule(&self) -> FillRule {
        unsafe { FillRule::from(ffi::cairo_get_fill_rule(self.0.as_ptr())) }
    }

    pub fn set_line_cap(&self, arg: LineCap) {
        unsafe { ffi::cairo_set_line_cap(self.0.as_ptr(), arg.into()) }
        self.status().expect("Failed to set line cap");
    }

    pub fn get_line_cap(&self) -> LineCap {
        unsafe { LineCap::from(ffi::cairo_get_line_cap(self.0.as_ptr())) }
    }

    pub fn set_line_join(&self, arg: LineJoin) {
        unsafe { ffi::cairo_set_line_join(self.0.as_ptr(), arg.into()) }
        self.status().expect("Failed to set line join");
    }

    pub fn get_line_join(&self) -> LineJoin {
        unsafe { LineJoin::from(ffi::cairo_get_line_join(self.0.as_ptr())) }
    }

    pub fn set_line_width(&self, arg: f64) {
        unsafe { ffi::cairo_set_line_width(self.0.as_ptr(), arg) }
        self.status().expect("Failed to set line width");
    }

    pub fn get_line_width(&self) -> f64 {
        unsafe { ffi::cairo_get_line_width(self.0.as_ptr()) }
    }

    pub fn set_miter_limit(&self, arg: f64) {
        unsafe { ffi::cairo_set_miter_limit(self.0.as_ptr(), arg) }
        self.status().expect("Failed to set miter limit");
    }

    pub fn get_miter_limit(&self) -> f64 {
        unsafe { ffi::cairo_get_miter_limit(self.0.as_ptr()) }
    }

    pub fn set_operator(&self, op: Operator) {
        unsafe {
            ffi::cairo_set_operator(self.0.as_ptr(), op.into());
        }
    }

    pub fn get_operator(&self) -> Operator {
        unsafe { Operator::from(ffi::cairo_get_operator(self.0.as_ptr())) }
    }

    pub fn set_tolerance(&self, arg: f64) {
        unsafe { ffi::cairo_set_tolerance(self.0.as_ptr(), arg) }
        self.status().expect("Failed to set tolerance");
    }

    pub fn get_tolerance(&self) -> f64 {
        unsafe { ffi::cairo_get_tolerance(self.0.as_ptr()) }
    }

    pub fn clip(&self) {
        unsafe { ffi::cairo_clip(self.0.as_ptr()) }
    }

    pub fn clip_preserve(&self) {
        unsafe { ffi::cairo_clip_preserve(self.0.as_ptr()) }
    }

    pub fn clip_extents(&self) -> (f64, f64, f64, f64) {
        let mut x1: f64 = 0.0;
        let mut y1: f64 = 0.0;
        let mut x2: f64 = 0.0;
        let mut y2: f64 = 0.0;

        unsafe {
            ffi::cairo_clip_extents(self.0.as_ptr(), &mut x1, &mut y1, &mut x2, &mut y2);
        }
        (x1, y1, x2, y2)
    }

    pub fn in_clip(&self, x: f64, y: f64) -> bool {
        unsafe { ffi::cairo_in_clip(self.0.as_ptr(), x, y).as_bool() }
    }

    pub fn reset_clip(&self) {
        unsafe { ffi::cairo_reset_clip(self.0.as_ptr()) }
        self.status().expect("Failed to reset clip");
    }

    pub fn copy_clip_rectangle_list(&self) -> Result<RectangleList, Error> {
        unsafe {
            let rectangle_list = ffi::cairo_copy_clip_rectangle_list(self.0.as_ptr());

            status_to_result((*rectangle_list).status)?;

            Ok(RectangleList {
                ptr: rectangle_list,
            })
        }
    }

    pub fn fill(&self) {
        unsafe { ffi::cairo_fill(self.0.as_ptr()) }
    }

    pub fn fill_preserve(&self) {
        unsafe { ffi::cairo_fill_preserve(self.0.as_ptr()) }
    }

    pub fn fill_extents(&self) -> (f64, f64, f64, f64) {
        let mut x1: f64 = 0.0;
        let mut y1: f64 = 0.0;
        let mut x2: f64 = 0.0;
        let mut y2: f64 = 0.0;

        unsafe {
            ffi::cairo_fill_extents(self.0.as_ptr(), &mut x1, &mut y1, &mut x2, &mut y2);
        }
        (x1, y1, x2, y2)
    }

    pub fn in_fill(&self, x: f64, y: f64) -> bool {
        unsafe { ffi::cairo_in_fill(self.0.as_ptr(), x, y).as_bool() }
    }

    pub fn mask(&self, pattern: &Pattern) {
        unsafe { ffi::cairo_mask(self.0.as_ptr(), pattern.to_raw_none()) }
    }

    pub fn mask_surface(&self, surface: &Surface, x: f64, y: f64) {
        unsafe {
            ffi::cairo_mask_surface(self.0.as_ptr(), surface.to_raw_none(), x, y);
        }
    }

    pub fn paint(&self) {
        unsafe { ffi::cairo_paint(self.0.as_ptr()) }
    }

    pub fn paint_with_alpha(&self, alpha: f64) {
        unsafe { ffi::cairo_paint_with_alpha(self.0.as_ptr(), alpha) }
    }

    pub fn stroke(&self) {
        unsafe { ffi::cairo_stroke(self.0.as_ptr()) }
    }

    pub fn stroke_preserve(&self) {
        unsafe { ffi::cairo_stroke_preserve(self.0.as_ptr()) }
    }

    pub fn stroke_extents(&self) -> (f64, f64, f64, f64) {
        let mut x1: f64 = 0.0;
        let mut y1: f64 = 0.0;
        let mut x2: f64 = 0.0;
        let mut y2: f64 = 0.0;

        unsafe {
            ffi::cairo_stroke_extents(self.0.as_ptr(), &mut x1, &mut y1, &mut x2, &mut y2);
        }
        (x1, y1, x2, y2)
    }

    pub fn in_stroke(&self, x: f64, y: f64) -> bool {
        unsafe { ffi::cairo_in_stroke(self.0.as_ptr(), x, y).as_bool() }
    }

    pub fn copy_page(&self) {
        unsafe { ffi::cairo_copy_page(self.0.as_ptr()) }
    }

    pub fn show_page(&self) {
        unsafe { ffi::cairo_show_page(self.0.as_ptr()) }
    }

    pub fn get_reference_count(&self) -> u32 {
        unsafe { ffi::cairo_get_reference_count(self.0.as_ptr()) }
    }

    // transformations stuff

    pub fn translate(&self, tx: f64, ty: f64) {
        unsafe { ffi::cairo_translate(self.0.as_ptr(), tx, ty) }
    }

    pub fn scale(&self, sx: f64, sy: f64) {
        unsafe { ffi::cairo_scale(self.0.as_ptr(), sx, sy) }
    }

    pub fn rotate(&self, angle: f64) {
        unsafe { ffi::cairo_rotate(self.0.as_ptr(), angle) }
    }

    pub fn transform(&self, matrix: Matrix) {
        unsafe {
            ffi::cairo_transform(self.0.as_ptr(), matrix.ptr());
        }
    }

    pub fn set_matrix(&self, matrix: Matrix) {
        unsafe {
            ffi::cairo_set_matrix(self.0.as_ptr(), matrix.ptr());
        }
    }

    pub fn get_matrix(&self) -> Matrix {
        let mut matrix = Matrix::null();
        unsafe {
            ffi::cairo_get_matrix(self.0.as_ptr(), matrix.mut_ptr());
        }
        matrix
    }

    pub fn identity_matrix(&self) {
        unsafe { ffi::cairo_identity_matrix(self.0.as_ptr()) }
    }

    pub fn user_to_device(&self, mut x: f64, mut y: f64) -> (f64, f64) {
        unsafe {
            ffi::cairo_user_to_device(self.0.as_ptr(), &mut x, &mut y);
            (x, y)
        }
    }

    pub fn user_to_device_distance(&self, mut dx: f64, mut dy: f64) -> (f64, f64) {
        unsafe {
            ffi::cairo_user_to_device_distance(self.0.as_ptr(), &mut dx, &mut dy);
            (dx, dy)
        }
    }

    pub fn device_to_user(&self, mut x: f64, mut y: f64) -> (f64, f64) {
        unsafe {
            ffi::cairo_device_to_user(self.0.as_ptr(), &mut x, &mut y);
            (x, y)
        }
    }

    pub fn device_to_user_distance(&self, mut dx: f64, mut dy: f64) -> (f64, f64) {
        unsafe {
            ffi::cairo_device_to_user_distance(self.0.as_ptr(), &mut dx, &mut dy);
            (dx, dy)
        }
    }

    // font stuff

    pub fn select_font_face(&self, family: &str, slant: FontSlant, weight: FontWeight) {
        unsafe {
            let family = CString::new(family).unwrap();
            ffi::cairo_select_font_face(
                self.0.as_ptr(),
                family.as_ptr(),
                slant.into(),
                weight.into(),
            )
        }
    }

    pub fn set_font_size(&self, size: f64) {
        unsafe { ffi::cairo_set_font_size(self.0.as_ptr(), size) }
    }

    // FIXME probably needs a heap allocation
    pub fn set_font_matrix(&self, matrix: Matrix) {
        unsafe { ffi::cairo_set_font_matrix(self.0.as_ptr(), matrix.ptr()) }
    }

    pub fn get_font_matrix(&self) -> Matrix {
        let mut matrix = Matrix::null();
        unsafe {
            ffi::cairo_get_font_matrix(self.0.as_ptr(), matrix.mut_ptr());
        }
        matrix
    }

    pub fn set_font_options(&self, options: &FontOptions) {
        unsafe { ffi::cairo_set_font_options(self.0.as_ptr(), options.to_raw_none()) }
    }

    pub fn get_font_options(&self) -> FontOptions {
        let out = FontOptions::new();
        unsafe {
            ffi::cairo_get_font_options(self.0.as_ptr(), out.to_raw_none());
        }
        out
    }

    pub fn set_font_face(&self, font_face: &FontFace) {
        unsafe { ffi::cairo_set_font_face(self.0.as_ptr(), font_face.to_raw_none()) }
    }

    pub fn get_font_face(&self) -> FontFace {
        unsafe { FontFace::from_raw_none(ffi::cairo_get_font_face(self.0.as_ptr())) }
    }

    pub fn set_scaled_font(&self, scaled_font: &ScaledFont) {
        unsafe { ffi::cairo_set_scaled_font(self.0.as_ptr(), scaled_font.to_raw_none()) }
    }

    pub fn get_scaled_font(&self) -> ScaledFont {
        unsafe { ScaledFont::from_raw_none(ffi::cairo_get_scaled_font(self.0.as_ptr())) }
    }

    pub fn show_text(&self, text: &str) {
        unsafe {
            let text = CString::new(text).unwrap();
            ffi::cairo_show_text(self.0.as_ptr(), text.as_ptr())
        }
    }

    pub fn show_glyphs(&self, glyphs: &[Glyph]) {
        unsafe { ffi::cairo_show_glyphs(self.0.as_ptr(), glyphs.as_ptr(), glyphs.len() as c_int) }
    }

    pub fn show_text_glyphs(
        &self,
        text: &str,
        glyphs: &[Glyph],
        clusters: &[TextCluster],
        cluster_flags: TextClusterFlags,
    ) {
        unsafe {
            let text = CString::new(text).unwrap();
            ffi::cairo_show_text_glyphs(
                self.0.as_ptr(),
                text.as_ptr(),
                -1_i32, //NULL terminated
                glyphs.as_ptr(),
                glyphs.len() as c_int,
                clusters.as_ptr(),
                clusters.len() as c_int,
                cluster_flags.into(),
            )
        }
    }

    pub fn font_extents(&self) -> FontExtents {
        let mut extents = FontExtents {
            ascent: 0.0,
            descent: 0.0,
            height: 0.0,
            max_x_advance: 0.0,
            max_y_advance: 0.0,
        };

        unsafe {
            ffi::cairo_font_extents(self.0.as_ptr(), &mut extents);
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
            let text = CString::new(text).unwrap();
            ffi::cairo_text_extents(self.0.as_ptr(), text.as_ptr(), &mut extents);
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
            ffi::cairo_glyph_extents(
                self.0.as_ptr(),
                glyphs.as_ptr(),
                glyphs.len() as c_int,
                &mut extents,
            );
        }

        extents
    }

    // paths stuff

    pub fn copy_path(&self) -> Path {
        unsafe { Path::from_raw_full(ffi::cairo_copy_path(self.0.as_ptr())) }
    }

    pub fn copy_path_flat(&self) -> Path {
        unsafe { Path::from_raw_full(ffi::cairo_copy_path_flat(self.0.as_ptr())) }
    }

    pub fn append_path(&self, path: &Path) {
        unsafe { ffi::cairo_append_path(self.0.as_ptr(), path.as_ptr()) }
    }

    pub fn has_current_point(&self) -> bool {
        unsafe { ffi::cairo_has_current_point(self.0.as_ptr()).as_bool() }
    }

    pub fn get_current_point(&self) -> (f64, f64) {
        unsafe {
            let mut x = 0.0;
            let mut y = 0.0;
            ffi::cairo_get_current_point(self.0.as_ptr(), &mut x, &mut y);
            (x, y)
        }
    }

    pub fn new_path(&self) {
        unsafe { ffi::cairo_new_path(self.0.as_ptr()) }
    }

    pub fn new_sub_path(&self) {
        unsafe { ffi::cairo_new_sub_path(self.0.as_ptr()) }
    }

    pub fn close_path(&self) {
        unsafe { ffi::cairo_close_path(self.0.as_ptr()) }
    }

    pub fn arc(&self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) {
        unsafe { ffi::cairo_arc(self.0.as_ptr(), xc, yc, radius, angle1, angle2) }
    }

    pub fn arc_negative(&self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) {
        unsafe { ffi::cairo_arc_negative(self.0.as_ptr(), xc, yc, radius, angle1, angle2) }
    }

    pub fn curve_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        unsafe { ffi::cairo_curve_to(self.0.as_ptr(), x1, y1, x2, y2, x3, y3) }
    }

    pub fn line_to(&self, x: f64, y: f64) {
        unsafe { ffi::cairo_line_to(self.0.as_ptr(), x, y) }
    }

    pub fn move_to(&self, x: f64, y: f64) {
        unsafe { ffi::cairo_move_to(self.0.as_ptr(), x, y) }
    }

    pub fn rectangle(&self, x: f64, y: f64, width: f64, height: f64) {
        unsafe { ffi::cairo_rectangle(self.0.as_ptr(), x, y, width, height) }
    }

    pub fn text_path(&self, str_: &str) {
        unsafe {
            let str_ = CString::new(str_).unwrap();
            ffi::cairo_text_path(self.0.as_ptr(), str_.as_ptr())
        }
    }

    pub fn glyph_path(&self, glyphs: &[Glyph]) {
        unsafe { ffi::cairo_glyph_path(self.0.as_ptr(), glyphs.as_ptr(), glyphs.len() as i32) }
    }

    pub fn rel_curve_to(&self, dx1: f64, dy1: f64, dx2: f64, dy2: f64, dx3: f64, dy3: f64) {
        unsafe { ffi::cairo_rel_curve_to(self.0.as_ptr(), dx1, dy1, dx2, dy2, dx3, dy3) }
    }

    pub fn rel_line_to(&self, dx: f64, dy: f64) {
        unsafe { ffi::cairo_rel_line_to(self.0.as_ptr(), dx, dy) }
    }

    pub fn rel_move_to(&self, dx: f64, dy: f64) {
        unsafe { ffi::cairo_rel_move_to(self.0.as_ptr(), dx, dy) }
    }

    pub fn path_extents(&self) -> (f64, f64, f64, f64) {
        let mut x1: f64 = 0.0;
        let mut y1: f64 = 0.0;
        let mut x2: f64 = 0.0;
        let mut y2: f64 = 0.0;

        unsafe {
            ffi::cairo_path_extents(self.0.as_ptr(), &mut x1, &mut y1, &mut x2, &mut y2);
        }
        (x1, y1, x2, y2)
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn tag_begin(&self, tag_name: &str, attributes: &str) {
        unsafe {
            let tag_name = CString::new(tag_name).unwrap();
            let attributes = CString::new(attributes).unwrap();
            ffi::cairo_tag_begin(self.0.as_ptr(), tag_name.as_ptr(), attributes.as_ptr())
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn tag_end(&self, tag_name: &str) {
        unsafe {
            let tag_name = CString::new(tag_name).unwrap();
            ffi::cairo_tag_end(self.0.as_ptr(), tag_name.as_ptr())
        }
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Context")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::Format;
    use crate::image_surface::ImageSurface;
    use crate::patterns::LinearGradient;

    fn create_ctx() -> Context {
        let surface = ImageSurface::create(Format::ARgb32, 10, 10).unwrap();
        Context::new(&surface)
    }

    #[test]
    fn drop_non_reference_pattern_from_ctx() {
        let ctx = create_ctx();
        ctx.get_source();
    }

    #[test]
    fn drop_non_reference_pattern() {
        let ctx = create_ctx();
        let pattern = LinearGradient::new(1.0f64, 2.0f64, 3.0f64, 4.0f64);
        ctx.set_source(&pattern);
    }

    #[test]
    fn clip_rectangle() {
        let ctx = create_ctx();
        let rect = ctx
            .copy_clip_rectangle_list()
            .expect("Failed to copy rectangle list");
        assert_eq!(
            format!("{:?}", rect),
            "RectangleList([Rectangle { x: 0.0, y: 0.0, width: 10.0, height: 10.0 }])"
        );
        assert_eq!(rect.to_string(), "RectangleList");
    }
}

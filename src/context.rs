// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use c_vec::CVec;
use std::mem::transmute;
use libc::{c_double, c_int};
use ::paths::Path;
use ::fonts::{TextExtents, TextCluster, FontExtents, ScaledFont, FontOptions, FontFace, Glyph};
use ::matrices::{Matrix, MatrixTrait};
use ffi::enums::{
    FontSlant,
    FontWeight,
    TextClusterFlags,
    Operator,
};
use Rectangle;
use ffi;

use ffi::{
    cairo_t,
    cairo_rectangle_list_t,
};
use ffi::enums::{Status, Antialias, LineCap, LineJoin, FillRule};
use ::patterns::{wrap_pattern, Pattern};
use surface::Surface;

pub struct RectangleVec {
    ptr: *mut cairo_rectangle_list_t,
    pub rectangles: CVec<Rectangle>,
}

impl Drop for RectangleVec {
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_rectangle_list_destroy(self.ptr);
        }
    }
}

pub struct Context(*mut cairo_t);

impl<'a> ToGlibPtr<'a, *mut ffi::cairo_t> for &'a Context {
    type Storage = &'a Context;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut ffi::cairo_t, &'a Context> {
        Stash(self.0, *self)
    }
}

impl FromGlibPtrNone<*mut ffi::cairo_t> for Context {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::cairo_t) -> Context {
        assert!(!ptr.is_null());
        ffi::cairo_reference(ptr);
        Context(ptr)
    }
}

impl FromGlibPtrFull<*mut ffi::cairo_t> for Context {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::cairo_t) -> Context {
        assert!(!ptr.is_null());
        Context(ptr)
    }
}

impl AsRef<Context> for Context {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl Clone for Context {
    fn clone(&self) -> Context {
        unsafe { from_glib_none(self.to_glib_none().0) }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ffi::cairo_destroy(self.0); }
    }
}

impl Context {
    pub fn ensure_status(&self) {
        self.status().ensure_valid();
    }

    pub fn new<T: AsRef<Surface>>(target: &T) -> Context {
        unsafe {
            from_glib_full(ffi::cairo_create(target.as_ref().to_glib_none().0))
        }
    }

    pub fn status(&self) -> Status {
        unsafe {
            ffi::cairo_status(self.0)
        }
    }

    pub fn save(&self) {
        unsafe {
            ffi::cairo_save(self.0)
        }
        self.ensure_status()
    }

    pub fn restore(&self) {
        unsafe {
            ffi::cairo_restore(self.0)
        }
        self.ensure_status()
    }

    pub fn get_target(&self) -> Surface {
        unsafe {
            from_glib_none(ffi::cairo_get_target(self.0))
        }
    }

    pub fn push_group(&self) {
        unsafe {
            ffi::cairo_push_group(self.0)
        }
    }

    /*
    pub fn push_group_with_content(&self, content: Content){
        unsafe {
            ffi::cairo_push_group_with_content(self.0, content)
        }
    }*/

    pub fn pop_group(&self) -> Box<Pattern> {
        unsafe {
            wrap_pattern(ffi::cairo_pop_group(self.0))
        }
    }

    pub fn pop_group_to_source(&self) {
        unsafe {
            ffi::cairo_pop_group_to_source(self.0)
        }
    }

    pub fn get_group_target(&self) -> Surface {
        unsafe {
            from_glib_none(ffi::cairo_get_group_target(self.0))
        }
    }

    pub fn set_source_rgb(&self, red: f64, green: f64, blue: f64) {
        unsafe {
            ffi::cairo_set_source_rgb(self.0, red, green, blue)
        }
    }

    pub fn set_source_rgba(&self, red: f64, green: f64, blue: f64, alpha: f64) {
        unsafe {
            ffi::cairo_set_source_rgba(self.0, red, green, blue, alpha)
        }
    }

    pub fn set_source(&self, source: &Pattern) {
        unsafe {
            ffi::cairo_set_source(self.0, source.get_ptr());
        }
        self.ensure_status();
    }

    pub fn get_source(&self) -> Box<Pattern> {
        unsafe {
            wrap_pattern(ffi::cairo_get_source(self.0))
        }
    }

    pub fn set_source_surface<T: AsRef<Surface>>(&self, surface: &T, x: f64, y: f64) {
        unsafe { ffi::cairo_set_source_surface(self.0, surface.as_ref().to_glib_none().0, x, y); }
    }

    pub fn set_antialias(&self, antialias : Antialias) {
        unsafe {
            ffi::cairo_set_antialias(self.0, antialias)
        }
        self.ensure_status()
    }

    pub fn get_antialias(&self) -> Antialias {
        unsafe {
            ffi::cairo_get_antialias(self.0)
        }
    }

    pub fn set_dash(&self, dashes: &[f64], offset: f64) {
        unsafe {
            ffi::cairo_set_dash(self.0, dashes.as_ptr(), dashes.len() as i32, offset)
        }
        self.ensure_status(); //Possible invalid dashes value
    }

    pub fn get_dash_count(&self) -> i32 {
        unsafe {
            ffi::cairo_get_dash_count(self.0)
        }
    }

    pub fn get_dash(&self) -> (Vec<f64>, f64) {
        let dash_count = self.get_dash_count() as usize;
        let mut dashes: Vec<f64> = Vec::with_capacity(dash_count);
        let mut offset: f64 = 0.0;

        unsafe {
            ffi::cairo_get_dash(self.0, dashes.as_mut_ptr(), &mut offset);
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

    pub fn set_fill_rule(&self, fill_rule : FillRule) {
        unsafe {
            ffi::cairo_set_fill_rule(self.0, fill_rule);
        }
        self.ensure_status();
    }

    pub fn get_fill_rule(&self) -> FillRule {
        unsafe {
            ffi::cairo_get_fill_rule(self.0)
        }
    }

    pub fn set_line_cap(&self, arg: LineCap) {
        unsafe {
            ffi::cairo_set_line_cap(self.0, arg)
        }
        self.ensure_status();
    }

    pub fn get_line_cap(&self) -> LineCap {
        unsafe {
            ffi::cairo_get_line_cap(self.0)
        }
    }

    pub fn set_line_join(&self, arg: LineJoin) {
        unsafe {
            ffi::cairo_set_line_join(self.0, arg)
        }
        self.ensure_status();
    }

    pub fn get_line_join(&self) -> LineJoin {
        unsafe {
            ffi::cairo_get_line_join(self.0)
        }
    }

    pub fn set_line_width(&self, arg: f64) {
        unsafe {
            ffi::cairo_set_line_width(self.0, arg)
        }
        self.ensure_status();
    }

    pub fn get_line_width(&self) -> f64 {
        unsafe {
            ffi::cairo_get_line_width(self.0)
        }
    }

    pub fn set_miter_limit(&self, arg: f64) {
        unsafe {
            ffi::cairo_set_miter_limit(self.0, arg)
        }
        self.ensure_status();
    }

    pub fn get_miter_limit(&self) -> f64 {
        unsafe {
            ffi::cairo_get_miter_limit(self.0)
        }
    }

    pub fn set_operator(&self, op: Operator) {
        unsafe {
            ffi::cairo_set_operator(self.0, op);
        }
    }

    pub fn get_operator(&self) -> Operator {
        unsafe {
            ffi::cairo_get_operator(self.0)
        }
    }

    pub fn set_tolerance(&self, arg: f64) {
        unsafe {
            ffi::cairo_set_tolerance(self.0, arg)
        }
        self.ensure_status();
    }

    pub fn get_tolerance(&self) -> f64 {
        unsafe {
            ffi::cairo_get_tolerance(self.0)
        }
    }

    pub fn clip(&self) {
        unsafe {
            ffi::cairo_clip(self.0)
        }
    }

    pub fn clip_preserve(&self) {
        unsafe {
            ffi::cairo_clip_preserve(self.0)
        }
    }

    pub fn clip_extents(&self) -> (f64, f64, f64, f64) {
        let mut x1: f64 = 0.0;
        let mut y1: f64 = 0.0;
        let mut x2: f64 = 0.0;
        let mut y2: f64 = 0.0;

        unsafe {
            ffi::cairo_clip_extents(self.0, &mut x1, &mut y1, &mut x2, &mut y2);
        }
        (x1, y1, x2, y2)
    }

    pub fn in_clip(&self, x:f64, y:f64) -> bool {
        unsafe {
            ffi::cairo_in_clip(self.0, x, y).as_bool()
        }
    }

    pub fn reset_clip(&self) {
        unsafe {
            ffi::cairo_reset_clip(self.0)
        }
        self.ensure_status()
    }

    pub fn copy_clip_rectangle_list(&self) -> RectangleVec {
        unsafe {
            let rectangle_list = ffi::cairo_copy_clip_rectangle_list(self.0);

            (*rectangle_list).status.ensure_valid();

            RectangleVec {
                ptr: rectangle_list,
                rectangles: CVec::new((*rectangle_list).rectangles,
                                      (*rectangle_list).num_rectangles as usize),
            }
        }
    }

    pub fn fill(&self) {
        unsafe {
            ffi::cairo_fill(self.0)
        }
    }

    pub fn fill_preserve(&self) {
        unsafe {
            ffi::cairo_fill_preserve(self.0)
        }
    }

    pub fn fill_extents(&self) -> (f64, f64, f64, f64) {
        let mut x1: f64 = 0.0;
        let mut y1: f64 = 0.0;
        let mut x2: f64 = 0.0;
        let mut y2: f64 = 0.0;

        unsafe {
            ffi::cairo_fill_extents(self.0, &mut x1, &mut y1, &mut x2, &mut y2);
        }
        (x1, y1, x2, y2)
    }

    pub fn in_fill(&self, x:f64, y:f64) -> bool {
        unsafe {
            ffi::cairo_in_fill(self.0, x, y).as_bool()
        }
    }

    pub fn mask(&self, pattern: &Pattern) {
        unsafe {
            ffi::cairo_mask(self.0, pattern.get_ptr())
        }
    }

    //fn ffi::cairo_mask_surface(cr: *mut cairo_t, surface: *mut cairo_surface_t, surface_x: c_double, surface_y: c_double);

    pub fn paint(&self) {
        unsafe {
            ffi::cairo_paint(self.0)
        }
    }

    pub fn paint_with_alpha(&self, alpha: f64) {
        unsafe {
            ffi::cairo_paint_with_alpha(self.0, alpha)
        }
    }

    pub fn stroke(&self) {
        unsafe {
            ffi::cairo_stroke(self.0)
        }
    }

    pub fn stroke_preserve(&self) {
        unsafe {
            ffi::cairo_stroke_preserve(self.0)
        }
    }

    pub fn stroke_extents(&self) -> (f64, f64, f64, f64) {
        let mut x1: f64 = 0.0;
        let mut y1: f64 = 0.0;
        let mut x2: f64 = 0.0;
        let mut y2: f64 = 0.0;

        unsafe {
            ffi::cairo_stroke_extents(self.0, &mut x1, &mut y1, &mut x2, &mut y2);
        }
        (x1, y1, x2, y2)
    }

    pub fn in_stroke(&self, x:f64, y:f64) -> bool {
        unsafe {
            ffi::cairo_in_stroke(self.0, x, y).as_bool()
        }
    }

    pub fn copy_page(&self) {
        unsafe {
            ffi::cairo_copy_page(self.0)
        }
    }

    pub fn show_page(&self) {
        unsafe {
            ffi::cairo_show_page(self.0)
        }
    }

    pub fn get_reference_count(&self) -> u32 {
        unsafe {
            ffi::cairo_get_reference_count(self.0)
        }
    }

    // transformations stuff

     pub fn translate(&self, tx: f64, ty: f64) {
        unsafe {
            ffi::cairo_translate(self.0, tx, ty)
        }
    }

    pub fn scale(&self, sx: f64, sy: f64) {
        unsafe {
            ffi::cairo_scale(self.0, sx, sy)
        }
    }

    pub fn rotate(&self, angle: f64) {
        unsafe {
            ffi::cairo_rotate(self.0, angle)
        }
    }

    pub fn transform(&self, matrix: Matrix) {
        unsafe {
            ffi::cairo_transform(self.0, &matrix);
        }
    }

    pub fn set_matrix(&self, matrix: Matrix) {
        unsafe {
            ffi::cairo_set_matrix(self.0, &matrix);
        }
    }

    pub fn get_matrix(&self) -> Matrix {
        let mut matrix = <Matrix as MatrixTrait>::null();
        unsafe {
            ffi::cairo_get_matrix(self.0, &mut matrix);
        }
        matrix
    }

    pub fn identity_matrix(&self) {
        unsafe {
            ffi::cairo_identity_matrix(self.0)
        }
    }

    pub fn user_to_device(&self, x: f64, y: f64) -> (f64, f64) {
        unsafe {
            let x_ptr: *mut c_double = transmute(Box::new(x));
            let y_ptr: *mut c_double = transmute(Box::new(y));

            ffi::cairo_user_to_device(self.0, x_ptr, y_ptr);

            let x_box: Box<f64> = transmute(x_ptr);
            let y_box: Box<f64> = transmute(y_ptr);

            (*x_box, *y_box)
        }
    }

    pub fn user_to_device_distance(&self, dx: f64, dy: f64) -> (f64, f64) {
        unsafe {
            let dx_ptr: *mut c_double = transmute(Box::new(dx));
            let dy_ptr: *mut c_double = transmute(Box::new(dy));

            ffi::cairo_user_to_device_distance(self.0, dx_ptr, dy_ptr);

            let dx_box: Box<f64> = transmute(dx_ptr);
            let dy_box: Box<f64> = transmute(dy_ptr);

            (*dx_box, *dy_box)
        }
    }

    pub fn device_to_user(&self, x: f64, y: f64) -> (f64, f64) {
        unsafe {
            let x_ptr: *mut c_double = transmute(Box::new(x));
            let y_ptr: *mut c_double = transmute(Box::new(y));

            ffi::cairo_device_to_user(self.0, x_ptr, y_ptr);

            let x_box: Box<f64> = transmute(x_ptr);
            let y_box: Box<f64> = transmute(y_ptr);

            (*x_box, *y_box)
        }
    }

    pub fn device_to_user_distance(&self, dx: f64, dy: f64) -> (f64, f64) {
        unsafe {
            let dx_ptr: *mut c_double = transmute(Box::new(dx));
            let dy_ptr: *mut c_double = transmute(Box::new(dy));

            ffi::cairo_device_to_user_distance(self.0, dx_ptr, dy_ptr);

            let dx_box: Box<f64> = transmute(dx_ptr);
            let dy_box: Box<f64> = transmute(dy_ptr);

            (*dx_box, *dy_box)
        }
    }

    // font stuff

    pub fn select_font_face(&self, family: &str, slant: FontSlant, weight: FontWeight) {
        unsafe {
            ffi::cairo_select_font_face(self.0, family.to_glib_none().0, slant, weight)
        }
    }

    pub fn set_font_size(&self, size: f64) {
        unsafe {
            ffi::cairo_set_font_size(self.0, size)
        }
    }

    // FIXME probably needs a heap allocation
    pub fn set_font_matrix(&self, matrix: Matrix) {
        unsafe {
            ffi::cairo_set_font_matrix(self.0, &matrix)
        }
    }

    pub fn get_font_matrix(&self) -> Matrix {
        let mut matrix = <Matrix as MatrixTrait>::null();
        unsafe {
            ffi::cairo_get_font_matrix(self.0, &mut matrix);
        }
        matrix
    }

    pub fn set_font_options(&self, options: FontOptions) {
        unsafe {
            ffi::cairo_set_font_options(self.0, options.get_ptr())
        }
    }

    pub fn get_font_options(&self) -> FontOptions {
        let out = FontOptions::new();
        unsafe {
            ffi::cairo_get_font_options(self.0, out.get_ptr());
        }
        out
    }

    pub fn set_font_face(&self, font_face: FontFace) {
        unsafe {
            ffi::cairo_set_font_face(self.0, font_face.get_ptr())
        }
    }

    pub fn get_font_face(&self) -> FontFace {
        unsafe {
            FontFace(ffi::cairo_get_font_face(self.0))
        }
    }

    pub fn set_scaled_font(&self, scaled_font: ScaledFont) {
        unsafe {
            ffi::cairo_set_scaled_font(self.0, scaled_font.get_ptr())
        }
    }

    pub fn get_scaled_font(&self) -> ScaledFont {
        unsafe {
            ScaledFont(ffi::cairo_get_scaled_font(self.0))
        }
    }

    pub fn show_text(&self, text: &str) {
        unsafe {
            ffi::cairo_show_text(self.0, text.to_glib_none().0)
        }
    }

    pub fn show_glyphs(&self, glyphs: &[Glyph]) {
        unsafe {
            ffi::cairo_show_glyphs(self.0, glyphs.as_ptr(), glyphs.len() as c_int)
        }
    }

    pub fn show_text_glyphs(&self,
                            text: &str,
                            glyphs: &[Glyph],
                            clusters: &[TextCluster],
                            cluster_flags: TextClusterFlags) {
        unsafe {
            ffi::cairo_show_text_glyphs(self.0,
                                        text.to_glib_none().0,
                                        -1 as c_int, //NULL terminated
                                        glyphs.as_ptr(),
                                        glyphs.len() as c_int,
                                        clusters.as_ptr(),
                                        clusters.len() as c_int,
                                        cluster_flags)
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
            ffi::cairo_font_extents(self.0, &mut extents);
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
            ffi::cairo_text_extents(self.0, text.to_glib_none().0, &mut extents);
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
            ffi::cairo_glyph_extents(self.0, glyphs.as_ptr(), glyphs.len() as c_int, &mut extents);
        }

        extents
    }

    // paths stuff

    pub fn copy_path(&self) -> Path {
        unsafe {
            Path::wrap(ffi::cairo_copy_path(self.0))
        }
    }

    pub fn copy_path_flat(&self) -> Path {
        unsafe {
            Path::wrap(ffi::cairo_copy_path_flat(self.0))
        }
    }

    pub fn append_path(&self, path: &Path) {
        unsafe {
            ffi::cairo_append_path(self.0, path.get_ptr())
        }
    }

    pub fn has_current_point(&self) -> bool {
        unsafe {
            ffi::cairo_has_current_point(self.0).as_bool()
        }
    }

    pub fn get_current_point(&self) -> (f64, f64) {
        unsafe {
            let x = transmute(Box::new(0.0f64));
            let y = transmute(Box::new(0.0f64));
            ffi::cairo_get_current_point(self.0, x, y);
            (*x, *y)
        }
    }

    pub fn new_path(&self) {
        unsafe {
            ffi::cairo_new_path(self.0)
        }
    }

    pub fn new_sub_path(&self) {
        unsafe {
            ffi::cairo_new_sub_path(self.0)
        }
    }

    pub fn close_path(&self) {
        unsafe {
            ffi::cairo_close_path(self.0)
        }
    }

    pub fn arc(&self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) {
        unsafe {
            ffi::cairo_arc(self.0, xc, yc, radius, angle1, angle2)
        }
    }

    pub fn arc_negative(&self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) {
        unsafe {
            ffi::cairo_arc_negative(self.0, xc, yc, radius, angle1, angle2)
        }
    }

    pub fn curve_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        unsafe {
            ffi::cairo_curve_to(self.0, x1, y1, x2, y2, x3, y3)
        }
    }

    pub fn line_to(&self, x: f64, y: f64) {
        unsafe {
            ffi::cairo_line_to(self.0, x, y)
        }
    }

    pub fn move_to(&self, x: f64, y: f64) {
        unsafe {
            ffi::cairo_move_to(self.0, x, y)
        }
    }

    pub fn rectangle(&self, x: f64, y: f64, width: f64, height: f64) {
        unsafe {
            ffi::cairo_rectangle(self.0, x, y, width, height)
        }
    }

    pub fn text_path(&self, str_: &str) {
        unsafe {
            ffi::cairo_text_path(self.0, str_.to_glib_none().0)
        }
    }

    //fn ffi::cairo_glyph_path(cr: *mut cairo_t, glyphs: *mut cairo_glyph_t, num_glyphs: isize);

    pub fn rel_curve_to(&self, dx1: f64, dy1: f64, dx2: f64, dy2: f64, dx3: f64, dy3: f64) {
        unsafe {
            ffi::cairo_rel_curve_to(self.0, dx1, dy1, dx2, dy2, dx3, dy3)
        }
    }

    pub fn rel_line_to(&self, dx: f64, dy: f64) {
        unsafe {
            ffi::cairo_rel_line_to(self.0, dx, dy)
        }
    }

    pub fn rel_move_to(&self, dx: f64, dy: f64) {
        unsafe {
            ffi::cairo_rel_move_to(self.0, dx, dy)
        }
    }
}

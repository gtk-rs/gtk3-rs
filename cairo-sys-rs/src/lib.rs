// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(non_camel_case_types)]

extern crate libc;

#[cfg(feature = "xlib")]
extern crate x11;

#[cfg(windows)]
extern crate winapi;

use libc::{c_void, c_int, c_uint, c_char, c_uchar, c_double, c_ulong};

#[cfg(feature = "xlib")]
use x11::xlib;

pub mod enums;

use enums::{
    Status,
    Content,
    Antialias,
    LineCap,
    LineJoin,
    FillRule,
    FontSlant,
    FontWeight,
    TextClusterFlags,
    FontType,
    SubpixelOrder,
    HintStyle,
    HintMetrics,
    Extend,
    Filter,
    PathDataType,
    PatternType,
    Format,
    SurfaceType,
    Operator,
};

#[repr(C)]
pub struct cairo_t(c_void);
#[repr(C)]
pub struct cairo_surface_t(c_void);
#[repr(C)]
pub struct cairo_pattern_t(c_void);
#[repr(C)]
pub struct cairo_fill_rule_t(c_void);
#[repr(C)]
pub struct cairo_antialias_t(c_void);
#[repr(C)]
pub struct cairo_line_join_t(c_void);
#[repr(C)]
pub struct cairo_line_cap_t(c_void);

#[cfg(feature = "xcb")]
#[repr(C)]
pub struct cairo_device_t(c_void);
#[cfg(feature = "xcb")]
#[repr(C)]
pub struct xcb_connection_t(c_void);
#[cfg(feature = "xcb")]
pub type xcb_drawable_t = u32;
#[cfg(feature = "xcb")]
pub type xcb_pixmap_t = u32;
#[cfg(feature = "xcb")]
#[repr(C)]
pub struct xcb_visualtype_t(c_void);
#[cfg(feature = "xcb")]
#[repr(C)]
pub struct xcb_screen_t(c_void);
#[cfg(feature = "xcb")]
#[repr(C)]
pub struct xcb_render_pictforminfo_t(c_void);

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct cairo_rectangle_t {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct cairo_rectangle_int_t {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
#[repr(C)]
pub struct cairo_rectangle_list_t {
    pub status: Status,
    pub rectangles: *mut cairo_rectangle_t,
    pub num_rectangles: c_int
}
#[repr(C)]
pub struct cairo_content_t(c_void);
#[repr(C)]
pub struct cairo_path_t {
    pub status: Status,
    pub data: *mut [c_double; 2],
    pub num_data: c_int
}
#[repr(C)]
pub struct cairo_path_data_header{
    pub data_type: PathDataType,
    pub length:    c_int
}
#[repr(C)]
pub struct cairo_glyph_t(c_void);
#[repr(C)]
pub struct cairo_region_t(c_void);
#[repr(C)]
pub struct cairo_font_face_t(c_void);
#[repr(C)]
pub struct cairo_scaled_font_t(c_void);
#[repr(C)]
pub struct cairo_font_options_t(c_void);
#[repr(C)]
pub struct cairo_extend_t(c_void);
#[repr(C)]
pub struct cairo_filter_t(c_void);
#[repr(C)]
pub struct cairo_region_overlap_t(c_void);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FontExtents {
    pub ascent: c_double,
    pub descent: c_double,
    pub height: c_double,
    pub max_x_advance: c_double,
    pub max_y_advance: c_double,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Glyph {
    pub index: c_ulong,
    pub x: c_double,
    pub y: c_double,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TextCluster {
    pub num_bytes: c_int,
    pub num_glyphs: c_int,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TextExtents {
    pub x_bearing: c_double,
    pub y_bearing: c_double,
    pub width: c_double,
    pub height: c_double,
    pub x_advance: c_double,
    pub y_advance: c_double,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Matrix {
    pub xx: c_double,
    pub yx: c_double,

    pub xy: c_double,
    pub yy: c_double,

    pub x0: c_double,
    pub y0: c_double,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct cairo_user_data_key_t {
    pub unused: c_int,
}
#[repr(C)]
pub struct cairo_bool_t{
    value: c_int
}

impl cairo_bool_t{
    pub fn as_bool(&self) -> bool{
        self.value != 0
    }
}

pub type cairo_destroy_func_t = Option<unsafe extern fn (*mut c_void)>;
pub type cairo_read_func_t = Option<unsafe extern fn (*mut c_void, *mut c_uchar, c_uint) -> Status>;
pub type cairo_write_func_t = Option<unsafe extern fn (*mut c_void, *mut c_uchar, c_uint) -> Status>;

extern "C" {
    //CAIRO CONTEXT
    pub fn cairo_create (target: *mut cairo_surface_t) -> *mut cairo_t;
    pub fn cairo_reference (cr: *mut cairo_t) -> *mut cairo_t;
    pub fn cairo_destroy (cr: *mut cairo_t);
    pub fn cairo_status (cr: *mut cairo_t) -> Status;
    pub fn cairo_save (cr: *mut cairo_t);
    pub fn cairo_restore (cr: *mut cairo_t);
    pub fn cairo_get_target (cr: *mut cairo_t) -> *mut cairo_surface_t;
    pub fn cairo_push_group (cr: *mut cairo_t);
    pub fn cairo_push_group_with_content (cr: *mut cairo_t, content: Content);
    pub fn cairo_pop_group (cr: *mut cairo_t) -> *mut cairo_pattern_t;
    pub fn cairo_pop_group_to_source (cr: *mut cairo_t);
    pub fn cairo_get_group_target (cr: *mut cairo_t) -> *mut cairo_surface_t;
    pub fn cairo_set_source_rgb (cr: *mut cairo_t, red: c_double, green: c_double, blue: c_double);
    pub fn cairo_set_source_rgba (cr: *mut cairo_t, red: c_double, green: c_double, blue: c_double, alpha: c_double);
    pub fn cairo_set_source (cr: *mut cairo_t, source: *mut cairo_pattern_t);
    pub fn cairo_set_source_surface (cr: *mut cairo_t, surface: *mut cairo_surface_t, x: c_double, y: c_double);
    pub fn cairo_get_source (cr: *mut cairo_t) -> *mut cairo_pattern_t;
    pub fn cairo_set_antialias (cr: *mut cairo_t, antialias: Antialias);
    pub fn cairo_get_antialias (cr: *mut cairo_t) -> Antialias;
    pub fn cairo_set_dash (cr: *mut cairo_t, dashes : *const c_double, num_dashes: c_int, offset: c_double);
    pub fn cairo_get_dash_count (cr: *mut cairo_t) -> c_int;
    pub fn cairo_get_dash (cr: *mut cairo_t, dashes: *mut c_double, offset: *mut c_double);
    pub fn cairo_set_fill_rule (cr: *mut cairo_t, fill_rule: FillRule);
    pub fn cairo_get_fill_rule (cr: *mut cairo_t) -> FillRule;
    pub fn cairo_set_line_cap (cr: *mut cairo_t, line_cap: LineCap);
    pub fn cairo_get_line_cap (cr: *mut cairo_t) -> LineCap;
    pub fn cairo_set_line_join (cr: *mut cairo_t, line_join: LineJoin);
    pub fn cairo_get_line_join (cr: *mut cairo_t) -> LineJoin;
    pub fn cairo_set_line_width (cr: *mut cairo_t, width: c_double);
    pub fn cairo_get_line_width (cr: *mut cairo_t) -> c_double;
    pub fn cairo_set_miter_limit (cr: *mut cairo_t, limit: c_double);
    pub fn cairo_get_miter_limit (cr: *mut cairo_t) -> c_double;
    pub fn cairo_set_operator (cr: *mut cairo_t, op: Operator);
    pub fn cairo_get_operator (cr: *mut cairo_t) -> Operator;
    pub fn cairo_set_tolerance (cr: *mut cairo_t, tolerance: c_double);
    pub fn cairo_get_tolerance (cr: *mut cairo_t) -> c_double;
    pub fn cairo_clip (cr: *mut cairo_t);
    pub fn cairo_clip_preserve (cr: *mut cairo_t);
    pub fn cairo_clip_extents (cr: *mut cairo_t, x1: *mut c_double, y1: *mut c_double, x2: *mut c_double, y2: *mut c_double);
    pub fn cairo_in_clip (cr: *mut cairo_t, x: c_double, y: c_double) -> cairo_bool_t;
    pub fn cairo_reset_clip (cr: *mut cairo_t);
    pub fn cairo_rectangle_list_destroy (rectangle_list: *mut cairo_rectangle_list_t);
    pub fn cairo_copy_clip_rectangle_list (cr: *mut cairo_t) -> *mut cairo_rectangle_list_t;
    pub fn cairo_fill (cr: *mut cairo_t);
    pub fn cairo_fill_preserve (cr: *mut cairo_t);
    pub fn cairo_fill_extents (cr: *mut cairo_t, x1: *mut c_double, y1: *mut c_double, x2: *mut c_double, y2: *mut c_double);
    pub fn cairo_in_fill (cr: *mut cairo_t, x: c_double, y: c_double) -> cairo_bool_t;
    pub fn cairo_mask (cr: *mut cairo_t, pattern: *mut cairo_pattern_t);
    pub fn cairo_mask_surface (cr: *mut cairo_t, surface: *mut cairo_surface_t, surface_x: c_double, surface_y: c_double);
    pub fn cairo_paint (cr: *mut cairo_t);
    pub fn cairo_paint_with_alpha (cr: *mut cairo_t, alpha: c_double);
    pub fn cairo_stroke (cr: *mut cairo_t);
    pub fn cairo_stroke_preserve (cr: *mut cairo_t);
    pub fn cairo_stroke_extents (cr: *mut cairo_t, x1: *mut c_double, y1: *mut c_double, x2: *mut c_double, y2: *mut c_double);
    pub fn cairo_in_stroke (cr: *mut cairo_t, x: c_double, y: c_double) -> cairo_bool_t;
    pub fn cairo_copy_page (cr: *mut cairo_t);
    pub fn cairo_show_page (cr: *mut cairo_t);
    pub fn cairo_get_reference_count (cr: *mut cairo_t) -> c_uint;

    //CAIRO UTILS: Error handling
    pub fn cairo_status_to_string (status : Status) -> *const c_char;


    //CAIRO PATHS
    pub fn cairo_copy_path(cr: *mut cairo_t) -> *mut cairo_path_t;
    pub fn cairo_copy_path_flat(cr: *mut cairo_t) -> *mut cairo_path_t;
    pub fn cairo_path_destroy(path: *mut cairo_path_t);
    pub fn cairo_append_path(cr: *mut cairo_t, path: *mut cairo_path_t);
    pub fn cairo_has_current_point(cr: *mut cairo_t) -> cairo_bool_t;
    pub fn cairo_get_current_point(cr: *mut cairo_t, x: *mut c_double, y: *mut c_double);
    pub fn cairo_new_path(cr: *mut cairo_t);
    pub fn cairo_new_sub_path(cr: *mut cairo_t);
    pub fn cairo_close_path(cr: *mut cairo_t);
    pub fn cairo_arc(cr: *mut cairo_t, xc: c_double, yc: c_double, radius: c_double, angle1: c_double, angle2: c_double);
    pub fn cairo_arc_negative(cr: *mut cairo_t, xc: c_double, yc: c_double, radius: c_double, angle1: c_double, angle2: c_double);
    pub fn cairo_curve_to(cr: *mut cairo_t, x1: c_double, y1: c_double, x2: c_double, y2: c_double, x3: c_double, y3: c_double);
    pub fn cairo_line_to(cr: *mut cairo_t, x: c_double, y: c_double);
    pub fn cairo_move_to(cr: *mut cairo_t, x: c_double, y: c_double);
    pub fn cairo_rectangle(cr: *mut cairo_t, x: c_double, y: c_double, width: c_double, height: c_double);
    pub fn cairo_glyph_path(cr: *mut cairo_t, glyphs: *mut Glyph, num_glyphs: c_int);
    pub fn cairo_text_path(cr: *mut cairo_t, utf8: *const c_char);
    pub fn cairo_rel_curve_to(cr: *mut cairo_t, dx1: c_double, dy1: c_double, dx2: c_double, dy2: c_double, dx3: c_double, dy3: c_double);
    pub fn cairo_rel_line_to(cr: *mut cairo_t, dx: c_double, dy: c_double);
    pub fn cairo_rel_move_to(cr: *mut cairo_t, dx: c_double, dy: c_double);
    pub fn cairo_path_extents(cr: *mut cairo_t, x1: *mut c_double, y1: *mut c_double, x2: *mut c_double, y2: *mut c_double);

    //CAIRO TRANSFORMATIONS
    pub fn cairo_translate(cr: *mut cairo_t, tx: c_double, ty: c_double);
    pub fn cairo_scale(cr: *mut cairo_t, sx: c_double, sy: c_double);
    pub fn cairo_rotate(cr: *mut cairo_t, angle: c_double);
    pub fn cairo_transform(cr: *mut cairo_t, matrix: *const Matrix);
    pub fn cairo_set_matrix(cr: *mut cairo_t, matrix: *const Matrix);
    pub fn cairo_get_matrix(cr: *mut cairo_t, matrix: *mut Matrix);
    pub fn cairo_identity_matrix(cr: *mut cairo_t);
    pub fn cairo_user_to_device(cr: *mut cairo_t, x: *mut c_double, y: *mut c_double);
    pub fn cairo_user_to_device_distance(cr: *mut cairo_t, dx: *mut c_double, dy: *mut c_double);
    pub fn cairo_device_to_user(cr: *mut cairo_t, x: *mut c_double, y: *mut c_double);
    pub fn cairo_device_to_user_distance(cr: *mut cairo_t, dx: *mut c_double, dy: *mut c_double);

    //CAIRO PATTERNS
    pub fn cairo_pattern_add_color_stop_rgb(pattern: *mut cairo_pattern_t, offset: c_double, red: c_double, green: c_double, blue: c_double);
    pub fn cairo_pattern_add_color_stop_rgba(pattern: *mut cairo_pattern_t, offset: c_double, red: c_double, green: c_double, blue: c_double, alpha: c_double);
    pub fn cairo_pattern_get_color_stop_count(pattern: *mut cairo_pattern_t, count: *mut c_int) -> Status;
    pub fn cairo_pattern_get_color_stop_rgba(pattern: *mut cairo_pattern_t, index: c_int, offset: *mut c_double, red: *mut c_double, green: *mut c_double, blue: *mut c_double, alpha: *mut c_double) -> Status;
    pub fn cairo_pattern_create_rgb(red: c_double, green: c_double, blue: c_double) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_create_rgba(red: c_double, green: c_double, blue: c_double, alpha: c_double) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_get_rgba(pattern: *mut cairo_pattern_t, red: *mut c_double, green: *mut c_double, blue: *mut c_double, alpha: *mut c_double) -> Status;
    pub fn cairo_pattern_create_for_surface(surface: *mut cairo_surface_t) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_get_surface(pattern: *mut cairo_pattern_t, surface: *mut *mut cairo_surface_t) -> Status;
    pub fn cairo_pattern_create_linear(x0: c_double, y0: c_double, x1: c_double, y1: c_double) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_get_linear_points(pattern: *mut cairo_pattern_t, x0: *mut c_double, y0: *mut c_double, x1: *mut c_double, y1: *mut c_double) -> Status;
    pub fn cairo_pattern_create_radial(cx0: c_double, cy0: c_double, radius0: c_double, cx1: c_double, cy1: c_double, radius1: c_double) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_get_radial_circles(pattern: *mut cairo_pattern_t, x0: *mut c_double, y0: *mut c_double, r0: *mut c_double, x1: *mut c_double, y1: *mut c_double, r1: *mut c_double) -> Status;
    pub fn cairo_pattern_create_mesh() -> *mut cairo_pattern_t;
    pub fn cairo_mesh_pattern_begin_patch(pattern: *mut cairo_pattern_t);
    pub fn cairo_mesh_pattern_end_patch(pattern: *mut cairo_pattern_t);
    pub fn cairo_mesh_pattern_move_to(pattern: *mut cairo_pattern_t, x: c_double, y: c_double);
    pub fn cairo_mesh_pattern_line_to(pattern: *mut cairo_pattern_t, x: c_double, y: c_double);
    pub fn cairo_mesh_pattern_curve_to(pattern: *mut cairo_pattern_t, x1: c_double, y1: c_double, x2: c_double, y2: c_double, x3: c_double, y3: c_double);
    pub fn cairo_mesh_pattern_set_control_point(pattern: *mut cairo_pattern_t, point_num: c_uint, x: c_double, y: c_double);
    pub fn cairo_mesh_pattern_set_corner_color_rgb(pattern: *mut cairo_pattern_t, corner_num: c_uint, red: c_double, green: c_double, blue: c_double);
    pub fn cairo_mesh_pattern_set_corner_color_rgba(pattern: *mut cairo_pattern_t, corner_num: c_uint, red: c_double, green: c_double, blue: c_double, alpha: c_double);
    pub fn cairo_mesh_pattern_get_patch_count(pattern: *mut cairo_pattern_t, count: *mut c_uint) -> Status;
    pub fn cairo_mesh_pattern_get_path(pattern: *mut cairo_pattern_t, patch_num: c_uint) -> *mut cairo_path_t;
    pub fn cairo_mesh_pattern_get_control_point(pattern: *mut cairo_pattern_t, patch_num: c_uint, point_num: c_uint, x: *mut c_double, y: *mut c_double) -> Status;
    pub fn cairo_mesh_pattern_get_corner_color_rgba(pattern: *mut cairo_pattern_t, patch_num: c_uint, corner_num: c_uint, red: *mut c_double, green: *mut c_double, blue: *mut c_double, alpha: *mut c_double) -> Status;
    pub fn cairo_pattern_reference(pattern: *mut cairo_pattern_t) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_destroy(pattern: *mut cairo_pattern_t);
    pub fn cairo_pattern_status(pattern: *mut cairo_pattern_t) -> Status;
    pub fn cairo_pattern_set_extend(pattern: *mut cairo_pattern_t, extend: Extend);
    pub fn cairo_pattern_get_extend(pattern: *mut cairo_pattern_t) -> Extend;
    pub fn cairo_pattern_set_filter(pattern: *mut cairo_pattern_t, filter: Filter);
    pub fn cairo_pattern_get_filter(pattern: *mut cairo_pattern_t) -> Filter;
    pub fn cairo_pattern_set_matrix(pattern: *mut cairo_pattern_t, matrix: *const Matrix);
    pub fn cairo_pattern_get_matrix(pattern: *mut cairo_pattern_t, matrix: *mut Matrix);
    pub fn cairo_pattern_get_type(pattern: *mut cairo_pattern_t) -> PatternType;
    pub fn cairo_pattern_get_reference_count(pattern: *mut cairo_pattern_t) -> c_uint;
    //pub fn cairo_pattern_set_user_data(pattern: *mut cairo_pattern_t, key: *mut cairo_user_data_key_t, user_data: *mut void, destroy: cairo_destroy_func_t) -> Status;
    //pub fn cairo_pattern_get_user_data(pattern: *mut cairo_pattern_t, key: *mut cairo_user_data_key_t) -> *mut void;

    //CAIRO REGIONS
    pub fn cairo_region_create() -> *mut cairo_region_t;
    pub fn cairo_region_create_rectangle(rectangle: *mut cairo_rectangle_int_t) -> *mut cairo_region_t;
    pub fn cairo_region_create_rectangles(rects: *mut cairo_rectangle_int_t, count: c_int) -> *mut cairo_region_t;
    pub fn cairo_region_copy(original: *mut cairo_region_t) -> *mut cairo_region_t;
    pub fn cairo_region_reference(region: *mut cairo_region_t) -> *mut cairo_region_t;
    pub fn cairo_region_destroy(region: *mut cairo_region_t);
    pub fn cairo_region_status(region: *mut cairo_region_t) -> Status;
    pub fn cairo_region_get_extents(region: *mut cairo_region_t, extents: *mut cairo_rectangle_int_t);
    pub fn cairo_region_num_rectangles(region: *mut cairo_region_t) -> c_int;
    pub fn cairo_region_get_rectangle(region: *mut cairo_region_t, nth: c_int, rectangle: *mut cairo_rectangle_int_t);
    pub fn cairo_region_is_empty(region: *mut cairo_region_t) -> cairo_bool_t;
    pub fn cairo_region_contains_point(region: *mut cairo_region_t, x: c_int, y: c_int) -> cairo_bool_t;
    //enum                cairo_region_overlap_t;
    pub fn cairo_region_contains_rectangle(region: *mut cairo_region_t, rectangle: *mut cairo_rectangle_int_t) -> cairo_region_overlap_t;
    pub fn cairo_region_equal(a: *mut cairo_region_t, b: *mut cairo_region_t) -> cairo_bool_t;
    pub fn cairo_region_translate(region: *mut cairo_region_t, dx: c_int, dy: c_int);
    pub fn cairo_region_intersect(dst: *mut cairo_region_t, other: *mut cairo_region_t) -> Status;
    pub fn cairo_region_intersect_rectangle(dst: *mut cairo_region_t, rectangle: *mut cairo_rectangle_int_t) -> Status;
    pub fn cairo_region_subtract(dst: *mut cairo_region_t, other: *mut cairo_region_t) -> Status;
    pub fn cairo_region_subtract_rectangle(dst: *mut cairo_region_t, rectangle: *mut cairo_rectangle_int_t) -> Status;
    pub fn cairo_region_union(dst: *mut cairo_region_t, other: *mut cairo_region_t) -> Status;
    pub fn cairo_region_union_rectangle(dst: *mut cairo_region_t, rectangle: *mut cairo_rectangle_int_t) -> Status;
    pub fn cairo_region_xor(dst: *mut cairo_region_t, other: *mut cairo_region_t) -> Status;
    pub fn cairo_region_xor_rectangle(dst: *mut cairo_region_t, rectangle: *mut cairo_rectangle_int_t) -> Status;

    //text
    pub fn cairo_select_font_face(cr: *mut cairo_t, family: *const c_char, slant: FontSlant, weight: FontWeight);
    pub fn cairo_set_font_size(cr: *mut cairo_t, size: c_double);
    pub fn cairo_set_font_matrix(cr: *mut cairo_t, matrix: *const Matrix);
    pub fn cairo_get_font_matrix(cr: *mut cairo_t, matrix: *mut Matrix);
    pub fn cairo_set_font_options(cr: *mut cairo_t, options: *mut cairo_font_options_t);
    pub fn cairo_get_font_options(cr: *mut cairo_t, options: *mut cairo_font_options_t);
    pub fn cairo_set_font_face(cr: *mut cairo_t, font_face: *mut cairo_font_face_t);
    pub fn cairo_get_font_face(cr: *mut cairo_t) -> *mut cairo_font_face_t;
    pub fn cairo_set_scaled_font(cr: *mut cairo_t, scaled_font: *mut cairo_scaled_font_t);
    pub fn cairo_get_scaled_font(cr: *mut cairo_t) -> *mut cairo_scaled_font_t;
    pub fn cairo_show_text(cr: *mut cairo_t, utf8: *const c_char);
    pub fn cairo_show_glyphs(cr: *mut cairo_t, glyphs: *const Glyph, num_glyphs: c_int);
    pub fn cairo_show_text_glyphs(cr: *mut cairo_t, utf8: *const c_char, utf8_len: c_int, glyphs: *const Glyph, num_glyphs: c_int, clusters: *const TextCluster, num_clusters: c_int, cluster_flags: TextClusterFlags);
    pub fn cairo_font_extents(cr: *mut cairo_t, extents: *mut FontExtents);
    pub fn cairo_text_extents(cr: *mut cairo_t, utf8: *const c_char, extents: *mut TextExtents);
    pub fn cairo_glyph_extents(cr: *mut cairo_t, glyphs: *const Glyph, num_glyphs: c_int, extents: *mut TextExtents);
    pub fn cairo_toy_font_face_create(family: *const c_char, slant: FontSlant, weight: FontWeight) -> *mut cairo_font_face_t;
    pub fn cairo_toy_font_face_get_family(font_face: *mut cairo_font_face_t) -> *const c_char;
    pub fn cairo_toy_font_face_get_slant(font_face: *mut cairo_font_face_t) -> FontSlant;
    pub fn cairo_toy_font_face_get_weight(font_face: *mut cairo_font_face_t) -> FontWeight;
    pub fn cairo_glyph_allocate(num_glyphs: c_int) -> *mut Glyph;
    pub fn cairo_glyph_free(glyphs: *mut Glyph);
    pub fn cairo_text_cluster_allocate(num_clusters: c_int) -> *mut TextCluster;
    pub fn cairo_text_cluster_free(clusters: *mut TextCluster);

    //CAIRO RASTER
    //pub fn cairo_pattern_create_raster_source(user_data: *mut void, content: Content, width: c_int, height: c_int) -> *mut cairo_pattern_t;
    //pub fn cairo_raster_source_pattern_set_callback_data(pattern: *mut cairo_pattern_t, data: *mut void);
    //pub fn cairo_raster_source_pattern_get_callback_data(pattern: *mut cairo_pattern_t) -> *mut void;
    /* FIXME how do we do these _func_t types?
    pub fn cairo_raster_source_pattern_set_acquire(pattern: *mut cairo_pattern_t, acquire: cairo_raster_source_acquire_func_t, release: cairo_raster_source_release_func_t);
    pub fn cairo_raster_source_pattern_get_acquire(pattern: *mut cairo_pattern_t, acquire: *mut cairo_raster_source_acquire_func_t, release: *mut cairo_raster_source_release_func_t);
    pub fn cairo_raster_source_pattern_set_snapshot(pattern: *mut cairo_pattern_t, snapshot: cairo_raster_source_snapshot_func_t);
    pub fn cairo_raster_source_pattern_get_snapshot(pattern: *mut cairo_pattern_t) -> cairo_raster_source_snapshot_func_t;
    pub fn cairo_raster_source_pattern_set_copy(pattern: *mut cairo_pattern_t, copy: cairo_raster_source_copy_func_t);
    pub fn cairo_raster_source_pattern_get_copy(pattern: *mut cairo_pattern_t) -> cairo_raster_source_copy_func_t;
    pub fn cairo_raster_source_pattern_set_finish(pattern: *mut cairo_pattern_t, finish: cairo_raster_source_finish_func_t);
    pub fn cairo_raster_source_pattern_get_finish(pattern: *mut cairo_pattern_t) -> cairo_raster_source_finish_func_t;
    */

    //cairo_surface_t     (*mut cairo_raster_source_acquire_func_t)
    //                                                        (pattern: *mut cairo_pattern_t, callback_data: *mut void, target: *mut cairo_surface_t, extents: *mut cairo_rectangle_int_t);
    //void                (*mut cairo_raster_source_release_func_t)
    //                                                        (pattern: *mut cairo_pattern_t, callback_data: *mut void, surface: *mut cairo_surface_t);
    //Status      (*mut cairo_raster_source_snapshot_func_t)
    //                                                        (pattern: *mut cairo_pattern_t, callback_data: *mut void);
    //Status      (*mut cairo_raster_source_copy_func_t)  (pattern: *mut cairo_pattern_t, callback_data: *mut void, other: *mut cairo_pattern_t);
    //void                (*mut cairo_raster_source_finish_func_t)
    //                                                        (pattern: *mut cairo_pattern_t, callback_data: *mut void);

    //CAIRO FONT
    pub fn cairo_font_face_reference(font_face: *mut cairo_font_face_t) -> *mut cairo_font_face_t;
    pub fn cairo_font_face_destroy(font_face: *mut cairo_font_face_t);
    pub fn cairo_font_face_status(font_face: *mut cairo_font_face_t) -> Status;
    pub fn cairo_font_face_get_type(font_face: *mut cairo_font_face_t) -> FontType;
    pub fn cairo_font_face_get_reference_count(font_face: *mut cairo_font_face_t) -> c_uint;
    //pub fn cairo_font_face_set_user_data(font_face: *mut cairo_font_face_t, key: *mut cairo_user_data_key_t, user_data: *mut void, destroy: cairo_destroy_func_t) -> Status;
    //pub fn cairo_font_face_get_user_data(font_face: *mut cairo_font_face_t, key: *mut cairo_user_data_key_t) -> *mut void;

    //CAIRO SCALED FONT
    pub fn cairo_scaled_font_create(font_face: *mut cairo_font_face_t, font_matrix: *mut Matrix, ctm: *mut Matrix, options: *mut cairo_font_options_t) -> *mut cairo_scaled_font_t;
    pub fn cairo_scaled_font_reference(scaled_font: *mut cairo_scaled_font_t) -> *mut cairo_scaled_font_t;
    pub fn cairo_scaled_font_destroy(scaled_font: *mut cairo_scaled_font_t);
    pub fn cairo_scaled_font_status(scaled_font: *mut cairo_scaled_font_t) -> Status;
    //                    FontExtents;
    pub fn cairo_scaled_font_extents(scaled_font: *mut cairo_scaled_font_t, extents: *mut FontExtents);
    //                    TextExtents;
    pub fn cairo_scaled_font_text_extents(scaled_font: *mut cairo_scaled_font_t, utf8: *mut c_char, extents: *mut TextExtents);
    pub fn cairo_scaled_font_glyph_extents(scaled_font: *mut cairo_scaled_font_t, glyphs: *mut Glyph, num_glyphs: c_int, extents: *mut TextExtents);
    pub fn cairo_scaled_font_text_to_glyphs(scaled_font: *mut cairo_scaled_font_t, x: c_double, y: c_double, utf8: *mut c_char, utf8_len: c_int, glyphs: *mut *mut Glyph, num_glyphs: *mut c_int, clusters: *mut *mut TextCluster, num_clusters: *mut c_int, cluster_flags: *mut TextClusterFlags) -> Status;
    pub fn cairo_scaled_font_get_font_face(scaled_font: *mut cairo_scaled_font_t) -> *mut cairo_font_face_t;
    pub fn cairo_scaled_font_get_font_options(scaled_font: *mut cairo_scaled_font_t, options: *mut cairo_font_options_t);
    pub fn cairo_scaled_font_get_font_matrix(scaled_font: *mut cairo_scaled_font_t, font_matrix: *mut Matrix);
    pub fn cairo_scaled_font_get_ctm(scaled_font: *mut cairo_scaled_font_t, ctm: *mut Matrix);
    pub fn cairo_scaled_font_get_scale_matrix(scaled_font: *mut cairo_scaled_font_t, scale_matrix: *mut Matrix);
    pub fn cairo_scaled_font_get_type(scaled_font: *mut cairo_scaled_font_t) -> FontType;
    pub fn cairo_scaled_font_get_reference_count(font_face: *mut cairo_scaled_font_t) -> c_uint;
    //pub fn cairo_scaled_font_set_user_data(scaled_font: *mut cairo_scaled_font_t, key: *mut cairo_user_data_key_t, user_data: *mut void, destroy: cairo_destroy_func_t) -> Status;
    //pub fn cairo_scaled_font_get_user_data(scaled_font: *mut cairo_scaled_font_t, key: *mut cairo_user_data_key_t) -> *mut void;

    //CAIRO FONT OPTIONS
    pub fn cairo_font_options_create() -> *mut cairo_font_options_t;
    pub fn cairo_font_options_copy(original: *mut cairo_font_options_t) -> *mut cairo_font_options_t;
    pub fn cairo_font_options_destroy(options: *mut cairo_font_options_t);
    pub fn cairo_font_options_status(options: *mut cairo_font_options_t) -> Status;
    pub fn cairo_font_options_merge(options: *mut cairo_font_options_t, other: *mut cairo_font_options_t);
    pub fn cairo_font_options_hash(options: *mut cairo_font_options_t) -> c_ulong;
    pub fn cairo_font_options_equal(options: *mut cairo_font_options_t, other: *mut cairo_font_options_t) -> cairo_bool_t;
    pub fn cairo_font_options_set_antialias(options: *mut cairo_font_options_t, antialias: Antialias);
    pub fn cairo_font_options_get_antialias(options: *mut cairo_font_options_t) -> Antialias;
    pub fn cairo_font_options_set_subpixel_order(options: *mut cairo_font_options_t, subpixel_order: SubpixelOrder);
    pub fn cairo_font_options_get_subpixel_order(options: *mut cairo_font_options_t) -> SubpixelOrder;
    pub fn cairo_font_options_set_hint_style(options: *mut cairo_font_options_t, hint_style: HintStyle);
    pub fn cairo_font_options_get_hint_style(options: *mut cairo_font_options_t) -> HintStyle;
    pub fn cairo_font_options_set_hint_metrics(options: *mut cairo_font_options_t, hint_metrics: HintMetrics);
    pub fn cairo_font_options_get_hint_metrics(options: *mut cairo_font_options_t) -> HintMetrics;

    // CAIRO MATRIX
    pub fn cairo_matrix_multiply(matrix: *mut Matrix, left: *const Matrix, right: *const Matrix);
    pub fn cairo_matrix_init(matrix: *mut Matrix, xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64);
    pub fn cairo_matrix_init_identity(matrix: *mut Matrix);
    pub fn cairo_matrix_translate(matrix: *mut Matrix, tx: f64, ty: f64);
    pub fn cairo_matrix_scale(matrix: *mut Matrix, sx: f64, sy: f64);
    pub fn cairo_matrix_rotate(matrix: *mut Matrix, angle: f64);
    pub fn cairo_matrix_invert(matrix: *mut Matrix) -> Status;
    pub fn cairo_matrix_transform_distance(matrix: *const Matrix, dx: *mut f64, dy: *mut f64);
    pub fn cairo_matrix_transform_point(matrix: *const Matrix, x: *mut f64, y: *mut f64);

    // CAIRO SURFACE
    pub fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    pub fn cairo_surface_flush(surface: *mut cairo_surface_t);
    pub fn cairo_surface_finish(surface: *mut cairo_surface_t);
    pub fn cairo_surface_status(surface: *mut cairo_surface_t) -> Status;
    pub fn cairo_surface_get_type(surface: *mut cairo_surface_t) -> SurfaceType;
    pub fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    pub fn cairo_surface_get_user_data(surface: *mut cairo_surface_t, key: *mut cairo_user_data_key_t) -> *mut c_void;
    pub fn cairo_surface_set_user_data(surface: *mut cairo_surface_t, key: *mut cairo_user_data_key_t, user_data: *mut c_void, destroy: cairo_destroy_func_t) -> Status;
    pub fn cairo_surface_get_reference_count(surface: *mut cairo_surface_t) -> c_uint;
    pub fn cairo_surface_mark_dirty(surface: *mut cairo_surface_t);
    pub fn cairo_surface_create_similar(surface: *mut cairo_surface_t, content: Content, width: c_int, height: c_int) -> *mut cairo_surface_t;

    // CAIRO IMAGE SURFACE
    pub fn cairo_image_surface_create(format: Format, width: c_int, height: c_int) -> *mut cairo_surface_t;
    pub fn cairo_image_surface_create_for_data(data: *mut u8, format: Format, width: c_int, height: c_int, stride: c_int) -> *mut cairo_surface_t;
    pub fn cairo_image_surface_get_data(surface: *mut cairo_surface_t) -> *mut u8;
    pub fn cairo_image_surface_get_format(surface: *mut cairo_surface_t) -> Format;
    pub fn cairo_image_surface_get_height(surface: *mut cairo_surface_t) -> c_int;
    pub fn cairo_image_surface_get_stride(surface: *mut cairo_surface_t) -> c_int;
    pub fn cairo_image_surface_get_width(surface: *mut cairo_surface_t) -> c_int;
    #[cfg(feature = "png")]
    pub fn cairo_image_surface_create_from_png_stream(read_func: cairo_read_func_t, closure: *mut c_void) -> *mut cairo_surface_t;
    #[cfg(feature = "png")]
    pub fn cairo_surface_write_to_png_stream(surface: *mut cairo_surface_t, write_func: cairo_write_func_t, closure: *mut c_void) -> Status;

    // CAIRO XCB
    #[cfg(feature = "xcb")]
    pub fn cairo_xcb_surface_create(connection: *mut xcb_connection_t,
                                    drawable: xcb_drawable_t,
                                    visual: *mut xcb_visualtype_t,
                                    width: c_int,
                                    height: c_int) -> *mut cairo_surface_t;
    #[cfg(feature = "xcb")]
    pub fn cairo_xcb_surface_create_for_bitmap(connection: *mut xcb_connection_t,
                                               screen: *mut xcb_screen_t,
                                               bitmap: xcb_pixmap_t,
                                               width: c_int,
                                               height: c_int) -> *mut cairo_surface_t;
    #[cfg(feature = "xcb")]
    pub fn cairo_xcb_surface_create_with_xrender_format(connection: *mut xcb_connection_t,
                                                        screen: *mut xcb_screen_t,
                                                        drawable: xcb_drawable_t,
                                                        format: *mut xcb_render_pictforminfo_t,
                                                        width: c_int,
                                                        height: c_int) -> *mut cairo_surface_t;
    #[cfg(feature = "xcb")]
    pub fn cairo_xcb_surface_set_size(surface: *mut cairo_surface_t,
                                      width: c_int,
                                      height: c_int);
    #[cfg(feature = "xcb")]
    pub fn cairo_xcb_surface_set_drawable(surface: *mut cairo_surface_t,
                                          drawable: xcb_drawable_t,
                                          width: c_int,
                                          height: c_int);
    #[cfg(feature = "xcb")]
    pub fn cairo_xcb_device_get_connection(device: *mut cairo_device_t) -> *mut xcb_connection_t;
    #[cfg(feature = "xcb")]
    pub fn cairo_xcb_device_debug_cap_xrender_version(device: *mut cairo_device_t,
                                                      major_version: c_int,
                                                      minor_version: c_int);
    #[cfg(feature = "xcb")]
    pub fn cairo_xcb_device_debug_cap_xshm_version(device: *mut cairo_device_t,
                                                   major_version: c_int,
                                                   minor_version: c_int);
    #[cfg(feature = "xcb")]
    pub fn cairo_xcb_device_debug_get_precision(device: *mut cairo_device_t) -> c_int;
    #[cfg(feature = "xlib")]
    pub fn cairo_xcb_device_debug_set_precision(device: *mut cairo_device_t,
                                                precision: c_int);

    // CAIRO XLIB SURFACE
    #[cfg(feature = "xlib")]
    pub fn cairo_xlib_surface_create(dpy: *mut xlib::Display,
                                     drawable: xlib::Drawable,
                                     visual: *mut xlib::Visual,
                                     width: c_int,
                                     height: c_int)
                                     -> *mut cairo_surface_t;
    #[cfg(feature = "xlib")]
    pub fn cairo_xlib_surface_create_for_bitmap(dpy: *mut xlib::Display,
                                                bitmap: xlib::Pixmap,
                                                screen: *mut xlib::Screen,
                                                width: c_int,
                                                height: c_int)
                                                -> *mut cairo_surface_t;
    #[cfg(feature = "xlib")]
    pub fn cairo_xlib_surface_set_size(surface: *mut cairo_surface_t,
                                       width: c_int,
                                       height: c_int);
    #[cfg(feature = "xlib")]
    pub fn cairo_xlib_surface_set_drawable(surface: *mut cairo_surface_t,
                                           drawable: xlib::Drawable,
                                           width: c_int,
                                           height: c_int);
    #[cfg(feature = "xlib")]
    pub fn cairo_xlib_surface_get_display(surface: *mut cairo_surface_t)
                                          -> *mut xlib::Display;
    #[cfg(feature = "xlib")]
    pub fn cairo_xlib_surface_get_drawable(surface: *mut cairo_surface_t)
                                           -> xlib::Drawable;
    #[cfg(feature = "xlib")]
    pub fn cairo_xlib_surface_get_screen(surface: *mut cairo_surface_t)
                                         -> *mut xlib::Screen;
    #[cfg(feature = "xlib")]
    pub fn cairo_xlib_surface_get_visual(surface: *mut cairo_surface_t)
                                         -> *mut xlib::Visual;
    #[cfg(feature = "xlib")]
    pub fn cairo_xlib_surface_get_depth(surface: *mut cairo_surface_t)
                                        -> c_int;
    #[cfg(feature = "xlib")]
    pub fn cairo_xlib_surface_get_width(surface: *mut cairo_surface_t)
                                        -> c_int;
    #[cfg(feature = "xlib")]
    pub fn cairo_xlib_surface_get_height(surface: *mut cairo_surface_t)
                                         -> c_int;

    // CAIRO WINDOWS SURFACE
    #[cfg(windows)]
    pub fn cairo_win32_surface_create(hdc: winapi::HDC) -> *mut cairo_surface_t;
    #[cfg(windows)]
    pub fn cairo_win32_surface_create_with_dib(format: Format,
                                               width: c_int,
                                               height: c_int)
                                               -> *mut cairo_surface_t;
    #[cfg(windows)]
    pub fn cairo_win32_surface_create_with_ddb(hdc: winapi::HDC,
                                               format: Format,
                                               width: c_int,
                                               height: c_int)
                                               -> *mut cairo_surface_t;
    #[cfg(windows)]
    pub fn cairo_win32_printing_surface_create(hdc: winapi::HDC) -> *mut cairo_surface_t;
    #[cfg(windows)]
    pub fn cairo_win32_surface_get_dc(surface: *mut cairo_surface_t) -> winapi::HDC;
    #[cfg(windows)]
    pub fn cairo_win32_surface_get_image(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
}

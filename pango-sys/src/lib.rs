// Copyright 2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(non_camel_case_types)]
#![allow(dead_code)]

extern crate libc;
extern crate glib_sys as glib_ffi;

pub mod enums;

use libc::{c_int, c_uint, c_char, c_double};
use glib_ffi::gboolean;

#[repr(C)]
pub struct PangoContext;
#[repr(C)]
pub struct PangoAttrList;
#[repr(C)]
pub struct PangoAttrIterator;
#[repr(C)]
pub struct PangoItem;
#[repr(C)]
pub struct PangoFontMap;
#[repr(C)]
pub struct PangoFontDescription;
#[repr(C)]
pub struct PangoLanguage;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PangoMatrix {
    pub xx: c_double,
    pub xy: c_double,
    pub yx: c_double,
    pub yy: c_double,
    pub x0: c_double,
    pub y0: c_double
}
#[repr(C)]
pub struct PangoFont;
#[repr(C)]
pub struct PangoFontset;
#[repr(C)]
pub struct PangoFontMetrics;
#[repr(C)]
pub struct PangoFontFamily;
#[repr(C)]
pub struct PangoAnalysis;
#[repr(C)]
pub struct PangoLogAttr;
#[repr(C)]
pub struct PangoGlyphString;
#[repr(C)]
pub struct PangoScript;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PangoRectangle {
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int
}

extern "C" {
    //=========================================================================
    // PangoItem                                                         NOT OK
    //=========================================================================
    pub fn pango_item_free                (item: *mut PangoItem);
    pub fn pango_item_copy                (item: *mut PangoItem) -> *mut PangoItem;
    pub fn pango_item_new                 () -> *mut PangoItem;
    pub fn pango_item_split               (item: *mut PangoItem, split_index: c_int, split_offset: c_int) -> *mut PangoItem;

    //=========================================================================
    // PangoContext                                                      NOT OK
    //=========================================================================
    pub fn pango_context_new              () -> *mut PangoContext;
    pub fn pango_context_changed          (context: *mut PangoContext);
    pub fn pango_context_get_serial       (context: *mut PangoContext) -> c_uint;
    pub fn pango_context_set_font_map     (context: *mut PangoContext, font_map: *mut PangoFontMap);
    pub fn pango_context_get_font_map     (context: *mut PangoContext) -> *mut PangoFontMap;
    pub fn pango_context_get_font_description(context: *mut PangoContext) -> *mut PangoFontDescription;
    pub fn pango_context_set_font_description(context: *mut PangoContext, desc: *const PangoFontDescription);
    pub fn pango_context_get_language     (context: *mut PangoContext) -> *mut PangoLanguage;
    pub fn pango_context_set_language     (context: *mut PangoContext, language: *mut PangoLanguage);
    pub fn pango_context_get_base_dir     (context: *mut PangoContext) -> enums::Direction;
    pub fn pango_context_set_base_dir     (context: *mut PangoContext, direction: enums::Direction);
    pub fn pango_context_get_base_gravity (context: *mut PangoContext) -> enums::Gravity;
    pub fn pango_context_set_base_gravity (context: *mut PangoContext, gravity: enums::Gravity);
    pub fn pango_context_get_gravity      (context: *mut PangoContext) -> enums::Gravity;
    pub fn pango_context_get_gravity_hint (context: *mut PangoContext) -> enums::GravityHint;
    pub fn pango_context_set_gravity_hint (context: *mut PangoContext, hint: enums::GravityHint);
    pub fn pango_context_get_matrix       (context: *mut PangoContext) -> *const PangoMatrix;
    pub fn pango_context_set_matrix       (context: *mut PangoContext, matrix: *const PangoMatrix);
    pub fn pango_context_load_font        (context: *mut PangoContext, desc: *const PangoFontDescription) -> *mut PangoFont;
    pub fn pango_context_load_fontset     (context: *mut PangoContext, desc: *const PangoFontDescription, language: *mut PangoLanguage) -> *mut PangoFontset;
    pub fn pango_context_get_metrics      (context: *mut PangoContext, desc: *const PangoFontDescription, language: *mut PangoLanguage) -> *mut PangoFontMetrics;
    pub fn pango_context_list_families    (context: *mut PangoContext, families: *mut *mut *mut PangoFontFamily, n_families: *mut c_int);

    //=========================================================================
    // PangoFontDescription                                              NOT OK
    //=========================================================================
    pub fn pango_font_description_new     () -> *mut PangoFontDescription;
    pub fn pango_font_description_copy    (desc: *const PangoFontDescription) -> *mut PangoFontDescription;
    //pub fn pango_font_description_copy_static(desc: *const PangoFontDescription) -> *mut PangoFontDescription;
    pub fn pango_font_description_hash    (desc: *const PangoFontDescription) -> c_uint;
    pub fn pango_font_description_equal   (desc1: *const PangoFontDescription, desc2: *const PangoFontDescription) -> gboolean;
    pub fn pango_font_description_free    (desc: *mut PangoFontDescription);
    pub fn pango_font_descriptions_free   (desc: *mut *mut PangoFontDescription, n_descs: c_int);
    pub fn pango_font_description_set_family(desc: *mut PangoFontDescription, family: *const c_char);
    //pub fn pango_font_description_set_family_static(desc: *mut PangoFontDescription, family: *const c_char);
    pub fn pango_font_description_get_family(desc: *const PangoFontDescription) -> *const c_char;
    pub fn pango_font_description_set_style(desc: *mut PangoFontDescription, style: enums::Style);
    pub fn pango_font_description_get_style(desc: *const PangoFontDescription) -> enums::Style;
    pub fn pango_font_description_set_variant(desc: *mut PangoFontDescription, variant: enums::Variant);
    pub fn pango_font_description_get_variant(desc: *const PangoFontDescription) -> enums::Variant;
    pub fn pango_font_description_set_weight(desc: *mut PangoFontDescription, weight: enums::Weight);
    pub fn pango_font_description_get_weight(desc: *const PangoFontDescription) -> enums::Weight;
    pub fn pango_font_description_set_stretch(desc: *mut PangoFontDescription, stretch: enums::Stretch);
    pub fn pango_font_description_get_stretch(desc: *const PangoFontDescription) -> enums::Stretch;
    pub fn pango_font_description_set_size(desc: *mut PangoFontDescription, size: c_int);
    pub fn pango_font_description_get_size(desc: *const PangoFontDescription) -> c_int;
    pub fn pango_font_description_set_absolute_size(desc: *mut PangoFontDescription, size: c_double);
    pub fn pango_font_description_get_size_is_absolute(desc: *const PangoFontDescription) -> gboolean;
    pub fn pango_font_description_set_gravity(desc: *mut PangoFontDescription, gravity: enums::Gravity);
    pub fn pango_font_description_get_gravity(desc: *const PangoFontDescription) -> enums::Gravity;
    pub fn pango_font_description_get_set_fields(desc: *const PangoFontDescription) -> enums::FontMask;
    pub fn pango_font_description_unset_fields(desc: *mut PangoFontDescription, to_unset: enums::FontMask);
    pub fn pango_font_description_merge   (desc: *mut PangoFontDescription, desc_to_merge: *const PangoFontDescription,
        replace_existing: gboolean);
    //pub fn pango_font_description_merge_static(desc: *mut PangoFontDescription, desc_to_merge: *const PangoFontDescription,
    //    replace_existing: gboolean);
    pub fn pango_font_description_better_match(desc: *const PangoFontDescription, old_match: *const PangoFontDescription,
        new_match: *const PangoFontDescription) -> gboolean;
    pub fn pango_font_description_from_string(str_: *const c_char) -> *mut PangoFontDescription;
    pub fn pango_font_description_to_string(desc: *const PangoFontDescription) -> *mut c_char;
    pub fn pango_font_description_to_filename(desc: *const PangoFontDescription) -> *mut c_char;

    //=========================================================================
    // PangoMatrix                                                       NOT OK
    //=========================================================================
    pub fn pango_gravity_get_for_matrix    (matrix: *const PangoMatrix) -> enums::Gravity;
    //pub fn pango_matrix_copy               (matrix: *const PangoMatrix) -> *mut PangoMatrix;
    //pub fn pango_matrix_free               (matrix: *mut PangoMatrix);
    pub fn pango_matrix_translate          (matrix: *mut PangoMatrix, t_x: c_double, t_y: c_double);
    pub fn pango_matrix_scale              (matrix: *mut PangoMatrix, scale_x: c_double, scale_y: c_double);
    pub fn pango_matrix_rotate             (matrix: *mut PangoMatrix, degrees: c_double);
    pub fn pango_matrix_concat             (matrix: *mut PangoMatrix, new_matrix: *const PangoMatrix);
    pub fn pango_matrix_transform_point    (matrix: *const PangoMatrix, x: *mut c_double, y: *mut c_double);
    pub fn pango_matrix_transform_distance (matrix: *const PangoMatrix, dx: *mut c_double, dy: *mut c_double);
    pub fn pango_matrix_transform_rectangle(matrix: *const PangoMatrix, rect: *mut PangoRectangle);
    pub fn pango_matrix_transform_pixel_rectangle(matrix: *const PangoMatrix, rect: *mut PangoRectangle);
    pub fn pango_matrix_get_font_scale_factor(matrix: *mut PangoMatrix) -> c_double;

    //=========================================================================
    // PangoGlyphString                                                  NOT OK
    //=========================================================================
    pub fn pango_glyph_string_new          () -> *mut PangoGlyphString;
    pub fn pango_glyph_string_copy         (string: *mut PangoGlyphString) -> *mut PangoGlyphString;
    pub fn pango_glyph_string_set_size     (string: *mut PangoGlyphString, new_len: c_int);
    pub fn pango_glyph_string_free         (string: *mut PangoGlyphString);
    pub fn pango_glyph_string_extents      (string: *mut PangoGlyphString, font: *mut PangoFont, ink_rect: *mut PangoRectangle,
        logical_rect: *mut PangoRectangle);
    pub fn pango_glyph_string_extents_range(string: *mut PangoGlyphString, start: c_int, end: c_int, font: *mut PangoFont,
        ink_rect: *mut PangoRectangle, logical_rect: *mut PangoRectangle);
    pub fn pango_glyph_string_get_width    (string: *mut PangoGlyphString) -> c_int;
    pub fn pango_glyph_string_index_to_x   (string: *mut PangoGlyphString, text: *mut c_char, length: c_int, analysis: *mut PangoAnalysis,
        index_: c_int, trailing: gboolean, x_pos: *mut c_int);
    pub fn pango_glyph_string_x_to_index   (string: *mut PangoGlyphString, text: *mut c_char, length: c_int, analysis: *mut PangoAnalysis,
        x_pos: c_int, index_: *mut c_int, trailing: *mut c_int);
    pub fn pango_glyph_string_get_logical_widths(glyphs: *mut PangoGlyphString, text: *mut c_char, length: c_int, embedding_level: c_int,
        logical_widths: *mut c_int);

    //=========================================================================
    // PangoScript                                                       NOT OK
    //=========================================================================
    pub fn pango_gravity_get_for_script    (script: enums::Script, base_gravity: enums::Gravity, hint: enums::GravityHint) -> enums::Gravity;
    pub fn pango_gravity_get_for_script_and_width(script: enums::Script, wide: gboolean, base_gravity: enums::Gravity,
        hint: enums::GravityHint) -> enums::Gravity;

    //=========================================================================
    // PangoGravity                                                      NOT OK
    //=========================================================================
    pub fn pango_gravity_to_rotation        (gravity: enums::Gravity) -> c_double;

    //=========================================================================
    // PangoDirection                                                    NOT OK
    //=========================================================================
    pub fn pango_unichar_direction          (ch: u32) -> enums::Direction;
    pub fn pango_find_base_dir              (text: *const c_char, length: c_int) -> enums::Direction;

    //=========================================================================
    // PangoBidiType                                                     NOT OK
    //=========================================================================
    pub fn pango_bidi_type_for_unichar      (ch: u32) -> enums::BidiType;

    //pub fn pango_itemize                  (context: *mut PangoContext, text: *const c_char, start_index: c_int, length: c_int,
    //    attrs: *mut PangoAttrList, cached_iter: *mut PangoAttrIterator) -> *mut GList;
    //pub fn pango_itemize_with_base_dir    (context: *mut PangoContext, direction: enums::Direction, text: *const c_char, start_index: c_int, length: c_int,
    //    attrs: *mut PangoAttrList, cached_iter: *mut PangoAttrIterator) -> *mut GList;
    //pub fn pango_reorder_items            (logical_items: *mut GList) -> *mut GList;
    pub fn pango_break                    (text: *const c_char, length: c_int, analysis: *mut PangoAnalysis, attrs: *mut PangoLogAttr,
        attrs_len: c_int);
    pub fn pango_get_log_attrs            (text: *const c_char, length: c_int, level: c_int, language: *mut PangoLanguage, log_attrs: *mut PangoLogAttr,
        attrs_len: c_int);
    pub fn pango_find_paragraph_boundary  (text: *const c_char, length: c_int, paragraph_delimiter_index: *mut c_int,
        next_paragraph_start: *mut c_int);
    pub fn pango_default_break            (text: *const c_char, length: c_int, analysis: *mut PangoAnalysis, attrs: *mut PangoLogAttr,
        attrs_len: c_int);
    pub fn pango_shape                    (text: *const c_char, length: c_int, analysis: *const PangoAnalysis, glyphs: *mut PangoGlyphString);
    pub fn pango_shape_full               (item_text: *const c_char, item_length: c_int, paragraph_text: *const c_char, paragraph_length: c_int,
        analysis: *const PangoAnalysis, glyphs: *mut PangoGlyphString);
}
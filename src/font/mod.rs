// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

mod font_face;
mod font_options;
mod scaled_font;

pub use enums::{
    Antialias, FontSlant, FontType, FontWeight, HintMetrics, HintStyle, SubpixelOrder,
    TextClusterFlags,
};

pub use ffi::{FontExtents, Glyph, TextCluster, TextExtents};

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

pub use self::font_face::FontFace;
pub use self::font_options::FontOptions;
pub use self::scaled_font::ScaledFont;

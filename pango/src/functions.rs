// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Analysis;
use crate::GlyphString;
use crate::Item;
#[cfg(any(feature = "v1_44", feature = "dox"))]
use crate::ShapeFlags;
use glib::translate::*;
use std::ptr;

pub fn reorder_items(logical_items: &[&Item]) -> Vec<Item> {
    unsafe {
        let stash_vec: Vec<_> = logical_items
            .iter()
            .rev()
            .map(|v| v.to_glib_none())
            .collect();
        let mut list: *mut glib::ffi::GList = ptr::null_mut();
        for stash in &stash_vec {
            list = glib::ffi::g_list_prepend(list, Ptr::to(stash.0));
        }

        FromGlibPtrContainer::from_glib_full(ffi::pango_reorder_items(list))
    }
}

pub fn shape_full(
    item_text: &str,
    paragraph_text: Option<&str>,
    analysis: &Analysis,
    glyphs: &mut GlyphString,
) {
    let paragraph_length = match paragraph_text {
        Some(s) => s.len(),
        None => 0,
    } as i32;
    let paragraph_text = paragraph_text.to_glib_none();
    let item_length = item_text.len() as i32;
    unsafe {
        ffi::pango_shape_full(
            item_text.to_glib_none().0,
            item_length,
            paragraph_text.0,
            paragraph_length,
            analysis.to_glib_none().0,
            glyphs.to_glib_none_mut().0,
        );
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
pub fn shape_with_flags(
    item_text: &str,
    paragraph_text: Option<&str>,
    analysis: &Analysis,
    glyphs: &mut GlyphString,
    flags: ShapeFlags,
) {
    let item_length = item_text.len() as i32;
    let paragraph_length = paragraph_text.map(|t| t.len() as i32).unwrap_or_default();
    unsafe {
        ffi::pango_shape_with_flags(
            item_text.to_glib_none().0,
            item_length,
            paragraph_text.to_glib_none().0,
            paragraph_length,
            analysis.to_glib_none().0,
            glyphs.to_glib_none_mut().0,
            flags.to_glib(),
        );
    }
}

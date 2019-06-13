// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib_sys;
use pango_sys;
use std::ptr;
use Analysis;
use GlyphString;
use Item;

pub fn reorder_items(logical_items: &[&Item]) -> Vec<Item> {
    unsafe {
        let stash_vec: Vec<_> = logical_items
            .iter()
            .rev()
            .map(|v| v.to_glib_none())
            .collect();
        let mut list: *mut glib_sys::GList = ptr::null_mut();
        for stash in &stash_vec {
            list = glib_sys::g_list_prepend(list, Ptr::to(stash.0));
        }

        FromGlibPtrContainer::from_glib_full(pango_sys::pango_reorder_items(list))
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
        pango_sys::pango_shape_full(
            item_text.to_glib_none().0,
            item_length,
            paragraph_text.0,
            paragraph_length,
            analysis.to_glib_none().0,
            glyphs.to_glib_none_mut().0,
        );
    }
}

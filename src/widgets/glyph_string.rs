// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use libc::c_int;

pub struct GlyphString {
    pointer: *mut ffi::PangoGlyphString
}

impl GlyphString {
    pub fn new() -> Option<GlyphString> {
        let tmp = unsafe { ffi::pango_glyph_string_new() };

        if tmp.is_null() {
            None
        } else {
            Some(GlyphString {
                pointer: tmp
            })
        }
    }

    pub fn copy(&self) -> Option<GlyphString> {
        let tmp = unsafe { ffi::pango_glyph_string_copy(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(GlyphString {
                pointer: tmp
            })
        }
    }

    pub fn set_size(&self, new_len: i32) {
        unsafe { ffi::pango_glyph_string_set_size(self.pointer, new_len as c_int) }
    }
}

impl Drop for GlyphString {
    fn drop(&mut self) {
        if self.pointer.is_null() {
            return;
        }
        unsafe { ffi::pango_glyph_string_free(self.pointer); }
        self.pointer = ::std::ptr::null_mut();
    }
}
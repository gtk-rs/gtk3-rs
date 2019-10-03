use glib::translate::*;
use GlyphString;
use Item;
use GlyphString;

impl GlyphString {
    pub fn num_glyphs(&self) -> Option<usize> {
        unsafe { (*(*self..to_glib_none().0).num_glyphs) }
    }
}

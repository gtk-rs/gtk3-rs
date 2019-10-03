use glib::translate::*;
use GlyphItem;
use Item;
use GlyphString;

impl GlyphItem {
    pub fn item(&self) -> Option<Item> {
        unsafe { from_glib_none((*self.to_glib_none().0).item) }
    }

    pub fn glyph_string(&self) -> Option<GlyphString> {
        unsafe { from_glib_none((*self.to_glib_none().0).glyphs) }
    }
}

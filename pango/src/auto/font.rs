// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Coverage;
use crate::EngineShape;
use crate::FontDescription;
#[cfg(any(feature = "v1_46", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_46")))]
use crate::FontFace;
use crate::FontMap;
use crate::FontMetrics;
use crate::Glyph;
use crate::Language;
use crate::Rectangle;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct Font(Object<ffi::PangoFont, ffi::PangoFontClass>);

    match fn {
        get_type => || ffi::pango_font_get_type(),
    }
}

pub const NONE_FONT: Option<&Font> = None;

pub trait FontExt: 'static {
    fn describe(&self) -> Option<FontDescription>;

    fn describe_with_absolute_size(&self) -> Option<FontDescription>;

    fn find_shaper(&self, language: &Language, ch: u32) -> Option<EngineShape>;

    fn get_coverage(&self, language: &Language) -> Option<Coverage>;

    #[cfg(any(feature = "v1_46", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_46")))]
    fn get_face(&self) -> Option<FontFace>;

    //#[cfg(any(feature = "v1_44", all(not(doctest), doc)))]
    //#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_44")))]
    //fn get_features(&self, features: /*Unimplemented*/&mut Fundamental: Pointer, num_features: &mut u32) -> u32;

    fn get_font_map(&self) -> Option<FontMap>;

    fn get_glyph_extents(&self, glyph: Glyph) -> (Rectangle, Rectangle);

    //#[cfg(any(feature = "v1_44", all(not(doctest), doc)))]
    //#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_44")))]
    //fn get_hb_font(&self) -> /*Ignored*/Option<harf_buzz::font_t>;

    fn get_metrics(&self, language: Option<&Language>) -> Option<FontMetrics>;

    #[cfg(any(feature = "v1_44", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_44")))]
    fn has_char(&self, wc: char) -> bool;
}

impl<O: IsA<Font>> FontExt for O {
    fn describe(&self) -> Option<FontDescription> {
        unsafe { from_glib_full(ffi::pango_font_describe(self.as_ref().to_glib_none().0)) }
    }

    fn describe_with_absolute_size(&self) -> Option<FontDescription> {
        unsafe {
            from_glib_full(ffi::pango_font_describe_with_absolute_size(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn find_shaper(&self, language: &Language, ch: u32) -> Option<EngineShape> {
        unsafe {
            from_glib_none(ffi::pango_font_find_shaper(
                self.as_ref().to_glib_none().0,
                mut_override(language.to_glib_none().0),
                ch,
            ))
        }
    }

    fn get_coverage(&self, language: &Language) -> Option<Coverage> {
        unsafe {
            from_glib_full(ffi::pango_font_get_coverage(
                self.as_ref().to_glib_none().0,
                mut_override(language.to_glib_none().0),
            ))
        }
    }

    #[cfg(any(feature = "v1_46", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_46")))]
    fn get_face(&self) -> Option<FontFace> {
        unsafe { from_glib_none(ffi::pango_font_get_face(self.as_ref().to_glib_none().0)) }
    }

    //#[cfg(any(feature = "v1_44", all(not(doctest), doc)))]
    //#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_44")))]
    //fn get_features(&self, features: /*Unimplemented*/&mut Fundamental: Pointer, num_features: &mut u32) -> u32 {
    //    unsafe { TODO: call ffi:pango_font_get_features() }
    //}

    fn get_font_map(&self) -> Option<FontMap> {
        unsafe { from_glib_none(ffi::pango_font_get_font_map(self.as_ref().to_glib_none().0)) }
    }

    fn get_glyph_extents(&self, glyph: Glyph) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_font_get_glyph_extents(
                self.as_ref().to_glib_none().0,
                glyph,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    //#[cfg(any(feature = "v1_44", all(not(doctest), doc)))]
    //#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_44")))]
    //fn get_hb_font(&self) -> /*Ignored*/Option<harf_buzz::font_t> {
    //    unsafe { TODO: call ffi:pango_font_get_hb_font() }
    //}

    fn get_metrics(&self, language: Option<&Language>) -> Option<FontMetrics> {
        unsafe {
            from_glib_full(ffi::pango_font_get_metrics(
                self.as_ref().to_glib_none().0,
                mut_override(language.to_glib_none().0),
            ))
        }
    }

    #[cfg(any(feature = "v1_44", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_44")))]
    fn has_char(&self, wc: char) -> bool {
        unsafe {
            from_glib(ffi::pango_font_has_char(
                self.as_ref().to_glib_none().0,
                wc.to_glib(),
            ))
        }
    }
}

impl fmt::Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Font")
    }
}

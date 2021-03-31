// Take a look at the license at the top of the repository in the LICENSE file.

use crate::AttrClass;
use crate::AttrType;
use crate::Attribute;
use crate::Color;
use crate::FontDescription;
use crate::Gravity;
use crate::GravityHint;
use crate::Language;
#[cfg(any(feature = "v1_46", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
use crate::Overline;
#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
use crate::ShowFlags;
use crate::Stretch;
use crate::Style;
use crate::Underline;
use crate::Variant;
use crate::Weight;
use glib::translate::*;

impl Attribute {
    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_attr_allow_breaks_new")]
    pub fn new_allow_breaks(allow_breaks: bool) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_allow_breaks_new(allow_breaks.to_glib())) }
    }

    #[cfg(any(feature = "v1_38", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_38")))]
    #[doc(alias = "pango_attr_background_alpha_new")]
    pub fn new_background_alpha(alpha: u16) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_background_alpha_new(alpha)) }
    }

    #[doc(alias = "pango_attr_background_new")]
    pub fn new_background(red: u16, green: u16, blue: u16) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_background_new(red, green, blue)) }
    }

    #[doc(alias = "pango_attr_fallback_new")]
    pub fn new_fallback(enable_fallback: bool) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_fallback_new(enable_fallback.to_glib())) }
    }

    #[doc(alias = "pango_attr_family_new")]
    pub fn new_family(family: &str) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_family_new(family.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_38", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_38")))]
    #[doc(alias = "pango_attr_font_features_new")]
    pub fn new_font_features(features: &str) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_font_features_new(features.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_38", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_38")))]
    #[doc(alias = "pango_attr_foreground_alpha_new")]
    pub fn new_foreground_alpha(alpha: u16) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_foreground_alpha_new(alpha)) }
    }

    #[doc(alias = "pango_attr_foreground_new")]
    pub fn new_foreground(red: u16, green: u16, blue: u16) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_foreground_new(red, green, blue)) }
    }

    #[doc(alias = "pango_attr_gravity_hint_new")]
    pub fn new_gravity_hint(hint: GravityHint) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_gravity_hint_new(hint.to_glib())) }
    }

    #[doc(alias = "pango_attr_gravity_new")]
    pub fn new_gravity(gravity: Gravity) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_gravity_new(gravity.to_glib())) }
    }

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_attr_insert_hyphens_new")]
    pub fn new_insert_hyphens(insert_hyphens: bool) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_insert_hyphens_new(insert_hyphens.to_glib())) }
    }

    #[doc(alias = "pango_attr_letter_spacing_new")]
    pub fn new_letter_spacing(letter_spacing: i32) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_letter_spacing_new(letter_spacing)) }
    }

    #[cfg(any(feature = "v1_46", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
    #[doc(alias = "pango_attr_overline_color_new")]
    pub fn new_overline_color(red: u16, green: u16, blue: u16) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_overline_color_new(red, green, blue)) }
    }

    #[cfg(any(feature = "v1_46", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
    #[doc(alias = "pango_attr_overline_new")]
    pub fn new_overline(overline: Overline) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_overline_new(overline.to_glib())) }
    }

    #[doc(alias = "pango_attr_rise_new")]
    pub fn new_rise(rise: i32) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_rise_new(rise)) }
    }

    #[doc(alias = "pango_attr_scale_new")]
    pub fn new_scale(scale_factor: f64) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_scale_new(scale_factor)) }
    }

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_attr_show_new")]
    pub fn new_show(flags: ShowFlags) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_show_new(flags.to_glib())) }
    }

    #[doc(alias = "pango_attr_size_new")]
    pub fn new_size(size: i32) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_size_new(size)) }
    }

    #[doc(alias = "pango_attr_size_new_absolute")]
    pub fn new_size_absolute(size: i32) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_size_new_absolute(size)) }
    }

    #[doc(alias = "pango_attr_stretch_new")]
    pub fn new_stretch(stretch: Stretch) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_stretch_new(stretch.to_glib())) }
    }

    #[doc(alias = "pango_attr_strikethrough_color_new")]
    pub fn new_strikethrough_color(red: u16, green: u16, blue: u16) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_strikethrough_color_new(red, green, blue)) }
    }

    #[doc(alias = "pango_attr_strikethrough_new")]
    pub fn new_strikethrough(strikethrough: bool) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_strikethrough_new(strikethrough.to_glib())) }
    }

    #[doc(alias = "pango_attr_style_new")]
    pub fn new_style(style: Style) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_style_new(style.to_glib())) }
    }

    #[doc(alias = "pango_attr_underline_color_new")]
    pub fn new_underline_color(red: u16, green: u16, blue: u16) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_underline_color_new(red, green, blue)) }
    }

    #[doc(alias = "pango_attr_underline_new")]
    pub fn new_underline(underline: Underline) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_underline_new(underline.to_glib())) }
    }

    #[doc(alias = "pango_attr_variant_new")]
    pub fn new_variant(variant: Variant) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_variant_new(variant.to_glib())) }
    }

    #[doc(alias = "pango_attr_weight_new")]
    pub fn new_weight(weight: Weight) -> Attribute {
        unsafe { from_glib_full(ffi::pango_attr_weight_new(weight.to_glib())) }
    }

    pub fn get_attr_class(&self) -> AttrClass {
        unsafe { from_glib_full((*self.to_glib_none().0).klass) }
    }

    pub fn get_start_index(&self) -> u32 {
        unsafe {
            let stash = self.to_glib_none();
            (*stash.0).start_index
        }
    }

    pub fn get_end_index(&self) -> u32 {
        unsafe {
            let stash = self.to_glib_none();
            (*stash.0).end_index
        }
    }

    pub fn set_start_index(&mut self, index: u32) {
        unsafe {
            let stash = self.to_glib_none_mut();
            (*stash.0).start_index = index;
        }
    }

    pub fn set_end_index(&mut self, index: u32) {
        unsafe {
            let stash = self.to_glib_none_mut();
            (*stash.0).end_index = index;
        }
    }

    pub fn downcast<T: IsAttribute>(self) -> Result<T, Attribute> {
        unsafe {
            if T::ATTR_TYPES.contains(&self.get_attr_class().type_()) {
                Ok(from_glib_full(self.to_glib_full()))
            } else {
                Err(self)
            }
        }
    }

    pub fn downcast_ref<T: IsAttribute>(&self) -> Option<&T> {
        unsafe {
            if T::ATTR_TYPES.contains(&self.get_attr_class().type_()) {
                Some(&*(self as *const Attribute as *const T))
            } else {
                None
            }
        }
    }
}

pub trait IsAttribute:
    FromGlibPtrFull<*const ffi::PangoAttribute>
    + FromGlibPtrFull<*mut ffi::PangoAttribute>
    + std::convert::AsRef<crate::Attribute>
    + 'static
{
    const ATTR_TYPES: &'static [AttrType];
    fn upcast(self) -> Attribute;
    fn upcast_ref(&self) -> &Attribute;
}

macro_rules! define_attribute_struct {
    ($rust_type:ident, $ffi_type:path, $attr_types:expr) => {

        #[cfg(any(feature = "v1_44", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
        glib::wrapper! {
            #[derive(Debug, PartialOrd, Ord, Hash)]
            pub struct $rust_type(Boxed<$ffi_type>);

            match fn {
                copy => |ptr| ffi::pango_attribute_copy(ptr as *const ffi::PangoAttribute) as *mut $ffi_type,
                free => |ptr| ffi::pango_attribute_destroy(ptr as *mut ffi::PangoAttribute),
                get_type => || ffi::pango_attribute_get_type(),
            }
        }

        #[cfg(not(any(feature = "v1_44", feature = "dox")))]
        glib::wrapper! {
            #[derive(Debug, PartialOrd, Ord, Hash)]
            pub struct $rust_type(Boxed<$ffi_type>);

            match fn {
                copy => |ptr| ffi::pango_attribute_copy(ptr as *const ffi::PangoAttribute) as *mut $ffi_type,
                free => |ptr| ffi::pango_attribute_destroy(ptr as *mut ffi::PangoAttribute),
            }
        }

        impl $rust_type {
            #[doc(alias = "pango_attribute_equal")]
            fn equal(&self, attr2: &$rust_type) -> bool {
                unsafe {
                    from_glib(ffi::pango_attribute_equal(
                        self.to_glib_none().0 as *const ffi::PangoAttribute,
                        attr2.to_glib_none().0 as *const ffi::PangoAttribute,
                    ))
                }
            }
        }

        impl PartialEq for $rust_type {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.equal(other)
            }
        }

        impl Eq for $rust_type {}

        impl IsAttribute for $rust_type {
            const ATTR_TYPES: &'static [AttrType] = $attr_types;

            fn upcast(self) -> crate::Attribute {
                unsafe { from_glib_full(self.to_glib_full() as *mut ffi::PangoAttribute) }
            }

            fn upcast_ref(&self) -> &crate::Attribute {
                &*self
            }
        }

        #[doc(hidden)]
        impl FromGlibPtrFull<*mut ffi::PangoAttribute> for $rust_type {
            unsafe fn from_glib_full(ptr: *mut ffi::PangoAttribute) -> Self {
                from_glib_full(ptr as *mut $ffi_type)
            }
        }

        #[doc(hidden)]
        impl FromGlibPtrFull<*const ffi::PangoAttribute> for $rust_type {
            unsafe fn from_glib_full(ptr: *const ffi::PangoAttribute) -> Self {
                from_glib_full(ptr as *const $ffi_type)
            }
        }

        impl std::convert::AsRef<crate::Attribute> for $rust_type {
            fn as_ref(&self) -> &crate::Attribute {
                &*self
            }
        }

        impl std::ops::Deref for $rust_type {
            type Target = crate::Attribute;

            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const $rust_type as *const crate::Attribute) }
            }
        }
    }
}

define_attribute_struct!(
    AttrColor,
    ffi::PangoAttrColor,
    &[
        AttrType::Foreground,
        AttrType::Background,
        AttrType::UnderlineColor,
        AttrType::StrikethroughColor,
        AttrType::OverlineColor
    ]
);

impl AttrColor {
    pub fn color(&self) -> Color {
        unsafe { from_glib_none((&self.0.color) as *const ffi::PangoColor) }
    }
}

define_attribute_struct!(
    AttrInt,
    ffi::PangoAttrInt,
    &[
        AttrType::AbsoluteSize,
        AttrType::AllowBreaks,
        AttrType::BackgroundAlpha,
        AttrType::Fallback,
        AttrType::FontFeatures,
        AttrType::ForegroundAlpha,
        AttrType::Gravity,
        AttrType::GravityHint,
        AttrType::InsertHyphens,
        AttrType::LetterSpacing,
        AttrType::Overline,
        AttrType::Rise,
        AttrType::Show,
        AttrType::Size,
        AttrType::Stretch,
        AttrType::Strikethrough,
        AttrType::Style,
        AttrType::Underline,
        AttrType::Variant,
        AttrType::Weight
    ]
);

impl AttrInt {
    pub fn value(&self) -> i32 {
        self.0.value
    }
}

define_attribute_struct!(AttrFloat, ffi::PangoAttrFloat, &[AttrType::Scale]);

impl AttrFloat {
    pub fn value(&self) -> f64 {
        self.0.value
    }
}

define_attribute_struct!(AttrFontDesc, ffi::PangoAttrFontDesc, &[AttrType::FontDesc]);

impl AttrFontDesc {
    pub fn value(&self) -> FontDescription {
        unsafe { from_glib_none(self.0.desc) }
    }
}

define_attribute_struct!(AttrLanguage, ffi::PangoAttrLanguage, &[AttrType::Language]);

impl AttrLanguage {
    pub fn value(&self) -> Language {
        unsafe { from_glib_none(self.0.value) }
    }
}

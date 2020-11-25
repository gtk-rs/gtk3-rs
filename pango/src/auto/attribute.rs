// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AttrClass;
use glib::translate::*;

#[cfg(any(feature = "v1_44", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_44")))]
glib::glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Attribute(Boxed<ffi::PangoAttribute>);

    match fn {
        copy => |ptr| ffi::pango_attribute_copy(mut_override(ptr)),
        free => |ptr| ffi::pango_attribute_destroy(ptr),
        get_type => || ffi::pango_attribute_get_type(),
    }
}

#[cfg(not(any(feature = "v1_44", all(not(doctest), doc))))]
glib::glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Attribute(Boxed<ffi::PangoAttribute>);

    match fn {
        copy => |ptr| ffi::pango_attribute_copy(mut_override(ptr)),
        free => |ptr| ffi::pango_attribute_destroy(ptr),
    }
}

impl Attribute {
    fn equal(&self, attr2: &Attribute) -> bool {
        unsafe {
            from_glib(ffi::pango_attribute_equal(
                self.to_glib_none().0,
                attr2.to_glib_none().0,
            ))
        }
    }

    pub fn init(&mut self, klass: &AttrClass) {
        unsafe {
            ffi::pango_attribute_init(self.to_glib_none_mut().0, klass.to_glib_none().0);
        }
    }
}

impl PartialEq for Attribute {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Attribute {}

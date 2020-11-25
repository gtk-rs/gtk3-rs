// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AttrType;
use crate::Attribute;
use glib::translate::*;
use std::mem;

#[cfg(any(feature = "v1_44", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_44")))]
glib::glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AttrIterator(Boxed<ffi::PangoAttrIterator>);

    match fn {
        copy => |ptr| ffi::pango_attr_iterator_copy(mut_override(ptr)),
        free => |ptr| ffi::pango_attr_iterator_destroy(ptr),
        get_type => || ffi::pango_attr_iterator_get_type(),
    }
}

#[cfg(not(any(feature = "v1_44", all(not(doctest), doc))))]
glib::glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AttrIterator(Boxed<ffi::PangoAttrIterator>);

    match fn {
        copy => |ptr| ffi::pango_attr_iterator_copy(mut_override(ptr)),
        free => |ptr| ffi::pango_attr_iterator_destroy(ptr),
    }
}

impl AttrIterator {
    pub fn get(&mut self, type_: AttrType) -> Option<Attribute> {
        unsafe {
            from_glib_none(ffi::pango_attr_iterator_get(
                self.to_glib_none_mut().0,
                type_.to_glib(),
            ))
        }
    }

    pub fn get_attrs(&mut self) -> Vec<Attribute> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::pango_attr_iterator_get_attrs(
                self.to_glib_none_mut().0,
            ))
        }
    }

    pub fn next(&mut self) -> bool {
        unsafe { from_glib(ffi::pango_attr_iterator_next(self.to_glib_none_mut().0)) }
    }

    pub fn range(&mut self) -> (i32, i32) {
        unsafe {
            let mut start = mem::MaybeUninit::uninit();
            let mut end = mem::MaybeUninit::uninit();
            ffi::pango_attr_iterator_range(
                self.to_glib_none_mut().0,
                start.as_mut_ptr(),
                end.as_mut_ptr(),
            );
            let start = start.assume_init();
            let end = end.assume_init();
            (start, end)
        }
    }
}

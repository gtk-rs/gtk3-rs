// Take a look at the license at the top of the repository in the LICENSE file.

use crate::AttrIterator;
use crate::Attribute;
use crate::FontDescription;
use crate::Language;
use glib::translate::*;

use std::ptr;

impl AttrIterator {
    #[doc(alias = "pango_attr_iterator_get_font")]
    pub fn get_font(
        &mut self,
        desc: &mut FontDescription,
        language: Option<&Language>,
        extra_attrs: &[&Attribute],
    ) {
        unsafe {
            let stash_vec: Vec<_> = extra_attrs.iter().rev().map(|v| v.to_glib_none()).collect();
            let mut list: *mut glib::ffi::GSList = ptr::null_mut();
            for stash in &stash_vec {
                list = glib::ffi::g_slist_prepend(list, Ptr::to(stash.0));
            }

            ffi::pango_attr_iterator_get_font(
                self.to_glib_none_mut().0,
                desc.to_glib_none_mut().0,
                &mut language.to_glib_none().0,
                &mut list,
            );
        }
    }
}

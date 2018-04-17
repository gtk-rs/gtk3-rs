// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use AttrIterator;
use Attribute;
use FontDescription;
use Language;

use ffi;
use glib_ffi;
use glib::translate::*;

use std::ptr;

impl AttrIterator {
    pub fn get_font<'a, P: Into<Option<&'a Language>>>(&mut self, desc: &mut FontDescription, language: P, extra_attrs: &[&Attribute]) {
        let language = language.into();
        unsafe {
            let stash_vec: Vec<_> = extra_attrs.iter().rev().map(|v| v.to_glib_none()).collect();
            let mut list: *mut glib_ffi::GSList = ptr::null_mut();
            for stash in &stash_vec {
                list = glib_ffi::g_slist_prepend(list, Ptr::to(stash.0));
            }

            ffi::pango_attr_iterator_get_font(self.to_glib_none_mut().0, desc.to_glib_none_mut().0, &mut language.to_glib_none().0, &mut list);
        }
    }
}

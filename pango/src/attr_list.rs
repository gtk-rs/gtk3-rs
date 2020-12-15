// Take a look at the license at the top of the repository in the LICENSE file.

use crate::AttrList;
use crate::Attribute;
use glib::translate::*;
use std::mem;

impl AttrList {
    pub fn change(&self, attr: Attribute) {
        unsafe {
            ffi::pango_attr_list_change(self.to_glib_none().0, attr.to_glib_none().0 as *mut _);
            mem::forget(attr); //As attr transferred fully
        }
    }

    #[cfg(any(feature = "v1_46", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
    fn equal(&self, other_list: &AttrList) -> bool {
        unsafe {
            from_glib(ffi::pango_attr_list_equal(
                self.to_glib_none().0,
                other_list.to_glib_none().0,
            ))
        }
    }

    pub fn insert(&self, attr: Attribute) {
        unsafe {
            ffi::pango_attr_list_insert(self.to_glib_none().0, attr.to_glib_none().0 as *mut _);
            mem::forget(attr); //As attr transferred fully
        }
    }

    pub fn insert_before(&self, attr: Attribute) {
        unsafe {
            ffi::pango_attr_list_insert_before(
                self.to_glib_none().0,
                attr.to_glib_none().0 as *mut _,
            );
            mem::forget(attr); //As attr transferred fully
        }
    }
}

#[cfg(any(feature = "v1_46", feature = "dox"))]
impl PartialEq for AttrList {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

#[cfg(any(feature = "v1_46", feature = "dox"))]
impl Eq for AttrList {}

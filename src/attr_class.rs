// Copyright 2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::{Stash, FromGlibPtrFull, FromGlibPtrNone, ToGlibPtr};

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut ffi::PangoAttrClass> for &'a AttrClass {
    type Storage = &'a AttrClass;

    fn to_glib_none(&self) -> Stash<'a, *mut ffi::PangoAttrClass, Self> {
        Stash(self.0, *self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::PangoAttrClass> for AttrClass {
    unsafe fn from_glib_none(ptr: *mut ffi::PangoAttrClass) -> Self {
        assert!(!ptr.is_null());
        AttrClass(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut ffi::PangoAttrClass> for AttrClass {
    unsafe fn from_glib_full(ptr: *mut ffi::PangoAttrClass) -> Self {
        assert!(!ptr.is_null());
        AttrClass(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::PangoAttrClass> for AttrClass {
    unsafe fn from_glib_none(ptr: *const ffi::PangoAttrClass) -> Self {
        assert!(!ptr.is_null());
        AttrClass(ptr as *mut _)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*const ffi::PangoAttrClass> for AttrClass {
    unsafe fn from_glib_full(ptr: *const ffi::PangoAttrClass) -> Self {
        assert!(!ptr.is_null());
        AttrClass(ptr as *mut _)
    }
}

pub struct AttrClass(*mut ffi::PangoAttrClass);

impl PartialEq for AttrClass {
    fn eq(&self, other: &AttrClass) -> bool {
        self.0 == other.0
    }
}

impl Eq for AttrClass {}

// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use glib::translate::*;
use pango_sys;
use std::fmt;
use std::mem;
use std::ptr;
use CoverageLevel;

#[cfg(any(feature = "v1_44", feature = "dox"))]
glib_wrapper! {
    pub struct Coverage(Object<pango_sys::PangoCoverage, CoverageClass>);

    match fn {
        get_type => || pango_sys::pango_coverage_get_type(),
    }
}

// There was no get_type() function before 1.44
#[cfg(not(any(feature = "v1_44", feature = "dox")))]
glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Coverage(Shared<pango_sys::PangoCoverage>);

    match fn {
        ref => |ptr| pango_sys::pango_coverage_ref(ptr),
        unref => |ptr| pango_sys::pango_coverage_unref(ptr),
    }
}

impl Coverage {
    pub fn new() -> Coverage {
        unsafe { from_glib_full(pango_sys::pango_coverage_new()) }
    }

    pub fn copy(&self) -> Option<Coverage> {
        unsafe { from_glib_full(pango_sys::pango_coverage_copy(self.to_glib_none().0)) }
    }

    pub fn get(&self, index_: i32) -> CoverageLevel {
        unsafe { from_glib(pango_sys::pango_coverage_get(self.to_glib_none().0, index_)) }
    }

    #[cfg_attr(feature = "v1_44", deprecated)]
    pub fn max(&self, other: &Coverage) {
        unsafe {
            pango_sys::pango_coverage_max(self.to_glib_none().0, other.to_glib_none().0);
        }
    }

    pub fn set(&self, index_: i32, level: CoverageLevel) {
        unsafe {
            pango_sys::pango_coverage_set(self.to_glib_none().0, index_, level.to_glib());
        }
    }

    #[cfg_attr(feature = "v1_44", deprecated)]
    pub fn to_bytes(&self) -> Vec<u8> {
        unsafe {
            let mut bytes = ptr::null_mut();
            let mut n_bytes = mem::MaybeUninit::uninit();
            pango_sys::pango_coverage_to_bytes(
                self.to_glib_none().0,
                &mut bytes,
                n_bytes.as_mut_ptr(),
            );
            FromGlibContainer::from_glib_full_num(bytes, n_bytes.assume_init() as usize)
        }
    }

    #[cfg_attr(feature = "v1_44", deprecated)]
    pub fn from_bytes(bytes: &[u8]) -> Option<Coverage> {
        let n_bytes = bytes.len() as i32;
        unsafe {
            from_glib_full(pango_sys::pango_coverage_from_bytes(
                bytes.to_glib_none().0,
                n_bytes,
            ))
        }
    }
}

impl Default for Coverage {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Coverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Coverage")
    }
}

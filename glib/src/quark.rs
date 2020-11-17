// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use crate::ffi;
use crate::translate::*;
use std::ffi::CStr;
use std::fmt;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[repr(transparent)]
pub struct Quark(ffi::GQuark);

impl Quark {
    pub fn from_string(s: &str) -> Quark {
        unsafe { from_glib(ffi::g_quark_from_string(s.to_glib_none().0)) }
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn to_string<'a>(&self) -> &'a str {
        unsafe {
            CStr::from_ptr(ffi::g_quark_to_string(self.to_glib()))
                .to_str()
                .unwrap()
        }
    }

    pub fn try_string(s: &str) -> Option<Quark> {
        unsafe {
            match ffi::g_quark_try_string(s.to_glib_none().0) {
                0 => None,
                x => Some(from_glib(x)),
            }
        }
    }
}

impl fmt::Debug for Quark {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(Quark::to_string(self))
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GQuark> for Quark {
    fn from_glib(value: ffi::GQuark) -> Self {
        Quark(value)
    }
}

#[doc(hidden)]
impl ToGlib for Quark {
    type GlibType = ffi::GQuark;

    fn to_glib(&self) -> ffi::GQuark {
        self.0
    }
}

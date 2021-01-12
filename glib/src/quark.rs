// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use std::ffi::CStr;
use std::fmt;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[repr(transparent)]
pub struct Quark(ffi::GQuark);

impl Quark {
    #[doc(alias = "g_quark_from_string")]
    pub fn from_string(s: &str) -> Quark {
        unsafe { from_glib(ffi::g_quark_from_string(s.to_glib_none().0)) }
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[doc(alias = "g_quark_to_string")]
    pub fn to_string<'a>(&self) -> &'a str {
        unsafe {
            CStr::from_ptr(ffi::g_quark_to_string(self.to_glib()))
                .to_str()
                .unwrap()
        }
    }

    #[doc(alias = "g_quark_try_string")]
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
    unsafe fn from_glib(value: ffi::GQuark) -> Self {
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

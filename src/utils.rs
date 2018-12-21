// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use std::ffi::CStr;
use std::fmt;

pub fn debug_reset_static_data() {
    unsafe { ffi::cairo_debug_reset_static_data() }
}

pub fn get_version_string() -> String {
    unsafe {
        let ptr = ffi::cairo_version_string();
        String::from_utf8_lossy(CStr::from_ptr(ptr).to_bytes()).into_owned()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub micro: u8,
}

impl Version {
    pub fn get_version() -> Version {
        let version = unsafe { ffi::cairo_version() };
        Version {
            major: (version / 10_000 % 100) as _,
            minor: (version / 100 % 100) as _,
            micro: (version % 100) as _,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.micro)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_versions() {
        assert_eq!(get_version_string(), Version::get_version().to_string());
    }
}

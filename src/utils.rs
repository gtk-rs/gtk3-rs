// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use translate::*;

/// Same as [`get_prgname()`].
///
/// [`get_prgname()`]: fn.get_prgname.html
pub fn get_program_name() -> Option<String> {
    get_prgname()
}

pub fn get_prgname() -> Option<String> {
    unsafe {
        from_glib_none(ffi::g_get_prgname())
    }
}

/// Same as [`set_prgname()`].
///
/// [`set_prgname()`]: fn.set_prgname.html
pub fn set_program_name(name: Option<&str>) {
    set_prgname(name)
}

pub fn set_prgname(name: Option<&str>) {
    unsafe {
        ffi::g_set_prgname(name.to_glib_none().0)
    }
}

#[cfg(unix)]
pub fn getenv(variable_name: &str) -> Option<String> {
    unsafe {
        from_glib_none(ffi::g_getenv(variable_name.to_glib_none().0))
    }
}

#[cfg(unix)]
pub fn setenv(variable_name: &str, value: &str, overwrite: bool) -> bool {
    unsafe {
        from_glib(ffi::g_setenv(variable_name.to_glib_none().0,
                                value.to_glib_none().0,
                                overwrite.to_glib()))
    }
}

#[cfg(unix)]
pub fn unsetenv(variable_name: &str) {
    unsafe {
        ffi::g_unsetenv(variable_name.to_glib_none().0)
    }
}

pub fn get_user_name() -> Option<String> {
    unsafe {
        from_glib_none(ffi::g_get_user_name())
    }
}

#[cfg(unix)]
pub fn get_current_dir() -> Option<String> {
    unsafe {
        from_glib_none(ffi::g_get_current_dir())
    }
}

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

#[cfg(not(windows))]
pub fn getenv(variable_name: &str) -> Option<String> {
    unsafe {
        from_glib_none(ffi::g_getenv(variable_name.to_glib_none().0))
    }
}

#[cfg(windows)]
pub fn getenv(variable_name: &str) -> Option<String> {
    use libc::c_char;
    extern "C" {
        fn g_getenv_utf8(variable: *const c_char) -> *const c_char;
    }

    unsafe {
        from_glib_none(g_getenv_utf8(variable_name.to_glib_none().0))
    }
}

#[cfg(not(windows))]
pub fn setenv(variable_name: &str, value: &str, overwrite: bool) -> bool {
    unsafe {
        from_glib(ffi::g_setenv(variable_name.to_glib_none().0,
                                value.to_glib_none().0,
                                overwrite.to_glib()))
    }
}

#[cfg(windows)]
pub fn setenv(variable_name: &str, value: &str, overwrite: bool) -> bool {
    use libc::c_char;
    extern "C" {
        fn g_setenv_utf8(variable: *const c_char, value: *const c_char, overwrite: ffi::gboolean) -> ffi::gboolean;
    }

    unsafe {
        from_glib(g_setenv_utf8(variable_name.to_glib_none().0,
                                value.to_glib_none().0,
                                overwrite.to_glib()))
    }
}

#[cfg(not(windows))]
pub fn unsetenv(variable_name: &str) {
    unsafe {
        ffi::g_unsetenv(variable_name.to_glib_none().0)
    }
}

#[cfg(windows)]
pub fn unsetenv(variable_name: &str) {
    use libc::c_char;
    extern "C" {
        fn g_unsetenv_utf8(variable: *const c_char);
    }

    unsafe {
        g_unsetenv_utf8(variable_name.to_glib_none().0)
    }
}

pub fn get_user_name() -> Option<String> {
    unsafe {
        from_glib_none(ffi::g_get_user_name())
    }
}

#[cfg(not(windows))]
pub fn get_current_dir() -> Option<String> {
    unsafe {
        from_glib_none(ffi::g_get_current_dir())
    }
}

#[cfg(windows)]
pub fn get_current_dir() -> Option<String> {
    use libc::c_char;
    extern "C" {
        fn g_get_current_dir_utf8() -> *mut c_char;
    }

    unsafe {
        from_glib_none(g_get_current_dir_utf8())
    }
}

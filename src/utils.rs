// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use error::BoolError;
use glib_sys;
use gstring::GString;
use std;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::ptr;
use translate::*;
use Error;

/// Same as [`get_prgname()`].
///
/// [`get_prgname()`]: fn.get_prgname.html
pub fn get_program_name() -> Option<String> {
    get_prgname()
}

pub fn get_prgname() -> Option<String> {
    unsafe { from_glib_none(glib_sys::g_get_prgname()) }
}

/// Same as [`set_prgname()`].
///
/// [`set_prgname()`]: fn.set_prgname.html
pub fn set_program_name(name: Option<&str>) {
    set_prgname(name)
}

pub fn set_prgname(name: Option<&str>) {
    unsafe { glib_sys::g_set_prgname(name.to_glib_none().0) }
}

pub fn getenv<K: AsRef<OsStr>>(variable_name: K) -> Option<OsString> {
    #[cfg(not(windows))]
    use glib_sys::g_getenv;
    #[cfg(windows)]
    use glib_sys::g_getenv_utf8 as g_getenv;

    unsafe { from_glib_none(g_getenv(variable_name.as_ref().to_glib_none().0)) }
}

pub fn setenv<K: AsRef<OsStr>, V: AsRef<OsStr>>(
    variable_name: K,
    value: V,
    overwrite: bool,
) -> Result<(), BoolError> {
    #[cfg(not(windows))]
    use glib_sys::g_setenv;
    #[cfg(windows)]
    use glib_sys::g_setenv_utf8 as g_setenv;

    unsafe {
        glib_result_from_gboolean!(
            g_setenv(
                variable_name.as_ref().to_glib_none().0,
                value.as_ref().to_glib_none().0,
                overwrite.to_glib(),
            ),
            "Failed to set environment variable"
        )
    }
}

pub fn unsetenv<K: AsRef<OsStr>>(variable_name: K) {
    #[cfg(not(windows))]
    use glib_sys::g_unsetenv;
    #[cfg(windows)]
    use glib_sys::g_unsetenv_utf8 as g_unsetenv;

    unsafe { g_unsetenv(variable_name.as_ref().to_glib_none().0) }
}

pub fn environ_getenv<K: AsRef<OsStr>>(envp: &[OsString], variable: K) -> Option<OsString> {
    unsafe {
        from_glib_none(glib_sys::g_environ_getenv(
            envp.to_glib_none().0,
            variable.as_ref().to_glib_none().0,
        ))
    }
}

pub fn get_user_name() -> Option<OsString> {
    #[cfg(not(all(windows, target_arch = "x86")))]
    use glib_sys::g_get_user_name;
    #[cfg(all(windows, target_arch = "x86"))]
    use glib_sys::g_get_user_name_utf8 as g_get_user_name;

    unsafe { from_glib_none(g_get_user_name()) }
}

pub fn get_real_name() -> Option<OsString> {
    #[cfg(not(all(windows, target_arch = "x86")))]
    use glib_sys::g_get_real_name;
    #[cfg(all(windows, target_arch = "x86"))]
    use glib_sys::g_get_real_name_utf8 as g_get_real_name;

    unsafe { from_glib_none(g_get_real_name()) }
}

pub fn get_current_dir() -> Option<PathBuf> {
    #[cfg(not(windows))]
    use glib_sys::g_get_current_dir;
    #[cfg(windows)]
    use glib_sys::g_get_current_dir_utf8 as g_get_current_dir;

    unsafe { from_glib_full(g_get_current_dir()) }
}

pub fn filename_to_uri<P: AsRef<Path>>(
    filename: P,
    hostname: Option<&str>,
) -> Result<GString, Error> {
    #[cfg(not(windows))]
    use glib_sys::g_filename_to_uri;
    #[cfg(windows)]
    use glib_sys::g_filename_to_uri_utf8 as g_filename_to_uri;

    let hostname = hostname.to_glib_none();
    unsafe {
        let mut error = std::ptr::null_mut();
        let ret = g_filename_to_uri(filename.as_ref().to_glib_none().0, hostname.0, &mut error);
        if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn filename_from_uri(uri: &str) -> Result<(std::path::PathBuf, Option<GString>), Error> {
    #[cfg(not(windows))]
    use glib_sys::g_filename_from_uri;
    #[cfg(windows)]
    use glib_sys::g_filename_from_uri_utf8 as g_filename_from_uri;

    unsafe {
        let mut hostname = ptr::null_mut();
        let mut error = ptr::null_mut();
        let ret = g_filename_from_uri(uri.to_glib_none().0, &mut hostname, &mut error);
        if error.is_null() {
            Ok((from_glib_full(ret), from_glib_full(hostname)))
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn find_program_in_path<P: AsRef<Path>>(program: P) -> Option<PathBuf> {
    #[cfg(not(all(windows, target_arch = "x86")))]
    use glib_sys::g_find_program_in_path;
    #[cfg(all(windows, target_arch = "x86"))]
    use glib_sys::g_find_program_in_path_utf8 as g_find_program_in_path;

    unsafe { from_glib_full(g_find_program_in_path(program.as_ref().to_glib_none().0)) }
}

pub fn get_home_dir() -> Option<std::path::PathBuf> {
    #[cfg(not(all(windows, target_arch = "x86")))]
    use glib_sys::g_get_home_dir;
    #[cfg(all(windows, target_arch = "x86"))]
    use glib_sys::g_get_home_dir_utf8 as g_get_home_dir;

    unsafe { from_glib_none(g_get_home_dir()) }
}

pub fn get_tmp_dir() -> Option<std::path::PathBuf> {
    #[cfg(not(all(windows, target_arch = "x86")))]
    use glib_sys::g_get_tmp_dir;
    #[cfg(all(windows, target_arch = "x86"))]
    use glib_sys::g_get_tmp_dir_utf8 as g_get_tmp_dir;

    unsafe { from_glib_none(g_get_tmp_dir()) }
}

pub fn mkstemp<P: AsRef<std::path::Path>>(tmpl: P) -> i32 {
    #[cfg(not(windows))]
    use glib_sys::g_mkstemp;
    #[cfg(windows)]
    use glib_sys::g_mkstemp_utf8 as g_mkstemp;

    unsafe { g_mkstemp(tmpl.as_ref().to_glib_none().0) }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Mutex;

    //Mutex to prevent run environment tests parallel
    lazy_static! {
        static ref LOCK: Mutex<()> = Mutex::new(());
    }

    const VAR_NAME: &str = "function_environment_test";

    fn check_getenv(val: &str) {
        let _data = LOCK.lock().unwrap();

        env::set_var(VAR_NAME, val);
        assert_eq!(env::var_os(VAR_NAME), Some(val.into()));
        assert_eq!(::getenv(VAR_NAME), Some(val.into()));

        let environ = ::get_environ();
        assert_eq!(::environ_getenv(&environ, VAR_NAME), Some(val.into()));
    }

    fn check_setenv(val: &str) {
        let _data = LOCK.lock().unwrap();

        ::setenv(VAR_NAME, val, true).unwrap();
        assert_eq!(env::var_os(VAR_NAME), Some(val.into()));
    }

    #[test]
    fn getenv() {
        check_getenv("Test");
        check_getenv("Тест"); // "Test" in Russian
    }

    #[test]
    fn setenv() {
        check_setenv("Test");
        check_setenv("Тест"); // "Test" in Russian
    }

    #[test]
    fn test_filename_from_uri() {
        use gstring::GString;
        use std::path::PathBuf;
        let uri: GString = "file:///foo/bar.txt".into();
        if let Ok((filename, hostname)) = ::filename_from_uri(&uri) {
            assert_eq!(filename, PathBuf::from(r"/foo/bar.txt"));
            assert_eq!(hostname, None);
        } else {
            unreachable!();
        }

        let uri: GString = "file://host/foo/bar.txt".into();
        if let Ok((filename, hostname)) = ::filename_from_uri(&uri) {
            assert_eq!(filename, PathBuf::from(r"/foo/bar.txt"));
            assert_eq!(hostname, Some(GString::from("host")));
        } else {
            unreachable!();
        }
    }
}

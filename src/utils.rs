// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use translate::*;
use UserDirectory;

pub fn get_application_name() -> Option<String> {
    unsafe {
        from_glib_full(ffi::g_get_application_name())
    }
}

pub fn set_application_name(name: Option<&str>) {
    unsafe {
        ffi::g_set_application_name(name.to_glib_none().0)
    }
}

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

pub fn get_environ() -> Vec<String> {
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::g_get_environ())
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

pub fn listenv() -> Vec<String> {
    unsafe {
        FromGlibPtrContainer::from_glib_none(ffi::g_listenv())
    }
}

pub fn get_user_name() -> Option<String> {
    unsafe {
        from_glib_none(ffi::g_get_user_name())
    }
}

pub fn get_real_name() -> Option<String> {
    unsafe {
        from_glib_none(ffi::g_get_real_name())
    }
}

pub fn get_user_cache_dir() -> Option<String> {
    unsafe {
        from_glib_none(ffi::g_get_user_cache_dir())
    }
}

pub fn get_user_data_dir() -> Option<String> {
    unsafe {
        from_glib_none(ffi::g_get_user_data_dir())
    }
}

pub fn get_user_config_dir() -> Option<String> {
    unsafe {
        from_glib_none(ffi::g_get_user_config_dir())
    }
}

pub fn get_user_runtime_dir() -> Option<String> {
    unsafe {
        from_glib_none(ffi::g_get_user_runtime_dir())
    }
}

pub fn get_user_special_dir(directory: UserDirectory) -> Option<String> {
    unsafe {
        from_glib_none(ffi::g_get_user_special_dir(directory.to_glib()))
    }
}

pub fn get_system_data_dirs() -> Vec<String> {
    unsafe {
        FromGlibPtrContainer::from_glib_none(ffi::g_get_system_data_dirs())
    }
}

pub fn get_system_config_dirs() -> Vec<String> {
    unsafe {
        FromGlibPtrContainer::from_glib_none(ffi::g_get_system_config_dirs())
    }
}

pub fn reload_user_special_dirs_cache() {
    unsafe {
        ffi::g_reload_user_special_dirs_cache()
    }
}

pub fn get_host_name() -> Option<String> {
    unsafe {
        from_glib_none(ffi::g_get_host_name())
    }
}

pub fn get_home_dir() -> Option<String> {
    unsafe {
        from_glib_none(ffi::g_get_home_dir())
    }
}

pub fn get_tmp_dir() -> Option<String> {
    unsafe {
        from_glib_none(ffi::g_get_tmp_dir())
    }
}

#[cfg(unix)]
pub fn get_current_dir() -> Option<String> {
    unsafe {
        from_glib_none(ffi::g_get_current_dir())
    }
}

pub fn path_is_absolute(file_name: &str) -> bool {
    unsafe {
        from_glib(ffi::g_path_is_absolute(file_name.to_glib_none().0))
    }
}

pub fn path_skip_root(file_name: &str) -> Option<String> {
    unsafe {
        from_glib_none(ffi::g_path_skip_root(file_name.to_glib_none().0))
    }
}

pub fn path_get_basename(file_name: &str) -> Option<String> {
    unsafe {
        from_glib_full(ffi::g_path_get_basename(file_name.to_glib_none().0))
    }
}

pub fn path_get_dirname(file_name: &str) -> Option<String> {
    unsafe {
        from_glib_full(ffi::g_path_get_dirname(file_name.to_glib_none().0))
    }
}

pub fn find_program_in_path(program: &str) -> Option<String> {
    unsafe {
        from_glib_full(ffi::g_find_program_in_path(program.to_glib_none().0))
    }
}

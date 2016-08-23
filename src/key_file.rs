// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc;
use ffi;
use translate::*;
use std;
use std::ptr;
use std::path;
use error::Error;
use auto::KeyFileFlags;

use KeyFile;

impl KeyFile {
    #[cfg(feature = "v2_40")]
    pub fn save_to_file<T: AsRef<std::path::Path>>(&self, filename: T) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_key_file_save_to_file(self.to_glib_none().0, filename.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn load_from_data_dirs<T: AsRef<std::path::Path>>(&self, file: T, flags: KeyFileFlags) -> Result<path::PathBuf, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let mut full_path: *mut libc::c_char = ptr::null_mut();
            let _ = ffi::g_key_file_load_from_data_dirs(self.to_glib_none().0,
                                                        file.as_ref().to_glib_none().0,
                                                        &mut full_path,
                                                        flags.to_glib(), &mut error);
            if error.is_null() {
                let path: String = from_glib_full(full_path);
                Ok(path::PathBuf::from(path))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn load_from_dirs<T: AsRef<std::path::Path>, U: AsRef<std::path::Path>>(&self, file: T, search_dirs: &[U],
                                                     flags: KeyFileFlags) -> Result<path::PathBuf, Error> {
        unsafe {
            let search_dirs: Vec<&std::path::Path> = search_dirs.iter().map(|p| p.as_ref()).collect();
            let mut error = ptr::null_mut();
            let mut full_path: *mut libc::c_char = ptr::null_mut();
            let _ = ffi::g_key_file_load_from_dirs(self.to_glib_none().0,
                                                   file.as_ref().to_glib_none().0,
                                                   search_dirs.to_glib_none().0,
                                                   &mut full_path,
                                                   flags.to_glib(), &mut error);
            if error.is_null() {
                let path: String = from_glib_full(full_path);
                Ok(path::PathBuf::from(path))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

}

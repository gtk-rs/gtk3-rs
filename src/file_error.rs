// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use error::ErrorDomain;
use ffi as glib_ffi;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileError {
    Exist,
    Isdir,
    Acces,
    Nametoolong,
    Noent,
    Notdir,
    Nxio,
    Nodev,
    Rofs,
    Txtbsy,
    Fault,
    Loop,
    Nospc,
    Nomem,
    Mfile,
    Nfile,
    Badf,
    Inval,
    Pipe,
    Again,
    Intr,
    Io,
    Perm,
    Nosys,
    Failed,
}

impl ErrorDomain for FileError {
    fn domain() -> glib_ffi::GQuark {
        unsafe { glib_ffi::g_file_error_quark() }
    }

    fn code(self) -> i32 {
        use self::FileError::*;
        match self {
            Exist => glib_ffi::G_FILE_ERROR_EXIST as i32,
            Isdir => glib_ffi::G_FILE_ERROR_ISDIR as i32,
            Acces => glib_ffi::G_FILE_ERROR_ACCES as i32,
            Nametoolong => glib_ffi::G_FILE_ERROR_NAMETOOLONG as i32,
            Noent => glib_ffi::G_FILE_ERROR_NOENT as i32,
            Notdir => glib_ffi::G_FILE_ERROR_NOTDIR as i32,
            Nxio => glib_ffi::G_FILE_ERROR_NXIO as i32,
            Nodev => glib_ffi::G_FILE_ERROR_NODEV as i32,
            Rofs => glib_ffi::G_FILE_ERROR_ROFS as i32,
            Txtbsy => glib_ffi::G_FILE_ERROR_TXTBSY as i32,
            Fault => glib_ffi::G_FILE_ERROR_FAULT as i32,
            Loop => glib_ffi::G_FILE_ERROR_LOOP as i32,
            Nospc => glib_ffi::G_FILE_ERROR_NOSPC as i32,
            Nomem => glib_ffi::G_FILE_ERROR_NOMEM as i32,
            Mfile => glib_ffi::G_FILE_ERROR_MFILE as i32,
            Nfile => glib_ffi::G_FILE_ERROR_NFILE as i32,
            Badf => glib_ffi::G_FILE_ERROR_BADF as i32,
            Inval => glib_ffi::G_FILE_ERROR_INVAL as i32,
            Pipe => glib_ffi::G_FILE_ERROR_PIPE as i32,
            Again => glib_ffi::G_FILE_ERROR_AGAIN as i32,
            Intr => glib_ffi::G_FILE_ERROR_INTR as i32,
            Io => glib_ffi::G_FILE_ERROR_IO as i32,
            Perm => glib_ffi::G_FILE_ERROR_PERM as i32,
            Nosys => glib_ffi::G_FILE_ERROR_NOSYS as i32,
            Failed => glib_ffi::G_FILE_ERROR_FAILED as i32,
        }
    }

    fn from(code: i32) -> Option<Self> {
        use self::FileError::*;
        match code {
            x if x == glib_ffi::G_FILE_ERROR_EXIST as i32 => Some(Exist),
            x if x == glib_ffi::G_FILE_ERROR_ISDIR as i32 => Some(Isdir),
            x if x == glib_ffi::G_FILE_ERROR_ACCES as i32 => Some(Acces),
            x if x == glib_ffi::G_FILE_ERROR_NAMETOOLONG as i32 => Some(Nametoolong),
            x if x == glib_ffi::G_FILE_ERROR_NOENT as i32 => Some(Noent),
            x if x == glib_ffi::G_FILE_ERROR_NOTDIR as i32 => Some(Notdir),
            x if x == glib_ffi::G_FILE_ERROR_NXIO as i32 => Some(Nxio),
            x if x == glib_ffi::G_FILE_ERROR_NODEV as i32 => Some(Nodev),
            x if x == glib_ffi::G_FILE_ERROR_ROFS as i32 => Some(Rofs),
            x if x == glib_ffi::G_FILE_ERROR_TXTBSY as i32 => Some(Txtbsy),
            x if x == glib_ffi::G_FILE_ERROR_FAULT as i32 => Some(Fault),
            x if x == glib_ffi::G_FILE_ERROR_LOOP as i32 => Some(Loop),
            x if x == glib_ffi::G_FILE_ERROR_NOSPC as i32 => Some(Nospc),
            x if x == glib_ffi::G_FILE_ERROR_NOMEM as i32 => Some(Nomem),
            x if x == glib_ffi::G_FILE_ERROR_MFILE as i32 => Some(Mfile),
            x if x == glib_ffi::G_FILE_ERROR_NFILE as i32 => Some(Nfile),
            x if x == glib_ffi::G_FILE_ERROR_BADF as i32 => Some(Badf),
            x if x == glib_ffi::G_FILE_ERROR_INVAL as i32 => Some(Inval),
            x if x == glib_ffi::G_FILE_ERROR_PIPE as i32 => Some(Pipe),
            x if x == glib_ffi::G_FILE_ERROR_AGAIN as i32 => Some(Again),
            x if x == glib_ffi::G_FILE_ERROR_INTR as i32 => Some(Intr),
            x if x == glib_ffi::G_FILE_ERROR_IO as i32 => Some(Io),
            x if x == glib_ffi::G_FILE_ERROR_PERM as i32 => Some(Perm),
            x if x == glib_ffi::G_FILE_ERROR_NOSYS as i32 => Some(Nosys),
            x if x == glib_ffi::G_FILE_ERROR_FAILED as i32 => Some(Failed),
            _ => Some(Failed),
        }
    }
}

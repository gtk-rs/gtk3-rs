// Take a look at the license at the top of the repository in the LICENSE file.

use crate::error::ErrorDomain;
use crate::translate::from_glib;
use crate::Quark;

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
    fn domain() -> Quark {
        unsafe { from_glib(ffi::g_file_error_quark()) }
    }

    fn code(self) -> i32 {
        use self::FileError::*;
        match self {
            Exist => ffi::G_FILE_ERROR_EXIST as i32,
            Isdir => ffi::G_FILE_ERROR_ISDIR as i32,
            Acces => ffi::G_FILE_ERROR_ACCES as i32,
            Nametoolong => ffi::G_FILE_ERROR_NAMETOOLONG as i32,
            Noent => ffi::G_FILE_ERROR_NOENT as i32,
            Notdir => ffi::G_FILE_ERROR_NOTDIR as i32,
            Nxio => ffi::G_FILE_ERROR_NXIO as i32,
            Nodev => ffi::G_FILE_ERROR_NODEV as i32,
            Rofs => ffi::G_FILE_ERROR_ROFS as i32,
            Txtbsy => ffi::G_FILE_ERROR_TXTBSY as i32,
            Fault => ffi::G_FILE_ERROR_FAULT as i32,
            Loop => ffi::G_FILE_ERROR_LOOP as i32,
            Nospc => ffi::G_FILE_ERROR_NOSPC as i32,
            Nomem => ffi::G_FILE_ERROR_NOMEM as i32,
            Mfile => ffi::G_FILE_ERROR_MFILE as i32,
            Nfile => ffi::G_FILE_ERROR_NFILE as i32,
            Badf => ffi::G_FILE_ERROR_BADF as i32,
            Inval => ffi::G_FILE_ERROR_INVAL as i32,
            Pipe => ffi::G_FILE_ERROR_PIPE as i32,
            Again => ffi::G_FILE_ERROR_AGAIN as i32,
            Intr => ffi::G_FILE_ERROR_INTR as i32,
            Io => ffi::G_FILE_ERROR_IO as i32,
            Perm => ffi::G_FILE_ERROR_PERM as i32,
            Nosys => ffi::G_FILE_ERROR_NOSYS as i32,
            Failed => ffi::G_FILE_ERROR_FAILED as i32,
        }
    }

    #[allow(clippy::cognitive_complexity)]
    fn from(code: i32) -> Option<Self> {
        use self::FileError::*;
        match code {
            x if x == ffi::G_FILE_ERROR_EXIST as i32 => Some(Exist),
            x if x == ffi::G_FILE_ERROR_ISDIR as i32 => Some(Isdir),
            x if x == ffi::G_FILE_ERROR_ACCES as i32 => Some(Acces),
            x if x == ffi::G_FILE_ERROR_NAMETOOLONG as i32 => Some(Nametoolong),
            x if x == ffi::G_FILE_ERROR_NOENT as i32 => Some(Noent),
            x if x == ffi::G_FILE_ERROR_NOTDIR as i32 => Some(Notdir),
            x if x == ffi::G_FILE_ERROR_NXIO as i32 => Some(Nxio),
            x if x == ffi::G_FILE_ERROR_NODEV as i32 => Some(Nodev),
            x if x == ffi::G_FILE_ERROR_ROFS as i32 => Some(Rofs),
            x if x == ffi::G_FILE_ERROR_TXTBSY as i32 => Some(Txtbsy),
            x if x == ffi::G_FILE_ERROR_FAULT as i32 => Some(Fault),
            x if x == ffi::G_FILE_ERROR_LOOP as i32 => Some(Loop),
            x if x == ffi::G_FILE_ERROR_NOSPC as i32 => Some(Nospc),
            x if x == ffi::G_FILE_ERROR_NOMEM as i32 => Some(Nomem),
            x if x == ffi::G_FILE_ERROR_MFILE as i32 => Some(Mfile),
            x if x == ffi::G_FILE_ERROR_NFILE as i32 => Some(Nfile),
            x if x == ffi::G_FILE_ERROR_BADF as i32 => Some(Badf),
            x if x == ffi::G_FILE_ERROR_INVAL as i32 => Some(Inval),
            x if x == ffi::G_FILE_ERROR_PIPE as i32 => Some(Pipe),
            x if x == ffi::G_FILE_ERROR_AGAIN as i32 => Some(Again),
            x if x == ffi::G_FILE_ERROR_INTR as i32 => Some(Intr),
            x if x == ffi::G_FILE_ERROR_IO as i32 => Some(Io),
            x if x == ffi::G_FILE_ERROR_PERM as i32 => Some(Perm),
            x if x == ffi::G_FILE_ERROR_NOSYS as i32 => Some(Nosys),
            x if x == ffi::G_FILE_ERROR_FAILED as i32 => Some(Failed),
            _ => Some(Failed),
        }
    }
}

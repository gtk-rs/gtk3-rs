// Take a look at the license at the top of the repository in the LICENSE file.

use crate::SocketAddress;
use crate::UnixSocketAddress;
use crate::UnixSocketAddressExt;
use crate::UnixSocketAddressType;
use glib::object::{Cast, IsA};
use glib::translate::*;
#[cfg(not(feature = "dox"))]
use std::ffi::OsStr;
#[cfg(unix)]
#[cfg(not(feature = "dox"))]
use std::os::unix::ffi::OsStrExt;
use std::path;
use std::ptr;
use std::slice;

#[derive(Debug)]
pub enum UnixSocketAddressPath<'a> {
    Path(&'a path::Path),
    Anonymous,
    Abstract(&'a [u8]),
    AbstractPadded(&'a [u8]),
}

impl<'a> UnixSocketAddressPath<'a> {
    fn to_type(&self) -> UnixSocketAddressType {
        use self::UnixSocketAddressPath::*;

        match *self {
            Path(_) => UnixSocketAddressType::Path,
            Anonymous => UnixSocketAddressType::Anonymous,
            Abstract(_) => UnixSocketAddressType::Abstract,
            AbstractPadded(_) => UnixSocketAddressType::AbstractPadded,
        }
    }
}

impl UnixSocketAddress {
    pub fn new(path: &path::Path) -> UnixSocketAddress {
        unsafe {
            SocketAddress::from_glib_full(ffi::g_unix_socket_address_new(path.to_glib_none().0))
                .unsafe_cast()
        }
    }

    pub fn with_type(address_type: UnixSocketAddressPath) -> Self {
        use self::UnixSocketAddressPath::*;

        let type_ = address_type.to_type();
        let (path, len) = match address_type {
            Path(path) => (path.to_glib_none().0, path.as_os_str().len()),
            Abstract(path) | AbstractPadded(path) => {
                (path.to_glib_none().0 as *mut libc::c_char, path.len())
            }
            Anonymous => (ptr::null_mut(), 0),
        };
        unsafe {
            SocketAddress::from_glib_full(ffi::g_unix_socket_address_new_with_type(
                path,
                len as i32,
                type_.to_glib(),
            ))
            .unsafe_cast()
        }
    }
}

pub trait UnixSocketAddressExtManual {
    fn get_path(&self) -> Option<UnixSocketAddressPath>;
}

impl<O: IsA<UnixSocketAddress>> UnixSocketAddressExtManual for O {
    fn get_path(&self) -> Option<UnixSocketAddressPath> {
        use self::UnixSocketAddressPath::*;

        let path = unsafe {
            let path = ffi::g_unix_socket_address_get_path(self.as_ref().to_glib_none().0);
            if path.is_null() {
                &[]
            } else {
                slice::from_raw_parts(path as *const u8, self.get_path_len())
            }
        };
        match self.get_address_type() {
            UnixSocketAddressType::Anonymous => Some(Anonymous),
            #[cfg(not(feature = "dox"))]
            UnixSocketAddressType::Path => Some(Path(path::Path::new(OsStr::from_bytes(path)))),
            #[cfg(feature = "dox")]
            UnixSocketAddressType::Path => unreachable!(),
            UnixSocketAddressType::Abstract => Some(Abstract(path)),
            UnixSocketAddressType::AbstractPadded => Some(AbstractPadded(path)),
            UnixSocketAddressType::Invalid | UnixSocketAddressType::__Unknown(_) => None,
        }
    }
}

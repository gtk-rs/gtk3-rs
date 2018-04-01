use std::ptr;

use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

use ffi;
use SocketAddress;
use UnixSocketAddress;
use UnixSocketAddressType;

use self::AddressType::*;

pub enum AddressType<'a> {
    Path(&'a str),
    Anonymous,
    Abstract(&'a str),
    AbstractPadded(&'a str),
}

impl<'a> AddressType<'a> {
    fn to_type(&self) -> UnixSocketAddressType {
        match *self {
            Path(_) => UnixSocketAddressType::Path,
            Anonymous => UnixSocketAddressType::Anonymous,
            Abstract(_) => UnixSocketAddressType::Abstract,
            AbstractPadded(_) => UnixSocketAddressType::AbstractPadded,
        }
    }
}

pub trait UnixSocketAddressExtManual {
    fn new_with_type(address_type: AddressType) -> UnixSocketAddress;
}

impl<O: IsA<UnixSocketAddress> + IsA<glib::object::Object>> UnixSocketAddressExtManual for O {
    fn new_with_type(address_type: AddressType) -> UnixSocketAddress {
        let type_ = address_type.to_type();
        let (path, len) =
            match address_type {
                Path(path) | Abstract(path) | AbstractPadded(path) => (path.to_glib_none().0, path.len()),
                Anonymous => (ptr::null_mut(), 0),
            };
        unsafe {
            SocketAddress::from_glib_full(ffi::g_unix_socket_address_new_with_type(path, len as i32, type_.to_glib()))
                .downcast_unchecked()
        }
    }
}

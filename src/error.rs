// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::io;
use IOErrorEnum;

impl From<IOErrorEnum> for io::ErrorKind {
    fn from(kind: IOErrorEnum) -> Self {
        match kind {
            IOErrorEnum::NotFound => io::ErrorKind::NotFound,
            IOErrorEnum::Exists => io::ErrorKind::AlreadyExists,
            IOErrorEnum::InvalidFilename => io::ErrorKind::InvalidInput,
            IOErrorEnum::InvalidArgument => io::ErrorKind::InvalidInput,
            IOErrorEnum::PermissionDenied => io::ErrorKind::PermissionDenied,
            IOErrorEnum::AddressInUse => io::ErrorKind::AddrInUse,
            IOErrorEnum::TimedOut => io::ErrorKind::TimedOut,
            IOErrorEnum::WouldBlock => io::ErrorKind::WouldBlock,
            IOErrorEnum::InvalidData => io::ErrorKind::InvalidData,
            IOErrorEnum::ConnectionRefused => io::ErrorKind::ConnectionRefused,
            IOErrorEnum::BrokenPipe => io::ErrorKind::BrokenPipe,
            IOErrorEnum::NotConnected => io::ErrorKind::NotConnected,
            _ => io::ErrorKind::Other,
        }
    }
}

pub(crate) fn to_std_io_result<T>(result: Result<T, glib::Error>) -> io::Result<T> {
    result.map_err(|g_error| match g_error.kind::<IOErrorEnum>() {
        Some(io_error_enum) => io::Error::new(io_error_enum.into(), g_error),
        None => io::Error::new(io::ErrorKind::Other, g_error),
    })
}

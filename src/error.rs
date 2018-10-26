// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Error;
use IOErrorEnum;
use std::io;

impl From<IOErrorEnum> for io::ErrorKind {
    fn from(kind: IOErrorEnum) -> Self {
        match kind {
            IOErrorEnum::Failed => io::ErrorKind::Other,
            IOErrorEnum::NotFound => io::ErrorKind::NotFound,
            IOErrorEnum::Exists => io::ErrorKind::AlreadyExists,
            IOErrorEnum::IsDirectory => io::ErrorKind::Other,
            IOErrorEnum::NotDirectory => io::ErrorKind::Other,
            IOErrorEnum::NotEmpty => io::ErrorKind::Other,
            IOErrorEnum::NotRegularFile => io::ErrorKind::Other,
            IOErrorEnum::NotSymbolicLink => io::ErrorKind::Other,
            IOErrorEnum::NotMountableFile => io::ErrorKind::Other,
            IOErrorEnum::FilenameTooLong => io::ErrorKind::Other,
            IOErrorEnum::InvalidFilename => io::ErrorKind::InvalidInput,
            IOErrorEnum::TooManyLinks => io::ErrorKind::Other,
            IOErrorEnum::NoSpace => io::ErrorKind::Other,
            IOErrorEnum::InvalidArgument => io::ErrorKind::InvalidInput,
            IOErrorEnum::PermissionDenied => io::ErrorKind::PermissionDenied,
            IOErrorEnum::NotSupported => io::ErrorKind::Other,
            IOErrorEnum::NotMounted => io::ErrorKind::Other,
            IOErrorEnum::AlreadyMounted => io::ErrorKind::Other,
            IOErrorEnum::Closed => io::ErrorKind::Other,
            IOErrorEnum::Cancelled => io::ErrorKind::Other,
            IOErrorEnum::Pending => io::ErrorKind::Other,
            IOErrorEnum::ReadOnly => io::ErrorKind::Other,
            IOErrorEnum::CantCreateBackup => io::ErrorKind::Other,
            IOErrorEnum::WrongEtag => io::ErrorKind::Other,
            IOErrorEnum::TimedOut => io::ErrorKind::TimedOut,
            IOErrorEnum::WouldRecurse => io::ErrorKind::Other,
            IOErrorEnum::Busy => io::ErrorKind::Other,
            IOErrorEnum::WouldBlock => io::ErrorKind::WouldBlock,
            IOErrorEnum::HostNotFound => io::ErrorKind::Other,
            IOErrorEnum::WouldMerge => io::ErrorKind::Other,
            IOErrorEnum::FailedHandled => io::ErrorKind::Other,
            IOErrorEnum::TooManyOpenFiles => io::ErrorKind::Other,
            IOErrorEnum::NotInitialized => io::ErrorKind::Other,
            IOErrorEnum::AddressInUse => io::ErrorKind::AddrInUse,
            IOErrorEnum::PartialInput => io::ErrorKind::Other,
            IOErrorEnum::InvalidData => io::ErrorKind::InvalidData,
            IOErrorEnum::DbusError => io::ErrorKind::Other,
            IOErrorEnum::HostUnreachable => io::ErrorKind::Other,
            IOErrorEnum::NetworkUnreachable => io::ErrorKind::Other,
            IOErrorEnum::ConnectionRefused => io::ErrorKind::ConnectionRefused,
            IOErrorEnum::ProxyFailed => io::ErrorKind::Other,
            IOErrorEnum::ProxyAuthFailed => io::ErrorKind::Other,
            IOErrorEnum::ProxyNeedAuth => io::ErrorKind::Other,
            IOErrorEnum::ProxyNotAllowed => io::ErrorKind::Other,
            IOErrorEnum::BrokenPipe => io::ErrorKind::BrokenPipe,
            IOErrorEnum::NotConnected => io::ErrorKind::NotConnected,
            IOErrorEnum::MessageTooLarge => io::ErrorKind::Other,
            IOErrorEnum::__Unknown(_) => io::ErrorKind::Other,
        }
    }
}

pub(crate) fn to_std_io_result<T>(result: Result<T, Error>) -> io::Result<T> {
    result.map_err(|g_error| {
        match g_error.kind::<IOErrorEnum>() {
            Some(io_error_enum) => io::Error::new(io_error_enum.into(), g_error),
            None => io::Error::new(io::ErrorKind::Other, g_error),
        }
    })
}

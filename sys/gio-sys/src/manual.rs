// Copyright 2013-2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#[cfg(not(target_family = "windows"))]
pub use self::libc_constants::*;
#[cfg(target_family = "windows")]
pub use self::windows_constants::*;

pub type GSocketFamily = libc::c_int;
pub type GSocketMsgFlags = libc::c_int;

#[cfg(target_family = "windows")]
mod windows_constants {
    extern crate winapi;

    pub const G_SOCKET_FAMILY_INVALID: super::GSocketFamily =
        self::winapi::shared::ws2def::AF_UNSPEC;
    pub const G_SOCKET_FAMILY_UNIX: super::GSocketFamily = self::winapi::shared::ws2def::AF_UNIX;
    pub const G_SOCKET_FAMILY_IPV4: super::GSocketFamily = self::winapi::shared::ws2def::AF_INET;
    pub const G_SOCKET_FAMILY_IPV6: super::GSocketFamily = self::winapi::shared::ws2def::AF_INET6;

    pub const G_SOCKET_MSG_NONE: super::GSocketMsgFlags = 0;
    pub const G_SOCKET_MSG_OOB: super::GSocketMsgFlags = self::winapi::um::winsock2::MSG_OOB;
    pub const G_SOCKET_MSG_PEEK: super::GSocketMsgFlags = self::winapi::um::winsock2::MSG_PEEK;
    pub const G_SOCKET_MSG_DONTROUTE: super::GSocketMsgFlags =
        self::winapi::um::winsock2::MSG_DONTROUTE;
}

#[cfg(not(target_family = "windows"))]
mod libc_constants {
    pub const G_SOCKET_FAMILY_INVALID: super::GSocketFamily = libc::AF_UNSPEC;
    pub const G_SOCKET_FAMILY_UNIX: super::GSocketFamily = libc::AF_UNIX;
    pub const G_SOCKET_FAMILY_IPV4: super::GSocketFamily = libc::AF_INET;
    pub const G_SOCKET_FAMILY_IPV6: super::GSocketFamily = libc::AF_INET6;

    pub const G_SOCKET_MSG_NONE: super::GSocketMsgFlags = 0;
    pub const G_SOCKET_MSG_OOB: super::GSocketMsgFlags = libc::MSG_OOB;
    pub const G_SOCKET_MSG_PEEK: super::GSocketMsgFlags = libc::MSG_PEEK;
    pub const G_SOCKET_MSG_DONTROUTE: super::GSocketMsgFlags = libc::MSG_DONTROUTE;
}

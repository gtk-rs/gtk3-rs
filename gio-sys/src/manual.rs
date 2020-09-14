// Copyright 2013-2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#[cfg(not(target_family = "windows"))]
use libc as af_constants;
#[cfg(target_family = "windows")]
use winapi::shared::ws2def as af_constants;

pub type GSocketFamily = libc::c_int;
pub const G_SOCKET_FAMILY_INVALID: GSocketFamily = af_constants::AF_UNSPEC;
pub const G_SOCKET_FAMILY_UNIX: GSocketFamily = af_constants::AF_UNIX;
pub const G_SOCKET_FAMILY_IPV4: GSocketFamily = af_constants::AF_INET;
pub const G_SOCKET_FAMILY_IPV6: GSocketFamily = af_constants::AF_INET6;

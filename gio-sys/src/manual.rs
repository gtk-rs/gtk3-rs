// Copyright 2013-2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc;
use libc::c_int;

pub type GSocketFamily = c_int;
pub const G_SOCKET_FAMILY_INVALID: GSocketFamily = 0;
pub const G_SOCKET_FAMILY_UNIX: GSocketFamily = 1;
pub const G_SOCKET_FAMILY_IPV4: GSocketFamily = libc::AF_INET;
pub const G_SOCKET_FAMILY_IPV6: GSocketFamily = libc::AF_INET6;

// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#[allow(unused_imports)]
use libc::{c_int, c_ushort, c_void};

#[cfg(windows)]
pub type GPid = *mut c_void;

#[cfg(not(windows))]
pub type GPid = c_int;

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(all(windows,target_arch="x86_64"))]
pub struct GPollFD {
    pub fd: i64,
    pub events: c_ushort,
    pub revents: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(not(all(windows,target_arch="x86_64")))]
pub struct GPollFD {
    pub fd: c_int,
    pub events: c_ushort,
    pub revents: c_ushort,
}

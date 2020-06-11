// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#[allow(unused_imports)]
use libc::{c_char, c_int, c_ushort, c_void};

#[cfg(unix)]
pub use libc::passwd;

#[cfg(all(not(unix), feature = "dox"))]
#[repr(C)]
pub struct passwd {
    pw_name: *mut c_char,
    pw_passwd: *mut c_char,
    pw_uid: u32,
    pw_gid: u32,
    pw_gecos: *mut c_char,
    pw_dir: *mut c_char,
    pw_shell: *mut c_char,
}

#[cfg(windows)]
pub type GPid = *mut c_void;

#[cfg(not(windows))]
pub type GPid = c_int;

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(all(windows, target_arch = "x86_64"))]
pub struct GPollFD {
    pub fd: i64,
    pub events: c_ushort,
    pub revents: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(not(all(windows, target_arch = "x86_64")))]
pub struct GPollFD {
    pub fd: c_int,
    pub events: c_ushort,
    pub revents: c_ushort,
}

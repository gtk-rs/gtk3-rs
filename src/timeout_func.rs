// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

pub mod timeout {
    use ffi;

    pub fn add<T>(interval: u32, func: fn(&mut T) -> i32, data: &T) -> u32 {
        let tmp = data as *const T;
        let tmp_f = func as ffi::gpointer;

        unsafe { ffi::g_timeout_add(interval, tmp_f, tmp as ffi::gpointer) }
    }

    pub fn add_seconds<T>(interval: u32, func: fn(&mut T) -> i32, data: &T) -> u32 {
        let tmp = data as *const T;
        let tmp_f = func as ffi::gpointer;

        unsafe { ffi::g_timeout_add_seconds(interval, tmp_f, tmp as ffi::gpointer) }
    }
}
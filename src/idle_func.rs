// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! callback idle functions

pub mod idle {
    use std::mem::transmute;
    use ffi;

    // pub fn add<T, F>(func: F, data: &mut T) -> u32
    //     where F: FnMut(&mut T) -> bool {
    pub fn add<T>(func: fn(&mut T) -> bool, data: &mut T) -> u32 {
        unsafe {
            ffi::g_idle_add(transmute(func), transmute(data))
        }
    }
}

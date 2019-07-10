// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib_sys;
use std::mem;
use translate::{StashMut, ToGlibPtrMut, Uninitialized};

pub use glib_sys::GTimeVal as TimeVal;

pub fn get_current_time() -> TimeVal {
    unsafe {
        let mut ret = mem::MaybeUninit::uninit();
        glib_sys::g_get_current_time(ret.as_mut_ptr());
        ret.assume_init()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut glib_sys::GTimeVal> for TimeVal {
    type Storage = &'a mut Self;

    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut glib_sys::GTimeVal, Self> {
        StashMut(self as *mut _, self)
    }
}

impl Uninitialized for TimeVal {
    unsafe fn uninitialized() -> TimeVal {
        mem::zeroed()
    }
}

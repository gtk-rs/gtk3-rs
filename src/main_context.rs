// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use translate::*;
use std::mem;

use MainContext;

impl MainContext {
    pub fn prepare(&self) -> (bool, i32) {
        unsafe {
            let mut priority = mem::uninitialized();

            let res = from_glib(ffi::g_main_context_prepare(self.to_glib_none().0, &mut priority));

            (res, priority)
        }
    }
}

// Copyright 2015, The Rust-gnome Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

use libc::c_void;

use ffi::{self, gpointer, GCallback};
use translate::ToGlibPtr;

pub unsafe fn connect(receiver: gpointer, signal_name: &str, trampoline: GCallback,
                      closure: *mut Box<Fn()>) -> u64 {
    let handle = ffi::g_signal_connect_data(receiver, signal_name.borrow_to_glib().0,
        trampoline, closure as gpointer, destroy_closure, 0) as u64;
    assert!(handle > 0);
    handle
}

extern "C" fn destroy_closure(ptr: *mut c_void, _: *mut c_void) {
    unsafe {
        let ptr = ptr as *mut Box<Fn()>;
        // destroy
        Box::from_raw(ptr);
    }
}

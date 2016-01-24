// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! `IMPL` Low level signal support.

use libc::c_void;

use gobject_ffi::{self, GCallback};
use translate::ToGlibPtr;

pub unsafe fn connect(receiver: *mut gobject_ffi::GObject, signal_name: &str, trampoline: GCallback,
                      closure: *mut Box<Fn() + 'static>) -> u64 {
    let handle = gobject_ffi::g_signal_connect_data(receiver, signal_name.to_glib_none().0,
        trampoline, closure as *mut _, Some(destroy_closure),
        gobject_ffi::GConnectFlags::empty()) as u64;
    assert!(handle > 0);
    handle
}

unsafe extern "C" fn destroy_closure(ptr: *mut c_void, _: *mut gobject_ffi::GClosure) {
    // destroy
    Box::<Box<Fn()>>::from_raw(ptr as *mut _);
}

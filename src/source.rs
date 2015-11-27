// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::cell::RefCell;
use std::mem::transmute;
use std::process;
use std::thread;
use glib_ffi::{self, gboolean, gpointer};
use translate::ToGlib;

pub struct Continue(pub bool);

impl ToGlib for Continue {
    type GlibType = gboolean;

    #[inline]
    fn to_glib(&self) -> gboolean {
        self.0.to_glib()
    }
}

struct CallbackGuard;

impl Drop for CallbackGuard {
    fn drop(&mut self) {
        if thread::panicking() {
            process::exit(101);
        }
    }
}

extern "C" fn trampoline(func: &RefCell<Box<FnMut() -> Continue + 'static>>) -> gboolean {
    let _guard = CallbackGuard;
    (&mut *func.borrow_mut())().to_glib()
}

unsafe extern "C" fn destroy_closure(ptr: gpointer) {
    let _guard = CallbackGuard;
    Box::<RefCell<Box<FnMut() -> Continue + 'static>>>::from_raw(ptr as *mut _);
}

fn into_raw<F: FnMut() -> Continue + Send + 'static>(func: F) -> gpointer {
    let func: Box<RefCell<Box<FnMut() -> Continue + Send + 'static>>> =
        Box::new(RefCell::new(Box::new(func)));
    Box::into_raw(func) as gpointer
}

pub fn idle_add<F>(func: F) -> u32
    where F: FnMut() -> Continue + Send + 'static {
    unsafe {
        glib_ffi::g_idle_add_full(glib_ffi::G_PRIORITY_DEFAULT_IDLE, transmute(trampoline),
            into_raw(func), Some(destroy_closure))
    }
}

pub fn timeout_add<F>(interval: u32, func: F) -> u32
    where F: FnMut() -> Continue + Send + 'static {
    unsafe {
        glib_ffi::g_timeout_add_full(glib_ffi::G_PRIORITY_DEFAULT, interval, transmute(trampoline),
            into_raw(func), Some(destroy_closure))
    }
}

pub fn timeout_add_seconds<F>(interval: u32, func: F) -> u32
    where F: FnMut() -> Continue + Send + 'static {
    unsafe {
        glib_ffi::g_timeout_add_seconds_full(glib_ffi::G_PRIORITY_DEFAULT, interval,
            transmute(trampoline), into_raw(func), Some(destroy_closure))
    }
}

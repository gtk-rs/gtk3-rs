// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::cell::RefCell;
use std::ops::DerefMut;
use std::mem::transmute;
use glib_ffi::{gboolean, gpointer, g_idle_add_full, g_timeout_add_full, g_timeout_add_seconds_full};
use translate::ToGlib;

pub struct Continue(pub bool);

impl ToGlib for Continue {
    type GlibType = gboolean;

    #[inline]
    fn to_glib(&self) -> gboolean {
        self.0.to_glib()
    }
}


// Box::into_raw stability workaround
unsafe fn into_raw<T>(b: Box<T>) -> *mut T { transmute(b) }

extern "C" fn trampoline(func: &RefCell<Box<FnMut() -> Continue + 'static>>) -> gboolean {
    func.borrow_mut().deref_mut()().to_glib()
}

unsafe extern "C" fn destroy_closure(ptr: gpointer) {
    // Box::from_raw API stability workaround
    let ptr = ptr as *mut RefCell<Box<FnMut() -> Continue + 'static>>;
    let _: Box<RefCell<Box<FnMut() -> Continue + 'static>>> = transmute(ptr);
}

const PRIORITY_DEFAULT: i32 = 0;
const PRIORITY_DEFAULT_IDLE: i32 = 200;


pub fn idle_add<F>(func: F) -> u32
    where F: FnMut() -> Continue + 'static {
    let f: Box<RefCell<Box<FnMut() -> Continue + 'static>>> = Box::new(RefCell::new(Box::new(func)));
    unsafe {
        g_idle_add_full(PRIORITY_DEFAULT_IDLE, transmute(trampoline),
            into_raw(f) as gpointer, Some(destroy_closure))
    }
}

pub fn timeout_add<F>(interval: u32, func: F) -> u32
    where F: FnMut() -> Continue + 'static {
    let f: Box<RefCell<Box<FnMut() -> Continue + 'static>>> = Box::new(RefCell::new(Box::new(func)));
    unsafe {
        g_timeout_add_full(PRIORITY_DEFAULT, interval, transmute(trampoline),
            into_raw(f) as gpointer, Some(destroy_closure))
    }
}

pub fn timeout_add_seconds<F>(interval: u32, func: F) -> u32
    where F: FnMut() -> Continue + 'static {
    let f: Box<RefCell<Box<FnMut() -> Continue + 'static>>> = Box::new(RefCell::new(Box::new(func)));
    unsafe {
        g_timeout_add_seconds_full(PRIORITY_DEFAULT, interval, transmute(trampoline),
            into_raw(f) as gpointer, Some(destroy_closure))
    }
}

// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! callback idle functions

pub mod idle {
    use std::cell::RefCell;
    use std::mem::transmute;
    use ffi;

    // Box::into_raw stability workaround
    unsafe fn into_raw<T>(b: Box<T>) -> *mut T { transmute(b) }

    // extern "C" fn trampoline(func: &RefCell<Box<FnMut() -> bool>>) -> ffi::gboolean {
    extern "C" fn trampoline(func: &RefCell<Box<Fn() -> bool>>) -> ffi::gboolean {
        func.borrow_mut()() as ffi::gboolean
    }

    extern "C" fn destroy_closure(ptr: ffi::gpointer) {
        unsafe {
            // Box::from_raw API stability workaround
            // let ptr = ptr as *mut RefCell<Box<FnMut() -> bool>>;
            // let _: Box<RefCell<Box<FnMut() -> bool>>> = ::std::mem::transmute(ptr);
            let ptr = ptr as *mut RefCell<Box<Fn() -> bool>>;
            let _: Box<RefCell<Box<Fn() -> bool>>> = ::std::mem::transmute(ptr);
        }
    }

    pub fn add<F>(func: F) -> u32
        // where F: FnMut() -> bool {
        // let f: Box<RefCell<Box<FnMut() -> bool>>> = Box::new(RefCell::new(Box::new(func)));
        where F: Fn() -> bool {
        let f: Box<RefCell<Box<Fn() -> bool>>> = Box::new(RefCell::new(Box::new(func)));
        unsafe {
            ffi::g_idle_add_full(200 /* = G_PRIORITY_DEFAULT_IDLE */,
                transmute(trampoline), into_raw(f) as ffi::gpointer, destroy_closure)
        }
    }
}

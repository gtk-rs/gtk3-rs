// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! callback idle functions

pub mod idle {
    use std::cell::RefCell;
    use std::ops::DerefMut;
    use std::mem::transmute;
    use ffi::{gboolean, g_idle_add_full};
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

    extern "C" fn destroy_closure(ptr: gpointer) {
        unsafe {
            // Box::from_raw API stability workaround
            let ptr = ptr as *mut RefCell<Box<FnMut() -> Continue + 'static>>;
            let _: Box<RefCell<Box<FnMut() -> Continue + 'static>>> = ::std::mem::transmute(ptr);
        }
    }

    pub fn add<F>(func: F) -> u32
        where F: FnMut() -> Continue + 'static {
        let f: Box<RefCell<Box<FnMut() -> Continue + 'static>>> = Box::new(RefCell::new(Box::new(func)));
        unsafe {
            g_idle_add_full(200 /* = G_PRIORITY_DEFAULT_IDLE */,
                transmute(trampoline), into_raw(f) as ffi::gpointer, destroy_closure)
        }
    }
}

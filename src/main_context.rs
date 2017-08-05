// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use translate::*;
use std::mem;
use std::mem::transmute;
use ffi as glib_ffi;
use ffi::{gpointer, gboolean};
use std::cell::RefCell;

use MainContext;
use Source;
use SourceId;

use source::{CallbackGuard, Priority};

impl MainContext {
    pub fn prepare(&self) -> (bool, i32) {
        unsafe {
            let mut priority = mem::uninitialized();

            let res = from_glib(ffi::g_main_context_prepare(self.to_glib_none().0, &mut priority));

            (res, priority)
        }
    }

    pub fn find_source_by_id(&self, source_id: SourceId) -> Option<Source> {
        unsafe {
            from_glib_none(ffi::g_main_context_find_source_by_id(self.to_glib_none().0, source_id.to_glib()))
        }
    }

    // FIXME: These can actually be FnOnce but require FnBox to
    // stabilize, or Box<FnOnce()> to be callable otherwise
    pub fn invoke<F>(&self, func: F)
    where F: FnMut() + Send + 'static {
        unsafe {
            glib_ffi::g_main_context_invoke_full(self.to_glib_none().0, glib_ffi::G_PRIORITY_DEFAULT_IDLE, Some(trampoline),
                into_raw(func), Some(destroy_closure))
        }
    }

    pub fn invoke_with_priority<F>(&self, priority: Priority, func: F)
    where F: FnMut() + Send + 'static {
        unsafe {
            glib_ffi::g_main_context_invoke_full(self.to_glib_none().0, priority.to_glib(), Some(trampoline),
                into_raw(func), Some(destroy_closure))
        }
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
unsafe extern "C" fn trampoline(func: gpointer) -> gboolean {
    let _guard = CallbackGuard::new();
    let func: &RefCell<Box<FnMut() + 'static>> = transmute(func);
    (&mut *func.borrow_mut())();

    glib_ffi::G_SOURCE_REMOVE
}

unsafe extern "C" fn destroy_closure(ptr: gpointer) {
    let _guard = CallbackGuard::new();
    Box::<RefCell<Box<FnMut() + 'static>>>::from_raw(ptr as *mut _);
}

fn into_raw<F: FnMut() + Send + 'static>(func: F) -> gpointer {
    let func: Box<RefCell<Box<FnMut() + Send + 'static>>> =
        Box::new(RefCell::new(Box::new(func)));
    Box::into_raw(func) as gpointer
}

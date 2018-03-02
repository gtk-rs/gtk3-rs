// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use translate::*;
use std::mem;
use std::mem::transmute;
use ffi as glib_ffi;
use ffi::{gpointer, gboolean};

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

    pub fn invoke<F>(&self, func: F)
    where F: FnOnce() + Send + 'static {
        self.invoke_with_priority(::PRIORITY_DEFAULT_IDLE, func);
    }

    pub fn invoke_with_priority<F>(&self, priority: Priority, func: F)
    where F: FnOnce() + Send + 'static {
        unsafe {
            let func = Box::into_raw(Box::new(Some(Box::new(func))));
            glib_ffi::g_main_context_invoke_full(self.to_glib_none().0, priority.to_glib(), Some(trampoline::<F>),
                func as gpointer, Some(destroy_closure::<F>))
        }
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
unsafe extern "C" fn trampoline<F: FnOnce() + Send + 'static>(func: gpointer) -> gboolean {
    let _guard = CallbackGuard::new();
    let func: &mut Option<Box<F>> = transmute(func);
    let func = func.take().expect("MainContext::invoke() closure called multiple times");
    func();
    glib_ffi::G_SOURCE_REMOVE
}

unsafe extern "C" fn destroy_closure<F: FnOnce() + Send + 'static>(ptr: gpointer) {
    let _guard = CallbackGuard::new();
    Box::<Option<Box<F>>>::from_raw(ptr as *mut _);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_invoke() {
        let l = ::MainLoop::new(None, false);
        let c = MainContext::default().unwrap();

        let l_clone = l.clone();
        thread::spawn(move || {
            c.invoke(move || l_clone.quit());
        });

        l.run();
    }
}

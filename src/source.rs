// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::cell::RefCell;
use std::mem::transmute;
use std::process;
use std::thread;
use ffi as glib_ffi;
use ffi::{gboolean, gpointer};
use translate::{from_glib, FromGlib, ToGlib};

/// The id of a source that is returned by `idle_add` and `timeout_add`.
///
/// A value of 0 is a good default as it is never a valid source ID.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Id(u32);

impl ToGlib for Id {
    type GlibType = u32;

    #[inline]
    fn to_glib(&self) -> u32 {
        self.0
    }
}

impl FromGlib<u32> for Id {
    #[inline]
    fn from_glib(val: u32) -> Id {
        Id(val)
    }
}

/// Continue calling the closure in the future iterations or drop it.
///
/// This is the return type of `idle_add` and `timeout_add` closures.
///
/// `Continue(true)` keeps the closure assigned, to be rerun when appropriate.
///
/// `Continue(false)` disconnects and drops it.
pub struct Continue(pub bool);

impl ToGlib for Continue {
    type GlibType = gboolean;

    #[inline]
    fn to_glib(&self) -> gboolean {
        self.0.to_glib()
    }
}

/// Unwinding propagation guard. Aborts the process if destroyed while
/// panicking.
pub struct CallbackGuard(());

impl CallbackGuard {
    pub fn new() -> CallbackGuard {
        CallbackGuard(())
    }
}

impl Drop for CallbackGuard {
    fn drop(&mut self) {
        if thread::panicking() {
            process::exit(101);
        }
    }
}

unsafe extern "C" fn trampoline(func: gpointer) -> gboolean {
    let _guard = CallbackGuard::new();
    let func: &RefCell<Box<FnMut() -> Continue + 'static>> = transmute(func);
    (&mut *func.borrow_mut())().to_glib()
}

unsafe extern "C" fn destroy_closure(ptr: gpointer) {
    let _guard = CallbackGuard::new();
    Box::<RefCell<Box<FnMut() -> Continue + 'static>>>::from_raw(ptr as *mut _);
}

fn into_raw<F: FnMut() -> Continue + Send + 'static>(func: F) -> gpointer {
    let func: Box<RefCell<Box<FnMut() -> Continue + Send + 'static>>> =
        Box::new(RefCell::new(Box::new(func)));
    Box::into_raw(func) as gpointer
}

/// Adds a closure to be called by the default main loop when it's idle.
///
/// `func` will be called repeatedly until it returns `Continue(false)`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
pub fn idle_add<F>(func: F) -> Id
where F: FnMut() -> Continue + Send + 'static {
    unsafe {
        from_glib(glib_ffi::g_idle_add_full(glib_ffi::G_PRIORITY_DEFAULT_IDLE, Some(trampoline),
            into_raw(func), Some(destroy_closure)))
    }
}

/// Adds a closure to be called by the default main loop at regular intervals
/// with millisecond granularity.
///
/// `func` will be called repeatedly every `interval` milliseconds until it
/// returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events. Prefer `timeout_add_seconds` when millisecond
/// precision is not necessary.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
pub fn timeout_add<F>(interval: u32, func: F) -> Id
where F: FnMut() -> Continue + Send + 'static {
    unsafe {
        from_glib(glib_ffi::g_timeout_add_full(glib_ffi::G_PRIORITY_DEFAULT, interval,
            Some(trampoline), into_raw(func), Some(destroy_closure)))
    }
}

/// Adds a closure to be called by the default main loop at regular intervals
/// with second granularity.
///
/// `func` will be called repeatedly every `interval` seconds until it
/// returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
pub fn timeout_add_seconds<F>(interval: u32, func: F) -> Id
where F: FnMut() -> Continue + Send + 'static {
    unsafe {
        from_glib(glib_ffi::g_timeout_add_seconds_full(glib_ffi::G_PRIORITY_DEFAULT, interval,
            Some(trampoline), into_raw(func), Some(destroy_closure)))
    }
}

/// Removes the source with the given id `source_id` from the default main context.
///
/// It is a programmer error to attempt to remove a non-existent source.
/// Note: source id are reused.
///
/// For historical reasons, the native function always returns true, so we
/// ignore it here.
pub fn source_remove(source_id: Id) {
    unsafe {
        glib_ffi::g_source_remove(source_id.to_glib());
    }
}

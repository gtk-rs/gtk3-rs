// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::cell::RefCell;
use std::mem::transmute;
use std::process;
use std::thread;
use ffi as glib_ffi;
use ffi::{gboolean, gpointer};
use translate::{from_glib, from_glib_full, FromGlib, ToGlib, ToGlibPtr};
use libc;

use Source;

/// The id of a source that is returned by `idle_add` and `timeout_add`.
#[derive(Debug, Eq, PartialEq)]
pub struct SourceId(u32);

impl ToGlib for SourceId {
    type GlibType = u32;

    #[inline]
    fn to_glib(&self) -> u32 {
        self.0
    }
}

impl FromGlib<u32> for SourceId {
    #[inline]
    fn from_glib(val: u32) -> SourceId {
        assert_ne!(val, 0);
        SourceId(val)
    }
}

/// Process identificator
#[derive(Debug, Eq, PartialEq)]
pub struct Pid(pub glib_ffi::GPid);

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

impl Default for CallbackGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for CallbackGuard {
    fn drop(&mut self) {
        use std::io::stderr;
        use std::io::Write;

        if thread::panicking() {
            let _ = stderr().write(b"Uncaught panic, exiting\n");
            process::abort();
        }
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
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

#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
unsafe extern "C" fn trampoline_child_watch(pid: glib_ffi::GPid, status: i32, func: gpointer) {
    let _guard = CallbackGuard::new();
    let func: &RefCell<Box<FnMut(Pid, i32) + 'static>> = transmute(func);
    (&mut *func.borrow_mut())(Pid(pid), status)
}

unsafe extern "C" fn destroy_closure_child_watch(ptr: gpointer) {
    let _guard = CallbackGuard::new();
    Box::<RefCell<Box<FnMut(Pid, i32) + 'static>>>::from_raw(ptr as *mut _);
}

fn into_raw_child_watch<F: FnMut(Pid, i32) + Send + 'static>(func: F) -> gpointer {
    let func: Box<RefCell<Box<FnMut(Pid, i32) + Send + 'static>>> =
        Box::new(RefCell::new(Box::new(func)));
    Box::into_raw(func) as gpointer
}

/// Adds a closure to be called by the default main loop when it's idle.
///
/// `func` will be called repeatedly until it returns `Continue(false)`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
pub fn idle_add<F>(func: F) -> SourceId
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
pub fn timeout_add<F>(interval: u32, func: F) -> SourceId
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
pub fn timeout_add_seconds<F>(interval: u32, func: F) -> SourceId
where F: FnMut() -> Continue + Send + 'static {
    unsafe {
        from_glib(glib_ffi::g_timeout_add_seconds_full(glib_ffi::G_PRIORITY_DEFAULT, interval,
            Some(trampoline), into_raw(func), Some(destroy_closure)))
    }
}

/// Adds a closure to be called by the main loop the returned `Source` is attached to when a child
/// process exits.
///
/// `func` will be called when `pid` exits
pub fn child_watch_add<'a, N: Into<Option<&'a str>>, F>(pid: Pid, func: F) -> SourceId
where F: FnMut(Pid, i32) + Send + 'static {
    unsafe {
        let trampoline = trampoline_child_watch as *mut libc::c_void;
        from_glib(glib_ffi::g_child_watch_add_full(glib_ffi::G_PRIORITY_DEFAULT, pid.0,
            Some(transmute(trampoline)), into_raw_child_watch(func), Some(destroy_closure_child_watch)))
    }
}

#[cfg(any(unix, feature = "dox"))]
/// Adds a closure to be called by the default main loop whenever a UNIX signal is raised.
///
/// `func` will be called repeatedly every time `signum` is raised until it
/// returns `Continue(false)`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
pub fn unix_signal_add<F>(signum: i32, func: F) -> SourceId
where F: FnMut() -> Continue + Send + 'static {
    unsafe {
        from_glib(glib_ffi::g_unix_signal_add_full(glib_ffi::G_PRIORITY_DEFAULT, signum,
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
pub fn source_remove(source_id: SourceId) {
    unsafe {
        glib_ffi::g_source_remove(source_id.to_glib());
    }
}

/// The priority of sources
///
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Priority(i32);

impl ToGlib for Priority {
    type GlibType = i32;

    #[inline]
    fn to_glib(&self) -> i32 {
        self.0
    }
}

impl FromGlib<i32> for Priority {
    #[inline]
    fn from_glib(val: i32) -> Priority {
        Priority(val)
    }
}

pub const PRIORITY_HIGH: Priority = Priority(glib_ffi::G_PRIORITY_HIGH);
pub const PRIORITY_DEFAULT: Priority = Priority(glib_ffi::G_PRIORITY_DEFAULT);
pub const PRIORITY_HIGH_IDLE: Priority = Priority(glib_ffi::G_PRIORITY_HIGH_IDLE);
pub const PRIORITY_DEFAULT_IDLE: Priority = Priority(glib_ffi::G_PRIORITY_DEFAULT_IDLE);
pub const PRIORITY_LOW: Priority = Priority(glib_ffi::G_PRIORITY_LOW);

/// Adds a closure to be called by the main loop the return `Source` is attached to when it's idle.
///
/// `func` will be called repeatedly until it returns `Continue(false)`.
pub fn idle_source_new<'a, N: Into<Option<&'a str>>, F>(name: N, priority: Priority, func: F) -> Source
where F: FnMut() -> Continue + Send + 'static {
    unsafe {
        let source = glib_ffi::g_idle_source_new();
        glib_ffi::g_source_set_callback(source, Some(trampoline), into_raw(func), Some(destroy_closure));
        glib_ffi::g_source_set_priority(source, priority.to_glib());

        let name = name.into();
        if let Some(name) = name {
            glib_ffi::g_source_set_name(source, name.to_glib_none().0);
        }

        from_glib_full(source)
    }
}

/// Adds a closure to be called by the main loop the returned `Source` is attached to at regular
/// intervals with millisecond granularity.
///
/// `func` will be called repeatedly every `interval` milliseconds until it
/// returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events. Prefer `timeout_add_seconds` when millisecond
/// precision is not necessary.
pub fn timeout_source_new<'a, N: Into<Option<&'a str>>, F>(interval: u32, name: N, priority: Priority, func: F) -> Source
where F: FnMut() -> Continue + Send + 'static {
    unsafe {
        let source = glib_ffi::g_timeout_source_new(interval);
        glib_ffi::g_source_set_callback(source, Some(trampoline), into_raw(func), Some(destroy_closure));
        glib_ffi::g_source_set_priority(source, priority.to_glib());

        let name = name.into();
        if let Some(name) = name {
            glib_ffi::g_source_set_name(source, name.to_glib_none().0);
        }

        from_glib_full(source)
    }
}

/// Adds a closure to be called by the main loop the returned `Source` is attached to at regular
/// intervals with second granularity.
///
/// `func` will be called repeatedly every `interval` seconds until it
/// returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events.
pub fn timeout_source_new_seconds<'a, N: Into<Option<&'a str>>, F>(interval: u32, name: N, priority: Priority, func: F) -> Source
where F: FnMut() -> Continue + Send + 'static {
    unsafe {
        let source = glib_ffi::g_timeout_source_new_seconds(interval);
        glib_ffi::g_source_set_callback(source, Some(trampoline), into_raw(func), Some(destroy_closure));
        glib_ffi::g_source_set_priority(source, priority.to_glib());

        let name = name.into();
        if let Some(name) = name {
            glib_ffi::g_source_set_name(source, name.to_glib_none().0);
        }

        from_glib_full(source)
    }
}

/// Adds a closure to be called by the main loop the returned `Source` is attached to when a child
/// process exits.
///
/// `func` will be called when `pid` exits
pub fn child_watch_source_new<'a, N: Into<Option<&'a str>>, F>(pid: Pid, name: N, priority: Priority, func: F) -> Source
where F: FnMut(Pid, i32) + Send + 'static {
    unsafe {
        let source = glib_ffi::g_child_watch_source_new(pid.0);
        let trampoline = trampoline_child_watch as *mut libc::c_void;
        glib_ffi::g_source_set_callback(source, Some(transmute(trampoline)), into_raw_child_watch(func), Some(destroy_closure_child_watch));
        glib_ffi::g_source_set_priority(source, priority.to_glib());

        let name = name.into();
        if let Some(name) = name {
            glib_ffi::g_source_set_name(source, name.to_glib_none().0);
        }

        from_glib_full(source)
    }
}

#[cfg(any(unix, feature = "dox"))]
/// Adds a closure to be called by the main loop the returned `Source` is attached to whenever a
/// UNIX signal is raised.
///
/// `func` will be called repeatedly every time `signum` is raised until it
/// returns `Continue(false)`.
pub fn unix_signal_source_new<'a, N: Into<Option<&'a str>>, F>(signum: i32, name: N, priority: Priority, func: F) -> Source
where F: FnMut() -> Continue + Send + 'static {
    unsafe {
        let source = glib_ffi::g_unix_signal_source_new(signum);
        glib_ffi::g_source_set_callback(source, Some(trampoline), into_raw(func), Some(destroy_closure));
        glib_ffi::g_source_set_priority(source, priority.to_glib());

        let name = name.into();
        if let Some(name) = name {
            glib_ffi::g_source_set_name(source, name.to_glib_none().0);
        }

        from_glib_full(source)
    }
}

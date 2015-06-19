// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Manages available sources of events for the main loop

use std::cell::RefCell;
use std::ops::DerefMut;
use std::mem::transmute;
use ffi::{gboolean, gpointer, g_idle_add_full, g_timeout_add_full, g_timeout_add_seconds_full};
use translate::ToGlib;

/// Return type of idle and timeout functions.
///
/// In the callback, return `Continue(true)` to continue scheduling the callback
/// in the main loop or `Continue(false)` to remove it from the main loop.
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
        let _: Box<RefCell<Box<FnMut() -> Continue + 'static>>> = transmute(ptr);
    }
}

const PRIORITY_DEFAULT: i32 = 0;
const PRIORITY_DEFAULT_IDLE: i32 = 200;


/// Adds a function to be called whenever there are no higher priority events pending to the default main loop.
///
/// The function is given the default idle priority, `PRIORITY_DEFAULT_IDLE`.
/// If the function returns `Continue(false)` it is automatically removed from
/// the list of event sources and will not be called again.
///
/// # Examples
///
/// ```ignore
/// let mut i = 0;
/// idle_add(move || {
///     println!("Idle: {}", i);
///     i += 1;
///     Continue(if i <= 10 { true } else { false })
/// });
/// ```
pub fn idle_add<F>(func: F) -> u32
    where F: FnMut() -> Continue + 'static {
    let f: Box<RefCell<Box<FnMut() -> Continue + 'static>>> = Box::new(RefCell::new(Box::new(func)));
    unsafe {
        g_idle_add_full(PRIORITY_DEFAULT_IDLE, transmute(trampoline),
            into_raw(f) as gpointer, destroy_closure)
    }
}

/// Sets a function to be called at regular intervals, with the default priority, `PRIORITY_DEFAULT`.
///
/// The function is called repeatedly until it returns `Continue(false)`, at which point the timeout is
/// automatically destroyed and the function will not be called again. The first call to the
/// function will be at the end of the first interval .
///
/// Note that timeout functions may be delayed, due to the processing of other event sources. Thus
/// they should not be relied on for precise timing. After each call to the timeout function, the
/// time of the next timeout is recalculated based on the current time and the given interval (it
/// does not try to 'catch up' time lost in delays).
///
/// If you want to have a timer in the "seconds" range and do not care about the exact time of the
/// first call of the timer, use the `timeout_add_seconds()` function; this function allows for more
/// optimizations and more efficient system power usage.
///
/// The interval given is in terms of monotonic time, not wall clock time.
/// See `g_get_monotonic_time()` in glib documentation.
///
/// # Examples
///
/// ```ignore
/// timeout_add(3000, || {
///     println!("This prints once every 3 seconds");
///     Continue(true)
/// });
/// ```
pub fn timeout_add<F>(interval: u32, func: F) -> u32
    where F: FnMut() -> Continue + 'static {
    let f: Box<RefCell<Box<FnMut() -> Continue + 'static>>> = Box::new(RefCell::new(Box::new(func)));
    unsafe {
        g_timeout_add_full(PRIORITY_DEFAULT, interval, transmute(trampoline),
            into_raw(f) as gpointer, destroy_closure)
    }
}

/// Sets a function to be called at regular intervals with the default priority, `PRIORITY_DEFAULT`.
///
/// The function is called repeatedly until it returns `Continue(false)`, at which point the timeout
/// is automatically destroyed and the function will not be called again.
///
/// Note that the first call of the timer may not be precise for timeouts of one second. If you need
/// finer precision and have such a timeout, you may want to use `timeout_add()` instead.
///
/// The interval given is in terms of monotonic time, not wall clock time.
/// See `g_get_monotonic_time()` in glib documentation.
///
/// # Examples
///
/// ```ignore
/// timeout_add_seconds(10, || {
///     println!("This prints once every 10 seconds");
///     Continue(true)
/// });
/// ```
pub fn timeout_add_seconds<F>(interval: u32, func: F) -> u32
    where F: FnMut() -> Continue + 'static {
    let f: Box<RefCell<Box<FnMut() -> Continue + 'static>>> = Box::new(RefCell::new(Box::new(func)));
    unsafe {
        g_timeout_add_seconds_full(PRIORITY_DEFAULT, interval, transmute(trampoline),
            into_raw(f) as gpointer, destroy_closure)
    }
}

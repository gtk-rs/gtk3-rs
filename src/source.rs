// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GSourceFunc functions

use std::cell::RefCell;
use std::ops::DerefMut;
use std::mem::transmute;
use ffi::{gboolean, gpointer, g_idle_add_full, g_timeout_add_full, g_timeout_add_seconds_full};
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
        let _: Box<RefCell<Box<FnMut() -> Continue + 'static>>> = transmute(ptr);
    }
}

const G_PRIORITY_DEFAULT: i32 = 0;
const G_PRIORITY_DEFAULT_IDLE: i32 = 200;


/// Adds a function to be called whenever there are no higher priority events pending to the default main loop.
/// The function is given the default idle priority, G_PRIORITY_DEFAULT_IDLE.
/// If the function returns FALSE it is automatically removed from
/// the list of event sources and will not be called again.
///
/// This internally creates a main loop source using g_idle_source_new()
/// and attaches it to the global GMainContext using g_source_attach(),
/// so the callback will be invoked in whichever thread is running that main context.
/// You can do these steps manually if you need greater control or to use a custom main context.
pub fn idle_add<F>(func: F) -> u32
    where F: FnMut() -> Continue + 'static {
    let f: Box<RefCell<Box<FnMut() -> Continue + 'static>>> = Box::new(RefCell::new(Box::new(func)));
    unsafe {
        g_idle_add_full(G_PRIORITY_DEFAULT_IDLE, transmute(trampoline),
            into_raw(f) as gpointer, destroy_closure)
    }
}

/// Sets a function to be called at regular intervals, with the default priority, G_PRIORITY_DEFAULT.
/// The function is called repeatedly until it returns FALSE, at which point the timeout is
/// automatically destroyed and the function will not be called again. The first call to the
/// function will be at the end of the first interval .
///
/// Note that timeout functions may be delayed, due to the processing of other event sources. Thus
/// they should not be relied on for precise timing. After each call to the timeout function, the
/// time of the next timeout is recalculated based on the current time and the given interval (it
/// does not try to 'catch up' time lost in delays).
///
/// If you want to have a timer in the "seconds" range and do not care about the exact time of the
/// first call of the timer, use the g_timeout_add_seconds() function; this function allows for more
/// optimizations and more efficient system power usage.
///
/// This internally creates a main loop source using g_timeout_source_new() and attaches it to the
/// global GMainContext using g_source_attach(), so the callback will be invoked in whichever thread
/// is running that main context. You can do these steps manually if you need greater control or to
/// use a custom main context.
///
/// The interval given is in terms of monotonic time, not wall clock time. See g_get_monotonic_time().
pub fn timeout_add<F>(interval: u32, func: F) -> u32
    where F: FnMut() -> Continue + 'static {
    let f: Box<RefCell<Box<FnMut() -> Continue + 'static>>> = Box::new(RefCell::new(Box::new(func)));
    unsafe {
        g_timeout_add_full(G_PRIORITY_DEFAULT, interval, transmute(trampoline),
            into_raw(f) as gpointer, destroy_closure)
    }
}

/// Sets a function to be called at regular intervals with the default priority, G_PRIORITY_DEFAULT.
/// The function is called repeatedly until it returns FALSE, at which point the timeout is automatically
/// destroyed and the function will not be called again.
///
/// This internally creates a main loop source using g_timeout_source_new_seconds() and attaches it to
/// the main loop context using g_source_attach(). You can do these steps manually if you need greater
/// control. Also see g_timeout_add_seconds_full().
///
/// Note that the first call of the timer may not be precise for timeouts of one second. If you need
/// finer precision and have such a timeout, you may want to use g_timeout_add() instead.
///
/// The interval given is in terms of monotonic time, not wall clock time. See g_get_monotonic_time().
pub fn timeout_add_seconds<F>(interval: u32, func: F) -> u32
    where F: FnMut() -> Continue + 'static {
    let f: Box<RefCell<Box<FnMut() -> Continue + 'static>>> = Box::new(RefCell::new(Box::new(func)));
    unsafe {
        g_timeout_add_seconds_full(G_PRIORITY_DEFAULT, interval, transmute(trampoline),
            into_raw(f) as gpointer, destroy_closure)
    }
}

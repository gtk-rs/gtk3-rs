// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! General — Library initialization and miscellaneous functions

use std::cell::Cell;
use std::ptr;
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};

use glib::translate::*;
use ffi;

thread_local! {
    static IS_MAIN_THREAD: Cell<bool> = Cell::new(false)
}

static INITIALIZED: AtomicBool = ATOMIC_BOOL_INIT;

/// Asserts that this is the main thread and either `gdk::init` or `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => (
        if !::rt::is_initialized_main_thread() {
            if ::rt::is_initialized() {
                panic!("GDK may only be used from the main thread.");
            }
            else {
                panic!("GDK has not been initialized. Call `gdk::init` or `gtk::init` first.");
            }
        }
    )
}

/// No-op.
macro_rules! skip_assert_initialized {
    () => ()
}

/// Asserts that neither `gdk::init` nor `gtk::init` has been called.
macro_rules! assert_not_initialized {
    () => (
        if ::rt::is_initialized() {
            panic!("This function has to be called before `gdk::init` or `gtk::init`.");
        }
    )
}

/// Returns `true` if GDK has been initialized.
#[inline]
pub fn is_initialized() -> bool {
    skip_assert_initialized!();
    INITIALIZED.load(Ordering::Acquire)
}

/// Returns `true` if GDK has been initialized and this is the main thread.
#[inline]
pub fn is_initialized_main_thread() -> bool {
    skip_assert_initialized!();
    IS_MAIN_THREAD.with(|c| c.get())
}

/// Informs this crate that GDK has been initialized and the current thread is the main one.
pub unsafe fn set_initialized() {
    skip_assert_initialized!();
    if is_initialized_main_thread() {
        return;
    }
    else if is_initialized() {
        panic!("Attempted to initialize GDK from two different threads.");
    }
    INITIALIZED.store(true, Ordering::Release);
    IS_MAIN_THREAD.with(|c| c.set(true));
}

pub fn init() {
    assert_not_initialized!();
    unsafe {
        ffi::gdk_init(ptr::null_mut(), ptr::null_mut());
        set_initialized();
    }
}

pub fn get_display_arg_name() -> Option<String> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gdk_get_display_arg_name())
    }
}

pub fn notify_startup_complete() {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_notify_startup_complete() }
}

pub fn notify_startup_complete_with_id(startup_id: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_notify_startup_complete_with_id(startup_id.to_glib_none().0);
    }
}

#[cfg(feature = "v3_10")]
pub fn set_allowed_backends(backends: &str) {
    assert_not_initialized!();
    unsafe {
        ffi::gdk_set_allowed_backends(backends.to_glib_none().0)
    }
}

pub fn get_program_class() -> Option<String> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gdk_get_program_class())
    }
}

pub fn set_program_class(program_class: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_set_program_class(program_class.to_glib_none().0)
    }
}

pub fn flush() {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_flush() }
}

pub fn screen_width() -> i32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_screen_width() }
}

pub fn screen_height() -> i32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_screen_height() }
}

pub fn screen_width_mm() -> i32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_screen_width_mm() }
}

pub fn screen_height_mm() -> i32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_screen_height_mm() }
}

pub fn beep() {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_flush() }
}

pub fn error_trap_push() {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_error_trap_push() }
}

pub fn error_trap_pop() -> i32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_error_trap_pop() }
}

pub fn error_trap_pop_ignored() {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_error_trap_pop_ignored() }
}

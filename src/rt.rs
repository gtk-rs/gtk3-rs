// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! General — Library initialization and miscellaneous functions

#[cfg(any(feature = "v3_16", feature = "dox"))]
use std::cell::Cell;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

#[cfg(any(feature = "v3_16", feature = "dox"))]
thread_local! {
    static IS_MAIN_THREAD: Cell<bool> = Cell::new(false)
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
static INITIALIZED: AtomicBool = ATOMIC_BOOL_INIT;

/// Asserts that this is the main thread and either `gdk::init` or `gtk::init` has been called.
#[cfg(any(feature = "v3_16", feature = "dox"))]
macro_rules! assert_initialized_main_thread {
    () => {
        if !::rt::is_initialized_main_thread() {
            if ::rt::is_initialized() {
                panic!("GDK may only be used from the main thread.");
            } else {
                panic!("GDK has not been initialized. Call `gdk::init` or `gtk::init` first.");
            }
        }
    };
}

/// No-op.
macro_rules! skip_assert_initialized {
    () => {};
}

/// Returns `true` if GDK has been initialized.
#[cfg(any(feature = "v3_16", feature = "dox"))]
#[inline]
pub fn is_initialized() -> bool {
    skip_assert_initialized!();
    INITIALIZED.load(Ordering::Acquire)
}

/// Returns `true` if GDK has been initialized and this is the main thread.
#[cfg(any(feature = "v3_16", feature = "dox"))]
#[inline]
pub fn is_initialized_main_thread() -> bool {
    skip_assert_initialized!();
    IS_MAIN_THREAD.with(|c| c.get())
}

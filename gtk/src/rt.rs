// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use libc::c_uint;
use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

#[cfg(target_os = "macos")]
extern "C" {
    fn pthread_main_np() -> i32;
}

thread_local! {
    static IS_MAIN_THREAD: Cell<bool> = Cell::new(false)
}

static INITIALIZED: AtomicBool = ATOMIC_BOOL_INIT;

/// Asserts that this is the main thread and `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {
        if !crate::rt::is_initialized_main_thread() {
            if crate::rt::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    };
}

/// No-op.
macro_rules! skip_assert_initialized {
    () => {};
}

/// Asserts that `gtk::init` has not been called.
#[allow(unused_macros)]
macro_rules! assert_not_initialized {
    () => {
        if ::rt::is_initialized() {
            panic!("This function has to be called before `gtk::init`.");
        }
    };
}

/// Returns `true` if GTK has been initialized.
#[inline]
pub fn is_initialized() -> bool {
    skip_assert_initialized!();
    INITIALIZED.load(Ordering::Acquire)
}

/// Returns `true` if GTK has been initialized and this is the main thread.
#[inline]
pub fn is_initialized_main_thread() -> bool {
    skip_assert_initialized!();
    IS_MAIN_THREAD.with(|c| c.get())
}

/// Informs this crate that GTK has been initialized and the current thread is the main one.
///
/// # Panics
///
/// This function will panic if you attempt to initialise GTK from more than
/// one thread.
///
/// # Safety
///
/// You must only call this if:
///
/// 1. You have initialised the underlying GTK library yourself.
/// 2. You did 1 on the thread with which you are calling this function
/// 3. You ensure that this thread is the main thread for the process.
pub unsafe fn set_initialized() {
    skip_assert_initialized!();
    if is_initialized_main_thread() {
        return;
    } else if is_initialized() {
        panic!("Attempted to initialize GTK from two different threads.");
    }

    //  OS X has its own notion of the main thread and init must be called on that thread.
    #[cfg(target_os = "macos")]
    {
        if pthread_main_np() == 0 {
            panic!("Attempted to initialize GTK on OSX from non-main thread");
        }
    }

    gdk::set_initialized();
    INITIALIZED.store(true, Ordering::Release);
    IS_MAIN_THREAD.with(|c| c.set(true));
}

/// Tries to initialize GTK+.
///
/// Call either this function or [`Application::new`][new] before using any
/// other GTK+ functions.
///
/// [new]: struct.Application.html#method.new
///
/// Note that this function calls `gtk_init_check()` rather than `gtk_init()`,
/// so will not cause the program to terminate if GTK could not be initialized.
/// Instead, an Ok is returned if the windowing system was successfully
/// initialized otherwise an Err is returned.
pub fn init() -> Result<(), glib::BoolError> {
    skip_assert_initialized!();
    if is_initialized_main_thread() {
        return Ok(());
    } else if is_initialized() {
        panic!("Attempted to initialize GTK from two different threads.");
    }
    unsafe {
        // We just want to keep the program's name since more arguments could lead to unwanted
        // behaviors...
        let argv = ::std::env::args().take(1).collect::<Vec<_>>();

        if from_glib(ffi::gtk_init_check(&mut 1, &mut argv.to_glib_none().0)) {
            if !glib::MainContext::default().acquire() {
                return Err(glib::bool_error!("Failed to acquire default main context"));
            }

            set_initialized();
            Ok(())
        } else {
            Err(glib::bool_error!("Failed to initialize GTK"))
        }
    }
}

pub fn main_quit() {
    assert_initialized_main_thread!();
    unsafe {
        if ffi::gtk_main_level() > 0 {
            ffi::gtk_main_quit();
        } else if cfg!(debug_assertions) {
            panic!("Attempted to quit a GTK main loop when none is running.");
        }
    }
}

pub fn get_major_version() -> u32 {
    skip_assert_initialized!();
    unsafe { ffi::gtk_get_major_version() as u32 }
}

pub fn get_minor_version() -> u32 {
    skip_assert_initialized!();
    unsafe { ffi::gtk_get_minor_version() as u32 }
}

pub fn get_micro_version() -> u32 {
    skip_assert_initialized!();
    unsafe { ffi::gtk_get_micro_version() as u32 }
}

pub fn get_binary_age() -> u32 {
    skip_assert_initialized!();
    unsafe { ffi::gtk_get_binary_age() as u32 }
}

pub fn get_interface_age() -> u32 {
    skip_assert_initialized!();
    unsafe { ffi::gtk_get_interface_age() as u32 }
}

pub fn check_version(
    required_major: u32,
    required_minor: u32,
    required_micro: u32,
) -> Option<String> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(ffi::gtk_check_version(
            required_major as c_uint,
            required_minor as c_uint,
            required_micro as c_uint,
        ))
    }
}

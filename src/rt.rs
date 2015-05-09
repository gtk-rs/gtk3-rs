// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! General — Library initialization and miscellaneous functions

use std::ptr;
use glib::translate::*;
use ffi;

pub fn init() {
    unsafe { ffi::gdk_init(ptr::null_mut(), ptr::null_mut()) }
}

/*pub fn init_check(argc: *mut c_int, argv: *mut *mut *mut c_char) -> bool {
    unsafe { ::glib::to_bool(ffi::gdk_init_check(argc, argv)) }
}

pub fn parse_args(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    unsafe { ffi::gdk_parse_args(argc, argv) }
}*/

pub fn get_display_arg_name() -> Option<String> {
    unsafe {
        from_glib_none(ffi::gdk_get_display_arg_name())
    }
}

pub fn notify_startup_complete() {
    unsafe { ffi::gdk_notify_startup_complete() }
}

pub fn notify_startup_complete_with_id(startup_id: &str) {
    unsafe {
        ffi::gdk_notify_startup_complete_with_id(startup_id.to_glib_none().0);
    }
}

#[cfg(feature = "gdk_3_10")]
pub fn set_allowed_backends(backends: &str) {
    unsafe {
        ffi::gdk_set_allowed_backends(backends.to_glib_none().0)
    }
}

pub fn get_program_class() -> Option<String> {
    unsafe {
        from_glib_none(ffi::gdk_get_program_class())
    }
}

pub fn set_program_class(program_class: &str) {
    unsafe {
        ffi::gdk_set_program_class(program_class.to_glib_none().0)
    }
}

pub fn flush() {
    unsafe { ffi::gdk_flush() }
}

pub fn screen_width() -> i32 {
    unsafe { ffi::gdk_screen_width() }
}

pub fn screen_height() -> i32 {
    unsafe { ffi::gdk_screen_height() }
}

pub fn screen_width_mm() -> i32 {
    unsafe { ffi::gdk_screen_width_mm() }
}

pub fn screen_height_mm() -> i32 {
    unsafe { ffi::gdk_screen_height_mm() }
}

pub fn beep() {
    unsafe { ffi::gdk_flush() }
}

pub fn error_trap_push() {
    unsafe { ffi::gdk_error_trap_push() }
}

pub fn error_trap_pop() {
    unsafe { ffi::gdk_error_trap_pop() }
}

pub fn error_trap_pop_ignored() {
    unsafe { ffi::gdk_error_trap_pop_ignored() }
}
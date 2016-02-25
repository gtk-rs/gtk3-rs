// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use display::Display;
use window::Window;
use ffi;

glib_wrapper! {
    pub struct GLContext(Object<ffi::GdkGLContext>);

    match fn {
        get_type => || ffi::gdk_gl_context_get_type(),
    }
}

impl GLContext {
    pub fn get_display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_gl_context_get_display(self.to_glib_none().0)) }
    }

    pub fn get_window(&self) -> Option<Window> {
        unsafe { from_glib_none(ffi::gdk_gl_context_get_window(self.to_glib_none().0)) }
    }

    pub fn get_shared_context(&self) -> Option<GLContext> {
        unsafe { from_glib_none(ffi::gdk_gl_context_get_shared_context(self.to_glib_none().0)) }
    }

    pub fn get_version(&self) -> (i32, i32) {
        let mut major = 0;
        let mut minor = 0;

        unsafe { ffi::gdk_gl_context_get_version(self.to_glib_none().0, &mut major, &mut minor) };
        (major, minor)
    }

    pub fn set_required_version(&self, major: i32, minor: i32) {
        unsafe { ffi::gdk_gl_context_set_required_version(self.to_glib_none().0, major, minor) }
    }

    pub fn set_debug_enabled(&self, enabled: bool) {
        unsafe { ffi::gdk_gl_context_set_debug_enabled(self.to_glib_none().0, enabled.to_glib()) }
    }

    pub fn get_debug_enabled(&self) -> bool {
        unsafe { from_glib(ffi::gdk_gl_context_get_debug_enabled(self.to_glib_none().0)) }
    }

    pub fn set_forward_compatible(&self, compatible: bool) {
        unsafe { ffi::gdk_gl_context_set_forward_compatible(self.to_glib_none().0, compatible.to_glib()) }
    }

    pub fn get_forward_compatible(&self) -> bool {
        unsafe { from_glib(ffi::gdk_gl_context_get_forward_compatible(self.to_glib_none().0)) }
    }

    pub fn make_current(&self) {
        unsafe { ffi::gdk_gl_context_make_current(self.to_glib_none().0) }
    }

    pub fn get_current() -> Option<GLContext> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gdk_gl_context_get_current()) }
    }

    pub fn clear_current() {
        assert_initialized_main_thread!();
        unsafe { ffi::gdk_gl_context_clear_current() }
    }
}

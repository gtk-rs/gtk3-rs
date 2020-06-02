// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gio_sys;
use glib::error::ErrorDomain;
use glib::translate::*;
use DBusMethodInvocation;

impl DBusMethodInvocation {
    pub fn return_error<T: ErrorDomain>(&self, error: T, message: &str) {
        unsafe {
            gio_sys::g_dbus_method_invocation_return_error_literal(
                self.to_glib_full(),
                T::domain().to_glib(),
                error.code(),
                message.to_glib_none().0,
            );
        }
    }

    pub fn return_gerror(&self, error: glib::Error) {
        unsafe {
            gio_sys::g_dbus_method_invocation_return_gerror(
                self.to_glib_full(),
                error.to_glib_none().0,
            );
        }
    }
}

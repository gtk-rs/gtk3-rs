// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use crate::Application;
use crate::ApplicationWindow;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;

impl ApplicationWindow {
    pub fn new<P: IsA<Application>>(application: &P) -> ApplicationWindow {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_application_window_new(
                application.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

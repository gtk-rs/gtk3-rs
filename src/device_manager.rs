// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::object::IsA;
use ffi;
use DeviceManager;

pub trait DeviceManagerExtManual {
    fn disable_multidevice();
}

impl<O: IsA<DeviceManager>> DeviceManagerExtManual for O {
    fn disable_multidevice() {
        assert_not_initialized!();
        unsafe { ffi::gdk_disable_multidevice() }
    }
}

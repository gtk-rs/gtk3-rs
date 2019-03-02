// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use DeviceManager;

impl DeviceManager {
    pub fn disable_multidevice() {
        assert_not_initialized!();
        unsafe { ffi::gdk_disable_multidevice() }
    }
}

// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib_container::GlibContainer;
use gio_ffi;

pub struct Permission {
    pointer: *mut gio_ffi::GPermission
}

impl Permission {
    pub fn get_allowed(&self) -> bool {
        match unsafe { gio_ffi::g_permission_get_allowed(self.pointer) } {
            0 => false,
            _ => true
        }
    }

    pub fn get_can_acquire(&self) -> bool {
        match unsafe { gio_ffi::g_permission_get_can_acquire(self.pointer) } {
            0 => false,
            _ => true
        }
    }

    pub fn get_can_release(&self) -> bool {
        match unsafe { gio_ffi::g_permission_get_can_release(self.pointer) } {
            0 => false,
            _ => true
        }
    }

    pub fn impl_update(&self, allowed: bool, can_acquire: bool, can_release: bool) {
        unsafe { gio_ffi::g_permission_impl_update(self.pointer,
            if allowed == true { 1 } else { 0 },
            if can_acquire == true { 1 } else { 0 },
            if can_release == true { 1 } else { 0 }) }
    }
}

/*impl Drop for Permission {
    fn drop(&mut self) {
        self.release();
    }
}*/

impl GlibContainer<*mut gio_ffi::GPermission> for Permission {
    fn wrap(pointer: *mut gio_ffi::GPermission) -> Permission {
        Permission {
            pointer: pointer
        }
    }

    fn unwrap(&self) -> *mut gio_ffi::GPermission {
        self.pointer
    }
}

// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GPermission — An object representing the permission to perform a certain action

use glib_container::GlibContainer;
use ffi;

pub struct Permission {
    pointer: *mut ffi::C_GPermission
}

impl Permission {
    pub fn get_allowed(&self) -> bool {
        match unsafe { ffi::g_permission_get_allowed(self.pointer) } {
            0 => false,
            _ => true
        }
    }

    pub fn get_can_acquire(&self) -> bool {
        match unsafe { ffi::g_permission_get_can_acquire(self.pointer) } {
            0 => false,
            _ => true
        }
    }

    pub fn get_can_release(&self) -> bool {
        match unsafe { ffi::g_permission_get_can_release(self.pointer) } {
            0 => false,
            _ => true
        }
    }

    pub fn impl_update(&self, allowed: bool, can_acquire: bool, can_release: bool) {
        unsafe { ffi::g_permission_impl_update(self.pointer,
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

impl GlibContainer<*mut ffi::C_GPermission> for Permission {
    fn wrap(pointer: *mut ffi::C_GPermission) -> Permission {
        Permission {
            pointer: pointer
        }
    }

    fn unwrap(&self) -> *mut ffi::C_GPermission {
        self.pointer
    }
}
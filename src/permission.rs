// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GPermission — An object representing the permission to perform a certain action

use glib_container::GlibContainer;
use gio_ffi;

pub struct Permission {
    pointer: *mut gio_ffi::GPermission
}

impl Permission {
    /// Gets the value of the 'allowed' property. This property is true if the caller
    /// currently has permission to perform the action that permission represents the
    /// permission to perform.
    pub fn get_allowed(&self) -> bool {
        match unsafe { gio_ffi::g_permission_get_allowed(self.pointer) } {
            0 => false,
            _ => true
        }
    }

    /// Gets the value of the 'can-acquire' property. This property is true if it is
    /// generally possible to acquire the permission by calling g_permission_acquire().
    pub fn get_can_acquire(&self) -> bool {
        match unsafe { gio_ffi::g_permission_get_can_acquire(self.pointer) } {
            0 => false,
            _ => true
        }
    }

    /// Gets the value of the 'can-release' property. This property is true if it is
    /// generally possible to release the permission by calling g_permission_release().
    pub fn get_can_release(&self) -> bool {
        match unsafe { gio_ffi::g_permission_get_can_release(self.pointer) } {
            0 => false,
            _ => true
        }
    }

    /// This function is called by the GPermission implementation to update the properties
    /// of the permission. You should never call this function except from a GPermission
    /// implementation.
    /// 
    /// GObject notify signals are generated, as appropriate.
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

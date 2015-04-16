// 
// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![macro_use]

macro_rules! impl_GObjectFunctions(
    ($gtk_struct:ident, $ffi_type:ident) => (
        impl $gtk_struct {
            pub fn unwrap_pointer(&self) -> *mut ffi::$ffi_type {
                self.pointer
            }

            pub fn wrap_pointer(pointer: *mut ffi::$ffi_type) -> $gtk_struct {
                $gtk_struct {
                    pointer: pointer
                }
            }
        }
    )
);
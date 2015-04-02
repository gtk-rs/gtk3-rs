// 
// This file is part of the rust-gnome project
// 

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
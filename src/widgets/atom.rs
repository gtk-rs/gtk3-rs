// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::{FromGlibPtr, ToGlibPtr};

pub struct Atom {
    pointer: ffi::C_GdkAtom
}

impl Atom {
    /*pub fn new() -> Atom {
        Atom {
            pointer: ::std::ptr::mut_null()
        }
    }*/

    pub fn intern(atom_name: &str, only_if_exists: bool) -> Option<Atom> {
        let tmp = unsafe {
            ffi::gdk_atom_intern(atom_name.borrow_to_glib().0, ::glib::to_gboolean(only_if_exists))
        };

        if tmp.is_null() {
            None
        } else {
            Some(Atom {
                pointer: tmp
            })
        }
    }

    pub fn intern_static_string(atom_name: &str) -> Option<Atom> {
        let tmp = unsafe {
            ffi::gdk_atom_intern_static_string(atom_name.borrow_to_glib().0)
        };

        if tmp.is_null() {
            None
        } else {
            Some(Atom {
                pointer: tmp
            })
        }
    }

    pub fn name(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::take(
                ffi::gdk_atom_name(self.pointer))
        }
    }

    // I can't use the gObject macros for this object
    pub fn wrap_pointer(pointer: ffi::C_GdkAtom) -> Atom {
        Atom {
            pointer: pointer
        }
    }

    pub fn unwrap_pointer(&self) -> ffi::C_GdkAtom {
        self.pointer
    }
}
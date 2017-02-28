// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Atom(ffi::GdkAtom);

pub const NONE: Atom = Atom(0 as *mut _);

impl Atom {
    pub fn intern(atom_name: &str) -> Atom {
        assert_initialized_main_thread!();
        unsafe { Atom(ffi::gdk_atom_intern(atom_name.to_glib_none().0, false.to_glib())) }
    }

    pub fn name(&self) -> String {
        unsafe { from_glib_full(ffi::gdk_atom_name(self.0)) }
    }
}

impl<'a> ToGlibPtr<'a, ffi::GdkAtom> for Atom {
    type Storage = ();

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, ffi::GdkAtom, Atom> {
        Stash(self.0, ())
    }
}

impl FromGlibPtrNone<ffi::GdkAtom> for Atom {
    #[inline]
    unsafe fn from_glib_none(ptr: ffi::GdkAtom) -> Atom { Atom(ptr) }
}

impl FromGlibPtrFull<ffi::GdkAtom> for Atom {
    #[inline]
    unsafe fn from_glib_full(_: ffi::GdkAtom) -> Atom { unimplemented!() }
}

impl<'a> From<&'a str> for Atom {
    fn from(r: &'a str) -> Atom {
        skip_assert_initialized!();
        Atom::intern(r)
    }
}

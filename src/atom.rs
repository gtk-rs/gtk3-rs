// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Atom(ffi::GdkAtom);

pub const NONE: Atom = Atom(0 as *mut _);
pub const SELECTION_PRIMARY: Atom = Atom(1 as *mut _);
pub const SELECTION_SECONDARY: Atom = Atom(2 as *mut _);
pub const SELECTION_CLIPBOARD: Atom = Atom(69 as *mut _);
pub const TARGET_BITMAP: Atom = Atom(5 as *mut _);
pub const TARGET_COLORMAP: Atom = Atom(7 as *mut _);
pub const TARGET_DRAWABLE: Atom = Atom(17 as *mut _);
pub const TARGET_PIXMAP: Atom = Atom(20 as *mut _);
pub const TARGET_STRING: Atom = Atom(31 as *mut _);
pub const SELECTION_TYPE_ATOM: Atom = Atom(4 as *mut _);
pub const SELECTION_TYPE_BITMAP: Atom = Atom(5 as *mut _);
pub const SELECTION_TYPE_COLORMAP: Atom = Atom(7 as *mut _);
pub const SELECTION_TYPE_DRAWABLE: Atom = Atom(17 as *mut _);
pub const SELECTION_TYPE_INTEGER: Atom = Atom(19 as *mut _);
pub const SELECTION_TYPE_PIXMAP: Atom = Atom(20 as *mut _);
pub const SELECTION_TYPE_WINDOW: Atom = Atom(33 as *mut _);
pub const SELECTION_TYPE_STRING: Atom = Atom(31 as *mut _);

impl Atom {
    pub fn intern(atom_name: &str) -> Atom {
        assert_initialized_main_thread!();
        unsafe { Atom(ffi::gdk_atom_intern(atom_name.to_glib_none().0, false.to_glib())) }
    }

    pub fn name(&self) -> String {
        unsafe { from_glib_full(ffi::gdk_atom_name(self.0)) }
    }

    pub unsafe fn value(&self) -> usize {
        self.0 as usize
    }
}

impl GlibPtrDefault for Atom {
    type GlibType = ffi::GdkAtom;
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

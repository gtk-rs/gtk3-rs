// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::translate::*;
use glib::GString;
use glib_sys;
use std::mem;
use std::ptr;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Atom(gdk_sys::GdkAtom);

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
        unsafe {
            Atom(gdk_sys::gdk_atom_intern(
                atom_name.to_glib_none().0,
                false.to_glib(),
            ))
        }
    }

    pub fn name(self) -> GString {
        unsafe { from_glib_full(gdk_sys::gdk_atom_name(self.0)) }
    }

    pub unsafe fn value(self) -> usize {
        self.0 as usize
    }
}

impl GlibPtrDefault for Atom {
    type GlibType = gdk_sys::GdkAtom;
}

#[doc(hidden)]
impl Uninitialized for Atom {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

impl<'a> ToGlibPtr<'a, gdk_sys::GdkAtom> for Atom {
    type Storage = ();

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, gdk_sys::GdkAtom, Atom> {
        Stash(self.0, ())
    }
}

impl<'a> ToGlibPtrMut<'a, *mut gdk_sys::GdkAtom> for Atom {
    type Storage = ();

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut gdk_sys::GdkAtom, Atom> {
        StashMut(&mut self.0, ())
    }
}

impl<'a> ToGlibContainerFromSlice<'a, *mut gdk_sys::GdkAtom> for &'a Atom {
    type Storage = (
        Vec<Stash<'a, gdk_sys::GdkAtom, &'a Atom>>,
        Option<Vec<gdk_sys::GdkAtom>>,
    );

    fn to_glib_none_from_slice(t: &'a [&'a Atom]) -> (*mut gdk_sys::GdkAtom, Self::Storage) {
        skip_assert_initialized!();

        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();
        let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
        v_ptr.push(ptr::null_mut());

        (v_ptr.as_ptr() as *mut gdk_sys::GdkAtom, (v, Some(v_ptr)))
    }

    fn to_glib_container_from_slice(t: &'a [&'a Atom]) -> (*mut gdk_sys::GdkAtom, Self::Storage) {
        skip_assert_initialized!();

        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();

        let v_ptr = unsafe {
            let v_ptr = glib_sys::g_malloc0(mem::size_of::<gdk_sys::GdkAtom>() * (t.len() + 1))
                as *mut gdk_sys::GdkAtom;

            for (i, s) in v.iter().enumerate() {
                ptr::write(v_ptr.offset(i as isize), s.0);
            }

            v_ptr
        };

        (v_ptr, (v, None))
    }

    fn to_glib_full_from_slice(_: &[&'a Atom]) -> *mut gdk_sys::GdkAtom {
        skip_assert_initialized!();

        unimplemented!()
    }
}

impl<'a> ToGlibContainerFromSlice<'a, *const gdk_sys::GdkAtom> for &'a Atom {
    type Storage = (
        Vec<Stash<'a, gdk_sys::GdkAtom, &'a Atom>>,
        Option<Vec<gdk_sys::GdkAtom>>,
    );

    fn to_glib_none_from_slice(t: &'a [&'a Atom]) -> (*const gdk_sys::GdkAtom, Self::Storage) {
        skip_assert_initialized!();

        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();
        let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
        v_ptr.push(ptr::null_mut());

        (v_ptr.as_ptr() as *const gdk_sys::GdkAtom, (v, Some(v_ptr)))
    }

    fn to_glib_container_from_slice(t: &'a [&'a Atom]) -> (*const gdk_sys::GdkAtom, Self::Storage) {
        skip_assert_initialized!();

        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();

        let v_ptr = unsafe {
            let v_ptr = glib_sys::g_malloc0(mem::size_of::<gdk_sys::GdkAtom>() * (t.len() + 1))
                as *mut gdk_sys::GdkAtom;

            for (i, s) in v.iter().enumerate() {
                ptr::write(v_ptr.offset(i as isize), s.0);
            }

            v_ptr as *const gdk_sys::GdkAtom
        };

        (v_ptr, (v, None))
    }

    fn to_glib_full_from_slice(_: &[&'a Atom]) -> *const gdk_sys::GdkAtom {
        skip_assert_initialized!();

        unimplemented!()
    }
}

impl FromGlibPtrNone<gdk_sys::GdkAtom> for Atom {
    #[inline]
    unsafe fn from_glib_none(ptr: gdk_sys::GdkAtom) -> Atom {
        Atom(ptr)
    }
}

impl FromGlibPtrBorrow<gdk_sys::GdkAtom> for Atom {
    #[inline]
    unsafe fn from_glib_borrow(ptr: gdk_sys::GdkAtom) -> glib::translate::Borrowed<Atom> {
        glib::translate::Borrowed::new(Atom(ptr))
    }
}

impl FromGlibPtrFull<gdk_sys::GdkAtom> for Atom {
    #[inline]
    unsafe fn from_glib_full(_: gdk_sys::GdkAtom) -> Atom {
        unimplemented!()
    }
}

impl FromGlibContainerAsVec<gdk_sys::GdkAtom, *mut gdk_sys::GdkAtom> for Atom {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut gdk_sys::GdkAtom, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib_none(ptr::read(ptr.offset(i as isize))));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut gdk_sys::GdkAtom, num: usize) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib_sys::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut gdk_sys::GdkAtom, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib_full(ptr::read(ptr.offset(i as isize))));
        }
        glib_sys::g_free(ptr as *mut _);
        res
    }
}

impl FromGlibPtrArrayContainerAsVec<gdk_sys::GdkAtom, *mut gdk_sys::GdkAtom> for Atom {
    unsafe fn from_glib_none_as_vec(ptr: *mut gdk_sys::GdkAtom) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, c_ptr_array_len(ptr))
    }

    unsafe fn from_glib_container_as_vec(ptr: *mut gdk_sys::GdkAtom) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, c_ptr_array_len(ptr))
    }

    unsafe fn from_glib_full_as_vec(ptr: *mut gdk_sys::GdkAtom) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, c_ptr_array_len(ptr))
    }
}

impl<'a> From<&'a str> for Atom {
    fn from(r: &'a str) -> Atom {
        skip_assert_initialized!();
        Atom::intern(r)
    }
}

// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib;
use glib::{IsA, Object};
use glib::translate::*;

use glib_ffi;
use gobject_ffi;
use ffi;
use ListStore;

use std::cmp::Ordering;

pub trait ListStoreExtManual {
    fn insert_sorted<P: IsA<glib::Object>, F: FnMut(&Object, &Object) -> Ordering>(&self, item: &P, compare_func: F) -> u32;

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn sort<F: FnMut(&Object, &Object) -> Ordering>(&self, compare_func: F);
}

impl<O: IsA<ListStore>> ListStoreExtManual for O {
    fn insert_sorted<P: IsA<glib::Object>, F: FnMut(&Object, &Object) -> Ordering>(&self, item: &P, compare_func: F) -> u32 {
        unsafe {
            let mut func = compare_func;
            let func_obj: &mut (FnMut(&Object, &Object) -> Ordering) = &mut func;
            let func_ptr = &func_obj as *const &mut (FnMut(&Object, &Object) -> Ordering) as glib_ffi::gpointer;

            ffi::g_list_store_insert_sorted(
                self.as_ref().to_glib_none().0,
                item.as_ref().to_glib_none().0,
                Some(compare_func_trampoline),
                func_ptr)
        }
    }

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn sort<F: FnMut(&Object, &Object) -> Ordering>(&self, compare_func: F) {
        unsafe {
            let mut func = compare_func;
            let func_obj: &mut (FnMut(&Object, &Object) -> Ordering) = &mut func;
            let func_ptr = &func_obj as *const &mut (FnMut(&Object, &Object) -> Ordering) as glib_ffi::gpointer;

            ffi::g_list_store_sort(
                self.as_ref().to_glib_none().0,
                Some(compare_func_trampoline),
                func_ptr)
        }
    }
}

unsafe extern "C" fn compare_func_trampoline(a: glib_ffi::gconstpointer, b: glib_ffi::gconstpointer, func: glib_ffi::gpointer) -> i32
{
    let func = func as *mut &mut (FnMut(&Object, &Object) -> Ordering);

    let a = from_glib_borrow(a as *mut gobject_ffi::GObject);
    let b = from_glib_borrow(b as *mut gobject_ffi::GObject);

    match (*func)(&a, &b) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

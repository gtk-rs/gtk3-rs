// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib_sys;
use gobject_sys;
use std::cmp::Ordering;
use std::ops;
use std::slice;
use translate::*;
use Value;

glib_wrapper! {
    #[derive(Debug)]
    pub struct ValueArray(Boxed<gobject_sys::GValueArray>);

    match fn {
        copy => |ptr| gobject_sys::g_value_array_copy(mut_override(ptr)),
        free => |ptr| gobject_sys::g_value_array_free(ptr),
        get_type => || gobject_sys::g_value_array_get_type(),
    }
}

impl ValueArray {
    pub fn new(n_prealloced: u32) -> ValueArray {
        unsafe { from_glib_full(gobject_sys::g_value_array_new(n_prealloced)) }
    }

    pub fn append(&mut self, value: &Value) {
        let value = value.to_glib_none();
        unsafe {
            gobject_sys::g_value_array_append(self.to_glib_none_mut().0, value.0);
        }
    }

    pub fn get_nth(&self, index_: u32) -> Option<Value> {
        unsafe {
            from_glib_none(gobject_sys::g_value_array_get_nth(
                mut_override(self.to_glib_none().0),
                index_,
            ))
        }
    }

    pub fn insert(&mut self, index_: u32, value: &Value) {
        let value = value.to_glib_none();
        unsafe {
            gobject_sys::g_value_array_insert(self.to_glib_none_mut().0, index_, value.0);
        }
    }

    pub fn prepend(&mut self, value: &Value) {
        let value = value.to_glib_none();
        unsafe {
            gobject_sys::g_value_array_prepend(self.to_glib_none_mut().0, value.0);
        }
    }

    pub fn remove(&mut self, index_: u32) {
        unsafe {
            gobject_sys::g_value_array_remove(self.to_glib_none_mut().0, index_);
        }
    }

    pub fn sort_with_data<F: FnMut(&Value, &Value) -> Ordering>(&mut self, compare_func: F) {
        unsafe extern "C" fn compare_func_trampoline(
            a: glib_sys::gconstpointer,
            b: glib_sys::gconstpointer,
            func: glib_sys::gpointer,
        ) -> i32 {
            let func = func as *mut &mut (dyn FnMut(&Value, &Value) -> Ordering);

            let a = &*(a as *const Value);
            let b = &*(b as *const Value);

            match (*func)(&a, &b) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            }
        }
        unsafe {
            let mut func = compare_func;
            let func_obj: &mut (dyn FnMut(&Value, &Value) -> Ordering) = &mut func;
            let func_ptr = &func_obj as *const &mut (dyn FnMut(&Value, &Value) -> Ordering)
                as glib_sys::gpointer;

            gobject_sys::g_value_array_sort_with_data(
                self.to_glib_none_mut().0,
                Some(compare_func_trampoline),
                func_ptr,
            );
        }
    }
}

impl ops::Deref for ValueArray {
    type Target = [Value];

    fn deref(&self) -> &[Value] {
        unsafe {
            slice::from_raw_parts(
                (*self.to_glib_none().0).values as *const Value,
                (*self.to_glib_none().0).n_values as usize,
            )
        }
    }
}

impl ops::DerefMut for ValueArray {
    fn deref_mut(&mut self) -> &mut [Value] {
        unsafe {
            slice::from_raw_parts_mut(
                (*self.to_glib_none().0).values as *mut Value,
                (*self.to_glib_none().0).n_values as usize,
            )
        }
    }
}

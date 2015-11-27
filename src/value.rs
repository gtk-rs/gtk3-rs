// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::mem;
use libc::c_char;
use gobject_ffi;
use super::{to_bool, to_gboolean};
use types::Type;
use translate::*;

pub trait ValuePublic {
    unsafe fn get(gvalue: &Value) -> Self;
    unsafe fn set(&self, gvalue: &mut Value);
}

pub struct Value {
    inner: gobject_ffi::GValue,
}

impl Value {
    pub unsafe fn new() -> Value {
        Value { inner: mem::zeroed() }
    }

    pub fn init(&mut self, _type: Type) {
        unsafe { gobject_ffi::g_value_init(&mut self.inner, _type.to_glib()); }
    }

    pub fn reset(&mut self) {
        unsafe { gobject_ffi::g_value_reset(&mut self.inner); }
    }

    pub unsafe fn unset(&mut self) {
        gobject_ffi::g_value_unset(&mut self.inner)
    }

    pub fn strdup_value_contents(&mut self) -> Option<String> {
        unsafe {
            from_glib_full(gobject_ffi::g_strdup_value_contents(&mut self.inner) as *const c_char)
        }
    }

    pub unsafe fn set_boolean(&mut self, v_boolean: bool) {
        gobject_ffi::g_value_set_boolean(&mut self.inner, to_gboolean(v_boolean))
    }

    pub unsafe fn get_boolean(&self) -> bool {
        to_bool(gobject_ffi::g_value_get_boolean(&self.inner))
    }

    pub unsafe fn set_schar(&mut self, v_char: i8) {
        gobject_ffi::g_value_set_schar(&mut self.inner, v_char)
    }

    pub unsafe fn get_schar(&self) -> i8 {
        gobject_ffi::g_value_get_schar(&self.inner)
    }

    pub unsafe fn set_uchar(&mut self, v_uchar: u8) {
        gobject_ffi::g_value_set_uchar(&mut self.inner, v_uchar)
    }

    pub unsafe fn get_uchar(&self) -> u8 {
        gobject_ffi::g_value_get_uchar(&self.inner)
    }

    pub unsafe fn set_int(&mut self, v_int: i32) {
        gobject_ffi::g_value_set_int(&mut self.inner, v_int)
    }

    pub unsafe fn get_int(&self) -> i32 {
        gobject_ffi::g_value_get_int(&self.inner)
    }

    pub unsafe fn set_uint(&mut self, v_uint: u32) {
        gobject_ffi::g_value_set_uint(&mut self.inner, v_uint)
    }

    pub unsafe fn get_uint(&self) -> u32 {
        gobject_ffi::g_value_get_uint(&self.inner)
    }

    pub unsafe fn set_long(&mut self, v_long: i64) {
        gobject_ffi::g_value_set_long(&mut self.inner, v_long as ::libc::c_long)
    }

    pub unsafe fn get_long(&self) -> i64 {
        gobject_ffi::g_value_get_long(&self.inner) as i64
    }

    pub unsafe fn set_ulong(&mut self, v_ulong: u64) {
        gobject_ffi::g_value_set_ulong(&mut self.inner, v_ulong as ::libc::c_ulong)
    }

    pub unsafe fn get_ulong(&self) -> u64 {
        gobject_ffi::g_value_get_ulong(&self.inner) as u64
    }

    pub unsafe fn set_int64(&mut self, v_int64: i64) {
        gobject_ffi::g_value_set_int64(&mut self.inner, v_int64)
    }

    pub unsafe fn get_int64(&self) -> i64 {
        gobject_ffi::g_value_get_int64(&self.inner)
    }

    pub unsafe fn set_uint64(&mut self, v_uint64: u64) {
        gobject_ffi::g_value_set_uint64(&mut self.inner, v_uint64)
    }

    pub unsafe fn get_uint64(&self) -> u64 {
        gobject_ffi::g_value_get_uint64(&self.inner)
    }

    pub unsafe fn set_float(&mut self, v_float: f32) {
        gobject_ffi::g_value_set_float(&mut self.inner, v_float)
    }

    pub unsafe fn get_float(&self) -> f32 {
        gobject_ffi::g_value_get_float(&self.inner)
    }

    pub unsafe fn set_double(&mut self, v_double: f64) {
        gobject_ffi::g_value_set_double(&mut self.inner, v_double)
    }

    pub unsafe fn get_double(&self) -> f64 {
        gobject_ffi::g_value_get_double(&self.inner)
    }

    pub unsafe fn set_string(&mut self, v_string: &str) {
        gobject_ffi::g_value_set_string(&mut self.inner, v_string.to_glib_none().0);
    }

    pub unsafe fn get_string(&self) -> Option<String> {
        from_glib_none(gobject_ffi::g_value_get_string(&self.inner))
    }

    pub unsafe fn set_boxed<T>(&mut self, v_box: &T) {
        gobject_ffi::g_value_set_boxed(&mut self.inner, ::std::mem::transmute(v_box))
    }

    /*pub fn take_boxed<T>(&self, v_box: &T) {
        gobject_ffi::g_value_take_boxed(&mut self.inner, ::std::mem::transmute(v_box))
    }*/

    pub unsafe fn get_boxed<'r, T>(&'r self) -> &'r T {
        ::std::mem::transmute(gobject_ffi::g_value_get_boxed(&self.inner))
    }

    /*pub unsafe fn dup_boxed<'r, T>(&'r self) -> &'r T {
        ::std::mem::transmute(gobject_ffi::g_value_dup_boxed(&mut self.inner))
    }*/

    pub unsafe fn set_pointer<T>(&mut self, v_pointer: &T) {
        gobject_ffi::g_value_set_pointer(&mut self.inner, ::std::mem::transmute(v_pointer))
    }

    pub unsafe fn get_pointer<'r, T>(&'r self) -> &'r T {
        ::std::mem::transmute(gobject_ffi::g_value_get_pointer(&self.inner))
    }

    pub unsafe fn set_object<T>(&mut self, v_object: &T) {
        gobject_ffi::g_value_set_object(&mut self.inner, ::std::mem::transmute(v_object))
    }

    pub unsafe fn get_object<'r, T>(&'r self) -> &'r T {
        ::std::mem::transmute(gobject_ffi::g_value_get_object(&self.inner))
    }

    // FIXME shouldn't be like that
    pub unsafe fn set_gtype(&mut self, v_gtype: Type) {
        gobject_ffi::g_value_set_gtype(&mut self.inner, v_gtype.to_glib())
    }

    // FIXME shouldn't be like that
    pub unsafe fn get_gtype(&self) -> Type {
        from_glib(gobject_ffi::g_value_get_gtype(&self.inner))
    }

    pub unsafe fn set<T: ValuePublic>(&mut self, val: &T) {
        val.set(self);
    }

    pub unsafe fn get<T: ValuePublic>(&self) -> T {
        ValuePublic::get(self)
    }

    pub fn compatible(src_type: Type, dest_type: Type) -> bool {
        unsafe { to_bool(gobject_ffi::g_value_type_compatible(src_type.to_glib(), dest_type.to_glib())) }
    }

    pub fn transformable(src_type: Type, dest_type: Type) -> bool {
        unsafe { to_bool(gobject_ffi::g_value_type_transformable(src_type.to_glib(), dest_type.to_glib())) }
    }

    pub fn as_ptr(&self) -> *const gobject_ffi::GValue {
        &self.inner
    }

    pub fn as_mut_ptr(&mut self) -> *mut gobject_ffi::GValue {
        &mut self.inner
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        unsafe { self.unset(); }
    }
}

impl ValuePublic for i32 {
    unsafe fn get(gvalue: &Value) -> i32 {
        gvalue.get_int()
    }

    unsafe fn set(&self, gvalue: &mut Value) {
        gvalue.set_int(*self)
    }
}

impl ValuePublic for u32 {
    unsafe fn get(gvalue: &Value) -> u32 {
        gvalue.get_uint()
    }

    unsafe fn set(&self, gvalue: &mut Value) {
        gvalue.set_uint(*self)
    }
}

impl ValuePublic for i64 {
    unsafe fn get(gvalue: &Value) -> i64 {
        gvalue.get_int64()
    }

    unsafe fn set(&self, gvalue: &mut Value) {
        gvalue.set_int64(*self)
    }
}

impl ValuePublic for u64 {
    unsafe fn get(gvalue: &Value) -> u64 {
        gvalue.get_uint64()
    }

    unsafe fn set(&self, gvalue: &mut Value) {
        gvalue.set_uint64(*self)
    }
}

impl ValuePublic for bool {
    unsafe fn get(gvalue: &Value) -> bool {
        gvalue.get_boolean()
    }

    unsafe fn set(&self, gvalue: &mut Value) {
        gvalue.set_boolean(*self)
    }
}

impl ValuePublic for i8 {
    unsafe fn get(gvalue: &Value) -> i8 {
        gvalue.get_schar()
    }

    unsafe fn set(&self, gvalue: &mut Value) {
        gvalue.set_schar(*self)
    }
}

impl ValuePublic for u8 {
    unsafe fn get(gvalue: &Value) -> u8 {
        gvalue.get_uchar()
    }

    unsafe fn set(&self, gvalue: &mut Value) {
        gvalue.set_uchar(*self)
    }
}

impl ValuePublic for f32 {
    unsafe fn get(gvalue: &Value) -> f32 {
        gvalue.get_float()
    }

    unsafe fn set(&self, gvalue: &mut Value) {
        gvalue.set_float(*self)
    }
}

impl ValuePublic for f64 {
    unsafe fn get(gvalue: &Value) -> f64 {
        gvalue.get_double()
    }

    unsafe fn set(&self, gvalue: &mut Value) {
        gvalue.set_double(*self)
    }
}

impl ValuePublic for Type {
    unsafe fn get(gvalue: &Value) -> Type {
        gvalue.get_gtype()
    }

    unsafe fn set(&self, gvalue: &mut Value) {
        gvalue.set_gtype(*self)
    }
}

impl ValuePublic for String {
    unsafe fn get(gvalue: &Value) -> String {
        match gvalue.get_string() {
            Some(s) => s,
            None => String::new()
        }
    }

    unsafe fn set(&self, gvalue: &mut Value) {
        gvalue.set_string(self.as_ref())
    }
}

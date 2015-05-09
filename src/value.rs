// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Generic values — A polymorphic type that can hold values of any other type

use std::mem;
use libc::c_char;
use ffi;
use super::{to_bool, to_gboolean};
use types::Type;
use translate::*;

pub trait ValuePublic {
    fn get(gvalue: &Value) -> Self;
    fn set(&self, gvalue: &mut Value);
}

// Possible improvment : store a function pointer inside the struct and make the struct templated
pub struct Value {
    inner: ffi::C_GValue,
}

impl Value {
    pub fn new() -> Value {
        unsafe { Value { inner: mem::zeroed() } }
    }

    pub fn init(&mut self, _type: Type) {
        unsafe { ffi::g_value_init(&mut self.inner, _type.to_glib()) }
    }

    pub fn reset(&mut self) {
        unsafe { ffi::g_value_reset(&mut self.inner) }
    }

    pub fn unset(&mut self) {
        unsafe { ffi::g_value_unset(&mut self.inner) }
    }

    pub fn strdup_value_contents(&mut self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_strdup_value_contents(&mut self.inner) as *const c_char)
        }
    }

    fn set_boolean(&mut self, v_boolean: bool) {
        unsafe { ffi::g_value_set_boolean(&mut self.inner, to_gboolean(v_boolean)) }
    }

    fn get_boolean(&self) -> bool {
        unsafe { to_bool(ffi::g_value_get_boolean(&self.inner)) }
    }

    fn set_schar(&mut self, v_char: i8) {
        unsafe { ffi::g_value_set_schar(&mut self.inner, v_char) }
    }

    fn get_schar(&self) -> i8 {
        unsafe { ffi::g_value_get_schar(&self.inner) }
    }

    fn set_uchar(&mut self, v_uchar: u8) {
        unsafe { ffi::g_value_set_uchar(&mut self.inner, v_uchar) }
    }

    fn get_uchar(&self) -> u8 {
        unsafe { ffi::g_value_get_uchar(&self.inner) }
    }

    fn set_int(&mut self, v_int: i32) {
        unsafe { ffi::g_value_set_int(&mut self.inner, v_int) }
    }

    fn get_int(&self) -> i32 {
        unsafe { ffi::g_value_get_int(&self.inner) }
    }

    fn set_uint(&mut self, v_uint: u32) {
        unsafe { ffi::g_value_set_uint(&mut self.inner, v_uint) }
    }

    fn get_uint(&self) -> u32 {
        unsafe { ffi::g_value_get_uint(&self.inner) }
    }

    pub fn set_long(&mut self, v_long: i64) {
        unsafe { ffi::g_value_set_long(&mut self.inner, v_long as ::libc::c_long) }
    }

    pub fn get_long(&self) -> i64 {
        unsafe { ffi::g_value_get_long(&self.inner) as i64 }
    }

    pub fn set_ulong(&mut self, v_ulong: u64) {
        unsafe { ffi::g_value_set_ulong(&mut self.inner, v_ulong as ::libc::c_ulong) }
    }

    pub fn get_ulong(&self) -> u64 {
        unsafe { ffi::g_value_get_ulong(&self.inner) as u64 }
    }

    fn set_int64(&mut self, v_int64: i64) {
        unsafe { ffi::g_value_set_int64(&mut self.inner, v_int64) }
    }

    fn get_int64(&self) -> i64 {
        unsafe { ffi::g_value_get_int64(&self.inner) }
    }

    fn set_uint64(&mut self, v_uint64: u64) {
        unsafe { ffi::g_value_set_uint64(&mut self.inner, v_uint64) }
    }

    fn get_uint64(&self) -> u64 {
        unsafe { ffi::g_value_get_uint64(&self.inner) }
    }

    fn set_float(&mut self, v_float: f32) {
        unsafe { ffi::g_value_set_float(&mut self.inner, v_float) }
    }

    fn get_float(&self) -> f32 {
        unsafe { ffi::g_value_get_float(&self.inner) }
    }

    fn set_double(&mut self, v_double: f64) {
        unsafe { ffi::g_value_set_double(&mut self.inner, v_double) }
    }

    fn get_double(&self) -> f64 {
        unsafe { ffi::g_value_get_double(&self.inner) }
    }

    // FIXME shouldn't be like that
    pub fn set_enum(&mut self, v_enum: Type) {
        unsafe { ffi::g_value_set_enum(&mut self.inner, v_enum.to_glib()) }
    }

    // FIXME shouldn't be like that
    pub fn get_enum(&self) -> Type {
        unsafe { from_glib(ffi::g_value_get_enum(&self.inner)) }
    }

    // FIXME shouldn't be like that
    pub fn set_flags(&mut self, v_flags: Type) {
        unsafe { ffi::g_value_set_flags(&mut self.inner, v_flags.to_glib()) }
    }

    // FIXME shouldn't be like that
    pub fn get_flags(&self) -> Type {
        unsafe { from_glib(ffi::g_value_get_flags(&self.inner)) }
    }

    fn set_string(&mut self, v_string: &str) {
        unsafe {
            ffi::g_value_set_string(&mut self.inner, v_string.to_glib_none().0);
        }
    }

    pub fn get_string(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_value_get_string(&self.inner))
        }
    }

    pub fn set_boxed<T>(&mut self, v_box: &T) {
        unsafe { ffi::g_value_set_boxed(&mut self.inner, ::std::mem::transmute(v_box)) }
    }

    /*pub fn take_boxed<T>(&self, v_box: &T) {
        unsafe { ffi::g_value_take_boxed(&mut self.inner, ::std::mem::transmute(v_box)) }
    }*/

    pub fn get_boxed<'r, T>(&'r self) -> &'r T {
        unsafe { ::std::mem::transmute(ffi::g_value_get_boxed(&self.inner)) }
    }

    /*pub fn dup_boxed<'r, T>(&'r self) -> &'r T {
        unsafe { ::std::mem::transmute(ffi::g_value_dup_boxed(&mut self.inner)) }
    }*/

    pub fn set_pointer<T>(&mut self, v_pointer: &T) {
        unsafe { ffi::g_value_set_pointer(&mut self.inner, ::std::mem::transmute(v_pointer)) }
    }

    pub fn get_pointer<'r, T>(&'r self) -> &'r T {
        unsafe { ::std::mem::transmute(ffi::g_value_get_pointer(&self.inner)) }
    }

    pub fn set_object<T>(&mut self, v_object: &T) {
        unsafe { ffi::g_value_set_object(&mut self.inner, ::std::mem::transmute(v_object)) }
    }

    pub fn get_object<'r, T>(&'r self) -> &'r T {
        unsafe { ::std::mem::transmute(ffi::g_value_get_object(&self.inner)) }
    }

    // FIXME shouldn't be like that
    fn set_gtype(&mut self, v_gtype: Type) {
        unsafe { ffi::g_value_set_gtype(&mut self.inner, v_gtype.to_glib()) }
    }

    // FIXME shouldn't be like that
    fn get_gtype(&self) -> Type {
        unsafe { from_glib(ffi::g_value_get_gtype(&self.inner)) }
    }

    pub fn set<T: ValuePublic>(&mut self, val: &T) {
        val.set(self);
    }

    pub fn get<T: ValuePublic>(&self) -> T {
        ValuePublic::get(self)
    }

    pub fn compatible(src_type: Type, dest_type: Type) -> bool {
        unsafe { to_bool(ffi::g_value_type_compatible(src_type.to_glib(), dest_type.to_glib())) }
    }

    pub fn transformable(src_type: Type, dest_type: Type) -> bool {
        unsafe { to_bool(ffi::g_value_type_transformable(src_type.to_glib(), dest_type.to_glib())) }
    }

    pub fn as_ptr(&self) -> *const ffi::C_GValue {
        &self.inner
    }

    pub fn as_mut_ptr(&mut self) -> *mut ffi::C_GValue {
        &mut self.inner
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        self.unset();
    }
}

impl ValuePublic for i32 {
    fn get(gvalue: &Value) -> i32 {
        gvalue.get_int()
    }

    fn set(&self, gvalue: &mut Value) {
        gvalue.set_int(*self)
    }
}

impl ValuePublic for u32 {
    fn get(gvalue: &Value) -> u32 {
        gvalue.get_uint()
    }

    fn set(&self, gvalue: &mut Value) {
        gvalue.set_uint(*self)
    }
}

impl ValuePublic for i64 {
    fn get(gvalue: &Value) -> i64 {
        gvalue.get_int64()
    }

    fn set(&self, gvalue: &mut Value) {
        gvalue.set_int64(*self)
    }
}

impl ValuePublic for u64 {
    fn get(gvalue: &Value) -> u64 {
        gvalue.get_uint64()
    }

    fn set(&self, gvalue: &mut Value) {
        gvalue.set_uint64(*self)
    }
}

impl ValuePublic for bool {
    fn get(gvalue: &Value) -> bool {
        gvalue.get_boolean()
    }

    fn set(&self, gvalue: &mut Value) {
        gvalue.set_boolean(*self)
    }
}

impl ValuePublic for i8 {
    fn get(gvalue: &Value) -> i8 {
        gvalue.get_schar()
    }

    fn set(&self, gvalue: &mut Value) {
        gvalue.set_schar(*self)
    }
}

impl ValuePublic for u8 {
    fn get(gvalue: &Value) -> u8 {
        gvalue.get_uchar()
    }

    fn set(&self, gvalue: &mut Value) {
        gvalue.set_uchar(*self)
    }
}

impl ValuePublic for f32 {
    fn get(gvalue: &Value) -> f32 {
        gvalue.get_float()
    }

    fn set(&self, gvalue: &mut Value) {
        gvalue.set_float(*self)
    }
}

impl ValuePublic for f64 {
    fn get(gvalue: &Value) -> f64 {
        gvalue.get_double()
    }

    fn set(&self, gvalue: &mut Value) {
        gvalue.set_double(*self)
    }
}

impl ValuePublic for Type {
    fn get(gvalue: &Value) -> Type {
        gvalue.get_gtype()
    }

    fn set(&self, gvalue: &mut Value) {
        gvalue.set_gtype(*self)
    }
}

impl ValuePublic for String {
    fn get(gvalue: &Value) -> String {
        match gvalue.get_string() {
            Some(s) => s,
            None => String::new()
        }
    }

    fn set(&self, gvalue: &mut Value) {
        gvalue.set_string(self.as_ref())
    }
}

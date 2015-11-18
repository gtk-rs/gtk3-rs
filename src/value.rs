// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Generic values — A polymorphic type that can hold values of any other type

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

/// Possible improvement : store a function pointer inside the struct and make the struct
/// templated
pub struct Value {
    inner: gobject_ffi::GValue,
}

impl Value {
    pub unsafe fn new() -> Value {
        Value { inner: mem::zeroed() }
    }

    /// Initializes value with the default value of type .
    pub fn init(&mut self, _type: Type) {
        unsafe { gobject_ffi::g_value_init(&mut self.inner, _type.to_glib()); }
    }

    /// Clears the current value in value and resets it to the default value (as if the
    /// value had just been initialized).
    pub fn reset(&mut self) {
        unsafe { gobject_ffi::g_value_reset(&mut self.inner); }
    }

    /// Clears the current value in value and "unsets" the type, this releases all resources
    /// associated with this GValue. An unset value is the same as an uninitialized (zero-filled)
    /// GValue structure.
    pub unsafe fn unset(&mut self) {
        gobject_ffi::g_value_unset(&mut self.inner)
    }

    /// Return a newly allocated string, which describes the contents of a GValue. The main
    /// purpose of this function is to describe GValue contents for debugging output, the way
    /// in which the contents are described may change between different GLib versions.
    pub fn strdup_value_contents(&mut self) -> Option<String> {
        unsafe {
            from_glib_full(gobject_ffi::g_strdup_value_contents(&mut self.inner) as *const c_char)
        }
    }

    /// Set the contents of a G_TYPE_BOOLEAN GValue to v_boolean.
    pub unsafe fn set_boolean(&mut self, v_boolean: bool) {
        gobject_ffi::g_value_set_boolean(&mut self.inner, to_gboolean(v_boolean))
    }

    /// Get the contents of a G_TYPE_BOOLEAN GValue.
    pub unsafe fn get_boolean(&self) -> bool {
        to_bool(gobject_ffi::g_value_get_boolean(&self.inner))
    }

    /// Set the contents of a G_TYPE_CHAR GValue to v_char .
    pub unsafe fn set_schar(&mut self, v_char: i8) {
        gobject_ffi::g_value_set_schar(&mut self.inner, v_char)
    }

    /// Get the contents of a G_TYPE_CHAR GValue.
    pub unsafe fn get_schar(&self) -> i8 {
        gobject_ffi::g_value_get_schar(&self.inner)
    }

    /// Set the contents of a G_TYPE_UCHAR GValue to v_uchar .
    pub unsafe fn set_uchar(&mut self, v_uchar: u8) {
        gobject_ffi::g_value_set_uchar(&mut self.inner, v_uchar)
    }

    /// Get the contents of a G_TYPE_UCHAR GValue.
    pub unsafe fn get_uchar(&self) -> u8 {
        gobject_ffi::g_value_get_uchar(&self.inner)
    }

    /// Set the contents of a G_TYPE_INT GValue to v_int.
    pub unsafe fn set_int(&mut self, v_int: i32) {
        gobject_ffi::g_value_set_int(&mut self.inner, v_int)
    }

    /// Get the contents of a G_TYPE_INT GValue.
    pub unsafe fn get_int(&self) -> i32 {
        gobject_ffi::g_value_get_int(&self.inner)
    }

    /// Set the contents of a G_TYPE_UINT GValue to v_uint .
    pub unsafe fn set_uint(&mut self, v_uint: u32) {
        gobject_ffi::g_value_set_uint(&mut self.inner, v_uint)
    }

    /// Get the contents of a G_TYPE_UINT GValue.
    pub unsafe fn get_uint(&self) -> u32 {
        gobject_ffi::g_value_get_uint(&self.inner)
    }

    /// Set the contents of a G_TYPE_LONG GValue to v_long .
    pub unsafe fn set_long(&mut self, v_long: i64) {
        gobject_ffi::g_value_set_long(&mut self.inner, v_long as ::libc::c_long)
    }

    /// Get the contents of a G_TYPE_LONG GValue.
    pub unsafe fn get_long(&self) -> i64 {
        gobject_ffi::g_value_get_long(&self.inner) as i64
    }

    /// Set the contents of a G_TYPE_ULONG GValue to v_ulong .
    pub unsafe fn set_ulong(&mut self, v_ulong: u64) {
        gobject_ffi::g_value_set_ulong(&mut self.inner, v_ulong as ::libc::c_ulong)
    }

    /// Get the contents of a G_TYPE_ULONG GValue.
    pub unsafe fn get_ulong(&self) -> u64 {
        gobject_ffi::g_value_get_ulong(&self.inner) as u64
    }

    /// Set the contents of a G_TYPE_INT64 GValue to v_int64 .
    pub unsafe fn set_int64(&mut self, v_int64: i64) {
        gobject_ffi::g_value_set_int64(&mut self.inner, v_int64)
    }

    /// Get the contents of a G_TYPE_INT64 GValue.
    pub unsafe fn get_int64(&self) -> i64 {
        gobject_ffi::g_value_get_int64(&self.inner)
    }

    /// Set the contents of a G_TYPE_UINT64 GValue to v_uint64 .
    pub unsafe fn set_uint64(&mut self, v_uint64: u64) {
        gobject_ffi::g_value_set_uint64(&mut self.inner, v_uint64)
    }

    /// Get the contents of a G_TYPE_UINT64 GValue.
    pub unsafe fn get_uint64(&self) -> u64 {
        gobject_ffi::g_value_get_uint64(&self.inner)
    }

    /// Set the contents of a G_TYPE_FLOAT GValue to v_float .
    pub unsafe fn set_float(&mut self, v_float: f32) {
        gobject_ffi::g_value_set_float(&mut self.inner, v_float)
    }

    /// Get the contents of a G_TYPE_FLOAT GValue.
    pub unsafe fn get_float(&self) -> f32 {
        gobject_ffi::g_value_get_float(&self.inner)
    }

    /// Set the contents of a G_TYPE_DOUBLE GValue to v_double .
    pub unsafe fn set_double(&mut self, v_double: f64) {
        gobject_ffi::g_value_set_double(&mut self.inner, v_double)
    }

    /// Get the contents of a G_TYPE_DOUBLE GValue.
    pub unsafe fn get_double(&self) -> f64 {
        gobject_ffi::g_value_get_double(&self.inner)
    }

    /// Set the contents of a G_TYPE_STRING GValue to v_string .
    pub unsafe fn set_string(&mut self, v_string: &str) {
        gobject_ffi::g_value_set_string(&mut self.inner, v_string.to_glib_none().0);
    }

    /// Get the contents of a G_TYPE_STRING GValue.
    pub unsafe fn get_string(&self) -> Option<String> {
        from_glib_none(gobject_ffi::g_value_get_string(&self.inner))
    }

    /// Set the contents of a G_TYPE_BOXED derived GValue to v_boxed .
    pub unsafe fn set_boxed<T>(&mut self, v_box: &T) {
        gobject_ffi::g_value_set_boxed(&mut self.inner, ::std::mem::transmute(v_box))
    }

    /*pub fn take_boxed<T>(&self, v_box: &T) {
        gobject_ffi::g_value_take_boxed(&mut self.inner, ::std::mem::transmute(v_box))
    }*/

    /// Get the contents of a G_TYPE_BOXED derived GValue.
    pub unsafe fn get_boxed<'r, T>(&'r self) -> &'r T {
        ::std::mem::transmute(gobject_ffi::g_value_get_boxed(&self.inner))
    }

    /*pub unsafe fn dup_boxed<'r, T>(&'r self) -> &'r T {
        ::std::mem::transmute(gobject_ffi::g_value_dup_boxed(&mut self.inner))
    }*/

    /// Set the contents of a pointer GValue to v_pointer .
    pub unsafe fn set_pointer<T>(&mut self, v_pointer: &T) {
        gobject_ffi::g_value_set_pointer(&mut self.inner, ::std::mem::transmute(v_pointer))
    }

    /// Get the contents of a pointer GValue.
    pub unsafe fn get_pointer<'r, T>(&'r self) -> &'r T {
        ::std::mem::transmute(gobject_ffi::g_value_get_pointer(&self.inner))
    }

    /// Set the contents of a G_TYPE_OBJECT derived GValue to v_object .
    /// 
    /// g_value_set_object() increases the reference count of v_object (the GValue holds a reference
    /// to v_object ). If you do not wish to increase the reference count of the object (i.e. you
    /// wish to pass your current reference to the GValue because you no longer need it), use
    /// g_value_take_object() instead.
    /// 
    /// It is important that your GValue holds a reference to v_object (either its own, or one it
    /// has taken) to ensure that the object won't be destroyed while the GValue still exists).
    pub unsafe fn set_object<T>(&mut self, v_object: &T) {
        gobject_ffi::g_value_set_object(&mut self.inner, ::std::mem::transmute(v_object))
    }

    /// Get the contents of a G_TYPE_OBJECT derived GValue.
    pub unsafe fn get_object<'r, T>(&'r self) -> &'r T {
        ::std::mem::transmute(gobject_ffi::g_value_get_object(&self.inner))
    }

    /// Set the contents of a G_TYPE_GTYPE GValue to v_gtype .
    // FIXME shouldn't be like that
    pub unsafe fn set_gtype(&mut self, v_gtype: Type) {
        gobject_ffi::g_value_set_gtype(&mut self.inner, v_gtype.to_glib())
    }

    /// Get the contents of a G_TYPE_GTYPE GValue.
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

    /// Returns whether a GValue of type src_type can be copied into a GValue of type dest_type .
    pub fn compatible(src_type: Type, dest_type: Type) -> bool {
        unsafe { to_bool(gobject_ffi::g_value_type_compatible(src_type.to_glib(), dest_type.to_glib())) }
    }

    /// Check whether g_value_transform() is able to transform values of type src_type into
    /// values of type dest_type . Note that for the types to be transformable, they must be
    /// compatible and a transform function must be registered.
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

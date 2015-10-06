// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Translation between GLib/GLib-based FFI types and their Rust counterparts.
//!
//! This module allows library bindings authors to decouple type translation
//! logic and use unified idioms at FFI boundaries. It also implements
//! translation of GLib core data types.
//!
//! `FromGlib`, `from_glib` and `ToGlib` translate simple types like `bool`.
//!
//! ```ignore
//!     pub fn set_accept_focus(&self, accept_focus: bool) {
//!         unsafe { glib_ffi::gdk_window_set_accept_focus(self.pointer, accept_focus.to_glib()) }
//!     }
//!
//!     pub fn get_accept_focus(&self) -> bool {
//!         unsafe { from_glib(glib_ffi::gdk_window_get_accept_focus(self.pointer)) }
//!     }
//! ```
//!
//! `FromGlibPtr` (`from_glib_none` and `from_glib_full`) and `ToGlibPtr` work on `gpointer`s
//! and support different modes of ownership transfer.
//!
//! ```ignore
//!     fn get_title(&self) -> Option<String> {
//!         unsafe {
//!             let title = glib_ffi::gtk_window_get_title(self.pointer);
//!             from_glib_none(title)
//!         }
//!     }
//! ```
//!
//! Letting the foreign library borrow pointers from the Rust side often
//! requires having a temporary variable of an intermediate type (e.g. `CString`).
//! A `Stash` contains the temporary storage and a pointer into it that
//! is valid for the lifetime of the `Stash`. As the lifetime of the `Stash` returned
//! from `to_glib_none` is at least the enclosing statement, you can avoid explicitly
//! binding the stash in most cases and just take the pointer out of it:
//!
//! ```ignore
//!     pub fn set_icon_name(&self, name: &str) {
//!         unsafe {
//!             glib_ffi::gdk_window_set_icon_name(self.pointer, name.to_glib_none().0)
//!         }
//!     }
//! ```

use std::collections::HashMap;
use std::ffi::{CString, CStr};
use std::mem;
use std::ptr;
use libc::{c_void, c_char, size_t};
use glib_ffi;

/// A pointer
pub trait Ptr: Copy + 'static {
    fn is_null(&self) -> bool;
    fn from<X>(ptr: *mut X) -> Self;
}

impl<T: 'static> Ptr for *const T {
    #[inline]
    fn is_null(&self) -> bool { (*self).is_null() }

    #[inline]
    fn from<X>(ptr: *mut X) -> *const T { ptr as *const T }
}

impl<T: 'static> Ptr for *mut T {
    #[inline]
    fn is_null(&self) -> bool { (*self).is_null() }

    #[inline]
    fn from<X>(ptr: *mut X) -> *mut T { ptr as *mut T }
}

/// Helper type that stores temporary values used for translation.
///
/// `P` is the foreign type pointer and the first element of the tuple.
///
/// `T` is the Rust type that is translated.
///
/// The second element of the tuple is the temporary storage defined
/// by the implementation of `ToGlibPtr<P> for T`
///
/// Say you want to pass a `*mut GdkWindowAttr` to a foreign function. The `Stash`
/// will own a `GdkWindowAttr` and a `CString` that `GdkWindowAttr::title` points into.
///
/// ```ignore
/// impl <'a> ToGlibPtr<'a, *mut glib_ffi::GdkWindowAttr> for WindowAttr {
///     type Storage = (Box<glib_ffi::GdkWindowAttr>, Stash<'a, *const c_char, Option<String>>);
///
///     fn to_glib_none(&'a self) -> Stash<*mut glib_ffi::GdkWindowAttr, WindowAttr> {
///         let title = self.title.to_glib_none();
///
///         let mut attrs = Box::new(glib_ffi::GdkWindowAttr {
///             title: title.0,
///             // ....
///         });
///
///         Stash(&mut *attrs, (attrs, title))
///     }
/// }
/// ```
pub struct Stash<'a, P: Copy, T: ?Sized + ToGlibPtr<'a, P>> (pub P, pub <T as ToGlibPtr<'a, P>>::Storage);

pub struct StashMut<'a, P: Copy, T: ?Sized> (pub P, pub <T as ToGlibPtrMut<'a, P>>::Storage)
    where T: ToGlibPtrMut<'a, P>;

/// Translate a simple type.
pub trait ToGlib {
    type GlibType;

    fn to_glib(&self) -> Self::GlibType;
}

impl ToGlib for () {
    type GlibType = ();

    #[inline]
    fn to_glib(&self) -> () {
        ()
    }
}

impl ToGlib for bool {
    type GlibType = glib_ffi::gboolean;

    #[inline]
    fn to_glib(&self) -> glib_ffi::gboolean {
        if *self { glib_ffi::GTRUE } else { glib_ffi::GFALSE }
    }
}

/// Translate to a pointer.
pub trait ToGlibPtr<'a, P: Copy> {
    type Storage;

    /// Transfer: none.
    ///
    /// The pointer in the `Stash` is only valid for the lifetime of the `Stash`.
    fn to_glib_none(&self) -> Stash<'a, P, Self>;

    /// Transfer: full.
    ///
    /// We transfer the ownership to the foreign library.
    fn to_glib_full(&self) -> P {
        unimplemented!();
    }
}
///
/// Translate to a pointer with a mutable borrow.
pub trait ToGlibPtrMut<'a, P: Copy> {
    type Storage;

    /// Transfer: none.
    ///
    /// The pointer in the `Stash` is only valid for the lifetime of the `Stash`.
    fn to_glib_none_mut(&'a mut self) -> StashMut<P, Self>;

    /// Transfer: full.
    ///
    /// We transfer the ownership to the foreign library.
    fn to_glib_full_mut(&'a mut self) -> P {
        unimplemented!();
    }
}

impl <'a, P: Ptr, T: ToGlibPtr<'a, P>> ToGlibPtr<'a, P> for Option<T> {
    type Storage = Option<<T as ToGlibPtr<'a, P>>::Storage>;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, P, Option<T>> {
        self.as_ref().map_or(Stash(Ptr::from::<()>(ptr::null_mut()), None), |s| {
            let s = s.to_glib_none();
            Stash(s.0, Some(s.1))
        })
    }

    #[inline]
    fn to_glib_full(&self) -> P {
        self.as_ref().map_or(Ptr::from::<()>(ptr::null_mut()), |s| s.to_glib_full())
    }
}

impl<'a> ToGlibPtr<'a, *const c_char> for &'a str {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *const c_char, &'a str> {
        let tmp = CString::new(*self).unwrap();
        Stash(tmp.as_ptr(), tmp)
    }

    #[inline]
    fn to_glib_full(&self) -> *const c_char {
        unsafe {
            glib_ffi::g_strndup(self.as_ptr() as *const c_char, self.len() as size_t)
                as *const c_char
        }
    }
}

impl<'a> ToGlibPtr<'a, *mut c_char> for &'a str {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut c_char, &'a str> {
        let tmp = CString::new(*self).unwrap();
        Stash(tmp.as_ptr() as *mut c_char, tmp)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut c_char {
        unsafe {
            glib_ffi::g_strndup(self.as_ptr() as *mut c_char, self.len() as size_t)
        }
    }
}

impl <'a> ToGlibPtr<'a, *const c_char> for String {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *const c_char, String> {
        let tmp = CString::new(&self[..]).unwrap();
        Stash(tmp.as_ptr(), tmp)
    }

    #[inline]
    fn to_glib_full(&self) -> *const c_char {
        unsafe {
            glib_ffi::g_strndup(self.as_ptr() as *const c_char, self.len() as size_t)
                as *const c_char
        }
    }
}

impl<'a> ToGlibPtr<'a, *const *const c_char> for &'a [&'a str] {
    type Storage = PtrArray<'a, *const c_char, &'a str>;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *const *const c_char, Self> {
        let mut tmp_vec: Vec<_> =
            self.into_iter().map(|v| v.to_glib_none()).collect();
        let mut ptr_vec: Vec<_> =
            tmp_vec.iter_mut().map(|v| v.0).collect();
        unsafe {
            let zero = mem::zeroed();
            ptr_vec.push(zero);
        }
        Stash(ptr_vec.as_ptr(), PtrArray(ptr_vec, tmp_vec))
    }
}

impl<'a, P: Ptr, T: ToGlibPtr<'a, P>> ToGlibPtr<'a, *mut P> for &'a [T] {
    type Storage = PtrArray<'a, P, T>;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut P, Self> {
        let mut tmp_vec: Vec<_> =
            self.into_iter().map(|v| v.to_glib_none()).collect();
        let mut ptr_vec: Vec<_> =
            tmp_vec.iter_mut().map(|v| v.0).collect();
        unsafe {
            let zero = mem::zeroed();
            ptr_vec.push(zero);
        }
        Stash(ptr_vec.as_mut_ptr(), PtrArray(ptr_vec, tmp_vec))
    }
}

/// Temporary storage for passing a `NULL` terminated array of pointers.
pub struct PtrArray<'a, P: Ptr, T: ?Sized + ToGlibPtr<'a, P>> (Vec<P>, Vec<Stash<'a, P, T>>);

impl<'a, P: Ptr, T: ToGlibPtr<'a, P> + 'a> PtrArray<'a, P, T> {
    /// Returns the length of the array not counting the `NULL` terminator.
    pub fn len(&self) -> usize {
        self.1.len()
    }
}

impl<'a> ToGlibPtr<'a, *mut glib_ffi::GHashTable> for HashMap<String, String> {
    type Storage = (HashTable);

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut glib_ffi::GHashTable, Self> {
        let ptr = self.to_glib_full();
        Stash(ptr, HashTable(ptr))
    }

    #[inline]
    fn to_glib_full(&self) -> *mut glib_ffi::GHashTable {
        unsafe {
            let ptr = glib_ffi::g_hash_table_new_full(Some(glib_ffi::g_str_hash),
                Some(glib_ffi::g_str_equal), Some(glib_ffi::g_free), Some(glib_ffi::g_free));
            for (k, v) in self {
                glib_ffi::g_hash_table_insert(ptr, k.to_glib_full() as *mut _,
                    v.to_glib_full() as *mut _);
            }
            ptr
        }
    }
}

pub struct HashTable(*mut glib_ffi::GHashTable);

impl Drop for HashTable {
    fn drop(&mut self) {
        unsafe { glib_ffi::g_hash_table_unref(self.0) }
    }
}

/// Translate a simple type.
pub trait FromGlib<T>: Sized {
    fn from_glib(val: T) -> Self;
}

/// Translate a simple type.
#[inline]
pub fn from_glib<G, T: FromGlib<G>>(val: G) -> T {
    FromGlib::from_glib(val)
}

impl FromGlib<glib_ffi::gboolean> for bool {
    #[inline]
    fn from_glib(val: glib_ffi::gboolean) -> bool {
        !(val == glib_ffi::GFALSE)
    }
}

impl FromGlib<i32> for Option<u32> {
    #[inline]
    fn from_glib(val: i32) -> Option<u32> {
        if val >= 0 {
            Some(val as u32)
        }
        else {
            None
        }
    }
}

impl FromGlib<i64> for Option<u64> {
    #[inline]
    fn from_glib(val: i64) -> Option<u64> {
        if val >= 0 {
            Some(val as u64)
        }
        else {
            None
        }
    }
}

impl FromGlib<i32> for Option<u64> {
    #[inline]
    fn from_glib(val: i32) -> Option<u64> {
        FromGlib::from_glib(val as i64)
    }
}

/// Translate from a pointer type.
pub trait FromGlibPtr<P: Ptr>: Sized {
    /// Transfer: none.
    unsafe fn from_glib_none(ptr: P) -> Self;

    /// Transfer: full.
    unsafe fn from_glib_full(ptr: P) -> Self;
}

/// Translate from a pointer type, transfer: none.
#[inline]
pub unsafe fn from_glib_none<P: Ptr, T: FromGlibPtr<P>>(ptr: P) -> T {
    FromGlibPtr::from_glib_none(ptr)
}

/// Translate from a pointer type, transfer: full (assume ownership).
#[inline]
pub unsafe fn from_glib_full<P: Ptr, T: FromGlibPtr<P>>(ptr: P) -> T {
    FromGlibPtr::from_glib_full(ptr)
}

impl<P: Ptr, T: FromGlibPtr<P>> FromGlibPtr<P> for Option<T> {
    #[inline]
    unsafe fn from_glib_none(ptr: P) -> Option<T> {
        if ptr.is_null() { None }
        else { Some(from_glib_none(ptr)) }
    }

    #[inline]
    unsafe fn from_glib_full(ptr: P) -> Option<T> {
        if ptr.is_null() { None }
        else { Some(from_glib_full(ptr)) }
    }
}

impl FromGlibPtr<*const c_char> for String {
    #[inline]
    unsafe fn from_glib_none(ptr: *const c_char) -> Self {
        assert!(!ptr.is_null());
        String::from_utf8_lossy(CStr::from_ptr(ptr).to_bytes()).into_owned()
    }

    #[inline]
    unsafe fn from_glib_full(ptr: *const c_char) -> Self {
        let res = from_glib_none(ptr);
        glib_ffi::g_free(ptr as *mut _);
        res
    }
}

impl FromGlibPtr<*mut c_char> for String {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut c_char) -> Self {
        assert!(!ptr.is_null());
        String::from_utf8_lossy(CStr::from_ptr(ptr).to_bytes()).into_owned()
    }

    #[inline]
    unsafe fn from_glib_full(ptr: *mut c_char) -> Self {
        let res = from_glib_none(ptr);
        glib_ffi::g_free(ptr as *mut _);
        res
    }
}

/// Translate from a container of pointers.
pub trait FromGlibPtrContainer<P: Ptr, PP: Ptr>: Sized {
    /// Transfer: none.
    unsafe fn from_glib_none(ptr: PP) -> Self;

    /// Transfer: none.
    ///
    /// `num` is the advised number of elements.
    unsafe fn from_glib_none_num(ptr: PP, num: usize) -> Self;

    /// Transfer: container.
    unsafe fn from_glib_container(ptr: PP) -> Self;

    /// Transfer: container.
    ///
    /// `num` is the advised number of elements.
    unsafe fn from_glib_container_num(ptr: PP, num: usize) -> Self;

    /// Transfer: full.
    unsafe fn from_glib_full(ptr: PP) -> Self;

    /// Transfer: full.
    ///
    /// `num` is the advised number of elements.
    unsafe fn from_glib_full_num(ptr: PP, num: usize) -> Self;
}

unsafe fn c_array_len<P: Ptr>(mut ptr: *const P) -> usize {
    let mut len = 0;

    if !ptr.is_null() {
        while !(*ptr).is_null() {
            len += 1;
            ptr = ptr.offset(1);
        }
    }
    len
}

impl <P: Ptr, T: FromGlibPtr<P>>
FromGlibPtrContainer<P, *const P>
for Vec<T> {
    unsafe fn from_glib_none(ptr: *const P) -> Vec<T> {
        let num = c_array_len(ptr);
        Vec::from_glib_none_num(ptr, num)
    }

    unsafe fn from_glib_none_num(mut ptr: *const P, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let mut res = Vec::with_capacity(num);
        while !(*ptr).is_null() {
            res.push(from_glib_none(*ptr));
            ptr = ptr.offset(1);
        }
        res
    }

    unsafe fn from_glib_container(ptr: *const P) -> Vec<T> {
        let num = c_array_len(ptr);
        FromGlibPtrContainer::from_glib_container_num(ptr, num)
    }

    unsafe fn from_glib_container_num(ptr: *const P, num: usize) -> Vec<T> {
        let res = FromGlibPtrContainer::from_glib_none_num(ptr, num);
        glib_ffi::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full(ptr: *const P) -> Vec<T> {
        let num = c_array_len(ptr);
        FromGlibPtrContainer::from_glib_full_num(ptr, num)
    }

    unsafe fn from_glib_full_num(mut ptr: *const P, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let mut res = Vec::with_capacity(num);
        while !(*ptr).is_null() {
            res.push(from_glib_full(*ptr));
            ptr = ptr.offset(1);
        }
        glib_ffi::g_free(ptr as *mut _);
        res
    }
}

impl <P: Ptr, T: FromGlibPtr<P>>
FromGlibPtrContainer<P, *mut P>
for Vec<T> {
    unsafe fn from_glib_none(ptr: *mut P) -> Vec<T> {
        FromGlibPtrContainer::from_glib_none(ptr as *const P)
    }

    unsafe fn from_glib_none_num(ptr: *mut P, num: usize) -> Vec<T> {
        FromGlibPtrContainer::from_glib_none_num(ptr as *const P, num)
    }

    unsafe fn from_glib_container(ptr: *mut P) -> Vec<T> {
        FromGlibPtrContainer::from_glib_container(ptr as *const P)
    }

    unsafe fn from_glib_container_num(ptr: *mut P, num: usize) -> Vec<T> {
        FromGlibPtrContainer::from_glib_container_num(ptr as *const P, num)
    }

    unsafe fn from_glib_full(ptr: *mut P) -> Vec<T> {
        FromGlibPtrContainer::from_glib_full(ptr as *const P)
    }

    unsafe fn from_glib_full_num(ptr: *mut P, num: usize) -> Vec<T> {
        FromGlibPtrContainer::from_glib_full_num(ptr as *const P, num)
    }
}

unsafe fn slist_len(mut ptr: *mut glib_ffi::GSList) -> usize {
    let mut len = 0;
    while !ptr.is_null() {
        ptr = (*ptr).next;
        len += 1;
    }
    len
}

impl <P: Ptr, T: FromGlibPtr<P>> FromGlibPtrContainer<P, *mut glib_ffi::GSList> for Vec<T> {
    unsafe fn from_glib_none(ptr: *mut glib_ffi::GSList) -> Vec<T> {
        let num = slist_len(ptr);
        FromGlibPtrContainer::from_glib_none_num(ptr, num)
    }

    unsafe fn from_glib_none_num(mut ptr: *mut glib_ffi::GSList, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let mut res = Vec::with_capacity(num);
        while !ptr.is_null() {
            let item_ptr: P = Ptr::from((*ptr).data);
            if !item_ptr.is_null() {
                res.push(from_glib_none(item_ptr));
            }
            ptr = (*ptr).next;
        }
        res
    }

    unsafe fn from_glib_container(ptr: *mut glib_ffi::GSList) -> Vec<T> {
        let num = slist_len(ptr);
        FromGlibPtrContainer::from_glib_container_num(ptr, num)
    }

    unsafe fn from_glib_container_num(ptr: *mut glib_ffi::GSList, num: usize) -> Vec<T> {
        let res = FromGlibPtrContainer::from_glib_none_num(ptr, num);
        if !ptr.is_null() {
            glib_ffi::g_slist_free(ptr as *mut _);
        }
        res
    }

    unsafe fn from_glib_full(ptr: *mut glib_ffi::GSList) -> Vec<T> {
        let num = slist_len(ptr);
        FromGlibPtrContainer::from_glib_container_num(ptr, num)
    }

    unsafe fn from_glib_full_num(mut ptr: *mut glib_ffi::GSList, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let orig_ptr = ptr;
        let mut res = Vec::with_capacity(num);
        while !ptr.is_null() {
            let item_ptr: P = Ptr::from((*ptr).data);
            if !item_ptr.is_null() {
                res.push(from_glib_full(item_ptr));
            }
            ptr = (*ptr).next;
        }
        if !orig_ptr.is_null() {
            glib_ffi::g_slist_free(orig_ptr as *mut _);
        }
        res
    }
}

unsafe fn list_len(mut ptr: *mut glib_ffi::GList) -> usize {
    let mut len = 0;
    while !ptr.is_null() {
        ptr = (*ptr).next;
        len += 1;
    }
    len
}

impl <P: Ptr, T: FromGlibPtr<P>> FromGlibPtrContainer<P, *mut glib_ffi::GList> for Vec<T> {
    unsafe fn from_glib_none(ptr: *mut glib_ffi::GList) -> Vec<T> {
        let num = list_len(ptr);
        FromGlibPtrContainer::from_glib_none_num(ptr, num)
    }

    unsafe fn from_glib_none_num(mut ptr: *mut glib_ffi::GList, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let mut res = Vec::with_capacity(num);
        while !ptr.is_null() {
            let item_ptr: P = Ptr::from((*ptr).data);
            if !item_ptr.is_null() {
                res.push(from_glib_none(item_ptr));
            }
            ptr = (*ptr).next;
        }
        res
    }

    unsafe fn from_glib_container(ptr: *mut glib_ffi::GList) -> Vec<T> {
        let num = list_len(ptr);
        FromGlibPtrContainer::from_glib_container_num(ptr, num)
    }

    unsafe fn from_glib_container_num(ptr: *mut glib_ffi::GList, num: usize) -> Vec<T> {
        let res = FromGlibPtrContainer::from_glib_none_num(ptr, num);
        if !ptr.is_null() {
            glib_ffi::g_list_free(ptr as *mut _);
        }
        res
    }

    unsafe fn from_glib_full(ptr: *mut glib_ffi::GList) -> Vec<T> {
        let num = list_len(ptr);
        FromGlibPtrContainer::from_glib_container_num(ptr, num)
    }

    unsafe fn from_glib_full_num(mut ptr: *mut glib_ffi::GList, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let orig_ptr = ptr;
        let mut res = Vec::with_capacity(num);
        while !ptr.is_null() {
            let mut item_ptr: P = mem::uninitialized();
            // item_ptr is a pointer but the compiler doesn't know
            let hack: *mut *mut c_void = mem::transmute(&mut item_ptr);
            *hack = (*ptr).data;
            if !item_ptr.is_null() {
                res.push(from_glib_full(item_ptr));
            }
            ptr = (*ptr).next;
        }
        if !orig_ptr.is_null() {
            glib_ffi::g_list_free(orig_ptr as *mut _);
        }
        res
    }
}

unsafe extern "C" fn read_string_hash_table(key: glib_ffi::gpointer, value: glib_ffi::gpointer,
                                            hash_map: glib_ffi::gpointer) {
    let key: String = from_glib_none(key as *const c_char);
    let value: String = from_glib_none(value as *const c_char);
    let hash_map: &mut HashMap<String, String> = mem::transmute(hash_map);
    hash_map.insert(key, value);
}

impl FromGlibPtrContainer<*const c_char, *mut glib_ffi::GHashTable> for HashMap<String, String> {
    unsafe fn from_glib_none(ptr: *mut glib_ffi::GHashTable) -> Self {
        let mut map = HashMap::new();
        glib_ffi::g_hash_table_foreach(ptr, Some(read_string_hash_table), mem::transmute(&mut map));
        map
    }

    unsafe fn from_glib_none_num(ptr: *mut glib_ffi::GHashTable, _: usize) -> Self {
        FromGlibPtrContainer::from_glib_none(ptr)
    }

    unsafe fn from_glib_container(ptr: *mut glib_ffi::GHashTable) -> Self {
        FromGlibPtrContainer::from_glib_full(ptr)
    }

    unsafe fn from_glib_container_num(ptr: *mut glib_ffi::GHashTable, _: usize) -> Self {
        FromGlibPtrContainer::from_glib_full(ptr)
    }

    unsafe fn from_glib_full(ptr: *mut glib_ffi::GHashTable) -> Self {
        let map = FromGlibPtrContainer::from_glib_none(ptr);
        glib_ffi::g_hash_table_unref(ptr);
        map
    }

    unsafe fn from_glib_full_num(ptr: *mut glib_ffi::GHashTable, _: usize) -> Self {
        FromGlibPtrContainer::from_glib_full(ptr)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use glib_ffi;
    use super::*;

    #[test]
    fn string_hash_map() {
        let mut map = HashMap::new();
        map.insert("A".into(), "1".into());
        map.insert("B".into(), "2".into());
        map.insert("C".into(), "3".into());
        let ptr: *mut glib_ffi::GHashTable = map.to_glib_full();
        let map = unsafe { HashMap::from_glib_full(ptr) };
        assert_eq!(map.get("A"), Some(&"1".into()));
        assert_eq!(map.get("B"), Some(&"2".into()));
        assert_eq!(map.get("C"), Some(&"3".into()));
    }
}

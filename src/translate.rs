// Copyright 2015, The Rust-GNOME Project Developers.
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
//!         unsafe { ffi::gdk_window_set_accept_focus(self.pointer, accept_focus.to_glib()) }
//!     }
//!
//!     pub fn get_accept_focus(&self) -> bool {
//!         unsafe { from_glib(ffi::gdk_window_get_accept_focus(self.pointer)) }
//!     }
//! ```
//!
//! `FromGlibPtr` (`from_glib_none` and `from_glib_full`) and `ToGlibPtr` work on `gpointer`s
//! and support different modes of ownership transfer.
//!
//! ```ignore
//!     fn get_title(&self) -> Option<String> {
//!         unsafe {
//!             let title = ffi::gtk_window_get_title(self.pointer);
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
//!             ffi::gdk_window_set_icon_name(self.pointer, name.to_glib_none().0)
//!         }
//!     }
//! ```

use std::iter::IntoIterator;
use std::ffi::{CString, CStr};
use std::mem;
use std::ptr;
use libc::{c_void, c_char};
use ffi;

/// A pointer
pub trait Ptr: Copy {
    fn is_null(&self) -> bool;
    fn from<X>(ptr: *mut X) -> Self;
}

impl <T> Ptr for *const T {
    #[inline]
    fn is_null(&self) -> bool { (*self).is_null() }

    #[inline]
    fn from<X>(ptr: *mut X) -> *const T { ptr as *const T }
}

impl <T> Ptr for *mut T {
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
/// impl <'a> ToGlibPtr<'a, *mut ffi::GdkWindowAttr> for WindowAttr {
///     type Storage = (Box<ffi::GdkWindowAttr>, Stash<'a, *const c_char, Option<String>>);
///
///     fn to_glib_none(&'a self) -> Stash<*mut ffi::GdkWindowAttr, WindowAttr> {
///         let title = self.title.to_glib_none();
///
///         let mut attrs = Box::new(ffi::GdkWindowAttr {
///             title: title.0,
///             // ....
///         });
///
///         Stash(&mut *attrs, (attrs, title))
///     }
/// }
/// ```
pub struct Stash<'a, P: Copy, T: ?Sized + ToGlibPtr<'a, P>> (pub P, pub <T as ToGlibPtr<'a, P>>::Storage);

/// A `Stash` for iterators.
pub struct IterStash<'a, P: Copy, T: ?Sized + IterToGlibPtr<'a, P>> (pub P, pub <T as IterToGlibPtr<'a, P>>::Storage);

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
    type GlibType = ffi::Gboolean;

    #[inline]
    fn to_glib(&self) -> ffi::Gboolean {
        if *self { ffi::GTRUE } else { ffi::GFALSE }
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

impl <'a> ToGlibPtr<'a, *const c_char> for str {
    type Storage = CString;

    fn to_glib_none(&self) -> Stash<'a, *const c_char, str> {
        let tmp = CString::new(self).unwrap();
        Stash(tmp.as_ptr(), tmp)
    }
}

impl <'a> ToGlibPtr<'a, *const c_char> for String {
    type Storage = CString;

    fn to_glib_none(&self) -> Stash<'a, *const c_char, String> {
        let tmp = CString::new(&self[..]).unwrap();
        Stash(tmp.as_ptr(), tmp)
    }
}

impl <'a, S: AsRef<str>> ToGlibPtr<'a, *const c_char> for Option<S> {
    type Storage = Option<CString>;

    fn to_glib_none(&self) -> Stash<'a, *const c_char, Option<S>> {
        let tmp = match self {
            &Some(ref s) => Some(CString::new(s.as_ref()).unwrap()),
            &None => None,
        };
        let ptr = tmp.as_ref().map_or(ptr::null(), |s| s.as_ptr());
        Stash(ptr, tmp)
    }
}

/// Translate an iterator to a pointer.
///
/// See `ToGlibPtr`.
pub trait IterToGlibPtr<'a, P: Copy> {
    type Storage;

    /// Transfer: none.
    fn to_glib_none(&'a self) -> IterStash<P, Self>;

    /// Transfer: container.
    ///
    /// Only give away the container ownership.
    fn to_glib_container(&'a self) -> IterStash<P, Self> {
        unimplemented!();
    }

    /// Transfer: full.
    ///
    /// We transfer the ownership to the foreign library.
    fn to_glib_full(&'a self) -> P {
        unimplemented!();
    }
}

impl <'a, S: AsRef<str>, I: ?Sized> IterToGlibPtr<'a, *const *const c_char> for I
where &'a I: IntoIterator<Item = &'a S> {
    type Storage = PtrArray<'a, *const c_char, str>;

    fn to_glib_none(&'a self) -> IterStash<*const *const c_char, I> {
        let mut tmp_vec: Vec<_> =
            self.into_iter().map(|v| AsRef::<str>::as_ref(v).to_glib_none()).collect();
        let mut ptr_vec: Vec<_> =
            tmp_vec.iter_mut().map(|v| v.0).collect();
        unsafe {
            let zero = mem::zeroed();
            ptr_vec.push(zero);
        }
        IterStash(ptr_vec.as_ptr(), PtrArray(ptr_vec, tmp_vec))
    }
}

impl <'a, P: Copy, T, I: ?Sized> IterToGlibPtr<'a, *mut P> for I
where T: ToGlibPtr<'a, P>, &'a I: IntoIterator<Item = &'a T> {
    type Storage = PtrArray<'a, P, T>;

    fn to_glib_none(&'a self) -> IterStash<*mut P, I> {
        let mut tmp_vec: Vec<_> =
            self.into_iter().map(|v| v.to_glib_none()).collect();
        let mut ptr_vec: Vec<_> =
            tmp_vec.iter_mut().map(|v| v.0).collect();
        unsafe {
            let zero = mem::zeroed();
            ptr_vec.push(zero);
        }
        IterStash(ptr_vec.as_mut_ptr(), PtrArray(ptr_vec, tmp_vec))
    }
}

/// Temporary storage for passing a `NULL` terminated array of pointers.
pub struct PtrArray<'a, P: Copy, T: ?Sized + ToGlibPtr<'a, P>> (Vec<P>, Vec<Stash<'a, P, T>>);

impl <'a, P: Copy, T: ToGlibPtr<'a, P>> PtrArray<'a, P, T> {
    /// Returns the length of the array not counting the `NULL` terminator.
    pub fn len(&self) -> usize {
        self.1.len()
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

impl FromGlib<ffi::Gboolean> for bool {
    #[inline]
    fn from_glib(val: ffi::Gboolean) -> bool {
        !(val == ffi::GFALSE)
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
        ffi::g_free(ptr as *mut _);
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
        ffi::g_free(ptr as *mut _);
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

    unsafe fn from_glib_none_num(mut ptr: *const P,
                         num: usize) -> Vec<T> {
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

    unsafe fn from_glib_container_num(ptr: *const P,
                             num: usize) -> Vec<T> {
        let res = FromGlibPtrContainer::from_glib_none_num(ptr, num);
        ffi::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full(ptr: *const P) -> Vec<T> {
        let num = c_array_len(ptr);
        FromGlibPtrContainer::from_glib_full_num(ptr, num)
    }

    unsafe fn from_glib_full_num(mut ptr: *const P,
                       num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let mut res = Vec::with_capacity(num);
        while !(*ptr).is_null() {
            res.push(from_glib_full(*ptr));
            ptr = ptr.offset(1);
        }
        ffi::g_free(ptr as *mut _);
        res
    }
}

unsafe fn slist_len(mut ptr: *mut ffi::GSList) -> usize {
    let mut len = 0;
    while !ptr.is_null() {
        ptr = (*ptr).next;
        len += 1;
    }
    len
}

impl <P: Ptr, T: FromGlibPtr<P>> FromGlibPtrContainer<P, *mut ffi::GSList> for Vec<T> {
    unsafe fn from_glib_none(ptr: *mut ffi::GSList) -> Vec<T> {
        let num = slist_len(ptr);
        FromGlibPtrContainer::from_glib_none_num(ptr, num)
    }

    unsafe fn from_glib_none_num(mut ptr: *mut ffi::GSList, num: usize) -> Vec<T> {
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

    unsafe fn from_glib_container(ptr: *mut ffi::GSList) -> Vec<T> {
        let num = slist_len(ptr);
        FromGlibPtrContainer::from_glib_container_num(ptr, num)
    }

    unsafe fn from_glib_container_num(ptr: *mut ffi::GSList, num: usize) -> Vec<T> {
        let res = FromGlibPtrContainer::from_glib_none_num(ptr, num);
        if !ptr.is_null() {
            ffi::g_slist_free(ptr as *mut _);
        }
        res
    }

    unsafe fn from_glib_full(ptr: *mut ffi::GSList) -> Vec<T> {
        let num = slist_len(ptr);
        FromGlibPtrContainer::from_glib_container_num(ptr, num)
    }

    unsafe fn from_glib_full_num(mut ptr: *mut ffi::GSList, num: usize) -> Vec<T> {
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
            ffi::g_slist_free(orig_ptr as *mut _);
        }
        res
    }
}

unsafe fn list_len(mut ptr: *mut ffi::GList) -> usize {
    let mut len = 0;
    while !ptr.is_null() {
        ptr = (*ptr).next;
        len += 1;
    }
    len
}

impl <P: Ptr, T: FromGlibPtr<P>> FromGlibPtrContainer<P, *mut ffi::GList> for Vec<T> {
    unsafe fn from_glib_none(ptr: *mut ffi::GList) -> Vec<T> {
        let num = list_len(ptr);
        FromGlibPtrContainer::from_glib_none_num(ptr, num)
    }

    unsafe fn from_glib_none_num(mut ptr: *mut ffi::GList, num: usize) -> Vec<T> {
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

    unsafe fn from_glib_container(ptr: *mut ffi::GList) -> Vec<T> {
        let num = list_len(ptr);
        FromGlibPtrContainer::from_glib_container_num(ptr, num)
    }

    unsafe fn from_glib_container_num(ptr: *mut ffi::GList, num: usize) -> Vec<T> {
        let res = FromGlibPtrContainer::from_glib_none_num(ptr, num);
        if !ptr.is_null() {
            ffi::g_list_free(ptr as *mut _);
        }
        res
    }

    unsafe fn from_glib_full(ptr: *mut ffi::GList) -> Vec<T> {
        let num = list_len(ptr);
        FromGlibPtrContainer::from_glib_container_num(ptr, num)
    }

    unsafe fn from_glib_full_num(mut ptr: *mut ffi::GList, num: usize) -> Vec<T> {
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
            ffi::g_list_free(orig_ptr as *mut _);
        }
        res
    }
}

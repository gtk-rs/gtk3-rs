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
//! `ToGlibPtr`, `FromGlibPtrNone`, `FromGlibPtrFull` and `FromGlibPtrBorrow` work on `gpointer`s
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

use std::char;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ffi::{CString, CStr};
#[cfg(not(windows))]
use std::ffi::OsString;
use std::mem;
#[cfg(not(windows))]
use std::os::unix::prelude::*;
use std::path::{Path, PathBuf};
use std::ptr;
use libc::{c_char, size_t};
use ffi as glib_ffi;

/// A pointer
pub trait Ptr: Copy + 'static {
    fn is_null(&self) -> bool;
    fn from<X>(ptr: *mut X) -> Self;
    fn to<X>(self) -> *mut X;
}

impl<T: 'static> Ptr for *const T {
    #[inline]
    fn is_null(&self) -> bool { (*self).is_null() }

    #[inline]
    fn from<X>(ptr: *mut X) -> *const T { ptr as *const T }

    #[inline]
    fn to<X>(self) -> *mut X { self as *mut X }
}

impl<T: 'static> Ptr for *mut T {
    #[inline]
    fn is_null(&self) -> bool { (*self).is_null() }

    #[inline]
    fn from<X>(ptr: *mut X) -> *mut T { ptr as *mut T }

    #[inline]
    fn to<X>(self) -> *mut X { self as *mut X }
}

/// Overrides pointer mutability.
///
/// Use when the C API should be specifying a const pointer but doesn't.
pub fn mut_override<T>(ptr: *const T) -> *mut T {
    ptr as *mut T
}

/// Overrides pointer constness.
///
/// Use when the C API need const pointer, but function with `IsA<T>` constraint,
/// that usaly don't have const pointer conversion.
pub fn const_override<T>(ptr: *mut T) -> *const T {
    ptr as *const T
}

/// A trait for creating an uninitialized value. Handy for receiving outparams.
pub trait Uninitialized {
    /// Returns an uninitialized value.
    unsafe fn uninitialized() -> Self;
}

/// Returns an uninitialized value.
#[inline]
pub unsafe fn uninitialized<T: Uninitialized>() -> T {
    T::uninitialized()
}

pub trait ToBool: Copy {
    fn to_bool(self) -> bool;
}

impl ToBool for bool {
    #[inline]
    fn to_bool(self) -> bool {
        self
    }
}

impl ToBool for glib_ffi::gboolean {
    #[inline]
    fn to_bool(self) -> bool {
        !(self == glib_ffi::GFALSE)
    }
}

/// Returns `Some(val)` if the condition is true and `None` otherwise.
#[inline]
pub fn some_if<B: ToBool, T, F: FnOnce() -> T>(cond: B, f: F) -> Option<T> {
    if cond.to_bool() {
        Some(f())
    }
    else {
        None
    }
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

impl ToGlib for char {
    type GlibType = u32;

    #[inline]
    fn to_glib(&self) -> u32 {
        *self as u32
    }
}

impl ToGlib for Option<char> {
    type GlibType = u32;

    #[inline]
    fn to_glib(&self) -> u32 {
        self.as_ref().map(|&c| c as u32).unwrap_or(0)
    }
}

impl ToGlib for Ordering {
    type GlibType = i32;

    #[inline]
    fn to_glib(&self) -> i32 {
        match *self {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
}

/// Provides the default pointer type to be used in some container conversions.
///
/// It's `*mut c_char` for `String`, `*mut GtkButton` for `gtk::Button`, etc.
pub trait GlibPtrDefault {
    type GlibType: Ptr;
}

impl<'a, T: ?Sized + GlibPtrDefault> GlibPtrDefault for &'a T {
    type GlibType = <T as GlibPtrDefault>::GlibType;
}

/// Translate to a pointer.
pub trait ToGlibPtr<'a, P: Copy> {
    type Storage;

    /// Transfer: none.
    ///
    /// The pointer in the `Stash` is only valid for the lifetime of the `Stash`.
    fn to_glib_none(&'a self) -> Stash<'a, P, Self>;

    /// Transfer: container.
    ///
    /// We transfer the container ownership to the foreign library retaining
    /// the elements ownership.
    fn to_glib_container(&'a self) -> Stash<'a, P, Self> {
        unimplemented!();
    }

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
}

impl<'a, P: Ptr, T: ToGlibPtr<'a, P>> ToGlibPtr<'a, P> for Option<T> {
    type Storage = Option<<T as ToGlibPtr<'a, P>>::Storage>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, P, Option<T>> {
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

impl <'a, 'opt: 'a, P: Ptr, T: ToGlibPtrMut<'a, P>> ToGlibPtrMut<'a, P> for Option<&'opt mut T> {
    type Storage = Option<<T as ToGlibPtrMut<'a, P>>::Storage>;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, P, Option<&'opt mut T>> {
        self.as_mut().map_or(StashMut(Ptr::from::<()>(ptr::null_mut()), None), |s| {
            let s = s.to_glib_none_mut();
            StashMut(s.0, Some(s.1))
        })
    }
}

impl<'a, P: Ptr, T: ?Sized + ToGlibPtr<'a, P>> ToGlibPtr<'a, P> for &'a T {
    type Storage = <T as ToGlibPtr<'a, P>>::Storage;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, P, Self> {
        let s = (*self).to_glib_none();
        Stash(s.0, s.1)
    }

    #[inline]
    fn to_glib_full(&self) -> P {
        (*self).to_glib_full()
    }
}

impl<'a> ToGlibPtr<'a, *const c_char> for str {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const c_char, Self> {
        let tmp = CString::new(self).unwrap();
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

impl<'a> ToGlibPtr<'a, *mut c_char> for str {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut c_char, Self> {
        let tmp = CString::new(self).unwrap();
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

impl <'a> ToGlibPtr<'a, *mut c_char> for String {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut c_char, String> {
        let tmp = CString::new(&self[..]).unwrap();
        Stash(tmp.as_ptr() as *mut c_char, tmp)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut c_char {
        unsafe {
            glib_ffi::g_strndup(self.as_ptr() as *const c_char, self.len() as size_t)
                as *mut c_char
        }
    }
}

impl GlibPtrDefault for str {
    type GlibType = *mut c_char;
}

impl GlibPtrDefault for String {
    type GlibType = *mut c_char;
}

#[cfg(not(windows))]
fn path_to_c(path: &Path) -> CString {
    // GLib paths on UNIX are always in the local encoding, just like in Rust
    //
    // Paths on UNIX must not contain NUL bytes, in which case the conversion
    // to a CString would fail. The only thing we can do then is to panic, as passing
    // NULL or the empty string to GLib would cause undefined behaviour.
    use std::os::unix::ffi::OsStrExt;
    CString::new(path.as_os_str().as_bytes())
        .expect("Invalid path with NUL bytes")
}

#[cfg(windows)]
fn path_to_c(path: &Path) -> CString {
    // GLib paths are always UTF-8 strings on Windows, while in Rust they are
    // WTF-8. As such, we need to convert to a UTF-8 string. This conversion can
    // fail, see https://simonsapin.github.io/wtf-8/#converting-wtf-8-utf-8
    //
    // It's not clear what we're supposed to do if it fails: the path is not
    // representable in UTF-8 and thus can't possibly be passed to GLib.
    // Passing NULL or the empty string to GLib can lead to undefined behaviour, so
    // the only safe option seems to be to simply panic here.
    let path_str = path.to_str()
        .expect("Path can't be represented as UTF-8")
        .to_owned();

    // On Windows, paths can have \\?\ prepended for long-path support. See
    // MSDN documentation about CreateFile
    //
    // We have to get rid of this and let GLib take care of all these
    // weirdnesses later
    if path_str.starts_with("\\\\?\\") {
        CString::new(path_str[4..].as_bytes())
    } else {
        CString::new(path_str.as_bytes())
    }.expect("Invalid path with NUL bytes")
}

impl<'a> ToGlibPtr<'a, *const c_char> for Path {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const c_char, Self> {
        let tmp = path_to_c(self);
        Stash(tmp.as_ptr(), tmp)
    }
}

impl<'a> ToGlibPtr<'a, *mut c_char> for Path {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut c_char, Self> {
        let tmp = path_to_c(self);
        Stash(tmp.as_ptr() as *mut c_char, tmp)
    }
}

impl<'a> ToGlibPtr<'a, *const c_char> for PathBuf {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const c_char, Self> {
        let tmp = path_to_c(self);
        Stash(tmp.as_ptr(), tmp)
    }
}

impl<'a> ToGlibPtr<'a, *mut c_char> for PathBuf {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut c_char, Self> {
        let tmp = path_to_c(self);
        Stash(tmp.as_ptr() as *mut c_char, tmp)
    }
}

impl GlibPtrDefault for Path {
    type GlibType = *mut c_char;
}

impl GlibPtrDefault for PathBuf {
    type GlibType = *mut c_char;
}

pub trait ToGlibContainerFromSlice<'a, P>
where Self: Sized {
    type Storage;

    fn to_glib_none_from_slice(t: &'a [Self]) -> (P, Self::Storage);
    fn to_glib_container_from_slice(t: &'a [Self]) -> (P, Self::Storage);
    fn to_glib_full_from_slice(t: &[Self]) -> P;
}

macro_rules! impl_to_glib_container_from_slice_fundamental {
    ($name:ty) => {
        impl<'a> ToGlibContainerFromSlice<'a, *mut $name> for $name {
            type Storage = &'a [$name];

            fn to_glib_none_from_slice(t: &'a [$name]) -> (*mut $name, &'a [$name]) {
                (t.as_ptr() as *mut $name, t)
            }

            fn to_glib_container_from_slice(t: &'a [$name]) -> (*mut $name, &'a [$name]) {
                (ToGlibContainerFromSlice::to_glib_full_from_slice(t), t)
            }

            fn to_glib_full_from_slice(t: &[$name]) -> *mut $name {
                if t.len() == 0 {
                    return ptr::null_mut();
                }

                unsafe {
                    let res = glib_ffi::g_malloc(mem::size_of::<$name>() * t.len()) as *mut $name;
                    ptr::copy_nonoverlapping(t.as_ptr(), res, t.len());
                    res
                }
            }
        }
    }
}

impl_to_glib_container_from_slice_fundamental!(u8);
impl_to_glib_container_from_slice_fundamental!(i8);
impl_to_glib_container_from_slice_fundamental!(u16);
impl_to_glib_container_from_slice_fundamental!(i16);
impl_to_glib_container_from_slice_fundamental!(u32);
impl_to_glib_container_from_slice_fundamental!(i32);
impl_to_glib_container_from_slice_fundamental!(u64);
impl_to_glib_container_from_slice_fundamental!(i64);
impl_to_glib_container_from_slice_fundamental!(f32);
impl_to_glib_container_from_slice_fundamental!(f64);

macro_rules! impl_to_glib_container_from_slice_string {
    ($name:ty, $ffi_name:ty) => {
        impl<'a> ToGlibContainerFromSlice<'a, *mut $ffi_name> for $name {
            type Storage = (Vec<Stash<'a, $ffi_name, $name>>, Option<Vec<$ffi_name>>);

            fn to_glib_none_from_slice(t: &'a [$name]) -> (*mut $ffi_name, Self::Storage) {
                let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();
                let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
                v_ptr.push(ptr::null_mut() as $ffi_name);

                (v_ptr.as_ptr() as *mut $ffi_name, (v, Some(v_ptr)))
            }

            fn to_glib_container_from_slice(t: &'a [$name]) -> (*mut $ffi_name, Self::Storage) {
                let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();

                let v_ptr = unsafe {
                    let v_ptr = glib_ffi::g_malloc0(mem::size_of::<$ffi_name>() * (t.len() + 1)) as *mut $ffi_name;

                    for (i, s) in v.iter().enumerate() {
                        ptr::write(v_ptr.offset(i as isize), s.0);
                    }

                    v_ptr
                };

                (v_ptr, (v, None))
            }

            fn to_glib_full_from_slice(t: &[$name]) -> *mut $ffi_name {
                unsafe {
                    let v_ptr = glib_ffi::g_malloc0(mem::size_of::<$ffi_name>() * (t.len() + 1)) as *mut $ffi_name;

                    for (i, s) in t.iter().enumerate() {
                        ptr::write(v_ptr.offset(i as isize), s.to_glib_full());
                    }

                    v_ptr
                }
            }
        }
    }
}

impl_to_glib_container_from_slice_string!(&'a str, *mut c_char);
impl_to_glib_container_from_slice_string!(&'a str, *const c_char);
impl_to_glib_container_from_slice_string!(String, *mut c_char);
impl_to_glib_container_from_slice_string!(String, *const c_char);
impl_to_glib_container_from_slice_string!(&'a Path, *mut c_char);
impl_to_glib_container_from_slice_string!(&'a Path, *const c_char);
impl_to_glib_container_from_slice_string!(PathBuf, *mut c_char);
impl_to_glib_container_from_slice_string!(PathBuf, *const c_char);

impl<'a, T> ToGlibContainerFromSlice<'a, *mut glib_ffi::GList> for T
where T: GlibPtrDefault + ToGlibPtr<'a, <T as GlibPtrDefault>::GlibType> {
    type Storage = (Option<List>, Vec<Stash<'a, <T as GlibPtrDefault>::GlibType, T>>);

    #[inline]
    fn to_glib_none_from_slice(t: &'a [T]) -> (*mut glib_ffi::GList, Self::Storage) {
        let stash_vec: Vec<_> =
            t.iter().rev().map(|v| v.to_glib_none()).collect();
        let mut list: *mut glib_ffi::GList = ptr::null_mut();
        unsafe {
            for stash in &stash_vec {
                list = glib_ffi::g_list_prepend(list, Ptr::to(stash.0));
            }
        }
        (list, (Some(List(list)), stash_vec))
    }

    #[inline]
    fn to_glib_container_from_slice(t: &'a [T]) -> (*mut glib_ffi::GList, Self::Storage) {
        let stash_vec: Vec<_> =
            t.iter().rev().map(|v| v.to_glib_none()).collect();
        let mut list: *mut glib_ffi::GList = ptr::null_mut();
        unsafe {
            for stash in &stash_vec {
                list = glib_ffi::g_list_prepend(list, Ptr::to(stash.0));
            }
        }
        (list, (None, stash_vec))
    }

    #[inline]
    fn to_glib_full_from_slice(t: &[T]) -> *mut glib_ffi::GList {
        let mut list: *mut glib_ffi::GList = ptr::null_mut();
        unsafe {
            for ptr in t.iter().rev().map(|v| v.to_glib_full()) {
                list = glib_ffi::g_list_prepend(list, Ptr::to(ptr));
            }
        }
        list
    }
}

impl<'a, T> ToGlibContainerFromSlice<'a, *const glib_ffi::GList> for T
where T: GlibPtrDefault + ToGlibPtr<'a, <T as GlibPtrDefault>::GlibType> {
    type Storage = (Option<List>, Vec<Stash<'a, <T as GlibPtrDefault>::GlibType, T>>);

    #[inline]
    fn to_glib_none_from_slice(t: &'a [T]) -> (*const glib_ffi::GList, Self::Storage) {
        let (list, stash) = ToGlibContainerFromSlice::<*mut glib_ffi::GList>::to_glib_none_from_slice(t);
        (list as *const glib_ffi::GList, stash)
    }

    #[inline]
    fn to_glib_container_from_slice(_t: &'a [T]) -> (*const glib_ffi::GList, Self::Storage) {
        unimplemented!()
    }

    #[inline]
    fn to_glib_full_from_slice(_t: &[T]) -> *const glib_ffi::GList {
        unimplemented!()
    }
}

pub struct List(*mut glib_ffi::GList);

impl Drop for List {
    fn drop(&mut self) {
        unsafe { glib_ffi::g_list_free(self.0) }
    }
}

impl<'a, T> ToGlibContainerFromSlice<'a, *mut glib_ffi::GSList> for &'a T
where T: GlibPtrDefault + ToGlibPtr<'a, <T as GlibPtrDefault>::GlibType> {
    type Storage = (Option<SList>, Vec<Stash<'a, <T as GlibPtrDefault>::GlibType, &'a T>>);

    #[inline]
    fn to_glib_none_from_slice(t: &'a [&'a T]) -> (*mut glib_ffi::GSList, Self::Storage) {
        let stash_vec: Vec<_> =
            t.iter().rev().map(|v| v.to_glib_none()).collect();
        let mut list: *mut glib_ffi::GSList = ptr::null_mut();
        unsafe {
            for stash in &stash_vec {
                list = glib_ffi::g_slist_prepend(list, Ptr::to(stash.0));
            }
        }
        (list, (Some(SList(list)), stash_vec))
    }

    #[inline]
    fn to_glib_container_from_slice(t: &'a [&'a T]) -> (*mut glib_ffi::GSList, Self::Storage) {
        let stash_vec: Vec<_> =
            t.iter().rev().map(|v| v.to_glib_none()).collect();
        let mut list: *mut glib_ffi::GSList = ptr::null_mut();
        unsafe {
            for stash in &stash_vec {
                list = glib_ffi::g_slist_prepend(list, Ptr::to(stash.0));
            }
        }
        (list, (None, stash_vec))
    }

    #[inline]
    fn to_glib_full_from_slice(t: &[&'a T]) -> *mut glib_ffi::GSList {
        let mut list: *mut glib_ffi::GSList = ptr::null_mut();
        unsafe {
            for ptr in t.iter().rev().map(|v| v.to_glib_full()) {
                list = glib_ffi::g_slist_prepend(list, Ptr::to(ptr));
            }
        }
        list
    }
}

impl<'a, T> ToGlibContainerFromSlice<'a, *const glib_ffi::GSList> for &'a T
where T: GlibPtrDefault + ToGlibPtr<'a, <T as GlibPtrDefault>::GlibType> {
    type Storage = (Option<SList>, Vec<Stash<'a, <T as GlibPtrDefault>::GlibType, &'a T>>);

    #[inline]
    fn to_glib_none_from_slice(t: &'a [&'a T]) -> (*const glib_ffi::GSList, Self::Storage) {
        let (list, stash) = ToGlibContainerFromSlice::<*mut glib_ffi::GSList>::to_glib_none_from_slice(t);
        (list as *const glib_ffi::GSList, stash)
    }

    #[inline]
    fn to_glib_container_from_slice(_t: &'a [&'a T]) -> (*const glib_ffi::GSList, Self::Storage) {
        unimplemented!()
    }

    #[inline]
    fn to_glib_full_from_slice(_t: &[&'a T]) -> *const glib_ffi::GSList {
        unimplemented!()
    }
}

pub struct SList(*mut glib_ffi::GSList);

impl Drop for SList {
    fn drop(&mut self) {
        unsafe { glib_ffi::g_slist_free(self.0) }
    }
}

impl<'a, P: Ptr, T: ToGlibContainerFromSlice<'a, P>> ToGlibPtr<'a, P> for [T] {
    type Storage = T::Storage;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, P, Self> {
        let result = ToGlibContainerFromSlice::to_glib_none_from_slice(self);
        Stash(result.0, result.1)
    }

    #[inline]
    fn to_glib_container(&'a self) -> Stash<'a, P, Self> {
        let result = ToGlibContainerFromSlice::to_glib_container_from_slice(self);
        Stash(result.0, result.1)
    }

    #[inline]
    fn to_glib_full(&self) -> P {
        ToGlibContainerFromSlice::to_glib_full_from_slice(self)
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
                let k: *mut c_char = k.to_glib_full();
                let v: *mut c_char = v.to_glib_full();
                glib_ffi::g_hash_table_insert(ptr, k as *mut _, v as *mut _);
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

impl FromGlib<u32> for char {
    #[inline]
    fn from_glib(val: u32) -> char {
        char::from_u32(val).expect("Valid Unicode character expected")
    }
}

impl FromGlib<i32> for Ordering {
    #[inline]
    fn from_glib(val: i32) -> Ordering {
        if val < 0 {
            Ordering::Less
        } else if val > 0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl FromGlib<u32> for Option<char> {
    #[inline]
    fn from_glib(val: u32) -> Option<char> {
        match val {
            0 => None,
            _ => char::from_u32(val),
        }
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

/// Translate from a pointer type without taking ownership, transfer: none.
pub trait FromGlibPtrNone<P: Ptr>: Sized {
    unsafe fn from_glib_none(ptr: P) -> Self;
}

/// Translate from a pointer type taking ownership, transfer: full.
pub trait FromGlibPtrFull<P: Ptr>: Sized {
    unsafe fn from_glib_full(ptr: P) -> Self;
}

/// Translate from a pointer type by borrowing. Don't increase the refcount
pub trait FromGlibPtrBorrow<P: Ptr>: Sized {
    unsafe fn from_glib_borrow(_ptr: P) -> Self {
        unimplemented!();
    }
}

/// Translate from a pointer type, transfer: none.
#[inline]
pub unsafe fn from_glib_none<P: Ptr, T: FromGlibPtrNone<P>>(ptr: P) -> T {
    FromGlibPtrNone::from_glib_none(ptr)
}

/// Translate from a pointer type, transfer: full (assume ownership).
#[inline]
pub unsafe fn from_glib_full<P: Ptr, T: FromGlibPtrFull<P>>(ptr: P) -> T {
    FromGlibPtrFull::from_glib_full(ptr)
}

/// Translate from a pointer type, borrowing the pointer.
#[inline]
pub unsafe fn from_glib_borrow<P: Ptr, T: FromGlibPtrBorrow<P>>(ptr: P) -> T {
    FromGlibPtrBorrow::from_glib_borrow(ptr)
}

impl<P: Ptr, T: FromGlibPtrNone<P>> FromGlibPtrNone<P> for Option<T> {
    #[inline]
    unsafe fn from_glib_none(ptr: P) -> Option<T> {
        if ptr.is_null() { None }
        else { Some(from_glib_none(ptr)) }
    }
}

impl<P: Ptr, T: FromGlibPtrBorrow<P>> FromGlibPtrBorrow<P> for Option<T> {
    #[inline]
    unsafe fn from_glib_borrow(ptr: P) -> Option<T> {
        if ptr.is_null() { None }
        else { Some(from_glib_borrow(ptr)) }
    }
}

impl<P: Ptr, T: FromGlibPtrFull<P>> FromGlibPtrFull<P> for Option<T> {
    #[inline]
    unsafe fn from_glib_full(ptr: P) -> Option<T> {
        if ptr.is_null() { None }
        else { Some(from_glib_full(ptr)) }
    }
}

impl FromGlibPtrNone<*const c_char> for String {
    #[inline]
    unsafe fn from_glib_none(ptr: *const c_char) -> Self {
        assert!(!ptr.is_null());
        String::from_utf8_lossy(CStr::from_ptr(ptr).to_bytes()).into_owned()
    }
}

impl FromGlibPtrFull<*const c_char> for String {
    #[inline]
    unsafe fn from_glib_full(ptr: *const c_char) -> Self {
        let res = from_glib_none(ptr);
        glib_ffi::g_free(ptr as *mut _);
        res
    }
}

impl FromGlibPtrNone<*mut c_char> for String {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut c_char) -> Self {
        assert!(!ptr.is_null());
        String::from_utf8_lossy(CStr::from_ptr(ptr).to_bytes()).into_owned()
    }
}

impl FromGlibPtrFull<*mut c_char> for String {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut c_char) -> Self {
        let res = from_glib_none(ptr);
        glib_ffi::g_free(ptr as *mut _);
        res
    }
}

#[cfg(not(windows))]
unsafe fn c_to_path_buf(ptr: *const c_char) -> PathBuf {
    assert!(!ptr.is_null());

    // GLib paths on UNIX are always in the local encoding, which can be
    // UTF-8 or anything else really, but is always a NUL-terminated string
    // and must not contain any other NUL bytes
    OsString::from_vec(CStr::from_ptr(ptr).to_bytes().to_vec())
        .into()
}

#[cfg(windows)]
unsafe fn c_to_path_buf(ptr: *const c_char) -> PathBuf {
    assert!(!ptr.is_null());

    // GLib paths on Windows are always UTF-8, as such we can convert to a String
    // first and then go to a PathBuf from there. Unless there is a bug
    // in the C library, the conversion from UTF-8 can never fail so we can
    // safely panic here if that ever happens
    String::from_utf8(CStr::from_ptr(ptr).to_bytes().into())
        .expect("Invalid, non-UTF8 path")
        .into()
}

impl FromGlibPtrNone<*const c_char> for PathBuf {
    #[inline]
    unsafe fn from_glib_none(ptr: *const c_char) -> Self {
        assert!(!ptr.is_null());
        c_to_path_buf(ptr)
    }
}

impl FromGlibPtrFull<*const c_char> for PathBuf {
    #[inline]
    unsafe fn from_glib_full(ptr: *const c_char) -> Self {
        let res = from_glib_none(ptr);
        glib_ffi::g_free(ptr as *mut _);
        res
    }
}

impl FromGlibPtrNone<*mut c_char> for PathBuf {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut c_char) -> Self {
        assert!(!ptr.is_null());
        c_to_path_buf(ptr)
    }
}

impl FromGlibPtrFull<*mut c_char> for PathBuf {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut c_char) -> Self {
        let res = from_glib_none(ptr);
        glib_ffi::g_free(ptr as *mut _);
        res
    }
}

/// Translate from a container.
pub trait FromGlibContainer<T, P: Ptr>: Sized {
    /// Transfer: none.
    ///
    /// `num` is the advised number of elements.
    unsafe fn from_glib_none_num(ptr: P, num: usize) -> Self;

    /// Transfer: container.
    ///
    /// `num` is the advised number of elements.
    unsafe fn from_glib_container_num(ptr: P, num: usize) -> Self;

    /// Transfer: full.
    ///
    /// `num` is the advised number of elements.
    unsafe fn from_glib_full_num(ptr: P, num: usize) -> Self;
}

/// Translate from a container of pointers.
pub trait FromGlibPtrContainer<P: Ptr, PP: Ptr>: FromGlibContainer<P, PP> + Sized {
    /// Transfer: none.
    unsafe fn from_glib_none(ptr: PP) -> Self;

    /// Transfer: container.
    unsafe fn from_glib_container(ptr: PP) -> Self;

    /// Transfer: full.
    unsafe fn from_glib_full(ptr: PP) -> Self;
}

pub unsafe fn c_ptr_array_len<P: Ptr>(mut ptr: *const P) -> usize {
    let mut len = 0;

    if !ptr.is_null() {
        while !(*ptr).is_null() {
            len += 1;
            ptr = ptr.offset(1);
        }
    }
    len
}

pub trait FromGlibContainerAsVec<T, P: Ptr>
where Self: Sized {
    unsafe fn from_glib_none_num_as_vec(ptr: P, num: usize) -> Vec<Self>;
    unsafe fn from_glib_container_num_as_vec(ptr: P, num: usize) -> Vec<Self>;
    unsafe fn from_glib_full_num_as_vec(ptr: P, num: usize) -> Vec<Self>;
}

pub trait FromGlibPtrArrayContainerAsVec<P: Ptr, PP: Ptr> : FromGlibContainerAsVec<P, PP>
where Self: Sized {
    unsafe fn from_glib_none_as_vec(ptr: PP) -> Vec<Self>;
    unsafe fn from_glib_container_as_vec(ptr: PP) -> Vec<Self>;
    unsafe fn from_glib_full_as_vec(ptr: PP) -> Vec<Self>;
}

macro_rules! impl_from_glib_container_as_vec_fundamental {
    ($name:ty) => {
        impl FromGlibContainerAsVec<$name, *const $name> for $name {
            unsafe fn from_glib_none_num_as_vec(ptr: *const $name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push(ptr::read(ptr.offset(i as isize)));
                }
                res
            }

            unsafe fn from_glib_container_num_as_vec(_: *const $name, _: usize) -> Vec<Self> {
                // Can't really free a *const
                unimplemented!();
            }

            unsafe fn from_glib_full_num_as_vec(_: *const $name, _: usize) -> Vec<Self> {
                // Can't really free a *const
                unimplemented!();
            }
        }

        impl FromGlibContainerAsVec<$name, *mut $name> for $name {
            unsafe fn from_glib_none_num_as_vec(ptr: *mut $name, num: usize) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *const _, num)
            }

            unsafe fn from_glib_container_num_as_vec(ptr: *mut $name, num: usize) -> Vec<Self> {
                let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
                glib_ffi::g_free(ptr as *mut _);
                res
            }

            unsafe fn from_glib_full_num_as_vec(ptr: *mut $name, num: usize) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, num)
            }
        }
    }
}

impl_from_glib_container_as_vec_fundamental!(u8);
impl_from_glib_container_as_vec_fundamental!(i8);
impl_from_glib_container_as_vec_fundamental!(u16);
impl_from_glib_container_as_vec_fundamental!(i16);
impl_from_glib_container_as_vec_fundamental!(u32);
impl_from_glib_container_as_vec_fundamental!(i32);
impl_from_glib_container_as_vec_fundamental!(u64);
impl_from_glib_container_as_vec_fundamental!(i64);
impl_from_glib_container_as_vec_fundamental!(f32);
impl_from_glib_container_as_vec_fundamental!(f64);

macro_rules! impl_from_glib_container_as_vec_string {
    ($name:ty, $ffi_name:ty) => {
        impl FromGlibContainerAsVec<$ffi_name, *const $ffi_name> for $name {
            unsafe fn from_glib_none_num_as_vec(ptr: *const $ffi_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push(from_glib_none(ptr::read(ptr.offset(i as isize)) as $ffi_name));
                }
                res
            }

            unsafe fn from_glib_container_num_as_vec(_: *const $ffi_name, _: usize) -> Vec<Self> {
                // Can't really free a *const
                unimplemented!();
            }

            unsafe fn from_glib_full_num_as_vec(_: *const $ffi_name, _: usize) -> Vec<Self> {
                // Can't really free a *const
                unimplemented!();
            }
        }

        impl FromGlibContainerAsVec<$ffi_name, *mut $ffi_name> for $name {
            unsafe fn from_glib_none_num_as_vec(ptr: *mut $ffi_name, num: usize) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *const _, num)
            }

            unsafe fn from_glib_container_num_as_vec(ptr: *mut $ffi_name, num: usize) -> Vec<Self> {
                let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
                glib_ffi::g_free(ptr as *mut _);
                res
            }

            unsafe fn from_glib_full_num_as_vec(ptr: *mut $ffi_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push(from_glib_full(ptr::read(ptr.offset(i as isize))));
                }
                glib_ffi::g_free(ptr as *mut _);
                res
            }
        }

        impl FromGlibPtrArrayContainerAsVec<$ffi_name, *mut $ffi_name> for $name {
            unsafe fn from_glib_none_as_vec(ptr: *mut $ffi_name) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_container_as_vec(ptr: *mut $ffi_name) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_full_as_vec(ptr: *mut $ffi_name) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, c_ptr_array_len(ptr))
            }
        }

        impl FromGlibPtrArrayContainerAsVec<$ffi_name, *const $ffi_name> for $name {
            unsafe fn from_glib_none_as_vec(ptr: *const $ffi_name) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_container_as_vec(ptr: *const $ffi_name) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_full_as_vec(ptr: *const $ffi_name) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, c_ptr_array_len(ptr))
            }
        }
    }
}

impl_from_glib_container_as_vec_string!(String, *const c_char);
impl_from_glib_container_as_vec_string!(String, *mut c_char);
impl_from_glib_container_as_vec_string!(PathBuf, *const c_char);
impl_from_glib_container_as_vec_string!(PathBuf, *mut c_char);

impl <P, PP: Ptr, T: FromGlibContainerAsVec<P, PP>> FromGlibContainer<P, PP> for Vec<T> {
    unsafe fn from_glib_none_num(ptr: PP, num: usize) -> Vec<T> {
        FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num)
    }

    unsafe fn from_glib_container_num(ptr: PP, num: usize) -> Vec<T> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, num)
    }

    unsafe fn from_glib_full_num(ptr: PP, num: usize) -> Vec<T> {
        FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, num)
    }
}

impl <P: Ptr, PP: Ptr, T: FromGlibPtrArrayContainerAsVec<P, PP>> FromGlibPtrContainer<P, PP> for Vec<T> {
    unsafe fn from_glib_none(ptr: PP) -> Vec<T> {
        FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr)
    }

    unsafe fn from_glib_container(ptr: PP) -> Vec<T> {
        FromGlibPtrArrayContainerAsVec::from_glib_container_as_vec(ptr)
    }

    unsafe fn from_glib_full(ptr: PP) -> Vec<T> {
        FromGlibPtrArrayContainerAsVec::from_glib_full_as_vec(ptr)
    }
}

impl<T> FromGlibContainerAsVec<<T as GlibPtrDefault>::GlibType, *mut glib_ffi::GSList> for T
where T: GlibPtrDefault + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType> + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType> {
    unsafe fn from_glib_none_num_as_vec(mut ptr: *mut glib_ffi::GSList, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let mut res = Vec::with_capacity(num);
        for _ in 0..num {
            let item_ptr: <T as GlibPtrDefault>::GlibType = Ptr::from((*ptr).data);
            if !item_ptr.is_null() {
                res.push(from_glib_none(item_ptr));
            }
            ptr = (*ptr).next;
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut glib_ffi::GSList, num: usize) -> Vec<T> {
        let res = FromGlibContainer::from_glib_none_num(ptr, num);
        if !ptr.is_null() {
            glib_ffi::g_slist_free(ptr as *mut _);
        }
        res
    }

    unsafe fn from_glib_full_num_as_vec(mut ptr: *mut glib_ffi::GSList, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let orig_ptr = ptr;
        let mut res = Vec::with_capacity(num);
        for _ in 0..num {
            let item_ptr: <T as GlibPtrDefault>::GlibType = Ptr::from((*ptr).data);
            if !item_ptr.is_null() {
                res.push(from_glib_full(item_ptr));
            }
            ptr = (*ptr).next;
        }
        glib_ffi::g_slist_free(orig_ptr as *mut _);
        res
    }
}

impl<T> FromGlibPtrArrayContainerAsVec<<T as GlibPtrDefault>::GlibType, *mut glib_ffi::GSList> for T
where T: GlibPtrDefault + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType> + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType> {
    unsafe fn from_glib_none_as_vec(ptr: *mut glib_ffi::GSList) -> Vec<T> {
        let num = glib_ffi::g_slist_length(ptr) as usize;
        FromGlibContainer::from_glib_none_num(ptr, num)
    }

    unsafe fn from_glib_container_as_vec(ptr: *mut glib_ffi::GSList) -> Vec<T> {
        let num = glib_ffi::g_slist_length(ptr) as usize;
        FromGlibContainer::from_glib_container_num(ptr, num)
    }

    unsafe fn from_glib_full_as_vec(ptr: *mut glib_ffi::GSList) -> Vec<T> {
        let num = glib_ffi::g_slist_length(ptr) as usize;
        FromGlibContainer::from_glib_full_num(ptr, num)
    }
}

impl<T> FromGlibContainerAsVec<<T as GlibPtrDefault>::GlibType, *mut glib_ffi::GList> for T
where T: GlibPtrDefault + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType> + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType> {
    unsafe fn from_glib_none_num_as_vec(mut ptr: *mut glib_ffi::GList, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let mut res = Vec::with_capacity(num);
        for _ in 0..num {
            let item_ptr: <T as GlibPtrDefault>::GlibType = Ptr::from((*ptr).data);
            if !item_ptr.is_null() {
                res.push(from_glib_none(item_ptr));
            }
            ptr = (*ptr).next;
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut glib_ffi::GList, num: usize) -> Vec<T> {
        let res = FromGlibContainer::from_glib_none_num(ptr, num);
        if !ptr.is_null() {
            glib_ffi::g_list_free(ptr as *mut _);
        }
        res
    }

    unsafe fn from_glib_full_num_as_vec(mut ptr: *mut glib_ffi::GList, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let orig_ptr = ptr;
        let mut res = Vec::with_capacity(num);
        for _ in 0..num {
            let item_ptr: <T as GlibPtrDefault>::GlibType = Ptr::from((*ptr).data);
            if !item_ptr.is_null() {
                res.push(from_glib_full(item_ptr));
            }
            ptr = (*ptr).next;
        }
        glib_ffi::g_list_free(orig_ptr as *mut _);
        res
    }
}

impl<T> FromGlibPtrArrayContainerAsVec<<T as GlibPtrDefault>::GlibType, *mut glib_ffi::GList> for T
where T: GlibPtrDefault + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType> + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType> {
    unsafe fn from_glib_none_as_vec(ptr: *mut glib_ffi::GList) -> Vec<T> {
        let num = glib_ffi::g_list_length(ptr) as usize;
        FromGlibContainer::from_glib_none_num(ptr, num)
    }

    unsafe fn from_glib_container_as_vec(ptr: *mut glib_ffi::GList) -> Vec<T> {
        let num = glib_ffi::g_list_length(ptr) as usize;
        FromGlibContainer::from_glib_container_num(ptr, num)
    }

    unsafe fn from_glib_full_as_vec(ptr: *mut glib_ffi::GList) -> Vec<T> {
        let num = glib_ffi::g_list_length(ptr) as usize;
        FromGlibContainer::from_glib_full_num(ptr, num)
    }
}

impl<T> FromGlibContainerAsVec<<T as GlibPtrDefault>::GlibType, *const glib_ffi::GList> for T
where T: GlibPtrDefault + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType> + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType> {
    unsafe fn from_glib_none_num_as_vec(ptr: *const glib_ffi::GList, num: usize) -> Vec<T> {
        FromGlibContainer::from_glib_none_num(mut_override(ptr), num)
    }

    unsafe fn from_glib_container_num_as_vec(_: *const glib_ffi::GList, _: usize) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }

    unsafe fn from_glib_full_num_as_vec(_: *const glib_ffi::GList, _: usize) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }
}


impl<T> FromGlibPtrArrayContainerAsVec<<T as GlibPtrDefault>::GlibType, *const glib_ffi::GList> for T
where T: GlibPtrDefault + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType> + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType> {
    unsafe fn from_glib_none_as_vec(ptr: *const glib_ffi::GList) -> Vec<T> {
        FromGlibPtrContainer::from_glib_none(mut_override(ptr))
    }

    unsafe fn from_glib_container_as_vec(_: *const glib_ffi::GList) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }

    unsafe fn from_glib_full_as_vec(_: *const glib_ffi::GList) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }
}

impl<T> FromGlibContainerAsVec<<T as GlibPtrDefault>::GlibType, *const glib_ffi::GSList> for T
where T: GlibPtrDefault + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType> + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType> {
    unsafe fn from_glib_none_num_as_vec(ptr: *const glib_ffi::GSList, num: usize) -> Vec<T> {
        FromGlibContainer::from_glib_none_num(mut_override(ptr), num)
    }

    unsafe fn from_glib_container_num_as_vec(_: *const glib_ffi::GSList, _: usize) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }

    unsafe fn from_glib_full_num_as_vec(_: *const glib_ffi::GSList, _: usize) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }
}


impl<T> FromGlibPtrArrayContainerAsVec<<T as GlibPtrDefault>::GlibType, *const glib_ffi::GSList> for T
where T: GlibPtrDefault + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType> + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType> {
    unsafe fn from_glib_none_as_vec(ptr: *const glib_ffi::GSList) -> Vec<T> {
        FromGlibPtrContainer::from_glib_none(mut_override(ptr))
    }

    unsafe fn from_glib_container_as_vec(_: *const glib_ffi::GSList) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }

    unsafe fn from_glib_full_as_vec(_: *const glib_ffi::GSList) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }
}

unsafe extern "C" fn read_string_hash_table(key: glib_ffi::gpointer, value: glib_ffi::gpointer,
                                            hash_map: glib_ffi::gpointer) {
    let key: String = from_glib_none(key as *const c_char);
    let value: String = from_glib_none(value as *const c_char);
    let hash_map: &mut HashMap<String, String> = &mut *(hash_map as *mut HashMap<String, String>);
    hash_map.insert(key, value);
}

impl FromGlibContainer<*const c_char, *mut glib_ffi::GHashTable> for HashMap<String, String> {
    unsafe fn from_glib_none_num(ptr: *mut glib_ffi::GHashTable, _: usize) -> Self {
        FromGlibPtrContainer::from_glib_none(ptr)
    }

    unsafe fn from_glib_container_num(ptr: *mut glib_ffi::GHashTable, _: usize) -> Self {
        FromGlibPtrContainer::from_glib_full(ptr)
    }

    unsafe fn from_glib_full_num(ptr: *mut glib_ffi::GHashTable, _: usize) -> Self {
        FromGlibPtrContainer::from_glib_full(ptr)
    }
}

impl FromGlibPtrContainer<*const c_char, *mut glib_ffi::GHashTable> for HashMap<String, String> {
    unsafe fn from_glib_none(ptr: *mut glib_ffi::GHashTable) -> Self {
        let mut map = HashMap::new();
        glib_ffi::g_hash_table_foreach(ptr, Some(read_string_hash_table),
                                       &mut map as *mut HashMap<String, String> as *mut _);
        map
    }

    unsafe fn from_glib_container(ptr: *mut glib_ffi::GHashTable) -> Self {
        FromGlibPtrContainer::from_glib_full(ptr)
    }

    unsafe fn from_glib_full(ptr: *mut glib_ffi::GHashTable) -> Self {
        let map = FromGlibPtrContainer::from_glib_none(ptr);
        glib_ffi::g_hash_table_unref(ptr);
        map
    }
}

#[cfg(test)]
mod tests {
    extern crate tempdir;
    use self::tempdir::TempDir;
    use std::fs;

    use std::collections::HashMap;
    use ffi as glib_ffi;
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

    #[test]
    fn string_array() {
        let v = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let stash = v.to_glib_none();
        let ptr: *mut *mut c_char = stash.0;
        let ptr_copy = unsafe { glib_ffi::g_strdupv(ptr) };

        let actual: Vec<String> = unsafe{ FromGlibPtrContainer::from_glib_full(ptr_copy) };
        assert_eq!(v, actual);
    }

    #[test]
    fn test_paths() {
        let tmp_dir = TempDir::new("glib-test").unwrap();

        // Test if passing paths to GLib and getting them back
        // gives us useful results
        let dir_1 = tmp_dir.path().join("abcd");
        fs::create_dir(&dir_1).unwrap();
        assert_eq!(::functions::path_get_basename(&dir_1), Some("abcd".into()));
        assert_eq!(::functions::path_get_basename(dir_1.canonicalize().unwrap()), Some("abcd".into()));
        assert_eq!(::functions::path_get_dirname(dir_1.canonicalize().unwrap()), Some(tmp_dir.path().into()));
        assert!(::functions::file_test(&dir_1, ::FileTest::EXISTS | ::FileTest::IS_DIR));
        assert!(::functions::file_test(&dir_1.canonicalize().unwrap(), ::FileTest::EXISTS | ::FileTest::IS_DIR));

        // And test with some non-ASCII characters
        let dir_2 = tmp_dir.as_ref().join("øäöü");
        fs::create_dir(&dir_2).unwrap();
        assert_eq!(::functions::path_get_basename(&dir_2), Some("øäöü".into()));
        assert_eq!(::functions::path_get_basename(dir_2.canonicalize().unwrap()), Some("øäöü".into()));
        assert_eq!(::functions::path_get_dirname(dir_2.canonicalize().unwrap()), Some(tmp_dir.path().into()));
        assert!(::functions::file_test(&dir_2, ::FileTest::EXISTS | ::FileTest::IS_DIR));
        assert!(::functions::file_test(&dir_2.canonicalize().unwrap(), ::FileTest::EXISTS | ::FileTest::IS_DIR));
    }
}

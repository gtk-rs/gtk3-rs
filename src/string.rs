// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi as glib_ffi;
use gobject_ffi;
use std::borrow;
use std::convert;
use std::fmt;
use std::mem;
use std::ops;
use std::ptr;
use std::slice;
use std::str;
use translate::*;

glib_wrapper! {
    /// A mutable text buffer that grows automatically.
    pub struct String(Boxed<glib_ffi::GString>);

    match fn {
        copy => |ptr| gobject_ffi::g_boxed_copy(glib_ffi::g_gstring_get_type(), ptr as *mut _) as *mut glib_ffi::GString,
        free => |ptr| gobject_ffi::g_boxed_free(glib_ffi::g_gstring_get_type(), ptr as *mut _),
        get_type => || glib_ffi::g_gstring_get_type(),
    }
}

impl String {
    pub fn new<T: AsRef<[u8]>>(data: T) -> String {
        let bytes = data.as_ref();
        unsafe { from_glib_full(glib_ffi::g_string_new_len(
                    bytes.as_ptr() as *const _, 
                    bytes.len() as isize)) }
    }

    pub fn append(&mut self, val: &str) -> &mut Self {
        unsafe { glib_ffi::g_string_append_len(
                    self.to_glib_none_mut().0, 
                    val.to_glib_none().0, 
                    val.len() as isize); }
        self
    }

    pub fn hash(&self) -> u32 {
        unsafe {
            glib_ffi::g_string_hash(self.to_glib_none().0)
        }
    }

    pub fn insert(&mut self, pos: isize, val: &str) -> &mut Self {
        unsafe { glib_ffi::g_string_insert_len(
                    self.to_glib_none_mut().0, 
                    pos, 
                    val.to_glib_none().0,
                    val.len() as isize); }
        self
    }

    pub fn overwrite(&mut self, pos: usize, val: &str) -> &mut Self {
        unsafe { glib_ffi::g_string_overwrite_len(
                    self.to_glib_none_mut().0, 
                    pos, 
                    val.to_glib_none().0,
                    val.len() as isize); }
        self
    }

    pub fn prepend(&mut self, val: &str) -> &mut Self {
        unsafe { glib_ffi::g_string_prepend_len(
                    self.to_glib_none_mut().0, 
                    val.to_glib_none().0,
                    val.len() as isize); }
        self
    }

    pub fn truncate(&mut self, len: usize) -> &mut Self {
        unsafe { glib_ffi::g_string_truncate(self.to_glib_none_mut().0, len); }
        self
    }

    /// Returns `&str` slice when contained data is valid UTF-8 string, or an error otherwise.
    pub fn to_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(self.as_ref())
    }

    /// Returns `Cow<str>` containing UTF-8 data. Invalid UTF-8 sequences are replaced with
    /// replacement character.
    pub fn to_string_lossy(&self) -> borrow::Cow<str> {
        ::std::string::String::from_utf8_lossy(self.as_ref())
    }
}

impl Default for String {
    /// Creates a new empty string.
    fn default() -> String {
        unsafe { from_glib_full(glib_ffi::g_string_new(ptr::null())) }
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string_lossy())
    }
}

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            from_glib(glib_ffi::g_string_equal(self.to_glib_none().0, 
                                               other.to_glib_none().0))
        }
    }
}

impl Eq for String {}

impl convert::AsRef<[u8]> for String {
    fn as_ref(&self) -> &[u8] {
        let ptr: *const u8 = (*self.0).str as _;
        let len: usize = (*self.0).len;
        unsafe { slice::from_raw_parts(ptr, len) }
    }
}

impl ops::Deref for String {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        let ptr: *const u8 = (*self.0).str as _;
        let len: usize = (*self.0).len;
        unsafe { slice::from_raw_parts(ptr, len) }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn append() {
        let mut s = ::String::new("");
        s.append("Hello").append(" ").append("there!");
        assert_eq!(&*s, b"Hello there!");
    }

    #[test]
    fn insert() {
        let mut s = ::String::new("foobaz");
        s.insert(3, "bar");
        assert_eq!(&*s, b"foobarbaz");
    }

    #[test]
    fn overwrite() {
        let mut s = ::String::new("abc");
        s.overwrite(2, "de");
        assert_eq!(&*s, b"abde");
    }

    #[test]
    fn prepend() {
        let mut s = ::String::new("456");
        s.prepend("123");
        assert_eq!(&*s, b"123456");
    }

    #[test]
    fn truncate() {
        let mut s = ::String::new("12345");
        s.truncate(10);
        assert_eq!(&*s, b"12345");
        s.truncate(2);
        assert_eq!(&*s, b"12");
    }

    #[test]
    fn default() {
        let s1: ::String = Default::default();
        assert_eq!(&*s1, b"");
    }

    #[test]
    fn display() {
        let s: ::String = ::String::new("This is a string."); 
        assert_eq!(
            &format!("{}", s),
            "This is a string.");
    }

    #[test]
    fn eq() {
        let a1 = ::String::new("a");
        let a2 = ::String::new("a");
        let b = ::String::new("b");
        assert_eq!(a1, a1);
        assert_eq!(a1, a2);
        assert_ne!(a1, b);
        assert_ne!(a2, b);
    }

    #[test]
    fn invalid_utf8() {
        let s = ::String::new(b"Hello \xF0\x90\x80World");
        assert!(s.to_str().is_err());
        assert_eq!(s.to_string_lossy(), "Hello �World");
    }
}

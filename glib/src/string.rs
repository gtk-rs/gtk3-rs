// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use std::borrow;
use std::cmp;
use std::convert;
use std::fmt;
use std::hash;
use std::ops;
use std::ptr;
use std::slice;
use std::str;

wrapper! {
    /// A mutable text buffer that grows automatically.
    pub struct String(Boxed<ffi::GString>);

    match fn {
        copy => |ptr| gobject_ffi::g_boxed_copy(ffi::g_gstring_get_type(), ptr as *mut _) as *mut ffi::GString,
        free => |ptr| gobject_ffi::g_boxed_free(ffi::g_gstring_get_type(), ptr as *mut _),
        get_type => || ffi::g_gstring_get_type(),
    }
}

unsafe impl Send for String {}
unsafe impl Sync for String {}

impl String {
    pub fn new<T: AsRef<[u8]>>(data: T) -> String {
        let bytes = data.as_ref();
        unsafe {
            from_glib_full(ffi::g_string_new_len(
                bytes.as_ptr() as *const _,
                bytes.len() as isize,
            ))
        }
    }

    pub fn append(&mut self, val: &str) -> &mut Self {
        unsafe {
            ffi::g_string_append_len(
                self.to_glib_none_mut().0,
                val.to_glib_none().0,
                val.len() as isize,
            );
        }
        self
    }

    pub fn insert(&mut self, pos: isize, val: &str) -> &mut Self {
        unsafe {
            ffi::g_string_insert_len(
                self.to_glib_none_mut().0,
                pos,
                val.to_glib_none().0,
                val.len() as isize,
            );
        }
        self
    }

    pub fn overwrite(&mut self, pos: usize, val: &str) -> &mut Self {
        unsafe {
            ffi::g_string_overwrite_len(
                self.to_glib_none_mut().0,
                pos,
                val.to_glib_none().0,
                val.len() as isize,
            );
        }
        self
    }

    pub fn prepend(&mut self, val: &str) -> &mut Self {
        unsafe {
            ffi::g_string_prepend_len(
                self.to_glib_none_mut().0,
                val.to_glib_none().0,
                val.len() as isize,
            );
        }
        self
    }

    pub fn truncate(&mut self, len: usize) -> &mut Self {
        unsafe {
            ffi::g_string_truncate(self.to_glib_none_mut().0, len);
        }
        self
    }

    /// Returns `&str` slice when contained data is valid UTF-8 string, or an error otherwise.
    pub fn to_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(self.as_ref())
    }

    /// Returns `Cow<str>` containing UTF-8 data. Invalid UTF-8 sequences are replaced with
    /// replacement character.
    pub fn to_string_lossy(&self) -> borrow::Cow<str> {
        std::string::String::from_utf8_lossy(self.as_ref())
    }
}

impl Default for String {
    /// Creates a new empty string.
    fn default() -> String {
        unsafe { from_glib_full(ffi::g_string_new(ptr::null())) }
    }
}

impl fmt::Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_string_lossy())
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_string_lossy())
    }
}

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            from_glib(ffi::g_string_equal(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }
}

impl Eq for String {}

impl cmp::PartialOrd for String {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for String {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl hash::Hash for String {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        hash::Hash::hash_slice(self.as_ref(), state)
    }
}

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
        let mut s = crate::String::new("");
        s.append("Hello").append(" ").append("there!");
        assert_eq!(&*s, b"Hello there!");
    }

    #[test]
    fn insert() {
        let mut s = crate::String::new("foobaz");
        s.insert(3, "bar");
        assert_eq!(&*s, b"foobarbaz");
    }

    #[test]
    fn overwrite() {
        let mut s = crate::String::new("abc");
        s.overwrite(2, "de");
        assert_eq!(&*s, b"abde");
    }

    #[test]
    fn prepend() {
        let mut s = crate::String::new("456");
        s.prepend("123");
        assert_eq!(&*s, b"123456");
    }

    #[test]
    fn truncate() {
        let mut s = crate::String::new("12345");
        s.truncate(10);
        assert_eq!(&*s, b"12345");
        s.truncate(2);
        assert_eq!(&*s, b"12");
    }

    #[test]
    fn default() {
        let s1: crate::String = Default::default();
        assert_eq!(&*s1, b"");
    }

    #[test]
    fn display() {
        let s: crate::String = crate::String::new("This is a string.");
        assert_eq!(&format!("{}", s), "This is a string.");
    }

    #[test]
    fn eq() {
        let a1 = crate::String::new("a");
        let a2 = crate::String::new("a");
        let b = crate::String::new("b");
        assert_eq!(a1, a2);
        assert_ne!(a1, b);
        assert_ne!(a2, b);
    }

    #[test]
    fn invalid_utf8() {
        let s = crate::String::new(b"Hello \xF0\x90\x80World");
        assert!(s.to_str().is_err());
        assert_eq!(s.to_string_lossy(), "Hello �World");
    }
}

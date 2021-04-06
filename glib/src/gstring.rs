// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use crate::types::{StaticType, Type};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::ffi::{CStr, CString, OsStr};
use std::fmt;
use std::hash;
use std::ops::Deref;
use std::os::raw::c_char;
use std::ptr;
use std::slice;
use std::string::String;

use crate::value::{FromValueOptional, SetValue, SetValueOptional, Value};

pub struct GString(Inner);

enum Inner {
    Native(Option<CString>),
    Foreign(*mut c_char, usize),
}

unsafe impl Send for GString {}
unsafe impl Sync for GString {}

impl GString {
    /// Create a new GString from a glib-originated string, taking ownership.
    ///
    /// # Safety
    ///
    /// The provided string must be a valid C string (i.e. `'0'` terminated).
    ///
    /// The provided pointer must be allocated by glib such that `g_free(ptr)`
    /// is valid, as the `GString` will call `g_free(ptr)` on drop.
    ///
    /// It is essential that noone else free this pointer as it owned by the
    /// returned `GString`.
    ///
    /// The underlying string must not be mutated, in particular in terms of
    /// length, underneath the `GString` instance.
    unsafe fn new(ptr: *mut c_char) -> Self {
        GString(Inner::Foreign(ptr, libc::strlen(ptr)))
    }

    /// Create a new GString from a glib-originated string, borrowing it rather
    /// than taking ownership.
    ///
    /// # Safety
    ///
    /// The provided string must be a valid C string (i.e. `'0'` terminated).
    ///
    /// It is essential that noone else free this pointer as it owned by the
    /// returned `GString` until the borrow is dropped.
    ///
    /// The underlying string must not be mutated, in particular in terms of
    /// length, underneath the `GString` instance for the duration of the borrow.
    unsafe fn new_borrowed(ptr: *const c_char) -> Borrowed<Self> {
        Borrowed::new(GString(Inner::Foreign(ptr as *mut _, libc::strlen(ptr))))
    }

    pub fn as_str(&self) -> &str {
        let cstr = match self {
            GString(Inner::Foreign(ptr, length)) => unsafe {
                if ptr.is_null() || length == &0 {
                    return "";
                }
                let bytes = slice::from_raw_parts(*ptr as *const u8, length + 1);
                CStr::from_bytes_with_nul_unchecked(bytes)
            },
            GString(Inner::Native(cstring)) => cstring
                .as_ref()
                .expect("Native shouldn't be empty")
                .as_c_str(),
        };
        cstr.to_str().unwrap()
    }
}

impl Clone for GString {
    fn clone(&self) -> GString {
        let cstring = CString::new(self.as_str().to_string()).expect("CString::new failed");

        GString(Inner::Native(Some(cstring)))
    }
}

impl fmt::Debug for GString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <&str as fmt::Debug>::fmt(&self.as_str(), f)
    }
}

impl Drop for GString {
    fn drop(&mut self) {
        if let GString(Inner::Foreign(ptr, _len)) = self {
            unsafe {
                ffi::g_free(*ptr as *mut _);
            }
        }
    }
}

impl fmt::Display for GString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl hash::Hash for GString {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        let bytes = match self {
            GString(Inner::Foreign(ptr, length)) => unsafe {
                if ptr.is_null() || length == &0 {
                    b""
                } else {
                    slice::from_raw_parts(*ptr as *const u8, length + 1)
                }
            },
            GString(Inner::Native(cstring)) => cstring
                .as_ref()
                .expect("Native shouldn't be empty")
                .as_bytes(),
        };
        state.write(bytes);
    }
}

impl Borrow<str> for GString {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Ord for GString {
    fn cmp(&self, other: &GString) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd for GString {
    fn partial_cmp(&self, other: &GString) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for GString {
    fn eq(&self, other: &GString) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<GString> for String {
    fn eq(&self, other: &GString) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<str> for GString {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<'a> PartialEq<&'a str> for GString {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str() == *other
    }
}

impl<'a> PartialEq<GString> for &'a str {
    fn eq(&self, other: &GString) -> bool {
        *self == other.as_str()
    }
}

impl PartialEq<String> for GString {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<GString> for str {
    fn eq(&self, other: &GString) -> bool {
        self == other.as_str()
    }
}

impl PartialOrd<GString> for String {
    fn partial_cmp(&self, other: &GString) -> Option<Ordering> {
        Some(self.cmp(&String::from(other.as_str())))
    }
}

impl PartialOrd<String> for GString {
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        Some(self.as_str().cmp(other.as_str()))
    }
}

impl PartialOrd<GString> for str {
    fn partial_cmp(&self, other: &GString) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialOrd<str> for GString {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        Some(self.as_str().cmp(other))
    }
}

impl Eq for GString {}

impl AsRef<str> for GString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<OsStr> for GString {
    fn as_ref(&self) -> &OsStr {
        OsStr::new(self.as_str())
    }
}

impl Deref for GString {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl From<GString> for String {
    #[inline]
    fn from(mut s: GString) -> Self {
        if let GString(Inner::Native(ref mut cstring)) = s {
            if let Ok(s) = cstring
                .take()
                .expect("Native shouldn't be empty")
                .into_string()
            {
                return s;
            }
        }
        String::from(s.as_str())
    }
}

impl From<GString> for Box<str> {
    #[inline]
    fn from(s: GString) -> Self {
        let st: String = s.into();
        st.into_boxed_str()
    }
}

impl From<String> for GString {
    #[inline]
    fn from(s: String) -> Self {
        s.into_bytes().into()
    }
}

impl From<Box<str>> for GString {
    #[inline]
    fn from(s: Box<str>) -> Self {
        s.as_bytes().to_vec().into()
    }
}

impl<'a> From<&'a str> for GString {
    #[inline]
    fn from(s: &'a str) -> Self {
        s.as_bytes().to_vec().into()
    }
}

impl From<Vec<u8>> for GString {
    #[inline]
    fn from(s: Vec<u8>) -> Self {
        let cstring = CString::new(s).expect("CString::new failed");
        cstring.into()
    }
}

impl From<CString> for GString {
    #[inline]
    fn from(s: CString) -> Self {
        GString(Inner::Native(Some(s)))
    }
}

impl<'a> From<&'a CStr> for GString {
    #[inline]
    fn from(c: &'a CStr) -> Self {
        CString::from(c).into()
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*const c_char> for GString {
    #[inline]
    unsafe fn from_glib_full(ptr: *const c_char) -> Self {
        GString::new(ptr as *mut _)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut u8> for GString {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut u8) -> Self {
        GString::new(ptr as *mut _)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut i8> for GString {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut i8) -> Self {
        GString::new(ptr as *mut _)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const c_char> for GString {
    #[inline]
    unsafe fn from_glib_none(ptr: *const c_char) -> Self {
        let cstr = CStr::from_ptr(ptr);
        cstr.into()
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut u8> for GString {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut u8) -> Self {
        let cstr = CStr::from_ptr(ptr as *mut _);
        cstr.into()
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut i8> for GString {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut i8) -> Self {
        let cstr = CStr::from_ptr(ptr as *mut _);
        cstr.into()
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const c_char> for GString {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *const c_char) -> Borrowed<Self> {
        GString::new_borrowed(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut u8> for GString {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut u8) -> Borrowed<Self> {
        GString::new_borrowed(ptr as *const c_char)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut i8> for GString {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut i8) -> Borrowed<Self> {
        GString::new_borrowed(ptr as *const c_char)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const c_char> for GString {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const c_char, Self> {
        Stash(self.as_ptr() as *const _, self)
    }

    #[inline]
    fn to_glib_full(&self) -> *const c_char {
        unsafe {
            ffi::g_strndup(self.as_ptr() as *const c_char, self.len() as libc::size_t)
                as *const c_char
        }
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut c_char> for GString {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut c_char, Self> {
        Stash(self.as_ptr() as *mut _, self)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut c_char {
        unsafe {
            ffi::g_strndup(self.as_ptr() as *const c_char, self.len() as libc::size_t)
                as *mut c_char
        }
    }
}

#[doc(hidden)]
impl<'a> FromGlibContainer<*const c_char, *const i8> for GString {
    unsafe fn from_glib_none_num(ptr: *const i8, num: usize) -> Self {
        if num == 0 || ptr.is_null() {
            return Self::from("");
        }
        let mut bytes = Vec::with_capacity(num + 1);
        let slice = slice::from_raw_parts(ptr as *const u8, num);
        bytes.extend_from_slice(slice);
        bytes.push(0);

        CStr::from_bytes_with_nul_unchecked(bytes.as_slice()).into()
    }

    unsafe fn from_glib_container_num(ptr: *const i8, num: usize) -> Self {
        GString(Inner::Foreign(ptr as *mut _, num))
    }

    unsafe fn from_glib_full_num(ptr: *const i8, num: usize) -> Self {
        GString(Inner::Foreign(ptr as *mut _, num))
    }
}

#[doc(hidden)]
impl<'a> FromGlibContainer<*const c_char, *mut i8> for GString {
    unsafe fn from_glib_none_num(ptr: *mut i8, num: usize) -> Self {
        FromGlibContainer::from_glib_none_num(ptr as *const i8, num)
    }

    unsafe fn from_glib_container_num(ptr: *mut i8, num: usize) -> Self {
        FromGlibContainer::from_glib_none_num(ptr as *const i8, num)
    }

    unsafe fn from_glib_full_num(ptr: *mut i8, num: usize) -> Self {
        FromGlibContainer::from_glib_none_num(ptr as *const i8, num)
    }
}

#[doc(hidden)]
impl<'a> FromGlibContainer<*const c_char, *const u8> for GString {
    unsafe fn from_glib_none_num(ptr: *const u8, num: usize) -> Self {
        FromGlibContainer::from_glib_none_num(ptr as *const i8, num)
    }

    unsafe fn from_glib_container_num(ptr: *const u8, num: usize) -> Self {
        FromGlibContainer::from_glib_none_num(ptr as *const i8, num)
    }

    unsafe fn from_glib_full_num(ptr: *const u8, num: usize) -> Self {
        FromGlibContainer::from_glib_none_num(ptr as *const i8, num)
    }
}

#[doc(hidden)]
impl<'a> FromGlibContainer<*const c_char, *mut u8> for GString {
    unsafe fn from_glib_none_num(ptr: *mut u8, num: usize) -> Self {
        FromGlibContainer::from_glib_none_num(ptr as *const i8, num)
    }

    unsafe fn from_glib_container_num(ptr: *mut u8, num: usize) -> Self {
        FromGlibContainer::from_glib_none_num(ptr as *const i8, num)
    }

    unsafe fn from_glib_full_num(ptr: *mut u8, num: usize) -> Self {
        FromGlibContainer::from_glib_none_num(ptr as *const i8, num)
    }
}

impl GlibPtrDefault for GString {
    type GlibType = *const c_char;
}

impl StaticType for GString {
    fn static_type() -> Type {
        String::static_type()
    }
}

impl StaticType for Vec<GString> {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_strv_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for GString {
    unsafe fn from_value_optional(value: &'a Value) -> Option<Self> {
        let val = value.to_glib_none().0;
        if val.is_null() {
            None
        } else {
            let ptr = gobject_ffi::g_value_dup_string(val);
            Some(GString::new(ptr))
        }
    }
}

impl SetValue for GString {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_take_string(value.to_glib_none_mut().0, this.to_glib_full())
    }
}

impl SetValueOptional for GString {
    unsafe fn set_value_optional(value: &mut Value, this: Option<&Self>) {
        gobject_ffi::g_value_take_string(value.to_glib_none_mut().0, this.to_glib_full())
    }
}

impl_from_glib_container_as_vec_string!(GString, *const c_char);
impl_from_glib_container_as_vec_string!(GString, *mut c_char);

#[cfg(test)]
#[allow(clippy::blacklisted_name)]
mod tests {
    use super::{FromGlibContainer, GString};
    use std::ffi::CString;

    #[test]
    fn test_gstring() {
        let data = CString::new("foo").unwrap();
        let ptr = data.as_ptr();

        unsafe {
            let ptr_copy = ffi::g_strdup(ptr);
            let gstring = GString::new(ptr_copy);
            assert_eq!(gstring.as_str(), "foo");
            let foo: Box<str> = gstring.into();
            assert_eq!(foo.as_ref(), "foo");
        }
    }

    #[test]
    fn test_owned_glib_string() {
        let data = CString::new("foo").unwrap();
        let ptr = data.as_ptr();
        unsafe {
            let ptr_copy = ffi::g_strdup(ptr);
            let gstr = GString::new(ptr_copy);
            assert_eq!(gstr, "foo");
        }
    }

    #[test]
    fn test_gstring_from_str() {
        let gstring: GString = "foo".into();
        assert_eq!(gstring.as_str(), "foo");
        let foo: Box<str> = gstring.into();
        assert_eq!(foo.as_ref(), "foo");
    }

    #[test]
    fn test_gstring_from_cstring() {
        let cstr = CString::new("foo").unwrap();
        let gstring = GString::from(cstr);
        assert_eq!(gstring.as_str(), "foo");
        let foo: Box<str> = gstring.into();
        assert_eq!(foo.as_ref(), "foo");
    }

    #[test]
    fn test_string_from_gstring() {
        let cstr = CString::new("foo").unwrap();
        let gstring = GString::from(cstr);
        assert_eq!(gstring.as_str(), "foo");
        let s = String::from(gstring);
        assert_eq!(s, "foo");
    }

    #[test]
    fn test_vec_u8_to_gstring() {
        let v: &[u8] = b"foo";
        let s: GString = Vec::from(v).into();
        assert_eq!(s.as_str(), "foo");
    }

    #[test]
    fn test_from_glib_container() {
        unsafe {
            let test_a: GString = FromGlibContainer::from_glib_container_num(
                ffi::g_strdup("hello_world".as_ptr() as *const i8),
                5,
            );
            assert_eq!("hello", test_a.as_str());

            let test_b: GString = FromGlibContainer::from_glib_none_num("hello_world".as_ptr(), 5);
            assert_eq!("hello", test_b.as_str());

            let test_c: GString = FromGlibContainer::from_glib_none_num(std::ptr::null::<i8>(), 0);
            assert_eq!("", test_c.as_str());

            let test_d: GString = FromGlibContainer::from_glib_none_num("".as_ptr(), 0);
            assert_eq!("", test_d.as_str());

            let test_e: GString = FromGlibContainer::from_glib_container_num(
                ffi::g_strdup(std::ptr::null::<i8>()),
                0,
            );
            assert_eq!("", test_e.as_str());
        }
    }

    #[test]
    fn test_hashmap() {
        use std::collections::HashMap;

        let cstr = CString::new("foo").unwrap();
        let gstring = GString::from(cstr);
        assert_eq!(gstring.as_str(), "foo");
        let mut h: HashMap<GString, i32> = HashMap::new();
        h.insert(gstring, 42);
        let gstring: GString = "foo".into();
        assert!(h.contains_key(&gstring));
    }
}

// Take a look at the license at the top of the repository in the LICENSE file.

//! # Examples
//!
//! ```
//! use glib::prelude::*; // or `use gtk::prelude::*;`
//! use glib::ByteArray;
//!
//! let ba = ByteArray::from(b"def");
//! ba.append(b"ghi").prepend(b"abc");
//! ba.remove_range(3, 3);
//! assert_eq!(ba, "abcghi".as_bytes());
//! ```

use crate::translate::*;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::Deref;
use std::ptr::NonNull;
use std::slice;

use crate::Bytes;

wrapper! {
    pub struct ByteArray(Shared<ffi::GByteArray>);

    match fn {
        ref => |ptr| ffi::g_byte_array_ref(ptr),
        unref => |ptr| ffi::g_byte_array_unref(ptr),
        get_type => || ffi::g_byte_array_get_type(),
    }
}

impl ByteArray {
    #[doc(alias = "g_byte_array_new")]
    pub fn new() -> ByteArray {
        unsafe { from_glib_full(ffi::g_byte_array_new()) }
    }

    #[doc(alias = "g_byte_array_sized_new")]
    pub fn with_capacity(size: usize) -> ByteArray {
        unsafe { from_glib_full(ffi::g_byte_array_sized_new(size as u32)) }
    }

    #[doc(alias = "g_byte_array_free_to_bytes")]
    pub fn into_gbytes(self) -> Bytes {
        unsafe {
            let s = mem::ManuallyDrop::new(self);
            from_glib_full(ffi::g_byte_array_free_to_bytes(mut_override(
                s.to_glib_none().0,
            )))
        }
    }

    #[doc(alias = "g_byte_array_append")]
    pub fn append<T: ?Sized + AsRef<[u8]>>(&self, data: &T) -> &Self {
        let bytes = data.as_ref();
        unsafe {
            ffi::g_byte_array_append(
                self.to_glib_none().0,
                bytes.as_ptr() as *const _,
                bytes.len() as u32,
            );
        }
        self
    }

    #[doc(alias = "g_byte_array_prepend")]
    pub fn prepend<T: ?Sized + AsRef<[u8]>>(&self, data: &T) -> &Self {
        let bytes = data.as_ref();
        unsafe {
            ffi::g_byte_array_prepend(
                self.to_glib_none().0,
                bytes.as_ptr() as *const _,
                bytes.len() as u32,
            );
        }
        self
    }

    #[doc(alias = "g_byte_array_remove_index")]
    pub fn remove_index(&self, index: usize) {
        unsafe {
            ffi::g_byte_array_remove_index(self.to_glib_none().0, index as u32);
        }
    }

    #[doc(alias = "g_byte_array_remove_index_fast")]
    pub fn remove_index_fast(&self, index: usize) {
        unsafe {
            ffi::g_byte_array_remove_index_fast(self.to_glib_none().0, index as u32);
        }
    }

    #[doc(alias = "g_byte_array_remove_range")]
    pub fn remove_range(&self, index: usize, length: usize) {
        unsafe {
            ffi::g_byte_array_remove_range(self.to_glib_none().0, index as u32, length as u32);
        }
    }

    #[doc(alias = "g_byte_array_set_size")]
    pub unsafe fn set_size(&self, size: usize) {
        ffi::g_byte_array_set_size(self.to_glib_none().0, size as u32);
    }

    #[doc(alias = "g_byte_array_sort_with_data")]
    pub fn sort<F: FnMut(&u8, &u8) -> Ordering>(&self, compare_func: F) {
        unsafe extern "C" fn compare_func_trampoline(
            a: ffi::gconstpointer,
            b: ffi::gconstpointer,
            func: ffi::gpointer,
        ) -> i32 {
            let func = func as *mut &mut (dyn FnMut(&u8, &u8) -> Ordering);

            let a = &*(a as *const u8);
            let b = &*(b as *const u8);

            match (*func)(&a, &b) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            }
        }
        unsafe {
            let mut func = compare_func;
            let func_obj: &mut (dyn FnMut(&u8, &u8) -> Ordering) = &mut func;
            let func_ptr =
                &func_obj as *const &mut (dyn FnMut(&u8, &u8) -> Ordering) as ffi::gpointer;

            ffi::g_byte_array_sort_with_data(
                self.to_glib_none().0,
                Some(compare_func_trampoline),
                func_ptr,
            );
        }
    }
}

impl AsRef<[u8]> for ByteArray {
    fn as_ref(&self) -> &[u8] {
        &*self
    }
}

impl<'a, T: ?Sized + Borrow<[u8]> + 'a> From<&'a T> for ByteArray {
    fn from(value: &'a T) -> ByteArray {
        let ba = ByteArray::new();
        ba.append(value.borrow());
        ba
    }
}

impl Deref for ByteArray {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        unsafe {
            let mut ptr = (*self.to_glib_none().0).data;
            let len = (*self.to_glib_none().0).len as usize;
            debug_assert!(!ptr.is_null() || len == 0);
            if ptr.is_null() {
                ptr = NonNull::dangling().as_ptr();
            }
            slice::from_raw_parts(ptr as *const u8, len)
        }
    }
}

impl Default for ByteArray {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for ByteArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ByteArray")
            .field("ptr", &self.to_glib_none().0)
            .field("data", &&self[..])
            .finish()
    }
}

macro_rules! impl_cmp {
    ($lhs:ty, $rhs: ty) => {
        #[allow(clippy::redundant_slicing)]
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                self[..].eq(&other[..])
            }
        }

        #[allow(clippy::redundant_slicing)]
        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                self[..].eq(&other[..])
            }
        }

        #[allow(clippy::redundant_slicing)]
        impl<'a, 'b> PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<Ordering> {
                self[..].partial_cmp(&other[..])
            }
        }

        #[allow(clippy::redundant_slicing)]
        impl<'a, 'b> PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<Ordering> {
                self[..].partial_cmp(&other[..])
            }
        }
    };
}

impl_cmp!(ByteArray, [u8]);
impl_cmp!(ByteArray, &'a [u8]);
impl_cmp!(&'a ByteArray, [u8]);
impl_cmp!(ByteArray, Vec<u8>);
impl_cmp!(&'a ByteArray, Vec<u8>);

impl PartialEq for ByteArray {
    fn eq(&self, other: &Self) -> bool {
        self[..] == other[..]
    }
}

impl Eq for ByteArray {}

impl Hash for ByteArray {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        Hash::hash_slice(&self[..], state)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn various() {
        let ba: ByteArray = Default::default();
        ba.append("foo").append("bar").prepend("baz");
        ba.remove_index(0);
        ba.remove_index_fast(1);
        ba.remove_range(1, 2);
        ba.sort(|a, b| a.cmp(b));
        unsafe { ba.set_size(3) };
        assert_eq!(ba, b"aab" as &[u8]);
        let abc: &[u8] = b"abc";
        assert_eq!(ByteArray::from(abc), b"abc" as &[u8]);
    }

    #[test]
    fn hash() {
        let b1 = ByteArray::from(b"this is a test");
        let b2 = ByteArray::from(b"this is a test");
        let b3 = ByteArray::from(b"test");
        let mut set = HashSet::new();
        set.insert(b1);
        assert!(set.contains(&b2));
        assert!(!set.contains(&b3));
    }
}

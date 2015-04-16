// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

// FIXME: @jeremyletang implements the new index traits when it's available

use std::mem;
use libc::c_void;
use std::iter::{FromIterator, IntoIterator};
use std::ops::Index;
use std::marker::PhantomData;

use glib_container::GlibContainer;
use ffi;

pub struct SList<T> {
    pointer: *mut ffi::C_GSList,
    _marker: PhantomData<T>
}

pub struct SElem<'a, T: 'a> {
    pointer: *mut ffi::C_GSList,
    _marker: PhantomData<&'a T>
}

impl<T> SList<T> {
    pub fn new() -> SList<T> {
        SList {
            pointer: ::std::ptr::null_mut(),
            _marker: PhantomData
        }
    }

    pub fn from_vec(values: Vec<T>) -> SList<T> {
        FromIterator::from_iter(values.into_iter())
    }

    pub fn from_slice(values: &[T]) -> SList<T> where T: Clone {
        let v: Vec<T> = values.iter().map(|x| (*x).clone()).collect();
        FromIterator::from_iter(v.into_iter())
    }

    pub fn append(&mut self, data: T) {
        unsafe {
            self.pointer = ffi::g_slist_append(self.pointer, mem::transmute(Box::new(data)));
        }
    }

    pub fn prepend(&mut self, data: T) {
        unsafe {
            self.pointer = ffi::g_slist_prepend(self.pointer, mem::transmute(Box::new(data)));
        }
    }

    pub fn nth(&self, n: u32) -> &T {
        unsafe {
            mem::transmute::<*mut c_void, &T>(ffi::g_slist_nth_data(self.pointer, n))
        }
    }

    pub fn last(&self) -> &T {
        let elem = unsafe { ffi::g_slist_last(self.pointer) };
        unsafe { mem::transmute::<*mut c_void, &T>((*elem).data)}
    }

    pub fn insert(&mut self, data: T, position: i32) {
        unsafe {
            self.pointer = ffi::g_slist_insert(self.pointer, mem::transmute(Box::new(data)), position);
        }
    }

    pub fn concat(&mut self, list: SList<T>) {
        unsafe {
            ffi::g_slist_concat(self.pointer, list.unwrap());
        }
    }

    pub fn reverse(&mut self) {
        unsafe {
            self.pointer = ffi::g_slist_reverse(self.pointer);
        }
    }

    pub fn iter(&self) -> SElem<T> {
        SElem {
            pointer: self.pointer,
            _marker: PhantomData
        }
    }

    pub fn len(&self) -> usize {
        unsafe { ffi::g_slist_length(self.pointer) as usize }
    }

    pub fn clear(&mut self) {
        unsafe {
            ffi::g_slist_free(self.pointer)
        }
    }

    pub fn extend<It: IntoIterator<Item=T>>(&mut self, it: It) {
        for elem in it {
            self.append(elem);
        }
    }
}

impl<T> Index<usize> for SList<T> {
    type Output = T;

    fn index<'a>(&'a self, _rhs: usize) -> &'a T {
        self.nth(_rhs as u32)
    }
}

impl<'a, T> Iterator for SElem<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.pointer.is_null() {
            None
        } else {
            let ret = unsafe { mem::transmute::<*mut c_void, &T>((*self.pointer).data)};
            unsafe { self.pointer = (*self.pointer).next; }
            Some(ret)
        }
    }
}

impl<T> FromIterator<T> for SList<T> {
    fn from_iter<It: IntoIterator<Item=T>>(it: It) -> SList<T> {
        let mut new_list = SList::new();
        new_list.extend(it);
        new_list
    }
}

impl<T> Clone for SList<T> {
    fn clone(&self) -> SList<T> {
        unsafe {
            GlibContainer::wrap(ffi::g_slist_copy(self.pointer))
        }
    }
}

impl<T> Drop for SList<T> {
    fn drop(&mut self) {
        unsafe { ffi::g_slist_free(self.pointer); }
    }
}

impl<T> GlibContainer<*mut ffi::C_GSList> for SList<T> {
    fn wrap(pointer: *mut ffi::C_GSList) -> SList<T> {
        SList {
            pointer: pointer,
            _marker: PhantomData
        }
    }

    fn unwrap(&self) -> *mut ffi::C_GSList {
        self.pointer
    }
}

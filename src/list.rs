// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

// FIXME: @jeremyletang implements the new index traits when it's available

use libc::c_void;
use std::mem;
use std::ops::Index;
use std::iter::{FromIterator, IntoIterator};
use std::marker::PhantomData;
use glib_ffi;

use glib_container::GlibContainer;

pub struct List<T> {
    pointer: *mut glib_ffi::GList,
    _marker: PhantomData<T>
}

pub struct Elem<'a, T: 'a> {
    pointer: *mut glib_ffi::GList,
    _marker: PhantomData<&'a T>
}

pub struct RevElem<'a, T: 'a> {
    pointer: *mut glib_ffi::GList,
    _marker: PhantomData<&'a T>
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            pointer: ::std::ptr::null_mut(),
            _marker: PhantomData
        }
    }

    pub fn from_vec(values: Vec<T>) -> List<T> {
        FromIterator::from_iter(values.into_iter())
    }

    pub fn from_slice(values: &[T]) -> List<T> where T: Clone {
        let v: Vec<T> = values.iter().map(|x| (*x).clone()).collect();
        FromIterator::from_iter(v.into_iter())
    }

    pub fn append(&mut self, data: T) {
        unsafe {
            self.pointer = glib_ffi::g_list_append(self.pointer, mem::transmute(Box::new(data)));
        }
    }

    pub fn prepend(&mut self, data: T) {
        unsafe {
            self.pointer = glib_ffi::g_list_prepend(self.pointer, mem::transmute(Box::new(data)));
        }
    }

    pub fn nth(&self, n: u32) -> &T {
        unsafe {
            mem::transmute::<*mut c_void, &T>(glib_ffi::g_list_nth_data(self.pointer, n))
        }
    }

    pub fn last(&self) -> &T {
        let elem = unsafe { glib_ffi::g_list_last(self.pointer) };
        unsafe { mem::transmute::<*mut c_void, &T>((*elem).data)}
    }

    pub fn first(&self) -> &T {
        let elem = unsafe { glib_ffi::g_list_first(self.pointer) };
        unsafe { mem::transmute::<*mut c_void, &T>((*elem).data)}
    }

    pub fn insert(&mut self, data: T, position: i32) {
        unsafe {
            self.pointer = glib_ffi::g_list_insert(self.pointer, mem::transmute(Box::new(data)), position);
        }
    }

    pub fn concat(&mut self, list: List<T>) {
        unsafe {
            glib_ffi::g_list_concat(self.pointer, list.unwrap());
        }
    }

    pub fn reverse(&mut self) {
        unsafe {
            self.pointer = glib_ffi::g_list_reverse(self.pointer);
        }
    }

    pub fn iter(&self) -> Elem<T> {
        Elem {
            pointer: unsafe { glib_ffi::g_list_first(self.pointer) },
            _marker: PhantomData
        }
    }

    pub fn rev_iter(&self) -> RevElem<T> {
        RevElem {
            pointer: unsafe { glib_ffi::g_list_last(self.pointer) },
            _marker: PhantomData
        }
    }

    pub fn len(&self) -> usize {
        unsafe { glib_ffi::g_list_length(self.pointer) as usize }
    }

    pub fn clear(&mut self) {
        unsafe {
            glib_ffi::g_list_free(self.pointer)
        }
    }

    pub fn extend<It: IntoIterator<Item=T>>(&mut self, it: It) {
        for elem in it {
            self.append(elem);
        }
    }
}

impl<T> Index<usize> for List<T> {
    type Output = T;

    fn index<'a>(&'a self, _rhs: usize) -> &'a T {
        self.nth(_rhs as u32)
    }
}

impl<'a, T> Iterator for Elem<'a, T> {
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

impl<'a, T> Iterator for RevElem<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.pointer.is_null() {
            None
        } else {
            let ret = unsafe { mem::transmute::<*mut c_void, &T>((*self.pointer).data)};
            unsafe { self.pointer = (*self.pointer).prev; }
            Some(ret)
        }
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<It: IntoIterator<Item=T>>(it: It) -> List<T> {
        let mut new_list = List::new();
        new_list.extend(it);
        new_list
    }
}

impl<T> Clone for List<T> {
    fn clone(&self) -> List<T> {
        unsafe {
            GlibContainer::wrap(glib_ffi::g_list_copy(self.pointer))
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        unsafe { glib_ffi::g_list_free(self.pointer); }
    }
}

impl<T> GlibContainer<*mut glib_ffi::GList> for List<T> {
    fn wrap(pointer: *mut glib_ffi::GList) -> List<T> {
        List {
            pointer: pointer,
            _marker: PhantomData
        }
    }

    fn unwrap(&self) -> *mut glib_ffi::GList {
        self.pointer
    }
}

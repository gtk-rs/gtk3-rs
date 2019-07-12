// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use pango_sys;
use std::mem;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Rectangle {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }
}

#[doc(hidden)]
impl Uninitialized for Rectangle {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const pango_sys::PangoRectangle> for Rectangle {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const pango_sys::PangoRectangle, Self> {
        let ptr: *const Rectangle = &*self;
        Stash(ptr as *const pango_sys::PangoRectangle, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut pango_sys::PangoRectangle> for Rectangle {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut pango_sys::PangoRectangle, Self> {
        let ptr: *mut Rectangle = &mut *self;
        StashMut(ptr as *mut pango_sys::PangoRectangle, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const pango_sys::PangoRectangle> for Rectangle {
    unsafe fn from_glib_none(ptr: *const pango_sys::PangoRectangle) -> Self {
        *(ptr as *const Rectangle)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut pango_sys::PangoRectangle> for Rectangle {
    unsafe fn from_glib_none(ptr: *mut pango_sys::PangoRectangle) -> Self {
        *(ptr as *mut Rectangle)
    }
}

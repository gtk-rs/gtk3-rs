// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
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
impl<'a> ToGlibPtr<'a, *const ffi::PangoRectangle> for Rectangle {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::PangoRectangle, Self> {
        let ptr: *const Rectangle = &*self;
        Stash(ptr as *const ffi::PangoRectangle, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::PangoRectangle> for Rectangle {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::PangoRectangle, Self> {
        let ptr: *mut Rectangle = &mut *self;
        StashMut(ptr as *mut ffi::PangoRectangle, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::PangoRectangle> for Rectangle {
    unsafe fn from_glib_none(ptr: *const ffi::PangoRectangle) -> Self {
        *(ptr as *const Rectangle)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::PangoRectangle> for Rectangle {
    unsafe fn from_glib_none(ptr: *mut ffi::PangoRectangle) -> Self {
        *(ptr as *mut Rectangle)
    }
}

use ffi;
use glib::translate::*;
use std::mem;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct RectangleInt {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[doc(hidden)]
impl Uninitialized for RectangleInt {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::uninitialized()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::cairo_rectangle_int_t> for RectangleInt {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::cairo_rectangle_int_t, Self> {
        let ptr: *const RectangleInt = &*self;
        Stash(ptr as *const ffi::cairo_rectangle_int_t, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::cairo_rectangle_int_t> for RectangleInt {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::cairo_rectangle_int_t, Self> {
        let ptr: *mut RectangleInt = &mut *self;
        StashMut(ptr as *mut ffi::cairo_rectangle_int_t, self)
    }
}

#[doc(hidden)]
impl FromGlibPtr<*const ffi::cairo_rectangle_int_t> for RectangleInt {
    unsafe fn from_glib_none(ptr: *const ffi::cairo_rectangle_int_t) -> Self {
        *(ptr as *const RectangleInt)
    }

    unsafe fn from_glib_full(_: *const ffi::cairo_rectangle_int_t) -> Self {
        panic!()
    }
}

#[doc(hidden)]
impl FromGlibPtr<*mut ffi::cairo_rectangle_int_t> for RectangleInt {
    unsafe fn from_glib_none(ptr: *mut ffi::cairo_rectangle_int_t) -> Self {
        *(ptr as *mut RectangleInt)
    }

    unsafe fn from_glib_full(_: *mut ffi::cairo_rectangle_int_t) -> Self {
        panic!()
    }
}

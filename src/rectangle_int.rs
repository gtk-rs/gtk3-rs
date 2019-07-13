use ffi;
#[cfg(feature = "use_glib")]
use glib::translate::*;
use std::fmt;
#[cfg(feature = "use_glib")]
use std::mem;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct RectangleInt {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl Uninitialized for RectangleInt {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::cairo_rectangle_int_t> for RectangleInt {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::cairo_rectangle_int_t, Self> {
        let ptr: *const RectangleInt = &*self;
        Stash(ptr as *const ffi::cairo_rectangle_int_t, self)
    }
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::cairo_rectangle_int_t> for RectangleInt {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::cairo_rectangle_int_t, Self> {
        let ptr: *mut RectangleInt = &mut *self;
        StashMut(ptr as *mut ffi::cairo_rectangle_int_t, self)
    }
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::cairo_rectangle_int_t> for RectangleInt {
    unsafe fn from_glib_none(ptr: *const ffi::cairo_rectangle_int_t) -> Self {
        *(ptr as *const RectangleInt)
    }
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::cairo_rectangle_int_t> for RectangleInt {
    unsafe fn from_glib_borrow(ptr: *mut ffi::cairo_rectangle_int_t) -> Self {
        *(ptr as *mut RectangleInt)
    }
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::cairo_rectangle_int_t> for RectangleInt {
    unsafe fn from_glib_none(ptr: *mut ffi::cairo_rectangle_int_t) -> Self {
        *(ptr as *mut RectangleInt)
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(
    RectangleInt,
    ffi::cairo_rectangle_int_t,
    ffi::gobject::cairo_gobject_rectangle_int_get_type
);

impl RectangleInt {
    pub fn to_raw_none(&self) -> *mut ffi::cairo_rectangle_int_t {
        let ptr = &*self as *const RectangleInt as usize;
        ptr as *mut ffi::cairo_rectangle_int_t
    }
}

impl fmt::Display for RectangleInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RectangleInt")
    }
}

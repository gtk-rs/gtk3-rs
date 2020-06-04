// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use enums::RegionOverlap;
use error::Error;
use ffi;
#[cfg(feature = "use_glib")]
use glib::translate::*;
use std::fmt;
use std::ptr;
use utils::status_to_result;
use RectangleInt;

use ffi::cairo_region_t;

#[derive(Debug)]
pub struct Region(ptr::NonNull<cairo_region_t>);

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut ffi::cairo_region_t> for &'a Region {
    type Storage = &'a Region;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut ffi::cairo_region_t, &'a Region> {
        Stash(self.0.as_ptr(), *self)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut ffi::cairo_region_t {
        unsafe { ffi::cairo_region_reference(self.0.as_ptr()) }
    }
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::cairo_region_t> for Region {
    type Storage = &'a mut Self;

    // FIXME: This is unsafe: regions are reference counted so we could get multiple mutable
    // references here
    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::cairo_region_t, Self> {
        StashMut(self.0.as_ptr(), self)
    }
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::cairo_region_t> for Region {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::cairo_region_t) -> Region {
        Self::from_raw_none(ptr)
    }
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::cairo_region_t> for Region {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::cairo_region_t) -> ::Borrowed<Region> {
        Self::from_raw_borrow(ptr)
    }
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl FromGlibPtrFull<*mut ffi::cairo_region_t> for Region {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::cairo_region_t) -> Region {
        Self::from_raw_full(ptr)
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(
    Region,
    ffi::cairo_region_t,
    ffi::gobject::cairo_gobject_region_get_type
);

impl Clone for Region {
    fn clone(&self) -> Region {
        unsafe { Self::from_raw_none(self.to_raw_none()) }
    }
}

impl Drop for Region {
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_region_destroy(self.0.as_ptr());
        }
    }
}

impl PartialEq for Region {
    fn eq(&self, other: &Region) -> bool {
        unsafe { ffi::cairo_region_equal(self.0.as_ptr(), other.0.as_ptr()).as_bool() }
    }
}

impl Eq for Region {}

impl Region {
    #[inline]
    pub unsafe fn from_raw_none(ptr: *mut ffi::cairo_region_t) -> Region {
        assert!(!ptr.is_null());
        ffi::cairo_region_reference(ptr);
        Region(ptr::NonNull::new_unchecked(ptr))
    }

    #[inline]
    pub unsafe fn from_raw_borrow(ptr: *mut ffi::cairo_region_t) -> ::Borrowed<Region> {
        assert!(!ptr.is_null());
        ::Borrowed::new(Region(ptr::NonNull::new_unchecked(ptr)))
    }

    #[inline]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_region_t) -> Region {
        assert!(!ptr.is_null());
        Region(ptr::NonNull::new_unchecked(ptr))
    }

    pub fn to_raw_none(&self) -> *mut ffi::cairo_region_t {
        self.0.as_ptr()
    }

    pub fn create() -> Region {
        unsafe { Self::from_raw_full(ffi::cairo_region_create()) }
    }

    pub fn create_rectangle(rectangle: &RectangleInt) -> Region {
        unsafe { Self::from_raw_full(ffi::cairo_region_create_rectangle(rectangle.to_raw_none())) }
    }

    pub fn create_rectangles(rectangles: &[RectangleInt]) -> Region {
        unsafe {
            Self::from_raw_full(ffi::cairo_region_create_rectangles(
                rectangles.as_ptr() as *mut ffi::cairo_rectangle_int_t,
                rectangles.len() as i32,
            ))
        }
    }

    pub fn copy(&self) -> Region {
        unsafe { Self::from_raw_full(ffi::cairo_region_copy(self.0.as_ptr())) }
    }

    pub fn get_extents(&self, rectangle: &mut RectangleInt) {
        unsafe { ffi::cairo_region_get_extents(self.0.as_ptr(), rectangle.to_raw_none()) }
    }

    pub fn num_rectangles(&self) -> i32 {
        unsafe { ffi::cairo_region_num_rectangles(self.0.as_ptr()) }
    }

    pub fn get_rectangle(&self, nth: i32) -> RectangleInt {
        unsafe {
            let rectangle: RectangleInt = ::std::mem::zeroed();
            ffi::cairo_region_get_rectangle(self.0.as_ptr(), nth, rectangle.to_raw_none());
            rectangle
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { ffi::cairo_region_is_empty(self.0.as_ptr()).as_bool() }
    }

    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        unsafe { ffi::cairo_region_contains_point(self.0.as_ptr(), x, y).as_bool() }
    }

    pub fn contains_rectangle(&self, rectangle: &RectangleInt) -> RegionOverlap {
        unsafe {
            RegionOverlap::from(ffi::cairo_region_contains_rectangle(
                self.0.as_ptr(),
                rectangle.to_raw_none(),
            ))
        }
    }

    pub fn translate(&self, dx: i32, dy: i32) {
        unsafe { ffi::cairo_region_translate(self.0.as_ptr(), dx, dy) }
    }

    pub fn intersect(&self, other: &Region) -> Result<(), Error> {
        unsafe {
            let status = ffi::cairo_region_intersect(self.0.as_ptr(), other.0.as_ptr());
            status_to_result(status)
        }
    }

    pub fn intersect_rectangle(&self, rectangle: &RectangleInt) -> Result<(), Error> {
        unsafe {
            let status =
                ffi::cairo_region_intersect_rectangle(self.0.as_ptr(), rectangle.to_raw_none());
            status_to_result(status)
        }
    }

    pub fn subtract(&self, other: &Region) -> Result<(), Error> {
        unsafe {
            let status = ffi::cairo_region_subtract(self.0.as_ptr(), other.0.as_ptr());
            status_to_result(status)
        }
    }

    pub fn subtract_rectangle(&self, rectangle: &RectangleInt) -> Result<(), Error> {
        unsafe {
            let status =
                ffi::cairo_region_subtract_rectangle(self.0.as_ptr(), rectangle.to_raw_none());
            status_to_result(status)
        }
    }

    pub fn union(&self, other: &Region) -> Result<(), Error> {
        unsafe {
            let status = ffi::cairo_region_union(self.0.as_ptr(), other.0.as_ptr());
            status_to_result(status)
        }
    }

    pub fn union_rectangle(&self, rectangle: &RectangleInt) -> Result<(), Error> {
        unsafe {
            let status =
                ffi::cairo_region_union_rectangle(self.0.as_ptr(), rectangle.to_raw_none());
            status_to_result(status)
        }
    }

    pub fn xor(&self, other: &Region) -> Result<(), Error> {
        unsafe {
            let status = ffi::cairo_region_xor(self.0.as_ptr(), other.0.as_ptr());
            status_to_result(status)
        }
    }

    pub fn xor_rectangle(&self, rectangle: &RectangleInt) -> Result<(), Error> {
        unsafe {
            let status = ffi::cairo_region_xor_rectangle(self.0.as_ptr(), rectangle.to_raw_none());
            status_to_result(status)
        }
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Region")
    }
}

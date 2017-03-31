// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::convert::{AsRef, From};
use std::mem;

use glib;
use glib::translate::*;
use glib_ffi;
use glib_ffi::gconstpointer;
use gobject_ffi;
use cairo::RectangleInt;
use ffi;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    pub fn intersect(&self, other: &Rectangle) -> Option<Rectangle> {
        unsafe {
            let mut ret = Rectangle::uninitialized();
            if from_glib(ffi::gdk_rectangle_intersect(self.to_glib_none().0, other.to_glib_none().0,
                    ret.to_glib_none_mut().0)) {
                Some(ret)
            }
            else {
                None
            }
        }
    }

    pub fn union(&self, other: &Rectangle) -> Rectangle {
        unsafe {
            let mut ret = Rectangle::uninitialized();
            ffi::gdk_rectangle_union(self.to_glib_none().0, other.to_glib_none().0,
                ret.to_glib_none_mut().0);
            ret
        }
    }
}

#[doc(hidden)]
impl Uninitialized for Rectangle {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::uninitialized()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GdkRectangle> for Rectangle {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GdkRectangle, Self> {
        let ptr: *const Rectangle = &*self;
        Stash(ptr as *const ffi::GdkRectangle, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GdkRectangle> for Rectangle {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GdkRectangle, Self> {
        let ptr: *mut Rectangle = &mut *self;
        StashMut(ptr as *mut ffi::GdkRectangle, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::GdkRectangle> for Rectangle {
    unsafe fn from_glib_none(ptr: *const ffi::GdkRectangle) -> Self {
        *(ptr as *const Rectangle)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::GdkRectangle> for Rectangle {
    unsafe fn from_glib_none(ptr: *mut ffi::GdkRectangle) -> Self {
        *(ptr as *mut Rectangle)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut ffi::GdkRectangle> for Rectangle {
    unsafe fn from_glib_full(ptr: *mut ffi::GdkRectangle) -> Self {
        let rect = *(ptr as *mut Rectangle);
        glib_ffi::g_free(ptr as *mut _);
        rect
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*const ffi::GdkRectangle> for Rectangle {
    unsafe fn from_glib_full(ptr: *const ffi::GdkRectangle) -> Self {
        let rect = *(ptr as *const Rectangle);
        glib_ffi::g_free(ptr as *mut _);
        rect
    }
}

impl AsRef<RectangleInt> for Rectangle {
    fn as_ref(&self) -> &RectangleInt {
        unsafe { mem::transmute(self) }
    }
}

impl From<RectangleInt> for Rectangle {
    fn from(r: RectangleInt) -> Rectangle {
        skip_assert_initialized!();
        unsafe { mem::transmute(r) }
    }
}

impl glib::StaticType for Rectangle {
    fn static_type() -> glib::types::Type {
        skip_assert_initialized!();
        unsafe { from_glib(ffi::gdk_rectangle_get_type()) }
    }
}

impl glib::value::FromValueOptional for Rectangle {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        from_glib_full(gobject_ffi::g_value_dup_boxed(value.to_glib_none().0) as *mut ffi::GdkRectangle)
    }
}

impl glib::value::SetValue for Rectangle {
    unsafe fn set_value(value: &mut glib::Value, this: &Self)  {
        gobject_ffi::g_value_set_boxed(value.to_glib_none_mut().0,
                                       this.to_glib_none().0 as gconstpointer)
    }
}

impl glib::value::SetValueOptional for Rectangle {
    unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
        gobject_ffi::g_value_set_boxed(value.to_glib_none_mut().0,
                                       this.to_glib_none().0 as gconstpointer)
    }
}

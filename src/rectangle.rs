// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use cairo::RectangleInt;
use gdk_sys;
use glib;
use glib::translate::*;
use glib_sys;
use glib_sys::gconstpointer;
use gobject_sys;
use std::convert::{AsRef, From};
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
    pub fn intersect(&self, other: &Rectangle) -> Option<Rectangle> {
        unsafe {
            let mut ret = Rectangle::uninitialized();
            if from_glib(gdk_sys::gdk_rectangle_intersect(
                self.to_glib_none().0,
                other.to_glib_none().0,
                ret.to_glib_none_mut().0,
            )) {
                Some(ret)
            } else {
                None
            }
        }
    }

    pub fn union(&self, other: &Rectangle) -> Rectangle {
        unsafe {
            let mut ret = Rectangle::uninitialized();
            gdk_sys::gdk_rectangle_union(
                self.to_glib_none().0,
                other.to_glib_none().0,
                ret.to_glib_none_mut().0,
            );
            ret
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
impl<'a> ToGlibPtr<'a, *const gdk_sys::GdkRectangle> for Rectangle {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const gdk_sys::GdkRectangle, Self> {
        let ptr: *const Rectangle = &*self;
        Stash(ptr as *const gdk_sys::GdkRectangle, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut gdk_sys::GdkRectangle> for Rectangle {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut gdk_sys::GdkRectangle, Self> {
        let ptr: *mut Rectangle = &mut *self;
        StashMut(ptr as *mut gdk_sys::GdkRectangle, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const gdk_sys::GdkRectangle> for Rectangle {
    unsafe fn from_glib_none(ptr: *const gdk_sys::GdkRectangle) -> Self {
        *(ptr as *const Rectangle)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut gdk_sys::GdkRectangle> for Rectangle {
    unsafe fn from_glib_none(ptr: *mut gdk_sys::GdkRectangle) -> Self {
        *(ptr as *mut Rectangle)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const gdk_sys::GdkRectangle> for Rectangle {
    unsafe fn from_glib_borrow(
        ptr: *const gdk_sys::GdkRectangle,
    ) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const Rectangle))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut gdk_sys::GdkRectangle> for Rectangle {
    unsafe fn from_glib_borrow(ptr: *mut gdk_sys::GdkRectangle) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut Rectangle))
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut gdk_sys::GdkRectangle> for Rectangle {
    unsafe fn from_glib_full(ptr: *mut gdk_sys::GdkRectangle) -> Self {
        let rect = *(ptr as *mut Rectangle);
        glib_sys::g_free(ptr as *mut _);
        rect
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*const gdk_sys::GdkRectangle> for Rectangle {
    unsafe fn from_glib_full(ptr: *const gdk_sys::GdkRectangle) -> Self {
        let rect = *(ptr as *const Rectangle);
        glib_sys::g_free(ptr as *mut _);
        rect
    }
}

impl AsRef<RectangleInt> for Rectangle {
    fn as_ref(&self) -> &RectangleInt {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl From<RectangleInt> for Rectangle {
    fn from(r: RectangleInt) -> Rectangle {
        skip_assert_initialized!();
        unsafe { *(&r as *const _ as *const _) }
    }
}

impl glib::StaticType for Rectangle {
    fn static_type() -> glib::types::Type {
        skip_assert_initialized!();
        unsafe { from_glib(gdk_sys::gdk_rectangle_get_type()) }
    }
}

impl<'a> glib::value::FromValueOptional<'a> for Rectangle {
    unsafe fn from_value_optional(value: &'a glib::Value) -> Option<Self> {
        from_glib_full(
            gobject_sys::g_value_dup_boxed(value.to_glib_none().0) as *mut gdk_sys::GdkRectangle
        )
    }
}

impl glib::value::SetValue for Rectangle {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        gobject_sys::g_value_set_boxed(
            value.to_glib_none_mut().0,
            this.to_glib_none().0 as gconstpointer,
        )
    }
}

impl glib::value::SetValueOptional for Rectangle {
    unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
        gobject_sys::g_value_set_boxed(
            value.to_glib_none_mut().0,
            this.to_glib_none().0 as gconstpointer,
        )
    }
}

// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::mem;
use libc::c_void;

#[cfg(feature = "use_glib")]
use glib::translate::*;
use ffi;
use ffi::enums::{
    Content,
    Status,
    SurfaceType,
};

#[derive(Debug)]
pub struct Surface(*mut ffi::cairo_surface_t, bool);

impl Surface {
    pub unsafe fn from_raw_none(ptr: *mut ffi::cairo_surface_t) -> Surface {
        assert!(!ptr.is_null());
        ffi::cairo_surface_reference(ptr);
        Surface(ptr, false)
    }

    pub unsafe fn from_raw_borrow(ptr: *mut ffi::cairo_surface_t) -> Surface {
        assert!(!ptr.is_null());
        Surface(ptr, true)
    }


    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_surface_t) -> Surface {
        assert!(!ptr.is_null());
        Surface(ptr, false)
    }

    pub fn to_raw_none(&self) -> *mut ffi::cairo_surface_t {
        self.0
    }

    pub fn create_similar(&self, content: Content, width: i32, height: i32) -> Surface {
        unsafe { Self::from_raw_full(ffi::cairo_surface_create_similar(self.0, content, width, height)) }
    }
}

#[cfg(feature = "use_glib")]
impl<'a> ToGlibPtr<'a, *mut ffi::cairo_surface_t> for Surface {
    type Storage = &'a Surface;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::cairo_surface_t, Self> {
        Stash(self.to_raw_none(), self)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut ffi::cairo_surface_t {
        unsafe {
            ffi::cairo_surface_reference(self.to_raw_none())
        }
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrNone<*mut ffi::cairo_surface_t> for Surface {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::cairo_surface_t) -> Surface {
        Self::from_raw_none(ptr)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrBorrow<*mut ffi::cairo_surface_t> for Surface {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::cairo_surface_t) -> Surface {
        Self::from_raw_borrow(ptr)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrFull<*mut ffi::cairo_surface_t> for Surface {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::cairo_surface_t) -> Surface {
        Self::from_raw_full(ptr)
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Surface, ffi::cairo_surface_t, ffi::gobject::cairo_gobject_surface_get_type);

impl AsRef<Surface> for Surface {
    fn as_ref(&self) -> &Surface {
        self
    }
}

impl Clone for Surface {
    fn clone(&self) -> Surface {
        unsafe { Self::from_raw_none(self.0) }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        if !self.1 {
            unsafe { ffi::cairo_surface_destroy(self.0); }
        }
    }
}

pub trait SurfaceExt {
    fn flush(&self);
    fn finish(&self);
    fn get_type(&self) -> SurfaceType;
    fn status(&self) -> Status;
}

impl<O: AsRef<Surface>> SurfaceExt for O {
    fn flush(&self) {
        unsafe { ffi::cairo_surface_flush(self.as_ref().0); }
    }

    fn finish(&self) {
        unsafe { ffi::cairo_surface_finish(self.as_ref().0); }
    }

    fn get_type(&self) -> SurfaceType {
        unsafe { ffi::cairo_surface_get_type(self.as_ref().0) }
    }

    fn status(&self) -> Status {
        unsafe { ffi::cairo_surface_status(self.as_ref().0) }
    }
}

pub(crate) trait SurfacePriv {
    unsafe fn set_user_data<K, T>(&self, key: &K, data: Box<T>) -> Result<(), Status>;
}

impl<O: AsRef<Surface>> SurfacePriv for O {
    unsafe fn set_user_data<K, T>(&self, key: &K, data: Box<T>) -> Result<(), Status> {
        let ptr: *mut T = Box::into_raw(data);

        assert_eq!(mem::size_of::<*mut c_void>(), mem::size_of_val(&ptr));

        let status = ffi::cairo_surface_set_user_data(self.as_ref().0, key as *const _ as *mut _,
            ptr as *mut c_void, Some(unbox::<T>));
        match status {
            Status::Success => Ok(()),
            x => Err(x),
        }
    }
}

unsafe extern "C" fn unbox<T>(data: *mut c_void) {
    let data: Box<T> = Box::from_raw(data as *mut T);
    drop(data);
}

// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ops::{Deref, DerefMut};
use std::slice;

use glib::translate::*;
use ffi;
use ffi::enums::{
    Format,
    SurfaceType,
};

use BorrowError;
use surface::{Surface, SurfaceExt, SurfacePriv};
use Status;

#[derive(Debug)]
pub struct ImageSurface(Surface);

impl ImageSurface {
    pub fn from(surface: Surface) -> Result<ImageSurface, Surface> {
        if surface.get_type() == SurfaceType::Image {
            Ok(ImageSurface(surface))
        }
        else {
            Err(surface)
        }
    }

    pub fn create(format: Format, width: i32, height: i32) -> ImageSurface {
        unsafe { from_glib_full(ffi::cairo_image_surface_create(format, width, height)) }
    }

    pub fn create_for_data<F>(data: Box<[u8]>, free: F, format: Format, width: i32, height: i32,
                              stride: i32) -> ImageSurface
    where F: FnOnce(Box<[u8]>) + 'static {
        assert!(data.len() >= (height * stride) as usize);
        unsafe {
            let mut data = Box::new(AsyncBorrow::new(data, free));
            let ptr = (*data).as_mut().as_mut_ptr();
            let surface = ImageSurface::from_glib_full(
                ffi::cairo_image_surface_create_for_data(ptr, format, width, height, stride));
            surface.set_user_data(&IMAGE_SURFACE_DATA, data).unwrap();
            surface
        }
    }

    pub fn get_data(&mut self) -> Result<ImageSurfaceData, BorrowError> {
        unsafe {
            if ffi::cairo_surface_get_reference_count(self.to_glib_none().0) > 1 {
                return Err(BorrowError::NonExclusive)
            }
            self.flush();
            match self.status() {
                Status::Success => (),
                status => return Err(BorrowError::from(status)),
            }
            if ffi::cairo_image_surface_get_data(self.to_glib_none().0).is_null() {
                return Err(BorrowError::from(Status::SurfaceFinished))
            }
            Ok(ImageSurfaceData::new(self))
        }
    }

    pub fn get_format(&self) -> Format {
        unsafe { ffi::cairo_image_surface_get_format(self.to_glib_none().0) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { ffi::cairo_image_surface_get_height(self.to_glib_none().0) }
    }

    pub fn get_stride(&self) -> i32 {
        unsafe { ffi::cairo_image_surface_get_stride(self.to_glib_none().0) }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { ffi::cairo_image_surface_get_width(self.to_glib_none().0) }
    }
}

static IMAGE_SURFACE_DATA: () = ();

impl<'a> ToGlibPtr<'a, *mut ffi::cairo_surface_t> for ImageSurface {
    type Storage = &'a Surface;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::cairo_surface_t, Self> {
        let stash = self.0.to_glib_none();
        Stash(stash.0, stash.1)
    }
}

impl FromGlibPtrNone<*mut ffi::cairo_surface_t> for ImageSurface {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::cairo_surface_t) -> ImageSurface {
        Self::from(from_glib_none(ptr)).unwrap()
    }
}

impl FromGlibPtrFull<*mut ffi::cairo_surface_t> for ImageSurface {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::cairo_surface_t) -> ImageSurface {
        Self::from(from_glib_full(ptr)).unwrap()
    }
}

impl AsRef<Surface> for ImageSurface {
    fn as_ref(&self) -> &Surface {
        &self.0
    }
}

impl Deref for ImageSurface {
    type Target = Surface;

    fn deref(&self) -> &Surface {
        &self.0
    }
}

impl Clone for ImageSurface {
    fn clone(&self) -> ImageSurface {
        unsafe { from_glib_none(self.to_glib_none().0) }
    }
}

pub struct ImageSurfaceData<'a> {
    surface: &'a mut ImageSurface,
    slice: &'a mut [u8],
    dirty: bool,
}

impl<'a> ImageSurfaceData<'a> {
    fn new(surface: &'a mut ImageSurface) -> ImageSurfaceData<'a> {
        unsafe {
            let ptr = ffi::cairo_image_surface_get_data(surface.to_glib_none().0);
            debug_assert!(!ptr.is_null());
            let len = (surface.get_stride() as usize) * (surface.get_height() as usize);
            ImageSurfaceData {
                surface: surface,
                slice: slice::from_raw_parts_mut(ptr, len),
                dirty: false,
            }
        }
    }
}

impl<'a> Drop for ImageSurfaceData<'a> {
    fn drop(&mut self) {
        if self.dirty {
            unsafe { ffi::cairo_surface_mark_dirty(self.surface.to_glib_none().0) }
        }
    }
}

impl<'a> Deref for ImageSurfaceData<'a> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.slice
    }
}

impl<'a> DerefMut for ImageSurfaceData<'a> {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.dirty = true;
        self.slice
    }
}

struct AsyncBorrow<T, F: FnOnce(T) + 'static> {
    data: Option<T>,
    free: Option<F>,
}

impl<T, F: FnOnce(T) + 'static> AsyncBorrow<T, F> {
    fn new(data: T, free: F) -> Self {
        AsyncBorrow { data: Some(data), free: Some(free) }
    }
}

impl<T, F: FnOnce(T) + 'static> AsMut<T> for AsyncBorrow<T, F> {
    fn as_mut(&mut self) -> &mut T {
        self.data.as_mut().unwrap()
    }
}

impl<T, F: FnOnce(T) + 'static> Drop for AsyncBorrow<T, F> {
    fn drop(&mut self) {
        if let (Some(x), Some(f)) = (self.data.take(), self.free.take()) {
            f(x);
        }
    }
}

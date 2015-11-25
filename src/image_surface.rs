// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ptr;
use std::slice;

use glib::translate::*;
use ffi;
use ffi::enums::{
    Format,
    SurfaceType,
};

use surface::{Surface, SurfaceExt, SurfacePriv};

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
        unsafe {
            let mut data = Box::new(AsyncBorrow::new(data, free));
            let ptr = (*data).as_mut().as_mut_ptr();
            let surface = ImageSurface::from_glib_full(
                ffi::cairo_image_surface_create_for_data(ptr, format, width, height, stride));
            surface.set_user_data(&IMAGE_SURFACE_DATA, data).unwrap();
            surface
        }
    }

    pub fn get_data(&self, buffer: &mut [u8]) {
        unsafe {
            let len = self.len();
            assert!(buffer.len() >= len);
            ptr::copy(self.get_data_unsafe().as_ptr(), buffer.as_mut_ptr(), len);
        }
    }

    pub unsafe fn get_data_unsafe<'a>(&self) -> &'a mut [u8] {
        let ptr = ffi::cairo_image_surface_get_data(self.to_glib_none().0);
        slice::from_raw_parts_mut(ptr, self.len())
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

    pub fn len(&self) -> usize {
        (self.get_stride() as usize) * (self.get_height() as usize)
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

impl FromGlibPtr<*mut ffi::cairo_surface_t> for ImageSurface {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::cairo_surface_t) -> ImageSurface {
        Self::from(from_glib_none(ptr)).unwrap()
    }

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

impl Clone for ImageSurface {
    fn clone(&self) -> ImageSurface {
        unsafe { from_glib_none(self.to_glib_none().0) }
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

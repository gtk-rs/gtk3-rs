// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::slice;

use enums::{Format, SurfaceType};
use ffi;
#[cfg(feature = "use_glib")]
use glib::translate::*;

use std::fmt;
use surface::Surface;
use BorrowError;
use Status;

declare_surface!(ImageSurface, SurfaceType::Image);

impl ImageSurface {
    pub fn create(format: Format, width: i32, height: i32) -> Result<ImageSurface, Status> {
        unsafe {
            Self::from_raw_full(ffi::cairo_image_surface_create(
                format.into(),
                width,
                height,
            ))
        }
    }

    pub fn create_for_data<D: AsMut<[u8]> + 'static>(
        data: D,
        format: Format,
        width: i32,
        height: i32,
        stride: i32,
    ) -> Result<ImageSurface, Status> {
        let mut data: Box<dyn AsMut<[u8]>> = Box::new(data);

        let (ptr, len) = {
            let data: &mut [u8] = (*data).as_mut();

            (data.as_mut_ptr(), data.len())
        };

        assert!(len >= (height * stride) as usize);
        let result = unsafe {
            ImageSurface::from_raw_full(ffi::cairo_image_surface_create_for_data(
                ptr,
                format.into(),
                width,
                height,
                stride,
            ))
        };
        if let Ok(surface) = &result {
            static IMAGE_SURFACE_DATA: crate::UserDataKey<Box<dyn AsMut<[u8]>>> =
                crate::UserDataKey::new();
            surface.set_user_data(&IMAGE_SURFACE_DATA, Rc::new(data))
        }
        result
    }

    pub fn get_data(&mut self) -> Result<ImageSurfaceData, BorrowError> {
        unsafe {
            if ffi::cairo_surface_get_reference_count(self.to_raw_none()) > 1 {
                return Err(BorrowError::NonExclusive);
            }
            self.flush();
            match self.status() {
                Status::Success => (),
                status => return Err(BorrowError::from(status)),
            }
            if ffi::cairo_image_surface_get_data(self.to_raw_none()).is_null() {
                return Err(BorrowError::from(Status::SurfaceFinished));
            }
            Ok(ImageSurfaceData::new(self))
        }
    }

    pub fn get_format(&self) -> Format {
        unsafe { Format::from(ffi::cairo_image_surface_get_format(self.to_raw_none())) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { ffi::cairo_image_surface_get_height(self.to_raw_none()) }
    }

    pub fn get_stride(&self) -> i32 {
        unsafe { ffi::cairo_image_surface_get_stride(self.to_raw_none()) }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { ffi::cairo_image_surface_get_width(self.to_raw_none()) }
    }
}

#[derive(Debug)]
pub struct ImageSurfaceData<'a> {
    surface: &'a mut ImageSurface,
    slice: &'a mut [u8],
    dirty: bool,
}

impl<'a> ImageSurfaceData<'a> {
    fn new(surface: &'a mut ImageSurface) -> ImageSurfaceData<'a> {
        unsafe {
            let ptr = ffi::cairo_image_surface_get_data(surface.to_raw_none());
            debug_assert!(!ptr.is_null());
            let len = (surface.get_stride() as usize) * (surface.get_height() as usize);
            ImageSurfaceData {
                surface,
                slice: slice::from_raw_parts_mut(ptr, len),
                dirty: false,
            }
        }
    }
}

impl<'a> Drop for ImageSurfaceData<'a> {
    fn drop(&mut self) {
        if self.dirty {
            unsafe { ffi::cairo_surface_mark_dirty(self.surface.to_raw_none()) }
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

impl<'a> fmt::Display for ImageSurfaceData<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImageSurfaceData")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_with_invalid_size_yields_error() {
        let result = ImageSurface::create(Format::ARgb32, 50000, 50000);
        assert!(result.is_err());
    }

    #[test]
    fn create_for_data_with_invalid_stride_yields_error() {
        let result = ImageSurface::create_for_data(vec![0u8; 10], Format::ARgb32, 1, 2, 5); // unaligned stride
        assert!(result.is_err());
    }

    #[test]
    fn create_with_valid_size() {
        let result = ImageSurface::create(Format::ARgb32, 10, 10);
        assert!(result.is_ok());

        let result = ImageSurface::create_for_data(vec![0u8; 40 * 10], Format::ARgb32, 10, 10, 40);
        assert!(result.is_ok());
    }
}

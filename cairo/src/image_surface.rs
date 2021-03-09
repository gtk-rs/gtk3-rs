// Take a look at the license at the top of the repository in the LICENSE file.

use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::slice;

use crate::enums::{Format, SurfaceType};
use crate::error::Error;
#[cfg(feature = "use_glib")]
use glib::translate::*;

use crate::surface::Surface;
use crate::utils::status_to_result;
use crate::BorrowError;
use std::fmt;

declare_surface!(ImageSurface, SurfaceType::Image);

impl ImageSurface {
    #[doc(alias = "cairo_image_surface_create")]
    pub fn create(format: Format, width: i32, height: i32) -> Result<ImageSurface, Error> {
        unsafe {
            Self::from_raw_full(ffi::cairo_image_surface_create(
                format.into(),
                width,
                height,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates an image surface for the provided pixel data.
    /// - The pointer `data` is the beginning of the underlying slice,
    ///   and at least `width * stride` succeeding bytes should be allocated.
    /// - `data` must live longer than any reference to the returned surface.
    /// - You have to free `data` by yourself.
    #[doc(alias = "cairo_image_surface_create_for_data")]
    pub unsafe fn create_for_data_unsafe(
        data: *mut u8,
        format: Format,
        width: i32,
        height: i32,
        stride: i32,
    ) -> Result<ImageSurface, Error> {
        ImageSurface::from_raw_full(ffi::cairo_image_surface_create_for_data(
            data,
            format.into(),
            width,
            height,
            stride,
        ))
    }

    #[doc(alias = "cairo_image_surface_create_for_data")]
    pub fn create_for_data<D: AsMut<[u8]> + 'static>(
        data: D,
        format: Format,
        width: i32,
        height: i32,
        stride: i32,
    ) -> Result<ImageSurface, Error> {
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

    #[doc(alias = "cairo_image_surface_get_data")]
    pub fn get_data(&mut self) -> Result<ImageSurfaceData, BorrowError> {
        unsafe {
            if ffi::cairo_surface_get_reference_count(self.to_raw_none()) > 1 {
                return Err(BorrowError::NonExclusive);
            }

            self.flush();
            let status = ffi::cairo_surface_status(self.to_raw_none());
            if let Some(err) = status_to_result(status).err() {
                return Err(BorrowError::from(err));
            }
            if ffi::cairo_image_surface_get_data(self.to_raw_none()).is_null() || is_finished(self)
            {
                return Err(BorrowError::from(Error::SurfaceFinished));
            }
            Ok(ImageSurfaceData::new(self))
        }
    }

    pub fn with_data<F: FnOnce(&[u8])>(&self, f: F) -> Result<(), BorrowError> {
        self.flush();
        unsafe {
            let status = ffi::cairo_surface_status(self.to_raw_none());
            if let Some(err) = status_to_result(status).err() {
                return Err(BorrowError::from(err));
            }
            let ptr = ffi::cairo_image_surface_get_data(self.to_raw_none());
            if ptr.is_null() || is_finished(self) {
                return Err(BorrowError::from(Error::SurfaceFinished));
            }
            let len = self.get_height() as usize * self.get_stride() as usize;
            f(slice::from_raw_parts(ptr, len));
        }
        Ok(())
    }

    #[doc(alias = "cairo_image_surface_get_format")]
    pub fn get_format(&self) -> Format {
        unsafe { Format::from(ffi::cairo_image_surface_get_format(self.to_raw_none())) }
    }

    #[doc(alias = "cairo_image_surface_get_height")]
    pub fn get_height(&self) -> i32 {
        unsafe { ffi::cairo_image_surface_get_height(self.to_raw_none()) }
    }

    #[doc(alias = "cairo_image_surface_get_stride")]
    pub fn get_stride(&self) -> i32 {
        unsafe { ffi::cairo_image_surface_get_stride(self.to_raw_none()) }
    }

    #[doc(alias = "cairo_image_surface_get_width")]
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
            self.surface.mark_dirty()
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

// Workaround for cairo not having a direct way to check if the surface is finished.
// See: https://gitlab.freedesktop.org/cairo/cairo/-/issues/406
fn is_finished(surface: &ImageSurface) -> bool {
    use super::Context;
    let ctxt = Context::new(surface);
    ctxt.status().is_err()
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

    #[test]
    fn no_crash_after_finish() {
        let mut surf = ImageSurface::create(Format::ARgb32, 1024, 1024).unwrap();

        surf.finish();

        assert!(surf.get_data().is_err());
    }
}

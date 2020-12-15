// Take a look at the license at the top of the repository in the LICENSE file.

use libc::{c_ulong, c_void};
use std::ffi::CString;
use std::fmt;
use std::ops::Deref;
use std::ptr;
use std::slice;

use crate::enums::{Content, Format, SurfaceType};
use crate::error::Error;
use crate::utils::status_to_result;
#[cfg(feature = "use_glib")]
use glib::translate::*;

use crate::device::Device;
use crate::image_surface::ImageSurface;
use crate::rectangle::Rectangle;
use crate::rectangle_int::RectangleInt;

#[derive(Debug)]
pub struct Surface(ptr::NonNull<ffi::cairo_surface_t>);

impl Surface {
    pub unsafe fn from_raw_none(ptr: *mut ffi::cairo_surface_t) -> Surface {
        assert!(!ptr.is_null());
        ffi::cairo_surface_reference(ptr);
        Surface(ptr::NonNull::new_unchecked(ptr))
    }

    pub unsafe fn from_raw_borrow(ptr: *mut ffi::cairo_surface_t) -> crate::Borrowed<Surface> {
        assert!(!ptr.is_null());
        crate::Borrowed::new(Surface(ptr::NonNull::new_unchecked(ptr)))
    }

    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_surface_t) -> Result<Surface, Error> {
        assert!(!ptr.is_null());
        let status = ffi::cairo_surface_status(ptr);
        status_to_result(status)?;
        Ok(Surface(ptr::NonNull::new_unchecked(ptr)))
    }

    pub fn to_raw_none(&self) -> *mut ffi::cairo_surface_t {
        self.0.as_ptr()
    }

    pub fn create_similar(
        &self,
        content: Content,
        width: i32,
        height: i32,
    ) -> Result<Surface, Error> {
        unsafe {
            Self::from_raw_full(ffi::cairo_surface_create_similar(
                self.0.as_ptr(),
                content.into(),
                width,
                height,
            ))
        }
    }

    pub fn create_for_rectangle(&self, bounds: Rectangle) -> Result<Surface, Error> {
        unsafe {
            Self::from_raw_full(ffi::cairo_surface_create_for_rectangle(
                self.0.as_ptr(),
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
            ))
        }
    }

    pub fn get_mime_data(&self, mime_type: &str) -> Option<Vec<u8>> {
        let data_ptr: *mut u8 = ptr::null_mut();
        let mut length: c_ulong = 0;
        unsafe {
            let mime_type = CString::new(mime_type).unwrap();
            ffi::cairo_surface_get_mime_data(
                self.to_raw_none(),
                mime_type.as_ptr(),
                &data_ptr,
                &mut length,
            );
            if !data_ptr.is_null() && length != 0 {
                Some(slice::from_raw_parts(data_ptr as *const u8, length as usize).to_vec())
            } else {
                None
            }
        }
    }

    pub unsafe fn get_mime_data_raw(&self, mime_type: &str) -> Option<&[u8]> {
        let data_ptr: *mut u8 = ptr::null_mut();
        let mut length: c_ulong = 0;
        let mime_type = CString::new(mime_type).unwrap();
        ffi::cairo_surface_get_mime_data(
            self.to_raw_none(),
            mime_type.as_ptr(),
            &data_ptr,
            &mut length,
        );
        if !data_ptr.is_null() && length != 0 {
            Some(slice::from_raw_parts(
                data_ptr as *const u8,
                length as usize,
            ))
        } else {
            None
        }
    }

    pub fn set_mime_data<T: AsRef<[u8]> + 'static>(
        &self,
        mime_type: &str,
        slice: T,
    ) -> Result<(), Error> {
        let b = Box::new(slice);
        let (size, data) = {
            let slice = (*b).as_ref();
            (slice.len(), slice.as_ptr())
        };

        let user_data = Box::into_raw(b);

        unsafe extern "C" fn unbox<T>(data: *mut c_void) {
            let data: Box<T> = Box::from_raw(data as *mut T);
            drop(data);
        }

        let status = unsafe {
            let mime_type = CString::new(mime_type).unwrap();
            ffi::cairo_surface_set_mime_data(
                self.to_raw_none(),
                mime_type.as_ptr(),
                data,
                size as c_ulong,
                Some(unbox::<T>),
                user_data as *mut _,
            )
        };
        status_to_result(status)
    }

    pub fn supports_mime_type(&self, mime_type: &str) -> bool {
        unsafe {
            let mime_type = CString::new(mime_type).unwrap();
            ffi::cairo_surface_supports_mime_type(self.0.as_ptr(), mime_type.as_ptr()).as_bool()
        }
    }

    pub fn get_device(&self) -> Option<Device> {
        unsafe {
            let device = ffi::cairo_surface_get_device(self.to_raw_none());
            if device.is_null() {
                None
            } else {
                Some(Device::from_raw_none(device))
            }
        }
    }

    pub fn set_device_offset(&self, x_offset: f64, y_offset: f64) {
        unsafe { ffi::cairo_surface_set_device_offset(self.to_raw_none(), x_offset, y_offset) }
    }

    pub fn get_device_offset(&self) -> (f64, f64) {
        let mut x_offset = 0.0f64;
        let mut y_offset = 0.0f64;
        unsafe {
            ffi::cairo_surface_get_device_offset(self.to_raw_none(), &mut x_offset, &mut y_offset);
        }
        (x_offset, y_offset)
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn set_device_scale(&self, x_scale: f64, y_scale: f64) {
        unsafe { ffi::cairo_surface_set_device_scale(self.to_raw_none(), x_scale, y_scale) }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_device_scale(&self) -> (f64, f64) {
        let mut x_scale = 0.0f64;
        let mut y_scale = 0.0f64;
        unsafe {
            ffi::cairo_surface_get_device_scale(self.to_raw_none(), &mut x_scale, &mut y_scale);
        }
        (x_scale, y_scale)
    }

    pub fn set_fallback_resolution(&self, x_pixels_per_inch: f64, y_pixels_per_inch: f64) {
        unsafe {
            ffi::cairo_surface_set_fallback_resolution(
                self.to_raw_none(),
                x_pixels_per_inch,
                y_pixels_per_inch,
            )
        }
    }

    pub fn get_fallback_resolution(&self) -> (f64, f64) {
        let mut x_pixels_per_inch = 0.0f64;
        let mut y_pixels_per_inch = 0.0f64;
        unsafe {
            ffi::cairo_surface_get_fallback_resolution(
                self.to_raw_none(),
                &mut x_pixels_per_inch,
                &mut y_pixels_per_inch,
            );
        }
        (x_pixels_per_inch, y_pixels_per_inch)
    }

    pub fn create_similar_image(
        &self,
        format: Format,
        width: i32,
        height: i32,
    ) -> Result<Surface, Error> {
        unsafe {
            Self::from_raw_full(ffi::cairo_surface_create_similar_image(
                self.to_raw_none(),
                format.into(),
                width,
                height,
            ))
        }
    }

    pub fn map_to_image(&self, extents: Option<RectangleInt>) -> Result<MappedImageSurface, Error> {
        unsafe {
            ImageSurface::from_raw_full(match extents {
                Some(ref e) => ffi::cairo_surface_map_to_image(self.to_raw_none(), e.to_raw_none()),
                None => ffi::cairo_surface_map_to_image(self.to_raw_none(), std::ptr::null()),
            })
            .map(|s| MappedImageSurface {
                original_surface: self.clone(),
                image_surface: s,
            })
        }
    }

    pub fn mark_dirty(&self) {
        unsafe { ffi::cairo_surface_mark_dirty(self.to_raw_none()) }
    }

    pub fn mark_dirty_rectangle(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { ffi::cairo_surface_mark_dirty_rectangle(self.to_raw_none(), x, y, width, height) }
    }

    user_data_methods! {
        ffi::cairo_surface_get_user_data,
        ffi::cairo_surface_set_user_data,
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
        unsafe { ffi::cairo_surface_reference(self.to_raw_none()) }
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
    unsafe fn from_glib_borrow(ptr: *mut ffi::cairo_surface_t) -> crate::Borrowed<Surface> {
        Self::from_raw_borrow(ptr)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrFull<*mut ffi::cairo_surface_t> for Surface {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::cairo_surface_t) -> Surface {
        Self::from_raw_full(ptr).unwrap()
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(
    Surface,
    ffi::cairo_surface_t,
    ffi::gobject::cairo_gobject_surface_get_type
);

impl Clone for Surface {
    fn clone(&self) -> Surface {
        unsafe { Self::from_raw_none(self.0.as_ptr()) }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_surface_destroy(self.0.as_ptr());
        }
    }
}

impl fmt::Display for Surface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Surface")
    }
}

impl Surface {
    pub fn flush(&self) {
        unsafe {
            ffi::cairo_surface_flush(self.0.as_ptr());
        }
    }

    pub fn finish(&self) {
        unsafe {
            ffi::cairo_surface_finish(self.0.as_ptr());
        }
    }

    pub fn get_type(&self) -> SurfaceType {
        unsafe { SurfaceType::from(ffi::cairo_surface_get_type(self.0.as_ptr())) }
    }
}

#[derive(Debug)]
pub struct MappedImageSurface {
    original_surface: Surface,
    image_surface: ImageSurface,
}

impl Deref for MappedImageSurface {
    type Target = ImageSurface;

    fn deref(&self) -> &ImageSurface {
        &self.image_surface
    }
}

impl Drop for MappedImageSurface {
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_surface_unmap_image(
                self.original_surface.to_raw_none(),
                self.image_surface.to_raw_none(),
            );
            ffi::cairo_surface_reference(self.image_surface.to_raw_none());
        }
    }
}

impl fmt::Display for MappedImageSurface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MappedImageSurface")
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::MIME_TYPE_PNG;
    use crate::Format;
    use crate::ImageSurface;

    #[test]
    fn mime_data() {
        let surface = ImageSurface::create(Format::ARgb32, 500, 500).unwrap();
        let data = surface.get_mime_data(MIME_TYPE_PNG);
        /* Initially the data for any mime type has to be none */
        assert!(data.is_none());

        assert!(surface.set_mime_data(MIME_TYPE_PNG, &[1u8, 10u8]).is_ok());
        let data = surface.get_mime_data(MIME_TYPE_PNG).unwrap();
        assert_eq!(data, &[1u8, 10u8]);
    }
}

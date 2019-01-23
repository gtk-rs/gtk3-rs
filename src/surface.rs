// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::mem;
use std::ptr;
use std::slice;
use libc::{c_ulong, c_void};
use std::ffi::CString;

#[cfg(feature = "use_glib")]
use glib::translate::*;
use ffi;
use ::enums::{
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
        unsafe {
            Self::from_raw_full(ffi::cairo_surface_create_similar(self.0,
                                                                  content.into(),
                                                                  width,
                                                                  height))
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
                Some(slice::from_raw_parts(
                    data_ptr as *const u8,
                    length as usize,
                    ).to_vec()
                )
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
                )
            )
        } else {
            None
        }
    }


    pub fn set_mime_data<T: AsRef<[u8]> + Send + 'static>(
        &self,
        mime_type: &str,
        slice: T) -> Result<(), Status> {
        let b = Box::new(slice);
        let (size, data) = {
            let slice = (*b).as_ref();
            (slice.len(), slice.as_ptr())
        };

        let user_data = Box::into_raw(b);

        let status = unsafe {
            let mime_type = CString::new(mime_type).unwrap();
            ffi::cairo_surface_set_mime_data(self.to_raw_none(),
                mime_type.as_ptr(),
                data,
                size as c_ulong,
                Some(unbox::<T>),
                user_data as *mut _,
            )
        };

        match Status::from(status) {
            Status::Success => Ok(()),
            x => Err(x),
        }
    }

    pub fn supports_mime_type(&self, mime_type: &str) -> bool {
        unsafe {
            let mime_type = CString::new(mime_type).unwrap();
            ffi::cairo_surface_supports_mime_type(self.0, mime_type.as_ptr()).as_bool()
        }
    }

    pub fn set_device_offset(&mut self, x_offset: f64, y_offset: f64) {
        unsafe { ffi::cairo_surface_set_device_offset(self.to_raw_none(), x_offset, y_offset) }
    }

    pub fn get_device_offset(&self) -> (f64, f64) {
        let mut x_offset = 0.0f64;
        let mut y_offset = 0.0f64;
        unsafe { ffi::cairo_surface_get_device_offset(self.to_raw_none(), &mut x_offset, &mut y_offset); }
        (x_offset, y_offset)
    }

    pub fn set_device_scale(&mut self, x_scale: f64, y_scale: f64) {
        unsafe { ffi::cairo_surface_set_device_scale(self.to_raw_none(), x_scale, y_scale) }
    }

    pub fn get_device_scale(&self) -> (f64, f64) {
        let mut x_scale = 0.0f64;
        let mut y_scale = 0.0f64;
        unsafe { ffi::cairo_surface_get_device_scale(self.to_raw_none(), &mut x_scale, &mut y_scale); }
        (x_scale, y_scale)
    }

    pub fn set_fallback_resolution(&mut self, x_pixels_per_inch: f64, y_pixels_per_inch: f64) {
        unsafe { ffi::cairo_surface_set_fallback_resolution(self.to_raw_none(), x_pixels_per_inch, y_pixels_per_inch) }
    }

    pub fn get_fallback_resolution(&self) -> (f64, f64) {
        let mut x_pixels_per_inch = 0.0f64;
        let mut y_pixels_per_inch = 0.0f64;
        unsafe { ffi::cairo_surface_get_fallback_resolution(self.to_raw_none(), &mut x_pixels_per_inch, &mut y_pixels_per_inch); }
        (x_pixels_per_inch, y_pixels_per_inch)
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
        unsafe { SurfaceType::from(ffi::cairo_surface_get_type(self.as_ref().0)) }
    }

    fn status(&self) -> Status {
        unsafe { Status::from(ffi::cairo_surface_status(self.as_ref().0)) }
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
        match Status::from(status) {
            Status::Success => Ok(()),
            x => Err(x),
        }
    }
}

unsafe extern "C" fn unbox<T>(data: *mut c_void) {
    let data: Box<T> = Box::from_raw(data as *mut T);
    drop(data);
}

#[cfg(test)]
mod tests {
    use Format;
    use ImageSurface;
    use constants::MIME_TYPE_PNG;

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

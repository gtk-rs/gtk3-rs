// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

/// The GdkPixbuf structure contains information that describes an image in memory.

use std::mem;
use std::ptr;
use std::slice;
use libc::c_uchar;
use glib::translate::*;
use glib::types::{StaticType, Type};
use glib::{Error, to_gboolean, GlibContainer};
use object::Object;
use ffi;

pub mod animation;
pub mod format;
pub mod loader;

pub use self::animation::{PixbufAnimation, PixbufAnimationIter, PixbufSimpleAnim};
pub use self::format::PixbufFormat;
pub use self::loader::PixbufLoader;

/// This is the main structure in the &gdk-pixbuf; library. It is used to represent images. It contains information about the image's pixel 
/// data, its color space, bits per sample, width and height, and the rowstride (the number of bytes between the start of one row and the 
/// start of the next).
pub type Pixbuf = Object<ffi::GdkPixbuf>;

impl StaticType for Pixbuf {
    fn static_type() -> Type { unsafe { from_glib(ffi::gdk_pixbuf_get_type()) } }
}

impl Pixbuf {
    pub unsafe fn new(colorspace: ::ColorSpace, has_alpha: bool, bits_per_sample: i32, width: i32,
            height: i32) -> Result<Pixbuf, ()> {
        Option::from_glib_full(ffi::gdk_pixbuf_new(colorspace, has_alpha.to_glib(),
                                                   bits_per_sample, width, height)).ok_or(())
    }

    /// Creates a `Pixbuf` using a `Vec` as image data.
    ///
    /// Only `bits_per_sample == 8` supported.
    pub fn new_from_vec(mut vec: Vec<u8>, colorspace: ::ColorSpace, has_alpha: bool,
            bits_per_sample: i32, width: i32, height: i32, row_stride: i32) -> Pixbuf {
        extern "C" fn destroy_vec(_: *mut c_uchar, data: ffi::gpointer) {
            unsafe{
                let _vec: Box<Vec<u8>> = mem::transmute(data); // the vector will be destroyed now
            }
        }

        assert!(bits_per_sample == 8);
        let n_channels = if has_alpha { 4 } else { 3 };
        let last_row_len = width * ((n_channels * bits_per_sample + 7) / 8);
        assert!(vec.len() == ((height - 1) * row_stride + last_row_len) as usize);
        let ptr = vec.as_mut_ptr();
        let vec: Box<Vec<u8>> = Box::new(vec);
        unsafe {
            from_glib_full(
                ffi::gdk_pixbuf_new_from_data(ptr, colorspace, has_alpha.to_glib(), bits_per_sample,
                    width, height, row_stride, destroy_vec, mem::transmute(vec)))
        }
    }

    pub fn new_from_file(filename: &str) -> Result<Pixbuf, Error> {
        let mut error = ptr::null_mut();
        let tmp = unsafe { ffi::gdk_pixbuf_new_from_file(filename.to_glib_none().0, &mut error) };

        if error.is_null() {
            assert!(!tmp.is_null());
            unsafe { Ok(from_glib_full(tmp)) }
        } else {
            Err(Error::wrap(error))
        }
    }

    pub fn new_from_file_at_size(filename: &str, width: i32, height: i32) -> Result<Pixbuf, Error> {
        let mut error = ptr::null_mut();
        let tmp = unsafe { ffi::gdk_pixbuf_new_from_file_at_size(filename.to_glib_none().0, width, height, &mut error) };

        if error.is_null() {
            assert!(!tmp.is_null());
            unsafe { Ok(from_glib_full(tmp)) }
        } else {
            Err(Error::wrap(error))
        }
    }

    pub fn new_from_file_at_scale(filename: &str, width: i32, height: i32, preserve_aspect_ratio: bool) -> Result<Pixbuf, Error> {
        let mut error = ptr::null_mut();
        let tmp = unsafe { ffi::gdk_pixbuf_new_from_file_at_scale(filename.to_glib_none().0, width, height,
            to_gboolean(preserve_aspect_ratio), &mut error) };

        if error.is_null() {
            assert!(!tmp.is_null());
            unsafe { Ok(from_glib_full(tmp)) }
        } else {
            Err(Error::wrap(error))
        }
    }

    pub fn get_file_info(filename: &str, width: &mut i32, height: &mut i32) -> Option<PixbufFormat> {
        let tmp = unsafe { ffi::gdk_pixbuf_get_file_info(filename.to_glib_none().0, width, height) };

        unsafe { from_glib_full(tmp) }
    }

    pub fn new_from_resource(resource_path: &str) -> Result<Pixbuf, Error> {
        let mut error = ptr::null_mut();
        let tmp = unsafe { ffi::gdk_pixbuf_new_from_resource(resource_path.to_glib_none().0, &mut error) };

        if error.is_null() {
            assert!(!tmp.is_null());
            unsafe { Ok(from_glib_full(tmp)) }
        } else {
            Err(Error::wrap(error))
        }
    }

    pub fn new_from_resource_at_scale(resource_path: &str, width: i32, height: i32, preserve_aspect_ratio: bool) -> Result<Pixbuf, Error> {
        let mut error = ptr::null_mut();
        let tmp = unsafe { ffi::gdk_pixbuf_new_from_resource_at_scale(resource_path.to_glib_none().0, width, height,
            to_gboolean(preserve_aspect_ratio), &mut error) };

        if error.is_null() {
            assert!(!tmp.is_null());
            unsafe { Ok(from_glib_full(tmp)) }
        } else {
            Err(Error::wrap(error))
        }
    }

    pub fn new_subpixbuf(&self, src_x: i32, src_y: i32, width: i32, height: i32) -> Pixbuf {
        unsafe { 
            from_glib_full(
                ffi::gdk_pixbuf_new_subpixbuf(self.to_glib_none().0, src_x, src_y, width, height))
        }
    }

    pub fn get_colorspace(&self) -> ::ColorSpace {
        unsafe { ffi::gdk_pixbuf_get_colorspace(self.to_glib_none().0) }
    }

    pub fn get_n_channels(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_n_channels(self.to_glib_none().0) }
    }

    pub fn get_has_alpha(&self) -> bool {
        unsafe { ::glib::to_bool(ffi::gdk_pixbuf_get_has_alpha(self.to_glib_none().0)) }
    }

    pub fn get_bits_per_sample(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_bits_per_sample(self.to_glib_none().0) }
    }

    pub unsafe fn get_pixels(&self) -> &mut [u8] {
        let mut len = 0;
        let ptr = ffi::gdk_pixbuf_get_pixels_with_length(self.to_glib_none().0, &mut len);
        slice::from_raw_parts_mut(ptr, len as usize)
    }

    pub fn get_width(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_width(self.to_glib_none().0) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_height(self.to_glib_none().0) }
    }

    pub fn get_rowstride(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_rowstride(self.to_glib_none().0) }
    }

    pub fn get_byte_length(&self) -> usize {
        unsafe { ffi::gdk_pixbuf_get_byte_length(self.to_glib_none().0) as usize }
    }

    pub fn get_option(&self, key: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gdk_pixbuf_get_option(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    /// a convenient function
    /// It won't work for pixbufs with images that are other than 8 bits per sample or channel, but it will work for most of the
    /// pixbufs that GTK+ uses.
    pub fn put_pixel(&self, x: i32, y: i32, red: u8, green: u8, blue: u8, alpha: u8) {
        unsafe {
            let n_channels = self.get_n_channels();
            let rowstride = self.get_rowstride();
            let pixels = self.get_pixels();
            let pos = (y * rowstride + x * n_channels) as usize;

            pixels[pos] = red;
            pixels[pos + 1] = green;
            pixels[pos + 2] = blue;
            pixels[pos + 3] = alpha;
        }
    }
}

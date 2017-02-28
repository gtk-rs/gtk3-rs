// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::mem;
use std::ptr;
use std::slice;
use libc::{c_void, c_uchar};
use glib::translate::*;
use glib::Error;
use gdk_pixbuf_ffi as ffi;

use {
    Colorspace,
    InterpType,
    PixbufFormat,
};

glib_wrapper! {
    pub struct Pixbuf(Object<ffi::GdkPixbuf>);

    match fn {
        get_type => || ffi::gdk_pixbuf_get_type(),
    }
}

impl Pixbuf {
    pub unsafe fn new(colorspace: Colorspace, has_alpha: bool, bits_per_sample: i32, width: i32,
            height: i32) -> Result<Pixbuf, ()> {
        Option::from_glib_full(ffi::gdk_pixbuf_new(colorspace, has_alpha.to_glib(),
                                                   bits_per_sample, width, height)).ok_or(())
    }

    pub fn new_from_vec(mut vec: Vec<u8>, colorspace: Colorspace, has_alpha: bool,
            bits_per_sample: i32, width: i32, height: i32, row_stride: i32) -> Pixbuf {
        unsafe extern "C" fn destroy_vec(_: *mut c_uchar, data: *mut c_void) {
            let _vec: Box<Vec<u8>> = mem::transmute(data); // the vector will be destroyed now
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
                    width, height, row_stride, Some(destroy_vec), mem::transmute(vec)))
        }
    }

    pub fn new_from_file(filename: &str) -> Result<Pixbuf, Error> {
        #[cfg(windows)]
        use gdk_pixbuf_ffi::gdk_pixbuf_new_from_file_utf8 as gdk_pixbuf_new_from_file;
        #[cfg(not(windows))]
        use gdk_pixbuf_ffi::gdk_pixbuf_new_from_file;

        unsafe {
            let mut error = ptr::null_mut();
            let ptr = gdk_pixbuf_new_from_file(filename.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn new_from_file_at_size(filename: &str, width: i32, height: i32) -> Result<Pixbuf, Error> {
        #[cfg(windows)]
        use gdk_pixbuf_ffi::gdk_pixbuf_new_from_file_at_size_utf8
            as gdk_pixbuf_new_from_file_at_size;
        #[cfg(not(windows))]
        use gdk_pixbuf_ffi::gdk_pixbuf_new_from_file_at_size;

        unsafe {
            let mut error = ptr::null_mut();
            let ptr = gdk_pixbuf_new_from_file_at_size(filename.to_glib_none().0, width, height,
                &mut error);
            if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn new_from_file_at_scale(filename: &str, width: i32, height: i32, preserve_aspect_ratio: bool) -> Result<Pixbuf, Error> {
        #[cfg(windows)]
        use gdk_pixbuf_ffi::gdk_pixbuf_new_from_file_at_scale_utf8
            as gdk_pixbuf_new_from_file_at_scale;
        #[cfg(not(windows))]
        use gdk_pixbuf_ffi::gdk_pixbuf_new_from_file_at_scale;

        unsafe {
            let mut error = ptr::null_mut();
            let ptr = gdk_pixbuf_new_from_file_at_scale(filename.to_glib_none().0, width, height,
                preserve_aspect_ratio.to_glib(), &mut error);
            if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn get_file_info(filename: &str, width: &mut i32, height: &mut i32)
            -> Option<PixbufFormat> {
        unsafe {
            from_glib_none(ffi::gdk_pixbuf_get_file_info(filename.to_glib_none().0, width, height))
        }
    }

    pub fn new_from_resource(resource_path: &str) -> Result<Pixbuf, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ptr = ffi::gdk_pixbuf_new_from_resource(resource_path.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn new_from_resource_at_scale(resource_path: &str, width: i32, height: i32, preserve_aspect_ratio: bool) -> Result<Pixbuf, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ptr = ffi::gdk_pixbuf_new_from_resource_at_scale(resource_path.to_glib_none().0,
                width, height, preserve_aspect_ratio.to_glib(), &mut error);
            if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn new_subpixbuf(&self, src_x: i32, src_y: i32, width: i32, height: i32) -> Pixbuf {
        unsafe {
            from_glib_full(
                ffi::gdk_pixbuf_new_subpixbuf(self.to_glib_none().0, src_x, src_y, width, height))
        }
    }

    pub fn get_colorspace(&self) -> Colorspace {
        unsafe { ffi::gdk_pixbuf_get_colorspace(self.to_glib_none().0) }
    }

    pub fn get_n_channels(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_n_channels(self.to_glib_none().0) }
    }

    pub fn get_has_alpha(&self) -> bool {
        unsafe { from_glib(ffi::gdk_pixbuf_get_has_alpha(self.to_glib_none().0)) }
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

    pub fn scale_simple(&self, dest_width: i32, dest_height: i32, interp_type: InterpType)
        -> Result<Pixbuf, ()> {
        unsafe {
            Option::from_glib_full(ffi::gdk_pixbuf_scale_simple(self.to_glib_none().0, dest_width,
                                                                dest_height, interp_type)).ok_or(())
        }
    }

    pub fn scale(&self, dest: &Pixbuf, dest_x: i32, dest_y: i32, dest_width: i32, dest_height: i32,
                 offset_x: f64, offset_y: f64, scale_x: f64, scale_y: f64,
                 interp_type: InterpType) {
        unsafe {
            ffi::gdk_pixbuf_scale(self.to_glib_none().0, dest.to_glib_none().0, dest_x, dest_y,
                                  dest_width, dest_height, offset_x, offset_y, scale_x, scale_y,
                                  interp_type);
        }
    }

    pub fn composite(&self, dest: &Pixbuf, dest_x: i32, dest_y: i32, dest_width: i32,
                     dest_height: i32, offset_x: f64, offset_y: f64, scale_x: f64, scale_y: f64,
                     interp_type: InterpType, overall_alpha: i32) {
        unsafe {
            ffi::gdk_pixbuf_composite(self.to_glib_none().0, dest.to_glib_none().0, dest_x, dest_y,
                                      dest_width, dest_height, offset_x, offset_y, scale_x,
                                      scale_y, interp_type, overall_alpha);
        }
    }

    pub fn flip(&self, horizontal: bool) -> Result<Pixbuf, ()> {
        unsafe {
            Option::from_glib_full((ffi::gdk_pixbuf_flip(self.to_glib_none().0,
                                                         horizontal.to_glib()))).ok_or(())
        }
    }

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

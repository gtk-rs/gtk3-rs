// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

/// The GdkPixbuf structure contains information that describes an image in memory.

use glib::to_gboolean;
use glib::translate::*;
use ffi;
use c_vec::CVec;

#[repr(C)]
/// This is the main structure in the &gdk-pixbuf; library. It is used to represent images. It contains information about the image's pixel 
/// data, its color space, bits per sample, width and height, and the rowstride (the number of bytes between the start of one row and the 
/// start of the next).
pub struct Pixbuf {
    pointer: *mut ffi::C_GdkPixbuf
}

impl Pixbuf {
    pub fn new(colorspace: ::ColorSpace, has_alpha: bool, bits_per_sample: i32, width: i32,
            height: i32) -> Option<Pixbuf> {
        match unsafe { ffi::gdk_pixbuf_new(colorspace, to_gboolean(has_alpha), bits_per_sample,
                width, height) } {
            pointer if !pointer.is_null() => Some(Pixbuf { pointer: pointer }),
            _ => None
        }
    }

    pub fn new_subpixbuf(&self, src_x: i32, src_y: i32, width: i32, height: i32) ->
            Option<Pixbuf> {
        match unsafe { ffi::gdk_pixbuf_new_subpixbuf(self.pointer, src_x, src_y, width, height) } {
            pointer if !pointer.is_null() => Some(Pixbuf { pointer: pointer }),
            _ => None
        }
    }

    pub fn get_colorspace(&self) -> ::ColorSpace {
        unsafe { ffi::gdk_pixbuf_get_colorspace(self.pointer as *const ffi::C_GdkPixbuf) }
    }

    pub fn get_n_channels(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_n_channels(self.pointer as *const ffi::C_GdkPixbuf) }
    }

    pub fn get_has_alpha(&self) -> bool {
        unsafe { ::glib::to_bool(ffi::gdk_pixbuf_get_has_alpha(self.pointer as *const ffi::C_GdkPixbuf)) }
    }

    pub fn get_bits_per_sample(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_bits_per_sample(self.pointer as *const ffi::C_GdkPixbuf) }
    }

    pub fn get_pixels_with_length(&self, length: &mut u32) -> Option<CVec<u8>> {
        let tmp = unsafe { ffi::gdk_pixbuf_get_pixels_with_length(self.pointer as *const ffi::C_GdkPixbuf, length) };

        unsafe {
            if tmp.is_null() {
                None
            } else {
                Some(CVec::new(tmp, *length as usize))
            }
        }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_width(self.pointer as *const ffi::C_GdkPixbuf) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_height(self.pointer as *const ffi::C_GdkPixbuf) }
    }

    pub fn get_rowstride(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_rowstride(self.pointer as *const ffi::C_GdkPixbuf) }
    }

    pub fn get_byte_length(&self) -> usize {
        unsafe { ffi::gdk_pixbuf_get_byte_length(self.pointer as *const ffi::C_GdkPixbuf) as usize }
    }

    pub fn get_option(&self, key: &str) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gdk_pixbuf_get_option(self.pointer as *const ffi::C_GdkPixbuf,
                                           key.to_glib_none().0))
        }
    }

    /// a convenient function
    /// It won't work for pixbufs with images that are other than 8 bits per sample or channel, but it will work for most of the
    /// pixbufs that GTK+ uses.
    pub fn put_pixel(&self, x: i32, y: i32, red: u8, green: u8, blue: u8, alpha: u8) {
        let n_channels = self.get_n_channels();
        let rowstride = self.get_rowstride();
        let mut length = 0u32;
        let pixels = self.get_pixels_with_length(&mut length);
        if pixels.is_none() {
            return;
        }
        let mut pixels = pixels.unwrap();
        let s_pixels = pixels.as_mut();
        let pos = (y * rowstride + x * n_channels) as usize;

        s_pixels[pos] = red;
        s_pixels[pos + 1] = green;
        s_pixels[pos + 2] = blue;
        s_pixels[pos + 3] = alpha;
    }
}

impl_GObjectFunctions!(Pixbuf, C_GdkPixbuf);

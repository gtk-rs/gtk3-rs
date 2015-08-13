// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

/// The GdkPixbuf structure contains information that describes an image in memory.

use std::mem;
use std::ptr;
use std::slice;
use libc::{c_void, c_uchar};
use glib::translate::*;
use glib::types::{StaticType, Type};
use glib::{Error, to_gboolean, GlibContainer};
use object::Object;
use gdk_pixbuf_ffi as ffi;

use {
    Colorspace,
    InterpType,
};

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
    pub unsafe fn new(colorspace: Colorspace, has_alpha: bool, bits_per_sample: i32, width: i32,
            height: i32) -> Result<Pixbuf, ()> {
        Option::from_glib_full(ffi::gdk_pixbuf_new(colorspace, has_alpha.to_glib(),
                                                   bits_per_sample, width, height)).ok_or(())
    }

    /// Creates a `Pixbuf` using a `Vec` as image data.
    ///
    /// Only `bits_per_sample == 8` supported.
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

        let mut error = ptr::null_mut();
        let tmp = unsafe { gdk_pixbuf_new_from_file(filename.to_glib_none().0, &mut error) };

        if error.is_null() {
            assert!(!tmp.is_null());
            unsafe { Ok(from_glib_full(tmp)) }
        } else {
            Err(Error::wrap(error))
        }
    }

    pub fn new_from_file_at_size(filename: &str, width: i32, height: i32) -> Result<Pixbuf, Error> {
        #[cfg(windows)]
        use gdk_pixbuf_ffi::gdk_pixbuf_new_from_file_at_size_utf8
            as gdk_pixbuf_new_from_file_at_size;
        #[cfg(not(windows))]
        use gdk_pixbuf_ffi::gdk_pixbuf_new_from_file_at_size;

        let mut error = ptr::null_mut();
        let tmp = unsafe {
            gdk_pixbuf_new_from_file_at_size(filename.to_glib_none().0, width, height, &mut error)
        };

        if error.is_null() {
            assert!(!tmp.is_null());
            unsafe { Ok(from_glib_full(tmp)) }
        } else {
            Err(Error::wrap(error))
        }
    }

    pub fn new_from_file_at_scale(filename: &str, width: i32, height: i32, preserve_aspect_ratio: bool) -> Result<Pixbuf, Error> {
        #[cfg(windows)]
        use gdk_pixbuf_ffi::gdk_pixbuf_new_from_file_at_scale_utf8
            as gdk_pixbuf_new_from_file_at_scale;
        #[cfg(not(windows))]
        use gdk_pixbuf_ffi::gdk_pixbuf_new_from_file_at_scale;

        let mut error = ptr::null_mut();
        let tmp = unsafe {
            gdk_pixbuf_new_from_file_at_scale(filename.to_glib_none().0, width, height,
                to_gboolean(preserve_aspect_ratio), &mut error)
        };

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

    pub fn get_colorspace(&self) -> Colorspace {
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

    /// Create a new GdkPixbuf containing a copy of the current pixbuf scaled
    /// to `dest_width` x `dest_height`. The calling pixbuf remains unaffected.
    /// `interp_type` should be `Nearest` if you want maximum speed (but when
    /// scaling down `Nearest` is usually unusably ugly). The default
    /// `interp_type` should be `Bilinear` which offers reasonable quality and
    /// speed.
    ///
    /// You can scale a sub-portion of `src` by creating a sub-pixbuf pointing
    /// into src ; see `new_subpixbuf()`.
    ///
    /// For more complicated scaling/compositing see `scale()` and
    /// `composite()`.
    pub fn scale_simple(&self, dest_width: i32, dest_height: i32, interp_type: InterpType)
        -> Result<Pixbuf, ()> {
        unsafe {
            Option::from_glib_full(ffi::gdk_pixbuf_scale_simple(self.to_glib_none().0, dest_width,
                                                                dest_height, interp_type)).ok_or(())
        }
    }

    /// Creates a transformation of the calling pixbuf by scaling by
    /// `scale_x` and `scale_y` then translating by `offset_x` and `offset_y`,
    /// then renders the rectangle (`dest_x`, `dest_y`, `dest_width`,
    /// `dest_height`) of the resulting pixbuf onto the destination pixbuf,
    /// replacing the previous contents.
    ///
    /// Try to use `scale_simple()` first; this function is the
    /// industrial-strength power tool you can fall back to if `scale_simple()`
    /// isn't powerful enough.
    ///
    /// If the source rectangle overlaps the destination rectangle on the same
    /// pixbuf, it will be overwritten during the scaling which results in
    /// rendering artifacts.
    pub fn scale(&self, dest: &Pixbuf, dest_x: i32, dest_y: i32, dest_width: i32, dest_height: i32,
                 offset_x: f64, offset_y: f64, scale_x: f64, scale_y: f64,
                 interp_type: InterpType) {
        unsafe {
            ffi::gdk_pixbuf_scale(self.to_glib_none().0, dest.to_glib_none().0, dest_x, dest_y,
                                  dest_width, dest_height, offset_x, offset_y, scale_x, scale_y,
                                  interp_type);
        }
    }
    
    /// Creates a transformation of the calling pixbuf by scaling by `scale_x`
    /// and `scale_y` then translating by `offset_x` and `offset_y`. This gives
    /// an image in the coordinates of the destination pixbuf. The rectangle
    /// (`dest_x`, `dest_y`, `dest_width`, `dest_height`) is then composited
    /// onto the corresponding rectangle of the original destination image.
    /// 
    /// ![Diagram of `composite` process](https://developer.gnome.org/gdk-pixbuf/unstable/composite.png)
    ///
    /// When the destination rectangle contains parts not in the source image,
    /// the data at the edges of the source image is replicated to infinity.
    pub fn composite(&self, dest: &Pixbuf, dest_x: i32, dest_y: i32, dest_width: i32,
                     dest_height: i32, offset_x: f64, offset_y: f64, scale_x: f64, scale_y: f64,
                     interp_type: InterpType, overall_alpha: i32) {
        unsafe {
            ffi::gdk_pixbuf_composite(self.to_glib_none().0, dest.to_glib_none().0, dest_x, dest_y,
                                      dest_width, dest_height, offset_x, offset_y, scale_x,
                                      scale_y, interp_type, overall_alpha);
        }
    }

    /// Flips a pixbuf horizontally or vertically and returns the result in a
    /// new pixbuf, or `Err` if not enough memory could be allocated for it.
    pub fn flip(&self, horizontal: bool) -> Result<Pixbuf, ()> {
        unsafe {
            Option::from_glib_full((ffi::gdk_pixbuf_flip(self.to_glib_none().0,
                                                         horizontal.to_glib()))).ok_or(())
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

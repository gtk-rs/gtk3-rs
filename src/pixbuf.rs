// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::mem;
use std::path::Path;
use std::ptr;
use std::slice;
use libc::{c_void, c_uchar};
use glib::object::IsA;
use glib::translate::*;
use glib::Error;
use gio;
use ffi;
use glib_ffi;
use gobject_ffi;
use gio_ffi;

use {
    Colorspace,
    Pixbuf,
    PixbufExt,
    PixbufFormat,
};

impl Pixbuf {
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
                ffi::gdk_pixbuf_new_from_data(ptr, colorspace.to_glib(), has_alpha.to_glib(), bits_per_sample,
                    width, height, row_stride, Some(destroy_vec), mem::transmute(vec)))
        }
    }

    pub fn new_from_file<T: AsRef<Path>>(filename: T) -> Result<Pixbuf, Error> {
        #[cfg(windows)]
        use ffi::gdk_pixbuf_new_from_file_utf8 as gdk_pixbuf_new_from_file;
        #[cfg(not(windows))]
        use ffi::gdk_pixbuf_new_from_file;

        unsafe {
            let mut error = ptr::null_mut();
            let ptr = gdk_pixbuf_new_from_file(filename.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn new_from_file_at_size<T: AsRef<Path>>(filename: T, width: i32, height: i32) -> Result<Pixbuf, Error> {
        #[cfg(windows)]
        use ffi::gdk_pixbuf_new_from_file_at_size_utf8
            as gdk_pixbuf_new_from_file_at_size;
        #[cfg(not(windows))]
        use ffi::gdk_pixbuf_new_from_file_at_size;

        unsafe {
            let mut error = ptr::null_mut();
            let ptr = gdk_pixbuf_new_from_file_at_size(filename.as_ref().to_glib_none().0, width, height,
                &mut error);
            if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn new_from_file_at_scale<T: AsRef<Path>>(filename: T, width: i32, height: i32, preserve_aspect_ratio: bool) -> Result<Pixbuf, Error> {
        #[cfg(windows)]
        use ffi::gdk_pixbuf_new_from_file_at_scale_utf8
            as gdk_pixbuf_new_from_file_at_scale;
        #[cfg(not(windows))]
        use ffi::gdk_pixbuf_new_from_file_at_scale;

        unsafe {
            let mut error = ptr::null_mut();
            let ptr = gdk_pixbuf_new_from_file_at_scale(filename.as_ref().to_glib_none().0, width, height,
                preserve_aspect_ratio.to_glib(), &mut error);
            if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn new_from_stream_async<'a, P: IsA<gio::InputStream>, Q: Into<Option<&'a gio::Cancellable>>, R: FnOnce(Result<Pixbuf, Error>) + Send + 'static>(stream: &P, cancellable: Q, callback: R) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<R>> = Box::new(Box::new(callback));
        unsafe extern "C" fn new_from_stream_async_trampoline<R: FnOnce(Result<Pixbuf, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let ptr = ffi::gdk_pixbuf_new_from_stream_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Box<R>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = new_from_stream_async_trampoline::<R>;
        unsafe {
            ffi::gdk_pixbuf_new_from_stream_async(stream.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    pub fn new_from_stream_at_scale_async<'a, P: IsA<gio::InputStream>, Q: Into<Option<&'a gio::Cancellable>>, R: FnOnce(Result<Pixbuf, Error>) + Send + 'static>(stream: &P, width: i32, height: i32, preserve_aspect_ratio: bool, cancellable: Q, callback: R) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<R>> = Box::new(Box::new(callback));
        unsafe extern "C" fn new_from_stream_at_scale_async_trampoline<R: FnOnce(Result<Pixbuf, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let ptr = ffi::gdk_pixbuf_new_from_stream_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Box<R>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = new_from_stream_at_scale_async_trampoline::<R>;
        unsafe {
            ffi::gdk_pixbuf_new_from_stream_at_scale_async(stream.to_glib_none().0, width, height, preserve_aspect_ratio.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    pub unsafe fn get_pixels(&self) -> &mut [u8] {
        let mut len = 0;
        let ptr = ffi::gdk_pixbuf_get_pixels_with_length(self.to_glib_none().0, &mut len);
        slice::from_raw_parts_mut(ptr, len as usize)
    }

    pub fn put_pixel(&self, x: i32, y: i32, red: u8, green: u8, blue: u8, alpha: u8) {
        unsafe {
            let n_channels = self.get_n_channels();
            assert!(n_channels == 3 || n_channels == 4);
            let rowstride = self.get_rowstride();
            let pixels = self.get_pixels();
            let pos = (y * rowstride + x * n_channels) as usize;

            pixels[pos] = red;
            pixels[pos + 1] = green;
            pixels[pos + 2] = blue;
            if n_channels == 4 {
                pixels[pos + 3] = alpha;
            }
        }
    }

    pub fn get_file_info<T: AsRef<Path>>(filename: T) -> Option<(PixbufFormat, i32, i32)> {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            let ret = ffi::gdk_pixbuf_get_file_info(filename.as_ref().to_glib_none().0, &mut width, &mut height);
            if !ret.is_null() {
                Some((from_glib_none(ret), width, height))
            } else {
                None
            }
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_file_info_async<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<Option<(PixbufFormat, i32, i32)>, Error>) + Send + 'static, T: AsRef<Path>>(filename: T, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn get_file_info_async_trampoline<Q: FnOnce(Result<Option<(PixbufFormat, i32, i32)>, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            let ret = ffi::gdk_pixbuf_get_file_info_finish(res, &mut width, &mut height, &mut error);
            let result = if !error.is_null() {
                Err(from_glib_full(error))
            } else if ret.is_null() {
                Ok(None)
            } else {
                Ok(Some((from_glib_none(ret), width, height)))
            };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = get_file_info_async_trampoline::<Q>;
        unsafe {
            ffi::gdk_pixbuf_get_file_info_async(filename.as_ref().to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    pub fn save_to_bufferv(&self, type_: &str, options: &[(&str, &str)]) -> Result<Vec<u8>, Error> {
        unsafe {
            let mut buffer = ptr::null_mut();
            let mut buffer_size = mem::uninitialized();
            let mut error = ptr::null_mut();
            let option_keys: Vec<&str> = options.iter().map(|o| o.0).collect();
            let option_values: Vec<&str> = options.iter().map(|o| o.1).collect();
            let _ = ffi::gdk_pixbuf_save_to_bufferv(self.to_glib_none().0, &mut buffer, &mut buffer_size, type_.to_glib_none().0, option_keys.to_glib_none().0, option_values.to_glib_none().0, &mut error);
            if error.is_null() { Ok(FromGlibContainer::from_glib_full_num(buffer, buffer_size as usize)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    pub fn save_to_streamv<'a, P: IsA<gio::OutputStream>, Q: Into<Option<&'a gio::Cancellable>>>(&self, stream: &P, type_: &str, options: &[(&str, &str)], cancellable: Q) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let option_keys: Vec<&str> = options.iter().map(|o| o.0).collect();
            let option_values: Vec<&str> = options.iter().map(|o| o.1).collect();
            let _ = ffi::gdk_pixbuf_save_to_streamv(self.to_glib_none().0, stream.to_glib_none().0, type_.to_glib_none().0, option_keys.to_glib_none().0, option_values.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn savev<T: AsRef<Path>>(&self, filename: T, type_: &str, options: &[(&str, &str)]) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let option_keys: Vec<&str> = options.iter().map(|o| o.0).collect();
            let option_values: Vec<&str> = options.iter().map(|o| o.1).collect();
            let _ = ffi::gdk_pixbuf_savev(self.to_glib_none().0, filename.as_ref().to_glib_none().0, type_.to_glib_none().0, option_keys.to_glib_none().0, option_values.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}

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

#[cfg(feature = "futures")]
use futures_core::Future;

use {
    Colorspace,
    Pixbuf,
    PixbufFormat,
};

impl Pixbuf {
    pub fn new_from_mut_slice<T: AsMut<[u8]>>(data: T, colorspace: Colorspace, has_alpha: bool,
            bits_per_sample: i32, width: i32, height: i32, row_stride: i32) -> Pixbuf {
        unsafe extern "C" fn destroy<T: AsMut<[u8]>>(_: *mut c_uchar, data: *mut c_void) {
            let _data: Box<T> = Box::from_raw(data as *mut T); // the data will be destroyed now
        }

        assert!(bits_per_sample == 8);
        let n_channels = if has_alpha { 4 } else { 3 };
        let last_row_len = width * ((n_channels * bits_per_sample + 7) / 8);

        let mut data: Box<T> = Box::new(data);

        let ptr = {
            let data: &mut [u8] = (*data).as_mut();
            assert!(data.len() == ((height - 1) * row_stride + last_row_len) as usize);
            data.as_mut_ptr()
        };

        unsafe {
            from_glib_full(
                ffi::gdk_pixbuf_new_from_data(ptr, colorspace.to_glib(), has_alpha.to_glib(), bits_per_sample,
                    width, height, row_stride, Some(destroy::<T>), Box::into_raw(data) as *mut _))
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
        let user_data: Box<R> = Box::new(callback);
        unsafe extern "C" fn new_from_stream_async_trampoline<R: FnOnce(Result<Pixbuf, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ptr = ffi::gdk_pixbuf_new_from_stream_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<R> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = new_from_stream_async_trampoline::<R>;
        unsafe {
            ffi::gdk_pixbuf_new_from_stream_async(stream.as_ref().to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    pub fn new_from_stream_async_future<P: IsA<gio::InputStream> + Clone + 'static>(stream: &P) -> Box<Future<Item = Pixbuf, Error=Error>> {
        use gio::GioFuture;

        let stream = stream.clone();
        GioFuture::new(&(), move |_obj, send| {
            use fragile::Fragile;

            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            Self::new_from_stream_async(
                &stream,
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    pub fn new_from_stream_at_scale_async<'a, P: IsA<gio::InputStream>, Q: Into<Option<&'a gio::Cancellable>>, R: FnOnce(Result<Pixbuf, Error>) + Send + 'static>(stream: &P, width: i32, height: i32, preserve_aspect_ratio: bool, cancellable: Q, callback: R) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<R> = Box::new(callback);
        unsafe extern "C" fn new_from_stream_at_scale_async_trampoline<R: FnOnce(Result<Pixbuf, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ptr = ffi::gdk_pixbuf_new_from_stream_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<R> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = new_from_stream_at_scale_async_trampoline::<R>;
        unsafe {
            ffi::gdk_pixbuf_new_from_stream_at_scale_async(stream.as_ref().to_glib_none().0, width, height, preserve_aspect_ratio.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    pub fn new_from_stream_at_scale_async_future<P: IsA<gio::InputStream> + Clone + 'static>(stream: &P, width: i32, height: i32, preserve_aspect_ratio: bool) -> Box<Future<Item = Pixbuf, Error=Error>> {
        use gio::GioFuture;

        let stream = stream.clone();
        GioFuture::new(&(), move |_obj, send| {
            use fragile::Fragile;

            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            Self::new_from_stream_at_scale_async(
                &stream,
                width,
                height,
                preserve_aspect_ratio,
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    #[cfg_attr(feature = "cargo-clippy", allow(mut_from_ref))]
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
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn get_file_info_async_trampoline<Q: FnOnce(Result<Option<(PixbufFormat, i32, i32)>, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
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
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = get_file_info_async_trampoline::<Q>;
        unsafe {
            ffi::gdk_pixbuf_get_file_info_async(filename.as_ref().to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_file_info_async_future<T: AsRef<Path> + Clone + 'static>(filename: T) -> Box<Future<Item = Option<(PixbufFormat, i32, i32)>, Error=Error>> {
        use gio::GioFuture;

        GioFuture::new(&(), move |_obj, send| {
            use fragile::Fragile;

            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            Self::get_file_info_async(
                filename,
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
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
            let _ = ffi::gdk_pixbuf_save_to_streamv(self.to_glib_none().0, stream.as_ref().to_glib_none().0, type_.to_glib_none().0, option_keys.to_glib_none().0, option_values.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    pub fn save_to_streamv_async<'a, P: IsA<gio::OutputStream>, Q: Into<Option<&'a gio::Cancellable>>, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, stream: &P, type_: &str, options: &[(&str, &str)], cancellable: Q, callback: R) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<R> = Box::new(callback);
        unsafe extern "C" fn save_to_streamv_async_trampoline<R: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let _ = ffi::gdk_pixbuf_save_to_stream_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<R> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = save_to_streamv_async_trampoline::<R>;
        unsafe {
            let option_keys: Vec<&str> = options.iter().map(|o| o.0).collect();
            let option_values: Vec<&str> = options.iter().map(|o| o.1).collect();
            ffi::gdk_pixbuf_save_to_streamv_async(self.to_glib_none().0, stream.as_ref().to_glib_none().0, type_.to_glib_none().0, option_keys.to_glib_none().0, option_values.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }

    }

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v2_36", feature = "dox"))]
    pub fn save_to_streamv_async_future<P: IsA<gio::OutputStream> + Clone + 'static>(&self, stream: &P, type_: &str, options: &[(&str, &str)]) -> Box<Future<Item = (Self, ()), Error = (Self, Error)>> {
        use gio::GioFuture;
        use fragile::Fragile;

        let stream = stream.clone();
        let type_ = String::from(type_);
        let options = options.iter().map(|&(k, v)| (String::from(k), String::from(v))).collect::<Vec<(String, String)>>();
        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            let options = options.iter().map(|&(ref k, ref v)| (k.as_str(), v.as_str())).collect::<Vec<(&str, &str)>>();

            obj.save_to_streamv_async(
                 &stream,
                 &type_,
                 options.as_slice(),
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
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

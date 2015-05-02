// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GdkPixbufLoader — Application-driven progressive image loading.

use std;
use ffi;
use glib::{to_bool, GlibContainer};
use glib::translate::ToGlibPtr;
use libc::{c_int, c_uint};

#[repr(C)]
pub struct PixbufLoader {
    pointer: *mut ffi::C_GdkPixbufLoader
}

impl PixbufLoader {
    /// Creates a new pixbuf loader object.
    ///
    /// # Failures
    /// Returns `None` if the pixbuf loader cannot be created.
    pub fn new() -> Option<PixbufLoader> {
        let tmp = unsafe { ffi::gdk_pixbuf_loader_new() };

        if tmp.is_null() {
            None
        } else {
            Some(PixbufLoader::wrap_pointer(tmp))
        }
    }

    /// Creates a new pixbuf loader object that always attempts to parse image
    /// data as if it were an image of type `image_type`, instead of
    /// identifying the type automatically. Useful if you want an error if the
    /// image isn't the expected type, for loading image formats that can't be
    /// reliably identified by looking at the data, or if the user manually
    /// forces a specific type.
    ///
    /// The list of supported image formats depends on what image loaders are
    /// installed, but typically "png", "jpeg", "gif", "tiff" and "xpm" are
    /// among the supported formats.
    ///
    /// # Failures
    /// Returns `None` if the pixbuf loader cannot be created.
    pub fn new_with_type(image_type: &str, error: &mut ::glib::Error) -> Option<PixbufLoader> {
        let tmp = unsafe { ffi::gdk_pixbuf_loader_new_with_type(image_type.borrow_to_glib().0, &mut error.unwrap()) };

        if tmp.is_null() {
            None
        } else {
            Some(PixbufLoader::wrap_pointer(tmp))
        }
    }

    /// Creates a new pixbuf loader object that always attempts to parse image
    /// data as if it were an image of mime type `mime_type`, instead of
    /// identifying the type automatically. Useful if you want an error if the
    /// image isn't the expected mime type, for loading image formats that can't
    /// be reliably identified by looking at the data, or if the user manually
    /// forces a specific mime type.
    ///
    /// The list of supported mime types depends on what image loaders are
    /// installed, but typically "image/png", "image/jpeg", "image/gif",
    /// "image/tiff" and "image/x-xpixmap" are among the supported mime types.
    ///
    /// # Failures
    /// Returns `None` if the pixbuf loader cannot be created.
    pub fn new_with_mime_type(mime_type: &str, error: &mut ::glib::Error) -> Option<PixbufLoader> {
        let tmp = unsafe { ffi::gdk_pixbuf_loader_new_with_mime_type(mime_type.borrow_to_glib().0, &mut error.unwrap()) };

        if tmp.is_null() {
            None
        } else {
            Some(PixbufLoader::wrap_pointer(tmp))
        }
    }

    /// Obtains the available information about the format of the currently
    /// loading image file.
    pub fn get_format(&self) -> Option<::PixbufFormat> {
        let tmp = unsafe { ffi::gdk_pixbuf_loader_get_format(self.unwrap_pointer()) };

        if tmp.is_null() {
            None
        } else {
            Some(::PixbufFormat::wrap_pointer(tmp))
        }
    }

    /// This will cause a pixbuf loader to parse the  bytes of an image stored
    /// in `buf`. It will return `Ok` if the data was loaded successfully, and
    /// `Err` if an error occurred. In the latter case, the loader will be
    /// closed, and will not accept further writes.
    pub fn loader_write(&self, buf: &[u8]) -> Result<(), ::glib::Error> {
        unsafe {
            let mut error: *mut ::glib::ffi::C_GError = std::ptr::null_mut();
            match to_bool(ffi::gdk_pixbuf_loader_write(self.unwrap_pointer(), buf.as_ptr(), buf.len() as c_uint, &mut error)) {
                true => Ok(()),
                false => Err(::glib::Error::wrap(error))
            }
        }
    }

    /*pub fn loader_write_bytes(&self, buffer: &glib::Bytes, error: &mut ::glib::Error) -> bool {
      gdk_pixbuf_loader_write_bytes
      }*/

    /// Causes the image to be scaled while it is loaded. The desired image
    /// size can be determined relative to the original size of the image by
    /// calling `set_size()` from a signal handler for the `size-prepared`
    /// signal.
    ///
    /// Attempts to set the desired image size are ignored after the emission
    /// of the `size-prepared` signal.
    pub fn set_size(&self, width: isize, height: isize) {
        unsafe { ffi::gdk_pixbuf_loader_set_size(self.unwrap_pointer(), width as c_int, height as c_int) }
    }

    /// Queries the `Pixbuf` that a pixbuf loader is currently creating. In
    /// general it only makes sense to call this function after the
    /// `area-prepared` signal has been emitted by the loader; this means that
    /// enough data has been read to know the size of the image that will be
    /// allocated. If the loader has not received enough data via
    /// `loader_write()`, then this function returns `None`. If the loader is
    /// an animation, it will return the "static image" of the animation.
    pub fn get_pixbuf(&self) -> Option<::Pixbuf> {
        let tmp = unsafe { ffi::gdk_pixbuf_loader_get_pixbuf(self.unwrap_pointer()) };

        if tmp.is_null() {
            None
        } else {
            Some(::Pixbuf::wrap_pointer(tmp))
        }
    }

    /*pub fn get_animation(&self) -> Option<::PixbufAnimation> {
      let tmp = unsafe { ffi::gdk_pixbuf_loader_get_animation(self.unwrap_pointer()) };

      if tmp.is_null() {
      None
      } else {
      Some(::PixbufAnimation::wrap_pointer(tmp))
      }
      }*/

    /// Informs a pixbuf loader that no further writes with `loader_write()`
    /// will occur, so that it can free its internal loading structures. Also,
    /// tries to parse any data that hasn't yet been parsed; if the remaining
    /// data is partial or corrupt, an error will be returned.
    pub fn close(&self) -> Result<(), ::glib::Error> {
        unsafe {
            let mut error: *mut ::glib::ffi::C_GError = std::ptr::null_mut();
            match to_bool(ffi::gdk_pixbuf_loader_close(self.unwrap_pointer(), &mut error)) {
                true => Ok(()),
                false => Err(::glib::Error::wrap(error))
            }
        }
    }
}

impl_GObjectFunctions!(PixbufLoader, C_GdkPixbufLoader);

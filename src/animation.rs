// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use super::Pixbuf;
use gdk_pixbuf_sys;
use glib::object::IsA;
use glib::translate::*;
use glib::{Error, TimeVal};
use std::future::Future;
use std::io::Read;
use std::path::Path;
use std::pin::Pin;
use std::ptr;

glib_wrapper! {
    pub struct PixbufAnimationIter(Object<gdk_pixbuf_sys::GdkPixbufAnimationIter, PixbufAnimationIterClass>);

    match fn {
        get_type => || gdk_pixbuf_sys::gdk_pixbuf_animation_iter_get_type(),
    }
}

impl PixbufAnimationIter {
    pub fn advance(&self, start_time: TimeVal) -> bool {
        unsafe {
            from_glib(gdk_pixbuf_sys::gdk_pixbuf_animation_iter_advance(
                self.to_glib_none().0,
                &start_time as *const _,
            ))
        }
    }

    pub fn get_pixbuf(&self) -> Pixbuf {
        unsafe {
            from_glib_none(gdk_pixbuf_sys::gdk_pixbuf_animation_iter_get_pixbuf(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_delay_time(&self) -> i32 {
        unsafe { gdk_pixbuf_sys::gdk_pixbuf_animation_iter_get_delay_time(self.to_glib_none().0) }
    }

    pub fn on_currently_loading_frame(&self) -> bool {
        unsafe {
            from_glib(
                gdk_pixbuf_sys::gdk_pixbuf_animation_iter_on_currently_loading_frame(
                    self.to_glib_none().0,
                ),
            )
        }
    }
}

glib_wrapper! {
    pub struct PixbufAnimation(Object<gdk_pixbuf_sys::GdkPixbufAnimation, PixbufAnimationClass>);

    match fn {
        get_type => || gdk_pixbuf_sys::gdk_pixbuf_animation_get_type(),
    }
}

impl PixbufAnimation {
    pub fn from_file<T: AsRef<Path>>(file: T) -> Result<PixbufAnimation, Error> {
        #[cfg(not(windows))]
        use gdk_pixbuf_sys::gdk_pixbuf_animation_new_from_file;
        #[cfg(windows)]
        use gdk_pixbuf_sys::gdk_pixbuf_animation_new_from_file_utf8 as gdk_pixbuf_animation_new_from_file;

        unsafe {
            let mut error = ptr::null_mut();
            let ptr =
                gdk_pixbuf_animation_new_from_file(file.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn from_resource(resource_path: &str) -> Result<PixbufAnimation, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ptr = gdk_pixbuf_sys::gdk_pixbuf_animation_new_from_resource(
                resource_path.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a `Pixbuf` from a type implementing `Read` (like `File`).
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use gdk_pixbuf::PixbufAnimation;
    ///
    /// let f = File::open("some_file").expect("failed to open animation");
    /// let pixbuf = PixbufAnimation::from_read(f).expect("failed to load animation");
    /// ```
    pub fn from_read<R: Read + Send + 'static>(r: R) -> Result<PixbufAnimation, Error> {
        PixbufAnimation::from_stream(&gio::ReadInputStream::new(r), None::<&gio::Cancellable>)
    }

    pub fn from_stream<P: IsA<gio::InputStream>, Q: IsA<gio::Cancellable>>(
        stream: &P,
        cancellable: Option<&Q>,
    ) -> Result<PixbufAnimation, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gdk_pixbuf_sys::gdk_pixbuf_animation_new_from_stream(
                stream.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn from_stream_async<
        'a,
        P: IsA<gio::InputStream>,
        Q: IsA<gio::Cancellable>,
        R: FnOnce(Result<PixbufAnimation, Error>) + Send + 'static,
    >(
        stream: &P,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let cancellable = cancellable.map(|p| p.as_ref());
        let user_data: Box<R> = Box::new(callback);
        unsafe extern "C" fn from_stream_async_trampoline<
            R: FnOnce(Result<PixbufAnimation, Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ptr = gdk_pixbuf_sys::gdk_pixbuf_animation_new_from_stream_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<R> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = from_stream_async_trampoline::<R>;
        unsafe {
            gdk_pixbuf_sys::gdk_pixbuf_animation_new_from_stream_async(
                stream.as_ref().to_glib_none().0,
                cancellable.to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn from_stream_async_future<P: IsA<gio::InputStream> + Clone + 'static>(
        stream: &P,
    ) -> Pin<Box<dyn Future<Output = Result<PixbufAnimation, Error>> + 'static>> {
        let stream = stream.clone();
        Box::pin(gio::GioFuture::new(&(), move |_obj, send| {
            let cancellable = gio::Cancellable::new();
            Self::from_stream_async(&stream, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }
}

pub trait PixbufAnimationExt {
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;
    fn get_iter(&self, start_time: TimeVal) -> PixbufAnimationIter;
    fn is_static_image(&self) -> bool;
    fn get_static_image(&self) -> Option<Pixbuf>;
}

impl<T: IsA<PixbufAnimation>> PixbufAnimationExt for T {
    fn get_width(&self) -> i32 {
        unsafe { gdk_pixbuf_sys::gdk_pixbuf_animation_get_width(self.as_ref().to_glib_none().0) }
    }

    fn get_height(&self) -> i32 {
        unsafe { gdk_pixbuf_sys::gdk_pixbuf_animation_get_height(self.as_ref().to_glib_none().0) }
    }

    fn get_iter(&self, start_time: TimeVal) -> PixbufAnimationIter {
        unsafe {
            from_glib_full(gdk_pixbuf_sys::gdk_pixbuf_animation_get_iter(
                self.as_ref().to_glib_none().0,
                &start_time as *const _,
            ))
        }
    }

    fn is_static_image(&self) -> bool {
        unsafe {
            from_glib(gdk_pixbuf_sys::gdk_pixbuf_animation_is_static_image(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_static_image(&self) -> Option<Pixbuf> {
        unsafe {
            from_glib_none(gdk_pixbuf_sys::gdk_pixbuf_animation_get_static_image(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

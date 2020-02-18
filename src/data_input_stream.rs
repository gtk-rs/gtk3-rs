// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gio_sys;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::mem;
use std::pin::Pin;
use std::ptr;
use Cancellable;
use DataInputStream;

pub trait DataInputStreamExtManual: 'static {
    fn read_line<P: IsA<Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<Vec<u8>, glib::Error>;

    fn read_line_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Vec<u8>, glib::Error>) + Send + 'static,
    >(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn read_line_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<u8>, glib::Error>> + 'static>>;

    fn read_line_utf8<P: IsA<Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<Option<GString>, glib::Error>;

    fn read_line_utf8_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Option<GString>, glib::Error>) + Send + 'static,
    >(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn read_line_utf8_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Option<GString>, glib::Error>> + 'static>>;

    #[cfg_attr(feature = "v2_56", deprecated)]
    fn read_until<P: IsA<Cancellable>>(
        &self,
        stop_chars: &[u8],
        cancellable: Option<&P>,
    ) -> Result<Vec<u8>, glib::Error>;

    #[cfg_attr(feature = "v2_56", deprecated)]
    fn read_until_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Vec<u8>, glib::Error>) + Send + 'static,
    >(
        &self,
        stop_chars: &[u8],
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    #[cfg_attr(feature = "v2_56", deprecated)]
    fn read_until_async_future(
        &self,
        stop_chars: &[u8],
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<u8>, glib::Error>> + 'static>>;

    fn read_upto<P: IsA<Cancellable>>(
        &self,
        stop_chars: &[u8],
        cancellable: Option<&P>,
    ) -> Result<Vec<u8>, glib::Error>;

    fn read_upto_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Vec<u8>, glib::Error>) + Send + 'static,
    >(
        &self,
        stop_chars: &[u8],
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn read_upto_async_future(
        &self,
        stop_chars: &[u8],
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<u8>, glib::Error>> + 'static>>;
}

impl<O: IsA<DataInputStream>> DataInputStreamExtManual for O {
    fn read_line<P: IsA<Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<Vec<u8>, glib::Error> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_data_input_stream_read_line(
                self.as_ref().to_glib_none().0,
                length.as_mut_ptr(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            let length = length.assume_init();
            if error.is_null() {
                Ok(FromGlibContainer::from_glib_full_num(ret, length))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_line_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Vec<u8>, glib::Error>) + Send + 'static,
    >(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn read_line_async_trampoline<
            Q: FnOnce(Result<Vec<u8>, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut length = mem::MaybeUninit::uninit();
            let ret = gio_sys::g_data_input_stream_read_line_finish(
                _source_object as *mut _,
                res,
                length.as_mut_ptr(),
                &mut error,
            );
            let length = length.assume_init();
            let result = if error.is_null() {
                Ok(FromGlibContainer::from_glib_full_num(ret, length))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_line_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_data_input_stream_read_line_async(
                self.as_ref().to_glib_none().0,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn read_line_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<u8>, glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.read_line_async(io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn read_line_utf8<P: IsA<Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<Option<GString>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_data_input_stream_read_line_utf8(
                self.as_ref().to_glib_none().0,
                ptr::null_mut(),
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

    fn read_line_utf8_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Option<GString>, glib::Error>) + Send + 'static,
    >(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn read_line_async_trampoline<
            Q: FnOnce(Result<Option<GString>, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_data_input_stream_read_line_finish(
                _source_object as *mut _,
                res,
                ptr::null_mut(),
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_line_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_data_input_stream_read_line_async(
                self.as_ref().to_glib_none().0,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn read_line_utf8_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Option<GString>, glib::Error>> + 'static>>
    {
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.read_line_utf8_async(io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn read_until<P: IsA<Cancellable>>(
        &self,
        stop_chars: &[u8],
        cancellable: Option<&P>,
    ) -> Result<Vec<u8>, glib::Error> {
        // Need to ensure that it does not contain a NUL byte and then NUL-terminate it ourselves
        assert!(!stop_chars.contains(&b'0'));
        let stop_chars = stop_chars
            .iter()
            .copied()
            .chain(std::iter::once(b'0'))
            .collect::<Vec<_>>();

        unsafe {
            let mut error = ptr::null_mut();
            let mut length = mem::MaybeUninit::uninit();
            let ret = gio_sys::g_data_input_stream_read_until(
                self.as_ref().to_glib_none().0,
                stop_chars.to_glib_none().0 as *const _,
                length.as_mut_ptr(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            let length = length.assume_init();
            if error.is_null() {
                Ok(FromGlibContainer::from_glib_full_num(
                    ret as *const u8,
                    length,
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_until_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Vec<u8>, glib::Error>) + Send + 'static,
    >(
        &self,
        stop_chars: &[u8],
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn read_until_async_trampoline<
            Q: FnOnce(Result<Vec<u8>, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut length = mem::MaybeUninit::uninit();
            let ret = gio_sys::g_data_input_stream_read_until_finish(
                _source_object as *mut _,
                res,
                length.as_mut_ptr(),
                &mut error,
            );
            let result = if error.is_null() {
                let length = length.assume_init();
                Ok(FromGlibContainer::from_glib_full_num(
                    ret as *const _,
                    length,
                ))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        // Need to ensure that it does not contain a NUL byte and then NUL-terminate it ourselves
        assert!(!stop_chars.contains(&b'0'));
        let stop_chars = stop_chars
            .iter()
            .copied()
            .chain(std::iter::once(b'0'))
            .collect::<Vec<_>>();

        let callback = read_until_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_data_input_stream_read_until_async(
                self.as_ref().to_glib_none().0,
                stop_chars.to_glib_none().0 as *const _,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn read_until_async_future(
        &self,
        stop_chars: &[u8],
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<u8>, glib::Error>> + 'static>> {
        let stop_chars = Vec::from(stop_chars);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.read_until_async(&stop_chars, io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn read_upto<P: IsA<Cancellable>>(
        &self,
        stop_chars: &[u8],
        cancellable: Option<&P>,
    ) -> Result<Vec<u8>, glib::Error> {
        let stop_chars_len = stop_chars.len() as isize;
        unsafe {
            let mut error = ptr::null_mut();
            let mut length = mem::MaybeUninit::uninit();
            let ret = gio_sys::g_data_input_stream_read_upto(
                self.as_ref().to_glib_none().0,
                stop_chars.to_glib_none().0 as *const _,
                stop_chars_len,
                length.as_mut_ptr(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                let length = length.assume_init();
                Ok(FromGlibContainer::from_glib_full_num(
                    ret as *const _,
                    length,
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_upto_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Vec<u8>, glib::Error>) + Send + 'static,
    >(
        &self,
        stop_chars: &[u8],
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let stop_chars_len = stop_chars.len() as isize;
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn read_upto_async_trampoline<
            Q: FnOnce(Result<Vec<u8>, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut length = mem::MaybeUninit::uninit();
            let ret = gio_sys::g_data_input_stream_read_upto_finish(
                _source_object as *mut _,
                res,
                length.as_mut_ptr(),
                &mut error,
            );
            let result = if error.is_null() {
                let length = length.assume_init();
                Ok(FromGlibContainer::from_glib_full_num(
                    ret as *const _,
                    length,
                ))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_upto_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_data_input_stream_read_upto_async(
                self.as_ref().to_glib_none().0,
                stop_chars.to_glib_none().0 as *const _,
                stop_chars_len,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn read_upto_async_future(
        &self,
        stop_chars: &[u8],
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<u8>, glib::Error>> + 'static>> {
        let stop_chars = Vec::from(stop_chars);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.read_upto_async(&stop_chars, io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }
}

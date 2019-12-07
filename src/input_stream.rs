// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use error::to_std_io_result;
use gio_sys;
use glib::object::IsA;
use glib::translate::*;
use glib::Priority;
use glib_sys;
use gobject_sys;
use std::io;
use std::mem;
use std::pin::Pin;
use std::ptr;
use Cancellable;
use InputStream;
use Seekable;
use SeekableExt;

pub trait InputStreamExtManual: Sized {
    fn read<B: AsMut<[u8]>, C: IsA<Cancellable>>(
        &self,
        buffer: B,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error>;

    fn read_all<B: AsMut<[u8]>, C: IsA<Cancellable>>(
        &self,
        buffer: B,
        cancellable: Option<&C>,
    ) -> Result<(usize, Option<glib::Error>), glib::Error>;

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn read_all_async<
        B: AsMut<[u8]> + Send + 'static,
        Q: FnOnce(Result<(B, usize, Option<glib::Error>), (B, glib::Error)>) + Send + 'static,
        C: IsA<Cancellable>,
    >(
        &self,
        buffer: B,
        io_priority: Priority,
        cancellable: Option<&C>,
        callback: Q,
    );

    fn read_async<
        B: AsMut<[u8]> + Send + 'static,
        Q: FnOnce(Result<(B, usize), (B, glib::Error)>) + Send + 'static,
        C: IsA<Cancellable>,
    >(
        &self,
        buffer: B,
        io_priority: Priority,
        cancellable: Option<&C>,
        callback: Q,
    );

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn read_all_async_future<B: AsMut<[u8]> + Send + 'static>(
        &self,
        buffer: B,
        io_priority: Priority,
    ) -> Pin<
        Box<
            dyn std::future::Future<
                    Output = Result<(B, usize, Option<glib::Error>), (B, glib::Error)>,
                > + 'static,
        >,
    >;

    fn read_async_future<B: AsMut<[u8]> + Send + 'static>(
        &self,
        buffer: B,
        io_priority: Priority,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(B, usize), (B, glib::Error)>> + 'static>>;

    fn into_read(self) -> InputStreamRead<Self>
    where
        Self: IsA<InputStream>,
    {
        InputStreamRead(self)
    }
}

impl<O: IsA<InputStream>> InputStreamExtManual for O {
    fn read<B: AsMut<[u8]>, C: IsA<Cancellable>>(
        &self,
        mut buffer: B,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let buffer = buffer.as_mut();
        let buffer_ptr = buffer.as_mut_ptr();
        let count = buffer.len();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_input_stream_read(
                self.as_ref().to_glib_none().0,
                buffer_ptr,
                count,
                gcancellable.0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_all<B: AsMut<[u8]>, C: IsA<Cancellable>>(
        &self,
        mut buffer: B,
        cancellable: Option<&C>,
    ) -> Result<(usize, Option<glib::Error>), glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let buffer = buffer.as_mut();
        let buffer_ptr = buffer.as_mut_ptr();
        let count = buffer.len();
        unsafe {
            let mut bytes_read = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_input_stream_read_all(
                self.as_ref().to_glib_none().0,
                buffer_ptr,
                count,
                bytes_read.as_mut_ptr(),
                gcancellable.0,
                &mut error,
            );

            let bytes_read = bytes_read.assume_init();
            if error.is_null() {
                Ok((bytes_read, None))
            } else if bytes_read != 0 {
                Ok((bytes_read, Some(from_glib_full(error))))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn read_all_async<
        B: AsMut<[u8]> + Send + 'static,
        Q: FnOnce(Result<(B, usize, Option<glib::Error>), (B, glib::Error)>) + Send + 'static,
        C: IsA<Cancellable>,
    >(
        &self,
        buffer: B,
        io_priority: Priority,
        cancellable: Option<&C>,
        callback: Q,
    ) {
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let mut user_data: Box<Option<(Q, B)>> = Box::new(Some((callback, buffer)));
        // Need to do this after boxing as the contents pointer might change by moving into the box
        let (count, buffer_ptr) = {
            let buffer = &mut (*user_data).as_mut().unwrap().1;
            let slice = (*buffer).as_mut();
            (slice.len(), slice.as_mut_ptr())
        };
        unsafe extern "C" fn read_all_async_trampoline<
            B: AsMut<[u8]> + Send + 'static,
            Q: FnOnce(Result<(B, usize, Option<glib::Error>), (B, glib::Error)>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut user_data: Box<Option<(Q, B)>> = Box::from_raw(user_data as *mut _);
            let (callback, buffer) = user_data.take().unwrap();

            let mut error = ptr::null_mut();
            let mut bytes_read = mem::MaybeUninit::uninit();
            let _ = gio_sys::g_input_stream_read_all_finish(
                _source_object as *mut _,
                res,
                bytes_read.as_mut_ptr(),
                &mut error,
            );

            let bytes_read = bytes_read.assume_init();
            let result = if error.is_null() {
                Ok((buffer, bytes_read, None))
            } else if bytes_read != 0 {
                Ok((buffer, bytes_read, Some(from_glib_full(error))))
            } else {
                Err((buffer, from_glib_full(error)))
            };

            callback(result);
        }
        let callback = read_all_async_trampoline::<B, Q>;
        unsafe {
            gio_sys::g_input_stream_read_all_async(
                self.as_ref().to_glib_none().0,
                buffer_ptr,
                count,
                io_priority.to_glib(),
                gcancellable.0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    fn read_async<
        B: AsMut<[u8]> + Send + 'static,
        Q: FnOnce(Result<(B, usize), (B, glib::Error)>) + Send + 'static,
        C: IsA<Cancellable>,
    >(
        &self,
        buffer: B,
        io_priority: Priority,
        cancellable: Option<&C>,
        callback: Q,
    ) {
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let mut user_data: Box<Option<(Q, B)>> = Box::new(Some((callback, buffer)));
        // Need to do this after boxing as the contents pointer might change by moving into the box
        let (count, buffer_ptr) = {
            let buffer = &mut (*user_data).as_mut().unwrap().1;
            let slice = (*buffer).as_mut();
            (slice.len(), slice.as_mut_ptr())
        };
        unsafe extern "C" fn read_async_trampoline<
            B: AsMut<[u8]> + Send + 'static,
            Q: FnOnce(Result<(B, usize), (B, glib::Error)>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut user_data: Box<Option<(Q, B)>> = Box::from_raw(user_data as *mut _);
            let (callback, buffer) = user_data.take().unwrap();

            let mut error = ptr::null_mut();
            let ret =
                gio_sys::g_input_stream_read_finish(_source_object as *mut _, res, &mut error);

            let result = if error.is_null() {
                Ok((buffer, ret as usize))
            } else {
                Err((buffer, from_glib_full(error)))
            };

            callback(result);
        }
        let callback = read_async_trampoline::<B, Q>;
        unsafe {
            gio_sys::g_input_stream_read_async(
                self.as_ref().to_glib_none().0,
                buffer_ptr,
                count,
                io_priority.to_glib(),
                gcancellable.0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn read_all_async_future<'a, B: AsMut<[u8]> + Send + 'static>(
        &self,
        buffer: B,
        io_priority: Priority,
    ) -> Pin<
        Box<
            dyn std::future::Future<
                    Output = Result<(B, usize, Option<glib::Error>), (B, glib::Error)>,
                > + 'static,
        >,
    > {
        Box::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.read_all_async(buffer, io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn read_async_future<'a, B: AsMut<[u8]> + Send + 'static>(
        &self,
        buffer: B,
        io_priority: Priority,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(B, usize), (B, glib::Error)>> + 'static>>
    {
        Box::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.read_async(buffer, io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }
}

#[derive(Debug)]
pub struct InputStreamRead<T: IsA<InputStream>>(T);

impl<T: IsA<InputStream>> InputStreamRead<T> {
    pub fn into_input_stream(self) -> T {
        self.0
    }

    pub fn input_stream(&self) -> &T {
        &self.0
    }
}

impl<T: IsA<InputStream>> io::Read for InputStreamRead<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let gio_result = self.0.as_ref().read(buf, ::NONE_CANCELLABLE);
        to_std_io_result(gio_result)
    }
}

impl<T: IsA<InputStream> + IsA<Seekable>> io::Seek for InputStreamRead<T> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        let (pos, type_) = match pos {
            io::SeekFrom::Start(pos) => (pos as i64, glib::SeekType::Set),
            io::SeekFrom::End(pos) => (pos, glib::SeekType::End),
            io::SeekFrom::Current(pos) => (pos, glib::SeekType::Cur),
        };
        let seekable: &Seekable = self.0.as_ref();
        let gio_result = seekable
            .seek(pos, type_, ::NONE_CANCELLABLE)
            .map(|_| seekable.tell() as u64);
        to_std_io_result(gio_result)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::MemoryInputStream;
    use glib::Bytes;
    use std::io::Read;
    use test_util::run_async;

    #[test]
    #[cfg(feature = "v2_44")]
    fn read_all_async() {
        let ret = run_async(|tx, l| {
            let b = Bytes::from_owned(vec![1, 2, 3]);
            let strm = MemoryInputStream::new_from_bytes(&b);

            let buf = vec![0; 10];
            strm.read_all_async(
                buf,
                glib::PRIORITY_DEFAULT_IDLE,
                ::NONE_CANCELLABLE,
                move |ret| {
                    tx.send(ret).unwrap();
                    l.quit();
                },
            );
        });

        let (buf, count, err) = ret.unwrap();
        assert_eq!(count, 3);
        assert!(err.is_none());
        assert_eq!(buf[0], 1);
        assert_eq!(buf[1], 2);
        assert_eq!(buf[2], 3);
    }

    #[test]
    fn read_all() {
        let b = Bytes::from_owned(vec![1, 2, 3]);
        let strm = MemoryInputStream::new_from_bytes(&b);
        let mut buf = vec![0; 10];

        let ret = strm.read_all(&mut buf, ::NONE_CANCELLABLE).unwrap();

        assert_eq!(ret.0, 3);
        assert!(ret.1.is_none());
        assert_eq!(buf[0], 1);
        assert_eq!(buf[1], 2);
        assert_eq!(buf[2], 3);
    }

    #[test]
    fn read() {
        let b = Bytes::from_owned(vec![1, 2, 3]);
        let strm = MemoryInputStream::new_from_bytes(&b);
        let mut buf = vec![0; 10];

        let ret = strm.read(&mut buf, ::NONE_CANCELLABLE);

        assert_eq!(ret.unwrap(), 3);
        assert_eq!(buf[0], 1);
        assert_eq!(buf[1], 2);
        assert_eq!(buf[2], 3);
    }

    #[test]
    fn read_async() {
        let ret = run_async(|tx, l| {
            let b = Bytes::from_owned(vec![1, 2, 3]);
            let strm = MemoryInputStream::new_from_bytes(&b);

            let buf = vec![0; 10];
            strm.read_async(
                buf,
                glib::PRIORITY_DEFAULT_IDLE,
                ::NONE_CANCELLABLE,
                move |ret| {
                    tx.send(ret).unwrap();
                    l.quit();
                },
            );
        });

        let (buf, count) = ret.unwrap();
        assert_eq!(count, 3);
        assert_eq!(buf[0], 1);
        assert_eq!(buf[1], 2);
        assert_eq!(buf[2], 3);
    }

    #[test]
    fn read_bytes_async() {
        let ret = run_async(|tx, l| {
            let b = Bytes::from_owned(vec![1, 2, 3]);
            let strm = MemoryInputStream::new_from_bytes(&b);

            strm.read_bytes_async(
                10,
                glib::PRIORITY_DEFAULT_IDLE,
                ::NONE_CANCELLABLE,
                move |ret| {
                    tx.send(ret).unwrap();
                    l.quit();
                },
            );
        });

        let bytes = ret.unwrap();
        assert_eq!(bytes, vec![1, 2, 3]);
    }

    #[test]
    fn skip_async() {
        let ret = run_async(|tx, l| {
            let b = Bytes::from_owned(vec![1, 2, 3]);
            let strm = MemoryInputStream::new_from_bytes(&b);

            strm.skip_async(
                10,
                glib::PRIORITY_DEFAULT_IDLE,
                ::NONE_CANCELLABLE,
                move |ret| {
                    tx.send(ret).unwrap();
                    l.quit();
                },
            );
        });

        let skipped = ret.unwrap();
        assert_eq!(skipped, 3);
    }

    #[test]
    fn std_io_read() {
        let b = Bytes::from_owned(vec![1, 2, 3]);
        let mut read = MemoryInputStream::new_from_bytes(&b).into_read();
        let mut buf = [0u8; 10];

        let ret = read.read(&mut buf);

        assert_eq!(ret.unwrap(), 3);
        assert_eq!(buf[0], 1);
        assert_eq!(buf[1], 2);
        assert_eq!(buf[2], 3);
    }

    #[test]
    fn into_input_stream() {
        let b = Bytes::from_owned(vec![1, 2, 3]);
        let stream = MemoryInputStream::new_from_bytes(&b);
        let stream_clone = stream.clone();
        let stream = stream.into_read().into_input_stream();

        assert_eq!(stream, stream_clone);
    }
}

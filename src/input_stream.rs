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
use std::ptr;
use Cancellable;
use Error;
use InputStream;

#[cfg(feature = "futures")]
use futures_core::Future;

pub trait InputStreamExtManual: Sized {
    fn read<B: AsMut<[u8]>>(&self, buffer: B, cancellable: Option<&Cancellable>) -> Result<usize, Error>;

    fn read_all<B: AsMut<[u8]>>(&self, buffer: B, cancellable: Option<&Cancellable>) -> Result<(usize, Option<Error>), Error>;

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn read_all_async<B: AsMut<[u8]> + Send + 'static, Q: FnOnce(Result<(B, usize, Option<Error>), (B, Error)>) + Send + 'static>(&self, buffer: B, io_priority: Priority, cancellable: Option<&Cancellable>, callback: Q);

    fn read_async<B: AsMut<[u8]> + Send + 'static, Q: FnOnce(Result<(B, usize), (B, Error)>) + Send + 'static>(&self, buffer: B, io_priority: Priority, cancellable: Option<&Cancellable>, callback: Q);

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn read_all_async_future<'a, B: AsMut<[u8]> + Send + 'static>(
        &self, buffer: B, io_priority: Priority
    ) -> Box<Future<Item = (Self, (B, usize, Option<Error>)), Error = (Self, (B, Error))>> where Self: Clone;

    #[cfg(feature = "futures")]
    fn read_async_future<'a, B: AsMut<[u8]> + Send + 'static>(
        &self, buffer: B, io_priority: Priority
    ) -> Box<Future<Item = (Self, (B, usize)), Error = (Self, (B, Error))>> where Self: Clone;

    fn into_read(self) -> InputStreamRead<Self> {
        InputStreamRead(self)
    }
}

impl<O: IsA<InputStream>> InputStreamExtManual for O {
    fn read<B: AsMut<[u8]>>(&self, mut buffer: B, cancellable: Option<&Cancellable>) -> Result<usize, Error> {
        let cancellable = cancellable.to_glib_none();
        let buffer = buffer.as_mut();
        let buffer_ptr = buffer.as_mut_ptr();
        let count = buffer.len();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_input_stream_read(self.as_ref().to_glib_none().0, buffer_ptr, count, cancellable.0, &mut error);
            if error.is_null() {
                Ok(ret as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_all<B: AsMut<[u8]>>(&self, mut buffer: B, cancellable: Option<&Cancellable>) -> Result<(usize, Option<Error>), Error> {
        let cancellable = cancellable.to_glib_none();
        let buffer = buffer.as_mut();
        let buffer_ptr = buffer.as_mut_ptr();
        let count = buffer.len();
        unsafe {
            let mut bytes_read = mem::uninitialized();
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_input_stream_read_all(self.as_ref().to_glib_none().0, buffer_ptr, count, &mut bytes_read, cancellable.0, &mut error);

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
    fn read_all_async<B: AsMut<[u8]> + Send + 'static, Q: FnOnce(Result<(B, usize, Option<Error>), (B, Error)>) + Send + 'static>(&self, buffer: B, io_priority: Priority, cancellable: Option<&Cancellable>, callback: Q) {
        let cancellable = cancellable.to_glib_none();
        let mut user_data: Box<Option<(Q, B)>> = Box::new(Some((callback, buffer)));
        // Need to do this after boxing as the contents pointer might change by moving into the box
        let (count, buffer_ptr) = {
            let buffer = &mut (*user_data).as_mut().unwrap().1;
            let slice = (*buffer).as_mut();
            (slice.len(), slice.as_mut_ptr())
        };
        unsafe extern "C" fn read_all_async_trampoline<B: AsMut<[u8]> + Send + 'static, Q: FnOnce(Result<(B, usize, Option<Error>), (B, Error)>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer)
        {
            let mut user_data: Box<Option<(Q, B)>> = Box::from_raw(user_data as *mut _);
            let (callback, buffer) = user_data.take().unwrap();

            let mut error = ptr::null_mut();
            let mut bytes_read = mem::uninitialized();
            let _ = gio_sys::g_input_stream_read_all_finish(_source_object as *mut _, res, &mut bytes_read, &mut error);

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
            gio_sys::g_input_stream_read_all_async(self.as_ref().to_glib_none().0, buffer_ptr, count, io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn read_async<B: AsMut<[u8]> + Send + 'static, Q: FnOnce(Result<(B, usize), (B, Error)>) + Send + 'static>(&self, buffer: B, io_priority: Priority, cancellable: Option<&Cancellable>, callback: Q) {
        let cancellable = cancellable.to_glib_none();
        let mut user_data: Box<Option<(Q, B)>> = Box::new(Some((callback, buffer)));
        // Need to do this after boxing as the contents pointer might change by moving into the box
        let (count, buffer_ptr) = {
            let buffer = &mut (*user_data).as_mut().unwrap().1;
            let slice = (*buffer).as_mut();
            (slice.len(), slice.as_mut_ptr())
        };
        unsafe extern "C" fn read_async_trampoline<B: AsMut<[u8]> + Send + 'static, Q: FnOnce(Result<(B, usize), (B, Error)>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer)
        {
            let mut user_data: Box<Option<(Q, B)>> = Box::from_raw(user_data as *mut _);
            let (callback, buffer) = user_data.take().unwrap();

            let mut error = ptr::null_mut();
            let ret = gio_sys::g_input_stream_read_finish(_source_object as *mut _, res, &mut error);

            let result = if error.is_null() {
                Ok((buffer, ret as usize))
            } else {
                Err((buffer, from_glib_full(error)))
            };

            callback(result);
        }
        let callback = read_async_trampoline::<B, Q>;
        unsafe {
            gio_sys::g_input_stream_read_async(self.as_ref().to_glib_none().0, buffer_ptr, count, io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn read_all_async_future<'a, B: AsMut<[u8]> + Send + 'static>(
        &self, buffer: B, io_priority: Priority
    ) -> Box<Future<Item = (Self, (B, usize, Option<Error>)), Error = (Self, (B, Error))>> where Self: Clone {
        use GioFuture;

        GioFuture::new(self, move |obj, send| {
            use fragile::Fragile;

            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.read_all_async(
                buffer,
                io_priority,
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

    #[cfg(feature = "futures")]
    fn read_async_future<'a, B: AsMut<[u8]> + Send + 'static>(
        &self, buffer: B, io_priority: Priority
    ) -> Box<Future<Item = (Self, (B, usize)), Error = (Self, (B, Error))>> where Self: Clone {
        use GioFuture;

        GioFuture::new(self, move |obj, send| {
            use fragile::Fragile;

            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.read_async(
                buffer,
                io_priority,
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
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct InputStreamRead<T: InputStreamExtManual>(T);

impl <T: InputStreamExtManual> InputStreamRead<T> {
    pub fn into_input_stream(self) -> T {
        self.0
    }

    pub fn input_stream(&self) -> &T {
        &self.0
    }
}

impl <T: InputStreamExtManual> io::Read for InputStreamRead<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let gio_result = self.0.read(buf, None);
        to_std_io_result(gio_result)
    }
}

#[cfg(test)]
mod tests {
    use glib::*;
    use std::io::Read;
    use test_util::run_async;
    use *;

    #[test]
    #[cfg(feature = "v2_44")]
    fn read_all_async() {
        let ret = run_async(|tx, l| {
            let b = Bytes::from_owned(vec![1, 2, 3]);
            let strm = MemoryInputStream::new_from_bytes(&b);

            let buf = vec![0;10];
            strm.read_all_async(buf, PRIORITY_DEFAULT_IDLE, None, move |ret| {
                tx.send(ret).unwrap();
                l.quit();
            });
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
        let mut buf = vec![0;10];

        let ret = strm.read_all(&mut buf, None).unwrap();

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
        let mut buf = vec![0;10];

        let ret = strm.read(&mut buf, None);

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

            let buf = vec![0;10];
            strm.read_async(buf, PRIORITY_DEFAULT_IDLE, None, move |ret| {
                tx.send(ret).unwrap();
                l.quit();
            });
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

            strm.read_bytes_async(10, PRIORITY_DEFAULT_IDLE, ::NONE_CANCELLABLE, move |ret| {
                tx.send(ret).unwrap();
                l.quit();
            });
        });

        let bytes = ret.unwrap();
        assert_eq!(bytes, vec![1, 2, 3]);
    }

    #[test]
    fn skip_async() {
        let ret = run_async(|tx, l| {
            let b = Bytes::from_owned(vec![1, 2, 3]);
            let strm = MemoryInputStream::new_from_bytes(&b);

            strm.skip_async(10, PRIORITY_DEFAULT_IDLE, ::NONE_CANCELLABLE, move |ret| {
                tx.send(ret).unwrap();
                l.quit();
            });
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

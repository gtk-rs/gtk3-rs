// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>


use Cancellable;
use Error;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib::Priority;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;
use InputStream;

pub trait InputStreamExtManual {
    fn read<'a, B: AsMut<[u8]>, P: Into<Option<&'a Cancellable>>>(&self, buffer: B, cancellable: P) -> Result<usize, Error>;

    fn read_all<'a, B: AsMut<[u8]>, P: Into<Option<&'a Cancellable>>>(&self, buffer: B, cancellable: P) -> Result<(usize, Option<Error>), Error>;

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn read_all_async<'a, B: AsMut<[u8]> + Send + 'static, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(B, usize, Option<Error>), (B, Error)>) + Send + 'static>(&self, buffer: B, io_priority: Priority, cancellable: P, callback: Q);

    fn read_async<'a, B: AsMut<[u8]> + Send + 'static, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(B, usize), (B, Error)>) + Send + 'static>(&self, buffer: B, io_priority: Priority, cancellable: P, callback: Q);
}

impl<O: IsA<InputStream>> InputStreamExtManual for O {
    fn read<'a, B: AsMut<[u8]>, P: Into<Option<&'a Cancellable>>>(&self, mut buffer: B, cancellable: P) -> Result<usize, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let buffer = buffer.as_mut();
        let buffer_ptr = buffer.as_mut_ptr();
        let count = buffer.len();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_input_stream_read(self.to_glib_none().0, buffer_ptr, count, cancellable.0, &mut error);
            if error.is_null() {
                Ok(ret as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_all<'a, B: AsMut<[u8]>, P: Into<Option<&'a Cancellable>>>(&self, mut buffer: B, cancellable: P) -> Result<(usize, Option<Error>), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let buffer = buffer.as_mut();
        let buffer_ptr = buffer.as_mut_ptr();
        let count = buffer.len();
        unsafe {
            let mut bytes_read = mem::uninitialized();
            let mut error = ptr::null_mut();
            let _ = ffi::g_input_stream_read_all(self.to_glib_none().0, buffer_ptr, count, &mut bytes_read, cancellable.0, &mut error);

            if error.is_null() {
                Ok((bytes_read, None))
            } else if bytes_read != 0 {
                Ok((bytes_read, Some(from_glib_full(error))))
            } else {
                Err( from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn read_all_async<'a, B: AsMut<[u8]> + Send + 'static, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(B, usize, Option<Error>), (B, Error)>) + Send + 'static>(&self, buffer: B, io_priority: Priority, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let mut buffer: Box<B> = Box::new(buffer);
        let (count, buffer_ptr) = {
            let slice = (*buffer).as_mut();
            (slice.len(), slice.as_mut_ptr())
        };
        let user_data: Box<Option<(Box<Q>, Box<B>)>> = Box::new(Some((Box::new(callback), buffer)));
        unsafe extern "C" fn read_all_async_trampoline<B: AsMut<[u8]> + Send + 'static, Q: FnOnce(Result<(B, usize, Option<Error>), (B, Error)>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();

            let mut user_data: Box<Option<(Box<Q>, Box<B>)>> = Box::from_raw(user_data as *mut _);
            let (callback, buffer) = user_data.take().unwrap();
            let buffer = *buffer;

            let mut error = ptr::null_mut();
            let mut bytes_read = mem::uninitialized();
            let _ = ffi::g_input_stream_read_all_finish(_source_object as *mut _, res, &mut bytes_read, &mut error);

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
            ffi::g_input_stream_read_all_async(self.to_glib_none().0, buffer_ptr, count, io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn read_async<'a, B: AsMut<[u8]> + Send + 'static, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(B, usize), (B, Error)>) + Send + 'static>(&self, buffer: B, io_priority: Priority, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let mut buffer: Box<B> = Box::new(buffer);
        let (count, buffer_ptr) = {
            let slice = (*buffer).as_mut();
            (slice.len(), slice.as_mut_ptr())
        };
        let user_data: Box<Option<(Box<Q>, Box<B>)>> = Box::new(Some((Box::new(callback), buffer)));
        unsafe extern "C" fn read_async_trampoline<B: AsMut<[u8]> + Send + 'static, Q: FnOnce(Result<(B, usize), (B, Error)>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut user_data: Box<Option<(Box<Q>, Box<B>)>> = Box::from_raw(user_data as *mut _);
            let (callback, buffer) = user_data.take().unwrap();
            let buffer = *buffer;

            let mut error = ptr::null_mut();
            let ret = ffi::g_input_stream_read_finish(_source_object as *mut _, res, &mut error);

            let result = if error.is_null() {
                Ok((buffer, ret as usize))
            } else {
                Err((buffer, from_glib_full(error)))
            };

            callback(result);
        }
        let callback = read_async_trampoline::<B, Q>;
        unsafe {
            ffi::g_input_stream_read_async(self.to_glib_none().0, buffer_ptr, count, io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }
}

#[cfg(all(test,any(feature = "v2_34", feature = "dox")))]
mod tests {
    use glib::*;
    use test_util::run_async;
    use *;

    #[test]
    #[cfg(all(test,any(feature = "v2_44", feature = "dox")))]
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
    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn read_bytes_async() {
        let ret = run_async(|tx, l| {
            let b = Bytes::from_owned(vec![1, 2, 3]);
            let strm = MemoryInputStream::new_from_bytes(&b);

            strm.read_bytes_async(10, PRIORITY_DEFAULT_IDLE, None, move |ret| {
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

            strm.skip_async(10, PRIORITY_DEFAULT_IDLE, None, move |ret| {
                tx.send(ret).unwrap();
                l.quit();
            });
        });

        let skipped = ret.unwrap();
        assert_eq!(skipped, 3);
    }
}

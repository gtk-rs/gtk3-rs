// Take a look at the license at the top of the repository in the LICENSE file.

use crate::error::to_std_io_result;
use crate::prelude::*;
use crate::Cancellable;
use crate::OutputStream;
use crate::Seekable;
use crate::SeekableExt;
use glib::object::IsA;
use glib::translate::*;
use glib::Priority;
use std::io;
use std::mem;
use std::pin::Pin;
use std::ptr;

pub trait OutputStreamExtManual: Sized + OutputStreamExt {
    fn write_async<
        B: AsRef<[u8]> + Send + 'static,
        Q: FnOnce(Result<(B, usize), (B, glib::Error)>) + Send + 'static,
        C: IsA<Cancellable>,
    >(
        &self,
        buffer: B,
        io_priority: Priority,
        cancellable: Option<&C>,
        callback: Q,
    );

    fn write_all<C: IsA<Cancellable>>(
        &self,
        buffer: &[u8],
        cancellable: Option<&C>,
    ) -> Result<(usize, Option<glib::Error>), glib::Error>;

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    fn write_all_async<
        B: AsRef<[u8]> + Send + 'static,
        Q: FnOnce(Result<(B, usize, Option<glib::Error>), (B, glib::Error)>) + Send + 'static,
        C: IsA<Cancellable>,
    >(
        &self,
        buffer: B,
        io_priority: Priority,
        cancellable: Option<&C>,
        callback: Q,
    );

    fn write_async_future<B: AsRef<[u8]> + Send + 'static>(
        &self,
        buffer: B,
        io_priority: Priority,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(B, usize), (B, glib::Error)>> + 'static>>;

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    fn write_all_async_future<B: AsRef<[u8]> + Send + 'static>(
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

    fn into_write(self) -> OutputStreamWrite<Self>
    where
        Self: IsA<OutputStream>,
    {
        OutputStreamWrite(self)
    }
}

impl<O: IsA<OutputStream>> OutputStreamExtManual for O {
    fn write_async<
        B: AsRef<[u8]> + Send + 'static,
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
        let user_data: Box<Option<(Q, B)>> = Box::new(Some((callback, buffer)));
        // Need to do this after boxing as the contents pointer might change by moving into the box
        let (count, buffer_ptr) = {
            let buffer = &(*user_data).as_ref().unwrap().1;
            let slice = buffer.as_ref();
            (slice.len(), slice.as_ptr())
        };
        unsafe extern "C" fn write_async_trampoline<
            B: AsRef<[u8]> + Send + 'static,
            Q: FnOnce(Result<(B, usize), (B, glib::Error)>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut user_data: Box<Option<(Q, B)>> = Box::from_raw(user_data as *mut _);
            let (callback, buffer) = user_data.take().unwrap();

            let mut error = ptr::null_mut();
            let ret = ffi::g_output_stream_write_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok((buffer, ret as usize))
            } else {
                Err((buffer, from_glib_full(error)))
            };
            callback(result);
        }
        let callback = write_async_trampoline::<B, Q>;
        unsafe {
            ffi::g_output_stream_write_async(
                self.as_ref().to_glib_none().0,
                mut_override(buffer_ptr),
                count,
                io_priority.to_glib(),
                gcancellable.0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    fn write_all<C: IsA<Cancellable>>(
        &self,
        buffer: &[u8],
        cancellable: Option<&C>,
    ) -> Result<(usize, Option<glib::Error>), glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let count = buffer.len() as usize;
        unsafe {
            let mut bytes_written = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = ffi::g_output_stream_write_all(
                self.as_ref().to_glib_none().0,
                buffer.to_glib_none().0,
                count,
                bytes_written.as_mut_ptr(),
                gcancellable.0,
                &mut error,
            );

            let bytes_written = bytes_written.assume_init();
            if error.is_null() {
                Ok((bytes_written, None))
            } else if bytes_written != 0 {
                Ok((bytes_written, Some(from_glib_full(error))))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    fn write_all_async<
        B: AsRef<[u8]> + Send + 'static,
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
        let user_data: Box<Option<(Q, B)>> = Box::new(Some((callback, buffer)));
        // Need to do this after boxing as the contents pointer might change by moving into the box
        let (count, buffer_ptr) = {
            let buffer = &(*user_data).as_ref().unwrap().1;
            let slice = buffer.as_ref();
            (slice.len(), slice.as_ptr())
        };
        unsafe extern "C" fn write_all_async_trampoline<
            B: AsRef<[u8]> + Send + 'static,
            Q: FnOnce(Result<(B, usize, Option<glib::Error>), (B, glib::Error)>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut user_data: Box<Option<(Q, B)>> = Box::from_raw(user_data as *mut _);
            let (callback, buffer) = user_data.take().unwrap();

            let mut error = ptr::null_mut();
            let mut bytes_written = mem::MaybeUninit::uninit();
            let _ = ffi::g_output_stream_write_all_finish(
                _source_object as *mut _,
                res,
                bytes_written.as_mut_ptr(),
                &mut error,
            );
            let bytes_written = bytes_written.assume_init();
            let result = if error.is_null() {
                Ok((buffer, bytes_written, None))
            } else if bytes_written != 0 {
                Ok((buffer, bytes_written, from_glib_full(error)))
            } else {
                Err((buffer, from_glib_full(error)))
            };
            callback(result);
        }
        let callback = write_all_async_trampoline::<B, Q>;
        unsafe {
            ffi::g_output_stream_write_all_async(
                self.as_ref().to_glib_none().0,
                mut_override(buffer_ptr),
                count,
                io_priority.to_glib(),
                gcancellable.0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    fn write_async_future<'a, B: AsRef<[u8]> + Send + 'static>(
        &self,
        buffer: B,
        io_priority: Priority,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(B, usize), (B, glib::Error)>> + 'static>>
    {
        Box::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.write_async(buffer, io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    fn write_all_async_future<'a, B: AsRef<[u8]> + Send + 'static>(
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
            obj.write_all_async(buffer, io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }
}

#[derive(Debug)]
pub struct OutputStreamWrite<T: IsA<OutputStream>>(T);

impl<T: IsA<OutputStream>> OutputStreamWrite<T> {
    pub fn into_output_stream(self) -> T {
        self.0
    }

    pub fn output_stream(&self) -> &T {
        &self.0
    }
}

impl<T: IsA<OutputStream>> io::Write for OutputStreamWrite<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let result = self
            .0
            .as_ref()
            .write(buf, crate::NONE_CANCELLABLE)
            .map(|size| size as usize);
        to_std_io_result(result)
    }

    fn flush(&mut self) -> io::Result<()> {
        let gio_result = self.0.as_ref().flush(crate::NONE_CANCELLABLE);
        to_std_io_result(gio_result)
    }
}

impl<T: IsA<OutputStream> + IsA<Seekable>> io::Seek for OutputStreamWrite<T> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        let (pos, type_) = match pos {
            io::SeekFrom::Start(pos) => (pos as i64, glib::SeekType::Set),
            io::SeekFrom::End(pos) => (pos, glib::SeekType::End),
            io::SeekFrom::Current(pos) => (pos, glib::SeekType::Cur),
        };
        let seekable: &Seekable = self.0.as_ref();
        let gio_result = seekable
            .seek(pos, type_, crate::NONE_CANCELLABLE)
            .map(|_| seekable.tell() as u64);
        to_std_io_result(gio_result)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test_util::run_async;
    use crate::MemoryInputStream;
    use crate::MemoryOutputStream;
    use glib::Bytes;
    use std::io::Write;

    #[test]
    fn splice_async() {
        let ret = run_async(|tx, l| {
            let input = MemoryInputStream::new();
            let b = Bytes::from_owned(vec![1, 2, 3]);
            input.add_bytes(&b);

            let strm = MemoryOutputStream::new_resizable();
            strm.splice_async(
                &input,
                crate::OutputStreamSpliceFlags::CLOSE_SOURCE,
                glib::PRIORITY_DEFAULT_IDLE,
                crate::NONE_CANCELLABLE,
                move |ret| {
                    tx.send(ret).unwrap();
                    l.quit();
                },
            );
        });

        assert_eq!(ret.unwrap(), 3);
    }

    #[test]
    fn write_async() {
        let ret = run_async(|tx, l| {
            let strm = MemoryOutputStream::new_resizable();

            let buf = vec![1, 2, 3];
            strm.write_async(
                buf,
                glib::PRIORITY_DEFAULT_IDLE,
                crate::NONE_CANCELLABLE,
                move |ret| {
                    tx.send(ret).unwrap();
                    l.quit();
                },
            );
        });

        let (buf, size) = ret.unwrap();
        assert_eq!(buf, vec![1, 2, 3]);
        assert_eq!(size, 3);
    }

    #[test]
    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn write_all_async() {
        let ret = run_async(|tx, l| {
            let strm = MemoryOutputStream::new_resizable();

            let buf = vec![1, 2, 3];
            strm.write_all_async(
                buf,
                glib::PRIORITY_DEFAULT_IDLE,
                crate::NONE_CANCELLABLE,
                move |ret| {
                    tx.send(ret).unwrap();
                    l.quit();
                },
            );
        });

        let (buf, size, err) = ret.unwrap();
        assert_eq!(buf, vec![1, 2, 3]);
        assert_eq!(size, 3);
        assert!(err.is_none());
    }

    #[test]
    fn write_bytes_async() {
        let ret = run_async(|tx, l| {
            let strm = MemoryOutputStream::new_resizable();

            let b = Bytes::from_owned(vec![1, 2, 3]);
            strm.write_bytes_async(
                &b,
                glib::PRIORITY_DEFAULT_IDLE,
                crate::NONE_CANCELLABLE,
                move |ret| {
                    tx.send(ret).unwrap();
                    l.quit();
                },
            );
        });

        assert_eq!(ret.unwrap(), 3);
    }

    #[test]
    fn std_io_write() {
        let b = Bytes::from_owned(vec![1, 2, 3]);
        let mut write = MemoryOutputStream::new_resizable().into_write();

        let ret = write.write(&b);

        let stream = write.into_output_stream();
        stream.close(crate::NONE_CANCELLABLE).unwrap();
        assert_eq!(ret.unwrap(), 3);
        assert_eq!(stream.steal_as_bytes(), [1, 2, 3].as_ref());
    }

    #[test]
    fn into_output_stream() {
        let stream = MemoryOutputStream::new_resizable();
        let stream_clone = stream.clone();
        let stream = stream.into_write().into_output_stream();

        assert_eq!(stream, stream_clone);
    }
}

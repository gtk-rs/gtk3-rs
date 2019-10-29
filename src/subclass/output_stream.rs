// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gio_sys;
use glib_sys;

use glib::subclass::prelude::*;
use glib::translate::*;

use glib::Error;

use Cancellable;
use InputStream;
use OutputStream;
use OutputStreamClass;
use OutputStreamSpliceFlags;

use std::mem;
use std::ptr;

pub trait OutputStreamImpl: OutputStreamImplExt + Send + 'static {
    fn write(
        &self,
        stream: &OutputStream,
        buffer: &[u8],
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error> {
        self.parent_write(stream, buffer, cancellable)
    }

    fn close(&self, stream: &OutputStream, cancellable: Option<&Cancellable>) -> Result<(), Error> {
        self.parent_close(stream, cancellable)
    }

    fn flush(&self, stream: &OutputStream, cancellable: Option<&Cancellable>) -> Result<(), Error> {
        self.parent_flush(stream, cancellable)
    }

    fn splice(
        &self,
        stream: &OutputStream,
        input_stream: &InputStream,
        flags: OutputStreamSpliceFlags,
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error> {
        self.parent_splice(stream, input_stream, flags, cancellable)
    }
}

pub trait OutputStreamImplExt {
    fn parent_write(
        &self,
        stream: &OutputStream,
        buffer: &[u8],
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error>;

    fn parent_close(
        &self,
        stream: &OutputStream,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error>;

    fn parent_flush(
        &self,
        stream: &OutputStream,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error>;

    fn parent_splice(
        &self,
        stream: &OutputStream,
        input_stream: &InputStream,
        flags: OutputStreamSpliceFlags,
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error>;
}

impl<T: OutputStreamImpl + ObjectImpl> OutputStreamImplExt for T {
    fn parent_write(
        &self,
        stream: &OutputStream,
        buffer: &[u8],
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gio_sys::GOutputStreamClass;
            let f = (*parent_class)
                .write_fn
                .expect("No parent class implementation for \"write\"");
            let mut err = ptr::null_mut();
            let res = f(
                stream.to_glib_none().0,
                mut_override(buffer.as_ptr()),
                buffer.len(),
                cancellable.to_glib_none().0,
                &mut err,
            );
            if res == -1 {
                Err(from_glib_full(err))
            } else {
                assert!(res >= 0);
                let res = res as usize;
                assert!(res <= buffer.len());
                Ok(res)
            }
        }
    }

    fn parent_close(
        &self,
        stream: &OutputStream,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gio_sys::GOutputStreamClass;
            let mut err = ptr::null_mut();
            if let Some(f) = (*parent_class).close_fn {
                if from_glib(f(
                    stream.to_glib_none().0,
                    cancellable.to_glib_none().0,
                    &mut err,
                )) {
                    Ok(())
                } else {
                    Err(from_glib_full(err))
                }
            } else {
                Ok(())
            }
        }
    }

    fn parent_flush(
        &self,
        stream: &OutputStream,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gio_sys::GOutputStreamClass;
            let mut err = ptr::null_mut();
            if let Some(f) = (*parent_class).flush {
                if from_glib(f(
                    stream.to_glib_none().0,
                    cancellable.to_glib_none().0,
                    &mut err,
                )) {
                    Ok(())
                } else {
                    Err(from_glib_full(err))
                }
            } else {
                Ok(())
            }
        }
    }

    fn parent_splice(
        &self,
        stream: &OutputStream,
        input_stream: &InputStream,
        flags: OutputStreamSpliceFlags,
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gio_sys::GOutputStreamClass;
            let mut err = ptr::null_mut();
            let f = (*parent_class)
                .splice
                .expect("No parent class implementation for \"splice\"");
            let res = f(
                stream.to_glib_none().0,
                input_stream.to_glib_none().0,
                flags.to_glib(),
                cancellable.to_glib_none().0,
                &mut err,
            );
            if res == -1 {
                Err(from_glib_full(err))
            } else {
                assert!(res >= 0);
                let res = res as usize;
                Ok(res)
            }
        }
    }
}

unsafe impl<T: ObjectSubclass + OutputStreamImpl> IsSubclassable<T> for OutputStreamClass {
    fn override_vfuncs(&mut self) {
        <glib::ObjectClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gio_sys::GOutputStreamClass);
            klass.write_fn = Some(stream_write::<T>);
            klass.close_fn = Some(stream_close::<T>);
            klass.flush = Some(stream_flush::<T>);
            klass.splice = Some(stream_splice::<T>);
        }
    }
}

unsafe extern "C" fn stream_write<T: ObjectSubclass>(
    ptr: *mut gio_sys::GOutputStream,
    buffer: *mut u8,
    count: usize,
    cancellable: *mut gio_sys::GCancellable,
    err: *mut *mut glib_sys::GError,
) -> isize
where
    T: OutputStreamImpl,
{
    use std::isize;
    use std::slice;

    assert!(count <= isize::MAX as usize);

    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: OutputStream = from_glib_borrow(ptr);

    match imp.write(
        &wrap,
        slice::from_raw_parts(buffer as *const u8, count),
        Option::<Cancellable>::from_glib_borrow(cancellable).as_ref(),
    ) {
        Ok(res) => {
            assert!(res <= isize::MAX as usize);
            assert!(res <= count);
            res as isize
        }
        Err(mut e) => {
            *err = e.to_glib_none_mut().0;
            mem::forget(e);
            -1
        }
    }
}

unsafe extern "C" fn stream_close<T: ObjectSubclass>(
    ptr: *mut gio_sys::GOutputStream,
    cancellable: *mut gio_sys::GCancellable,
    err: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean
where
    T: OutputStreamImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: OutputStream = from_glib_borrow(ptr);

    match imp.close(
        &wrap,
        Option::<Cancellable>::from_glib_borrow(cancellable).as_ref(),
    ) {
        Ok(_) => glib_sys::GTRUE,
        Err(mut e) => {
            *err = e.to_glib_none_mut().0;
            mem::forget(e);
            glib_sys::GFALSE
        }
    }
}

unsafe extern "C" fn stream_flush<T: ObjectSubclass>(
    ptr: *mut gio_sys::GOutputStream,
    cancellable: *mut gio_sys::GCancellable,
    err: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean
where
    T: OutputStreamImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: OutputStream = from_glib_borrow(ptr);

    match imp.flush(
        &wrap,
        Option::<Cancellable>::from_glib_borrow(cancellable).as_ref(),
    ) {
        Ok(_) => glib_sys::GTRUE,
        Err(mut e) => {
            *err = e.to_glib_none_mut().0;
            mem::forget(e);
            glib_sys::GFALSE
        }
    }
}

unsafe extern "C" fn stream_splice<T: ObjectSubclass>(
    ptr: *mut gio_sys::GOutputStream,
    input_stream: *mut gio_sys::GInputStream,
    flags: gio_sys::GOutputStreamSpliceFlags,
    cancellable: *mut gio_sys::GCancellable,
    err: *mut *mut glib_sys::GError,
) -> isize
where
    T: OutputStreamImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: OutputStream = from_glib_borrow(ptr);

    match imp.splice(
        &wrap,
        &from_glib_borrow(input_stream),
        from_glib(flags),
        Option::<Cancellable>::from_glib_borrow(cancellable).as_ref(),
    ) {
        Ok(res) => {
            use std::isize;
            assert!(res <= isize::MAX as usize);
            res as isize
        }
        Err(mut e) => {
            *err = e.to_glib_none_mut().0;
            mem::forget(e);
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use glib;
    use glib::subclass;
    use std::cell::RefCell;

    struct SimpleOutputStream {
        sum: RefCell<usize>,
    }

    impl ObjectSubclass for SimpleOutputStream {
        const NAME: &'static str = "SimpleOutputStream";
        type ParentType = OutputStream;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;

        glib_object_subclass!();

        fn new() -> Self {
            Self {
                sum: RefCell::new(0),
            }
        }
    }

    impl ObjectImpl for SimpleOutputStream {
        glib_object_impl!();
    }

    impl OutputStreamImpl for SimpleOutputStream {
        fn write(
            &self,
            _stream: &OutputStream,
            buffer: &[u8],
            _cancellable: Option<&Cancellable>,
        ) -> Result<usize, Error> {
            let mut sum = self.sum.borrow_mut();
            for b in buffer {
                *sum += *b as usize;
            }

            Ok(buffer.len())
        }
    }

    #[test]
    fn test_simple_stream() {
        let stream = glib::Object::new(SimpleOutputStream::get_type(), &[])
            .unwrap()
            .downcast::<::OutputStream>()
            .unwrap();

        assert_eq!(*SimpleOutputStream::from_instance(&stream).sum.borrow(), 0);
        assert_eq!(
            stream.write(&[1, 2, 3, 4, 5], crate::NONE_CANCELLABLE),
            Ok(5)
        );
        assert_eq!(*SimpleOutputStream::from_instance(&stream).sum.borrow(), 15);
    }
}

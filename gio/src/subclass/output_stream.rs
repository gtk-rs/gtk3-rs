// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;
use glib::translate::*;

use glib::{Cast, Error};

use crate::Cancellable;
use crate::InputStream;
use crate::OutputStream;
use crate::OutputStreamSpliceFlags;

use std::mem;
use std::ptr;

pub trait OutputStreamImpl: ObjectImpl + OutputStreamImplExt + Send {
    fn write(
        &self,
        stream: &Self::Type,
        buffer: &[u8],
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error> {
        self.parent_write(stream, buffer, cancellable)
    }

    fn close(&self, stream: &Self::Type, cancellable: Option<&Cancellable>) -> Result<(), Error> {
        self.parent_close(stream, cancellable)
    }

    fn flush(&self, stream: &Self::Type, cancellable: Option<&Cancellable>) -> Result<(), Error> {
        self.parent_flush(stream, cancellable)
    }

    fn splice(
        &self,
        stream: &Self::Type,
        input_stream: &InputStream,
        flags: OutputStreamSpliceFlags,
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error> {
        self.parent_splice(stream, input_stream, flags, cancellable)
    }
}

pub trait OutputStreamImplExt: ObjectSubclass {
    fn parent_write(
        &self,
        stream: &Self::Type,
        buffer: &[u8],
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error>;

    fn parent_close(
        &self,
        stream: &Self::Type,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error>;

    fn parent_flush(
        &self,
        stream: &Self::Type,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error>;

    fn parent_splice(
        &self,
        stream: &Self::Type,
        input_stream: &InputStream,
        flags: OutputStreamSpliceFlags,
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error>;
}

impl<T: OutputStreamImpl> OutputStreamImplExt for T {
    fn parent_write(
        &self,
        stream: &Self::Type,
        buffer: &[u8],
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GOutputStreamClass;
            let f = (*parent_class)
                .write_fn
                .expect("No parent class implementation for \"write\"");
            let mut err = ptr::null_mut();
            let res = f(
                stream.unsafe_cast_ref::<OutputStream>().to_glib_none().0,
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
        stream: &Self::Type,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GOutputStreamClass;
            let mut err = ptr::null_mut();
            if let Some(f) = (*parent_class).close_fn {
                if from_glib(f(
                    stream.unsafe_cast_ref::<OutputStream>().to_glib_none().0,
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
        stream: &Self::Type,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GOutputStreamClass;
            let mut err = ptr::null_mut();
            if let Some(f) = (*parent_class).flush {
                if from_glib(f(
                    stream.unsafe_cast_ref::<OutputStream>().to_glib_none().0,
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
        stream: &Self::Type,
        input_stream: &InputStream,
        flags: OutputStreamSpliceFlags,
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GOutputStreamClass;
            let mut err = ptr::null_mut();
            let f = (*parent_class)
                .splice
                .expect("No parent class implementation for \"splice\"");
            let res = f(
                stream.unsafe_cast_ref::<OutputStream>().to_glib_none().0,
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

unsafe impl<T: OutputStreamImpl> IsSubclassable<T> for OutputStream {
    fn class_init(class: &mut ::glib::Class<Self>) {
        <glib::Object as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.write_fn = Some(stream_write::<T>);
        klass.close_fn = Some(stream_close::<T>);
        klass.flush = Some(stream_flush::<T>);
        klass.splice = Some(stream_splice::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <glib::Object as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn stream_write<T: OutputStreamImpl>(
    ptr: *mut ffi::GOutputStream,
    buffer: *mut u8,
    count: usize,
    cancellable: *mut ffi::GCancellable,
    err: *mut *mut glib::ffi::GError,
) -> isize {
    use std::isize;
    use std::slice;

    assert!(count <= isize::MAX as usize);

    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<OutputStream> = from_glib_borrow(ptr);

    match imp.write(
        &wrap.unsafe_cast_ref(),
        slice::from_raw_parts(buffer as *const u8, count),
        Option::<Cancellable>::from_glib_borrow(cancellable)
            .as_ref()
            .as_ref(),
    ) {
        Ok(res) => {
            assert!(res <= isize::MAX as usize);
            assert!(res <= count);
            res as isize
        }
        Err(e) => {
            let mut e = mem::ManuallyDrop::new(e);
            *err = e.to_glib_none_mut().0;
            -1
        }
    }
}

unsafe extern "C" fn stream_close<T: OutputStreamImpl>(
    ptr: *mut ffi::GOutputStream,
    cancellable: *mut ffi::GCancellable,
    err: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<OutputStream> = from_glib_borrow(ptr);

    match imp.close(
        &wrap.unsafe_cast_ref(),
        Option::<Cancellable>::from_glib_borrow(cancellable)
            .as_ref()
            .as_ref(),
    ) {
        Ok(_) => glib::ffi::GTRUE,
        Err(e) => {
            let mut e = mem::ManuallyDrop::new(e);
            *err = e.to_glib_none_mut().0;
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn stream_flush<T: OutputStreamImpl>(
    ptr: *mut ffi::GOutputStream,
    cancellable: *mut ffi::GCancellable,
    err: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<OutputStream> = from_glib_borrow(ptr);

    match imp.flush(
        &wrap.unsafe_cast_ref(),
        Option::<Cancellable>::from_glib_borrow(cancellable)
            .as_ref()
            .as_ref(),
    ) {
        Ok(_) => glib::ffi::GTRUE,
        Err(e) => {
            let mut e = mem::ManuallyDrop::new(e);
            *err = e.to_glib_none_mut().0;
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn stream_splice<T: OutputStreamImpl>(
    ptr: *mut ffi::GOutputStream,
    input_stream: *mut ffi::GInputStream,
    flags: ffi::GOutputStreamSpliceFlags,
    cancellable: *mut ffi::GCancellable,
    err: *mut *mut glib::ffi::GError,
) -> isize {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<OutputStream> = from_glib_borrow(ptr);

    match imp.splice(
        &wrap.unsafe_cast_ref(),
        &from_glib_borrow(input_stream),
        from_glib(flags),
        Option::<Cancellable>::from_glib_borrow(cancellable)
            .as_ref()
            .as_ref(),
    ) {
        Ok(res) => {
            use std::isize;
            assert!(res <= isize::MAX as usize);
            res as isize
        }
        Err(e) => {
            let mut e = mem::ManuallyDrop::new(e);
            *err = e.to_glib_none_mut().0;
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use std::cell::RefCell;

    mod imp {
        use super::*;

        #[derive(Default)]
        pub struct SimpleOutputStream {
            pub sum: RefCell<usize>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for SimpleOutputStream {
            const NAME: &'static str = "SimpleOutputStream";
            type Type = super::SimpleOutputStream;
            type ParentType = OutputStream;
        }

        impl ObjectImpl for SimpleOutputStream {}

        impl OutputStreamImpl for SimpleOutputStream {
            fn write(
                &self,
                _stream: &Self::Type,
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
    }

    glib::wrapper! {
        pub struct SimpleOutputStream(ObjectSubclass<imp::SimpleOutputStream>)
            @extends OutputStream;
    }

    #[test]
    fn test_simple_stream() {
        let stream = glib::Object::new::<SimpleOutputStream>(&[]).unwrap();

        assert_eq!(
            *imp::SimpleOutputStream::from_instance(&stream).sum.borrow(),
            0
        );
        assert_eq!(
            stream.write(&[1, 2, 3, 4, 5], crate::NONE_CANCELLABLE),
            Ok(5)
        );
        assert_eq!(
            *imp::SimpleOutputStream::from_instance(&stream).sum.borrow(),
            15
        );
    }
}

// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;
use glib::translate::*;

use glib::{Cast, Error};

use crate::Cancellable;
use crate::InputStream;

use std::mem;
use std::ptr;

pub trait InputStreamImpl: ObjectImpl + InputStreamImplExt + Send {
    fn read(
        &self,
        stream: &Self::Type,
        buffer: &mut [u8],
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error> {
        self.parent_read(stream, buffer, cancellable)
    }

    fn close(&self, stream: &Self::Type, cancellable: Option<&Cancellable>) -> Result<(), Error> {
        self.parent_close(stream, cancellable)
    }

    fn skip(
        &self,
        stream: &Self::Type,
        count: usize,
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error> {
        self.parent_skip(stream, count, cancellable)
    }
}

pub trait InputStreamImplExt: ObjectSubclass {
    fn parent_read(
        &self,
        stream: &Self::Type,
        buffer: &mut [u8],
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error>;

    fn parent_close(
        &self,
        stream: &Self::Type,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error>;

    fn parent_skip(
        &self,
        stream: &Self::Type,
        count: usize,
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error>;
}

impl<T: InputStreamImpl> InputStreamImplExt for T {
    fn parent_read(
        &self,
        stream: &Self::Type,
        buffer: &mut [u8],
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GInputStreamClass;
            let f = (*parent_class)
                .read_fn
                .expect("No parent class implementation for \"read\"");
            let mut err = ptr::null_mut();
            let res = f(
                stream.unsafe_cast_ref::<InputStream>().to_glib_none().0,
                buffer.as_mut_ptr() as glib::ffi::gpointer,
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
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GInputStreamClass;
            let mut err = ptr::null_mut();
            if let Some(f) = (*parent_class).close_fn {
                if from_glib(f(
                    stream.unsafe_cast_ref::<InputStream>().to_glib_none().0,
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

    fn parent_skip(
        &self,
        stream: &Self::Type,
        count: usize,
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GInputStreamClass;
            let mut err = ptr::null_mut();
            let f = (*parent_class)
                .skip
                .expect("No parent class implementation for \"skip\"");
            let res = f(
                stream.unsafe_cast_ref::<InputStream>().to_glib_none().0,
                count,
                cancellable.to_glib_none().0,
                &mut err,
            );
            if res == -1 {
                Err(from_glib_full(err))
            } else {
                assert!(res >= 0);
                let res = res as usize;
                assert!(res <= count);
                Ok(res)
            }
        }
    }
}

unsafe impl<T: InputStreamImpl> IsSubclassable<T> for InputStream {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <glib::Object as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.read_fn = Some(stream_read::<T>);
        klass.close_fn = Some(stream_close::<T>);
        klass.skip = Some(stream_skip::<T>);
    }
}

unsafe extern "C" fn stream_read<T: InputStreamImpl>(
    ptr: *mut ffi::GInputStream,
    buffer: glib::ffi::gpointer,
    count: usize,
    cancellable: *mut ffi::GCancellable,
    err: *mut *mut glib::ffi::GError,
) -> isize {
    use std::isize;
    use std::slice;

    assert!(count <= isize::MAX as usize);

    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<InputStream> = from_glib_borrow(ptr);

    match imp.read(
        wrap.unsafe_cast_ref(),
        slice::from_raw_parts_mut(buffer as *mut u8, count),
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

unsafe extern "C" fn stream_close<T: InputStreamImpl>(
    ptr: *mut ffi::GInputStream,
    cancellable: *mut ffi::GCancellable,
    err: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<InputStream> = from_glib_borrow(ptr);

    match imp.close(
        wrap.unsafe_cast_ref(),
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

unsafe extern "C" fn stream_skip<T: InputStreamImpl>(
    ptr: *mut ffi::GInputStream,
    count: usize,
    cancellable: *mut ffi::GCancellable,
    err: *mut *mut glib::ffi::GError,
) -> isize {
    use std::isize;

    assert!(count <= isize::MAX as usize);

    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<InputStream> = from_glib_borrow(ptr);

    match imp.skip(
        wrap.unsafe_cast_ref(),
        count,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use crate::subclass::prelude::*;
    use glib::subclass;
    use std::cell::RefCell;

    mod imp {
        use super::*;

        pub struct SimpleInputStream {
            pub pos: RefCell<usize>,
        }

        impl ObjectSubclass for SimpleInputStream {
            const NAME: &'static str = "SimpleInputStream";
            type Type = super::SimpleInputStream;
            type ParentType = InputStream;
            type Interfaces = (crate::Seekable,);
            type Instance = subclass::simple::InstanceStruct<Self>;
            type Class = subclass::simple::ClassStruct<Self>;

            glib::object_subclass!();

            fn new() -> Self {
                Self {
                    pos: RefCell::new(0),
                }
            }
        }

        impl ObjectImpl for SimpleInputStream {}

        impl InputStreamImpl for SimpleInputStream {
            fn read(
                &self,
                _stream: &Self::Type,
                buffer: &mut [u8],
                _cancellable: Option<&Cancellable>,
            ) -> Result<usize, Error> {
                let mut pos = self.pos.borrow_mut();
                for b in buffer.iter_mut() {
                    *b = ((*pos) % 255) as u8;
                    *pos += 1;
                }
                Ok(buffer.len())
            }
        }

        impl SeekableImpl for SimpleInputStream {
            fn tell(&self, _seekable: &Self::Type) -> i64 {
                *self.pos.borrow() as i64
            }

            fn can_seek(&self, _seekable: &Self::Type) -> bool {
                true
            }

            fn seek(
                &self,
                _seekable: &Self::Type,
                offset: i64,
                type_: glib::SeekType,
                _cancellable: Option<&Cancellable>,
            ) -> Result<(), glib::Error> {
                let mut pos = self.pos.borrow_mut();
                match type_ {
                    glib::SeekType::Set => {
                        *pos = offset as usize;
                        Ok(())
                    }
                    glib::SeekType::Cur => {
                        if offset < 0 {
                            *pos -= (-offset) as usize;
                        } else {
                            *pos += offset as usize;
                        }

                        Ok(())
                    }
                    glib::SeekType::End => Err(glib::Error::new(
                        crate::IOErrorEnum::NotSupported,
                        "Can't seek relative to end",
                    )),
                    _ => unreachable!(),
                }
            }

            fn can_truncate(&self, _seekable: &Self::Type) -> bool {
                false
            }
            fn truncate(
                &self,
                _seekable: &Self::Type,
                _offset: i64,
                _cancellable: Option<&Cancellable>,
            ) -> Result<(), Error> {
                unimplemented!()
            }
        }
    }

    glib::wrapper! {
        pub struct SimpleInputStream(ObjectSubclass<imp::SimpleInputStream>)
            @extends InputStream;
    }

    #[test]
    fn test_simple_stream() {
        let stream = glib::Object::new::<SimpleInputStream>(&[]).unwrap();

        let mut buf = [0; 16];
        assert_eq!(stream.read(&mut buf, crate::NONE_CANCELLABLE), Ok(16));
        assert_eq!(
            &buf,
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );

        assert_eq!(stream.skip(2, crate::NONE_CANCELLABLE), Ok(2));

        assert_eq!(stream.read(&mut buf, crate::NONE_CANCELLABLE), Ok(16));
        assert_eq!(
            &buf,
            &[18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33]
        );

        let seekable = stream.dynamic_cast_ref::<crate::Seekable>().unwrap();
        assert_eq!(seekable.tell(), 34);
        assert!(seekable.can_seek());

        assert_eq!(
            seekable.seek(0, glib::SeekType::Set, crate::NONE_CANCELLABLE),
            Ok(())
        );

        assert_eq!(seekable.tell(), 0);
        assert_eq!(stream.read(&mut buf, crate::NONE_CANCELLABLE), Ok(16));
        assert_eq!(
            &buf,
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );

        assert_eq!(stream.close(crate::NONE_CANCELLABLE), Ok(()));
    }
}

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
use InputStreamClass;

use std::mem;
use std::ptr;

pub trait InputStreamImpl: InputStreamImplExt + Send + 'static {
    fn read(
        &self,
        stream: &InputStream,
        buffer: &mut [u8],
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error> {
        self.parent_read(stream, buffer, cancellable)
    }

    fn close(&self, stream: &InputStream, cancellable: Option<&Cancellable>) -> Result<(), Error> {
        self.parent_close(stream, cancellable)
    }

    fn skip(
        &self,
        stream: &InputStream,
        count: usize,
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error> {
        self.parent_skip(stream, count, cancellable)
    }
}

pub trait InputStreamImplExt {
    fn parent_read(
        &self,
        stream: &InputStream,
        buffer: &mut [u8],
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error>;

    fn parent_close(
        &self,
        stream: &InputStream,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error>;

    fn parent_skip(
        &self,
        stream: &InputStream,
        count: usize,
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error>;
}

impl<T: InputStreamImpl + ObjectImpl> InputStreamImplExt for T {
    fn parent_read(
        &self,
        stream: &InputStream,
        buffer: &mut [u8],
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gio_sys::GInputStreamClass;
            let f = (*parent_class)
                .read_fn
                .expect("No parent class implementation for \"read\"");
            let mut err = ptr::null_mut();
            let res = f(
                stream.to_glib_none().0,
                buffer.as_mut_ptr() as glib_sys::gpointer,
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
        stream: &InputStream,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gio_sys::GInputStreamClass;
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

    fn parent_skip(
        &self,
        stream: &InputStream,
        count: usize,
        cancellable: Option<&Cancellable>,
    ) -> Result<usize, Error> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gio_sys::GInputStreamClass;
            let mut err = ptr::null_mut();
            let f = (*parent_class)
                .skip
                .expect("No parent class implementation for \"skip\"");
            let res = f(
                stream.to_glib_none().0,
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

unsafe impl<T: ObjectSubclass + InputStreamImpl> IsSubclassable<T> for InputStreamClass {
    fn override_vfuncs(&mut self) {
        <glib::ObjectClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gio_sys::GInputStreamClass);
            klass.read_fn = Some(stream_read::<T>);
            klass.close_fn = Some(stream_close::<T>);
            klass.skip = Some(stream_skip::<T>);
        }
    }
}

unsafe extern "C" fn stream_read<T: ObjectSubclass>(
    ptr: *mut gio_sys::GInputStream,
    buffer: glib_sys::gpointer,
    count: usize,
    cancellable: *mut gio_sys::GCancellable,
    err: *mut *mut glib_sys::GError,
) -> isize
where
    T: InputStreamImpl,
{
    use std::isize;
    use std::slice;

    assert!(count <= isize::MAX as usize);

    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: InputStream = from_glib_borrow(ptr);

    match imp.read(
        &wrap,
        slice::from_raw_parts_mut(buffer as *mut u8, count),
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
    ptr: *mut gio_sys::GInputStream,
    cancellable: *mut gio_sys::GCancellable,
    err: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean
where
    T: InputStreamImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: InputStream = from_glib_borrow(ptr);

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

unsafe extern "C" fn stream_skip<T: ObjectSubclass>(
    ptr: *mut gio_sys::GInputStream,
    count: usize,
    cancellable: *mut gio_sys::GCancellable,
    err: *mut *mut glib_sys::GError,
) -> isize
where
    T: InputStreamImpl,
{
    use std::isize;

    assert!(count <= isize::MAX as usize);

    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: InputStream = from_glib_borrow(ptr);

    match imp.skip(
        &wrap,
        count,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use crate::subclass::prelude::*;
    use crate::Seekable;
    use glib;
    use glib::subclass;
    use std::cell::RefCell;

    struct SimpleInputStream {
        pos: RefCell<usize>,
    }

    impl ObjectSubclass for SimpleInputStream {
        const NAME: &'static str = "SimpleInputStream";
        type ParentType = InputStream;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;

        glib_object_subclass!();

        fn new() -> Self {
            Self {
                pos: RefCell::new(0),
            }
        }

        fn type_init(type_: &mut subclass::InitializingType<Self>) {
            type_.add_interface::<crate::Seekable>();
        }
    }

    impl ObjectImpl for SimpleInputStream {
        glib_object_impl!();
    }

    impl InputStreamImpl for SimpleInputStream {
        fn read(
            &self,
            _stream: &InputStream,
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
        fn tell(&self, _seekable: &Seekable) -> i64 {
            *self.pos.borrow() as i64
        }

        fn can_seek(&self, _seekable: &Seekable) -> bool {
            true
        }

        fn seek(
            &self,
            _seekable: &Seekable,
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

        fn can_truncate(&self, _seekable: &Seekable) -> bool {
            false
        }
        fn truncate(
            &self,
            _seekable: &Seekable,
            _offset: i64,
            _cancellable: Option<&Cancellable>,
        ) -> Result<(), Error> {
            unimplemented!()
        }
    }

    #[test]
    fn test_simple_stream() {
        let stream = glib::Object::new(SimpleInputStream::get_type(), &[])
            .unwrap()
            .downcast::<::InputStream>()
            .unwrap();

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

        let seekable = stream.dynamic_cast_ref::<Seekable>().unwrap();
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

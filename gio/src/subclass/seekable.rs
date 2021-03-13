// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use glib::Cast;
use glib::Error;
use glib::SeekType;

use glib::subclass::prelude::*;

use std::mem;
use std::ptr;

use crate::Cancellable;
use crate::Seekable;

pub trait SeekableImpl: ObjectImpl + Send {
    fn tell(&self, seekable: &Self::Type) -> i64;
    fn can_seek(&self, seekable: &Self::Type) -> bool;
    fn seek(
        &self,
        seekable: &Self::Type,
        offset: i64,
        type_: SeekType,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error>;
    fn can_truncate(&self, seekable: &Self::Type) -> bool;
    fn truncate(
        &self,
        seekable: &Self::Type,
        offset: i64,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error>;
}

pub trait SeekableImplExt: ObjectSubclass {
    fn parent_tell(&self, seekable: &Self::Type) -> i64;
    fn parent_can_seek(&self, seekable: &Self::Type) -> bool;
    fn parent_seek(
        &self,
        seekable: &Self::Type,
        offset: i64,
        type_: SeekType,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error>;
    fn parent_can_truncate(&self, seekable: &Self::Type) -> bool;
    fn parent_truncate(
        &self,
        seekable: &Self::Type,
        offset: i64,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error>;
}

impl<T: SeekableImpl> SeekableImplExt for T {
    fn parent_tell(&self, seekable: &Self::Type) -> i64 {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().get_parent_interface::<Seekable>() as *const ffi::GSeekableIface;

            let func = (*parent_iface)
                .tell
                .expect("no parent \"tell\" implementation");
            func(seekable.unsafe_cast_ref::<Seekable>().to_glib_none().0)
        }
    }

    fn parent_can_seek(&self, seekable: &Self::Type) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().get_parent_interface::<Seekable>() as *const ffi::GSeekableIface;

            let func = (*parent_iface)
                .can_seek
                .expect("no parent \"can_seek\" implementation");
            let ret = func(seekable.unsafe_cast_ref::<Seekable>().to_glib_none().0);
            from_glib(ret)
        }
    }

    fn parent_seek(
        &self,
        seekable: &Self::Type,
        offset: i64,
        type_: SeekType,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().get_parent_interface::<Seekable>() as *const ffi::GSeekableIface;

            let func = (*parent_iface)
                .seek
                .expect("no parent \"seek\" implementation");

            let mut err = ptr::null_mut();
            func(
                seekable.unsafe_cast_ref::<Seekable>().to_glib_none().0,
                offset,
                type_.to_glib(),
                cancellable.to_glib_none().0,
                &mut err,
            );

            if err.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(err))
            }
        }
    }

    fn parent_can_truncate(&self, seekable: &Self::Type) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().get_parent_interface::<Seekable>() as *const ffi::GSeekableIface;

            let func = (*parent_iface)
                .can_truncate
                .expect("no parent \"can_truncate\" implementation");
            let ret = func(seekable.unsafe_cast_ref::<Seekable>().to_glib_none().0);
            from_glib(ret)
        }
    }

    fn parent_truncate(
        &self,
        seekable: &Self::Type,
        offset: i64,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().get_parent_interface::<Seekable>() as *const ffi::GSeekableIface;

            let func = (*parent_iface)
                .truncate_fn
                .expect("no parent \"truncate\" implementation");

            let mut err = ptr::null_mut();
            func(
                seekable.unsafe_cast_ref::<Seekable>().to_glib_none().0,
                offset,
                cancellable.to_glib_none().0,
                &mut err,
            );

            if err.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(err))
            }
        }
    }
}

unsafe impl<T: SeekableImpl> IsImplementable<T> for Seekable {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.tell = Some(seekable_tell::<T>);
        iface.can_seek = Some(seekable_can_seek::<T>);
        iface.seek = Some(seekable_seek::<T>);
        iface.can_truncate = Some(seekable_can_truncate::<T>);
        iface.truncate_fn = Some(seekable_truncate::<T>);
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn seekable_tell<T: SeekableImpl>(seekable: *mut ffi::GSeekable) -> i64 {
    let instance = &*(seekable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.tell(from_glib_borrow::<_, Seekable>(seekable).unsafe_cast_ref())
}

unsafe extern "C" fn seekable_can_seek<T: SeekableImpl>(
    seekable: *mut ffi::GSeekable,
) -> glib::ffi::gboolean {
    let instance = &*(seekable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.can_seek(from_glib_borrow::<_, Seekable>(seekable).unsafe_cast_ref())
        .to_glib()
}

unsafe extern "C" fn seekable_seek<T: SeekableImpl>(
    seekable: *mut ffi::GSeekable,
    offset: i64,
    type_: glib::ffi::GSeekType,
    cancellable: *mut ffi::GCancellable,
    err: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(seekable as *mut T::Instance);
    let imp = instance.get_impl();

    match imp.seek(
        from_glib_borrow::<_, Seekable>(seekable).unsafe_cast_ref(),
        offset,
        from_glib(type_),
        Option::<Cancellable>::from_glib_borrow(cancellable)
            .as_ref()
            .as_ref(),
    ) {
        Ok(()) => glib::ffi::GTRUE,
        Err(e) => {
            let mut e = mem::ManuallyDrop::new(e);
            *err = e.to_glib_none_mut().0;
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn seekable_can_truncate<T: SeekableImpl>(
    seekable: *mut ffi::GSeekable,
) -> glib::ffi::gboolean {
    let instance = &*(seekable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.can_truncate(from_glib_borrow::<_, Seekable>(seekable).unsafe_cast_ref())
        .to_glib()
}

unsafe extern "C" fn seekable_truncate<T: SeekableImpl>(
    seekable: *mut ffi::GSeekable,
    offset: i64,
    cancellable: *mut ffi::GCancellable,
    err: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(seekable as *mut T::Instance);
    let imp = instance.get_impl();

    match imp.truncate(
        from_glib_borrow::<_, Seekable>(seekable).unsafe_cast_ref(),
        offset,
        Option::<Cancellable>::from_glib_borrow(cancellable)
            .as_ref()
            .as_ref(),
    ) {
        Ok(()) => glib::ffi::GTRUE,
        Err(e) => {
            let mut e = mem::ManuallyDrop::new(e);
            *err = e.to_glib_none_mut().0;
            glib::ffi::GFALSE
        }
    }
}

// Take a look at the license at the top of the repository in the LICENSE file.

use super::icon::IconImpl;
use crate::{AsyncResult, Cancellable, InputStream, LoadableIcon};
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, GString, IsA};

#[derive(Debug)]
pub struct AsyncReadyCallback {
    pub callback: ffi::GAsyncReadyCallback,
    pub user_data: glib::ffi::gpointer,
}

pub trait LoadableIconImpl: IconImpl {
    fn load(
        &self,
        icon: &Self::Type,
        size: i32,
        cancellable: Option<&Cancellable>,
    ) -> Result<(InputStream, Option<GString>), glib::Error>;
    fn load_async(
        &self,
        icon: &Self::Type,
        size: i32,
        cancellable: Option<&Cancellable>,
        callback: AsyncReadyCallback,
    );
    fn load_finish(
        &self,
        icon: &Self::Type,
        result: AsyncResult,
    ) -> Result<(InputStream, Option<GString>), glib::Error>;
}

unsafe impl<T: LoadableIconImpl> IsImplementable<T> for LoadableIcon
where
    <T as ObjectSubclass>::Type: IsA<glib::Object>,
{
    unsafe extern "C" fn interface_init(
        iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
        let icon_iface = &mut *(iface as *mut ffi::GLoadableIconIface);

        icon_iface.load = Some(icon_load::<T>);
        icon_iface.load_async = Some(icon_load_async::<T>);
        icon_iface.load_finish = Some(icon_load_finish::<T>);
    }
}

unsafe extern "C" fn icon_load<T: LoadableIconImpl>(
    icon: *mut ffi::GLoadableIcon,
    size: i32,
    typeptr: *mut *mut libc::c_char,
    cancellableptr: *mut ffi::GCancellable,
    errorptr: *mut *mut glib::ffi::GError,
) -> *mut ffi::GInputStream {
    let instance = &*(icon as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow::<_, LoadableIcon>(icon);

    let cancellable: Borrowed<Option<Cancellable>> = from_glib_borrow(cancellableptr);

    let ret = imp.load(wrap.unsafe_cast_ref(), size, cancellable.as_ref().as_ref());
    match ret {
        Ok((stream, some_type)) => {
            if let Some(type_) = some_type {
                *typeptr = type_.to_glib_none().0;
            }

            stream.to_glib_full()
        }
        Err(err) => {
            *errorptr = err.to_glib_full() as *mut _;
            *typeptr = std::ptr::null_mut();

            std::ptr::null_mut()
        }
    }
}

unsafe extern "C" fn icon_load_async<T: LoadableIconImpl>(
    icon: *mut ffi::GLoadableIcon,
    size: i32,
    cancellableptr: *mut ffi::GCancellable,
    callbackptr: ffi::GAsyncReadyCallback,
    dataptr: glib::ffi::gpointer,
) {
    let instance = &*(icon as *mut T::Instance);
    let imp = instance.get_impl();
    let cancellable: Borrowed<Option<Cancellable>> = from_glib_borrow(cancellableptr);
    let callback = AsyncReadyCallback {
        callback: callbackptr,
        user_data: dataptr,
    };
    imp.load_async(
        from_glib_borrow::<_, LoadableIcon>(icon).unsafe_cast_ref(),
        size,
        cancellable.as_ref().as_ref(),
        callback,
    )
}

unsafe extern "C" fn icon_load_finish<T: LoadableIconImpl>(
    icon: *mut ffi::GLoadableIcon,
    resultptr: *mut ffi::GAsyncResult,
    typeptr: *mut *mut libc::c_char,
    errorptr: *mut *mut glib::ffi::GError,
) -> *mut ffi::GInputStream {
    let instance = &*(icon as *mut T::Instance);
    let imp = instance.get_impl();
    let result: AsyncResult = from_glib_full(resultptr);

    let ret = imp.load_finish(
        from_glib_borrow::<_, LoadableIcon>(icon).unsafe_cast_ref(),
        result,
    );
    match ret {
        Ok((stream, some_type)) => {
            if let Some(type_) = some_type {
                *typeptr = type_.to_glib_none().0;
            }

            stream.to_glib_full()
        }
        Err(err) => {
            *errorptr = err.to_glib_full() as *mut _;
            *typeptr = std::ptr::null_mut();

            std::ptr::null_mut()
        }
    }
}

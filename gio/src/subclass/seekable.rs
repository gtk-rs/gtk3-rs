// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use gio_sys;
use glib_sys;

use glib::translate::*;
use glib::Cast;
use glib::Error;
use glib::SeekType;

use glib::subclass::prelude::*;

use std::mem;

use Cancellable;
use Seekable;

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

unsafe impl<T: SeekableImpl> IsImplementable<T> for Seekable {
    unsafe extern "C" fn interface_init(
        iface: glib_sys::gpointer,
        _iface_data: glib_sys::gpointer,
    ) {
        let seekable_iface = &mut *(iface as *mut gio_sys::GSeekableIface);

        seekable_iface.tell = Some(seekable_tell::<T>);
        seekable_iface.can_seek = Some(seekable_can_seek::<T>);
        seekable_iface.seek = Some(seekable_seek::<T>);
        seekable_iface.can_truncate = Some(seekable_can_truncate::<T>);
        seekable_iface.truncate_fn = Some(seekable_truncate::<T>);
    }
}

unsafe extern "C" fn seekable_tell<T: SeekableImpl>(seekable: *mut gio_sys::GSeekable) -> i64 {
    let instance = &*(seekable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.tell(from_glib_borrow::<_, Seekable>(seekable).unsafe_cast_ref())
}

unsafe extern "C" fn seekable_can_seek<T: SeekableImpl>(
    seekable: *mut gio_sys::GSeekable,
) -> glib_sys::gboolean {
    let instance = &*(seekable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.can_seek(from_glib_borrow::<_, Seekable>(seekable).unsafe_cast_ref())
        .to_glib()
}

unsafe extern "C" fn seekable_seek<T: SeekableImpl>(
    seekable: *mut gio_sys::GSeekable,
    offset: i64,
    type_: glib_sys::GSeekType,
    cancellable: *mut gio_sys::GCancellable,
    err: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean {
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
        Ok(()) => glib_sys::GTRUE,
        Err(e) => {
            let mut e = mem::ManuallyDrop::new(e);
            *err = e.to_glib_none_mut().0;
            glib_sys::GFALSE
        }
    }
}

unsafe extern "C" fn seekable_can_truncate<T: SeekableImpl>(
    seekable: *mut gio_sys::GSeekable,
) -> glib_sys::gboolean {
    let instance = &*(seekable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.can_truncate(from_glib_borrow::<_, Seekable>(seekable).unsafe_cast_ref())
        .to_glib()
}

unsafe extern "C" fn seekable_truncate<T: SeekableImpl>(
    seekable: *mut gio_sys::GSeekable,
    offset: i64,
    cancellable: *mut gio_sys::GCancellable,
    err: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean {
    let instance = &*(seekable as *mut T::Instance);
    let imp = instance.get_impl();

    match imp.truncate(
        from_glib_borrow::<_, Seekable>(seekable).unsafe_cast_ref(),
        offset,
        Option::<Cancellable>::from_glib_borrow(cancellable)
            .as_ref()
            .as_ref(),
    ) {
        Ok(()) => glib_sys::GTRUE,
        Err(e) => {
            let mut e = mem::ManuallyDrop::new(e);
            *err = e.to_glib_none_mut().0;
            glib_sys::GFALSE
        }
    }
}

// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gio_sys;
use glib_sys;

use glib::translate::*;
use glib::Error;
use glib::SeekType;

use glib::subclass::prelude::*;

use std::mem;

use Cancellable;
use Seekable;

pub trait SeekableImpl: ObjectImpl + Send + 'static {
    fn tell(&self, seekable: &Seekable) -> i64;
    fn can_seek(&self, seekable: &Seekable) -> bool;
    fn seek(
        &self,
        seekable: &Seekable,
        offset: i64,
        type_: SeekType,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error>;
    fn can_truncate(&self, seekable: &Seekable) -> bool;
    fn truncate(
        &self,
        seekable: &Seekable,
        offset: i64,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error>;
}

unsafe impl<T: ObjectSubclass + SeekableImpl> IsImplementable<T> for Seekable {
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

unsafe extern "C" fn seekable_tell<T: ObjectSubclass>(seekable: *mut gio_sys::GSeekable) -> i64
where
    T: SeekableImpl,
{
    let instance = &*(seekable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.tell(&from_glib_borrow(seekable))
}

unsafe extern "C" fn seekable_can_seek<T: ObjectSubclass>(
    seekable: *mut gio_sys::GSeekable,
) -> glib_sys::gboolean
where
    T: SeekableImpl,
{
    let instance = &*(seekable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.can_seek(&from_glib_borrow(seekable)).to_glib()
}

unsafe extern "C" fn seekable_seek<T: ObjectSubclass>(
    seekable: *mut gio_sys::GSeekable,
    offset: i64,
    type_: glib_sys::GSeekType,
    cancellable: *mut gio_sys::GCancellable,
    err: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean
where
    T: SeekableImpl,
{
    let instance = &*(seekable as *mut T::Instance);
    let imp = instance.get_impl();

    match imp.seek(
        &from_glib_borrow(seekable),
        offset,
        from_glib(type_),
        Option::<Cancellable>::from_glib_borrow(cancellable).as_ref(),
    ) {
        Ok(()) => glib_sys::GTRUE,
        Err(mut e) => {
            *err = e.to_glib_none_mut().0;
            mem::forget(e);
            glib_sys::GFALSE
        }
    }
}

unsafe extern "C" fn seekable_can_truncate<T: ObjectSubclass>(
    seekable: *mut gio_sys::GSeekable,
) -> glib_sys::gboolean
where
    T: SeekableImpl,
{
    let instance = &*(seekable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.can_truncate(&from_glib_borrow(seekable)).to_glib()
}

unsafe extern "C" fn seekable_truncate<T: ObjectSubclass>(
    seekable: *mut gio_sys::GSeekable,
    offset: i64,
    cancellable: *mut gio_sys::GCancellable,
    err: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean
where
    T: SeekableImpl,
{
    let instance = &*(seekable as *mut T::Instance);
    let imp = instance.get_impl();

    match imp.truncate(
        &from_glib_borrow(seekable),
        offset,
        Option::<Cancellable>::from_glib_borrow(cancellable).as_ref(),
    ) {
        Ok(()) => glib_sys::GTRUE,
        Err(mut e) => {
            *err = e.to_glib_none_mut().0;
            mem::forget(e);
            glib_sys::GFALSE
        }
    }
}

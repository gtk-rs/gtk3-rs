// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gio_sys;
use glib_sys;

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use glib::Error;

use Cancellable;
use IOStream;
use IOStreamClass;

use std::mem;
use std::ptr;

pub trait IOStreamImpl: IOStreamImplExt + Send + 'static {
    fn get_input_stream(&self, stream: &IOStream) -> crate::InputStream {
        self.parent_get_input_stream(stream)
    }

    fn get_output_stream(&self, stream: &IOStream) -> crate::OutputStream {
        self.parent_get_output_stream(stream)
    }

    fn close(&self, stream: &IOStream, cancellable: Option<&Cancellable>) -> Result<(), Error> {
        self.parent_close(stream, cancellable)
    }
}

pub trait IOStreamImplExt {
    fn parent_get_input_stream(&self, stream: &IOStream) -> crate::InputStream;

    fn parent_get_output_stream(&self, stream: &IOStream) -> crate::OutputStream;

    fn parent_close(
        &self,
        stream: &IOStream,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error>;
}

impl<T: IOStreamImpl + ObjectImpl> IOStreamImplExt for T {
    fn parent_get_input_stream(&self, stream: &IOStream) -> crate::InputStream {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gio_sys::GIOStreamClass;
            let f = (*parent_class)
                .get_input_stream
                .expect("No parent class implementation for \"get_input_stream\"");
            from_glib_none(f(stream.to_glib_none().0))
        }
    }

    fn parent_get_output_stream(&self, stream: &IOStream) -> crate::OutputStream {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gio_sys::GIOStreamClass;
            let f = (*parent_class)
                .get_output_stream
                .expect("No parent class implementation for \"get_output_stream\"");
            from_glib_none(f(stream.to_glib_none().0))
        }
    }

    fn parent_close(
        &self,
        stream: &IOStream,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gio_sys::GIOStreamClass;
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
}

unsafe impl<T: ObjectSubclass + IOStreamImpl> IsSubclassable<T> for IOStreamClass {
    fn override_vfuncs(&mut self) {
        <glib::ObjectClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gio_sys::GIOStreamClass);
            klass.get_input_stream = Some(stream_get_input_stream::<T>);
            klass.get_output_stream = Some(stream_get_output_stream::<T>);
            klass.close_fn = Some(stream_close::<T>);
        }
    }
}

lazy_static! {
    pub static ref OUTPUT_STREAM_QUARK: glib::Quark =
        glib::Quark::from_string("gtk-rs-subclass-output-stream");
    pub static ref INPUT_STREAM_QUARK: glib::Quark =
        glib::Quark::from_string("gtk-rs-subclass-input-stream");
}

unsafe extern "C" fn stream_get_input_stream<T: ObjectSubclass>(
    ptr: *mut gio_sys::GIOStream,
) -> *mut gio_sys::GInputStream
where
    T: IOStreamImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: IOStream = from_glib_borrow(ptr);

    let ret = imp.get_input_stream(&wrap);

    // Ensure that a) the stream stays alive as long as the IO stream instance and
    // b) that the same stream is returned every time. This is a requirement by the
    // IO stream API.
    let old_ptr = gobject_sys::g_object_get_qdata(ptr as *mut _, INPUT_STREAM_QUARK.to_glib());
    if !old_ptr.is_null() {
        assert_eq!(
            old_ptr as *mut _,
            ret.as_ptr(),
            "Did not return same input stream again"
        );
    }

    unsafe extern "C" fn unref(ptr: glib_sys::gpointer) {
        gobject_sys::g_object_unref(ptr as *mut _);
    }
    gobject_sys::g_object_set_qdata_full(
        ptr as *mut _,
        INPUT_STREAM_QUARK.to_glib(),
        ret.as_ptr() as *mut _,
        Some(unref),
    );

    ret.to_glib_none().0
}

unsafe extern "C" fn stream_get_output_stream<T: ObjectSubclass>(
    ptr: *mut gio_sys::GIOStream,
) -> *mut gio_sys::GOutputStream
where
    T: IOStreamImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: IOStream = from_glib_borrow(ptr);

    let ret = imp.get_output_stream(&wrap);

    // Ensure that a) the stream stays alive as long as the IO stream instance and
    // b) that the same stream is returned every time. This is a requirement by the
    // IO stream API.
    let old_ptr = gobject_sys::g_object_get_qdata(ptr as *mut _, OUTPUT_STREAM_QUARK.to_glib());
    if !old_ptr.is_null() {
        assert_eq!(
            old_ptr as *mut _,
            ret.as_ptr(),
            "Did not return same output stream again"
        );
    }

    unsafe extern "C" fn unref(ptr: glib_sys::gpointer) {
        gobject_sys::g_object_unref(ptr as *mut _);
    }
    gobject_sys::g_object_set_qdata_full(
        ptr as *mut _,
        OUTPUT_STREAM_QUARK.to_glib(),
        ret.as_ptr() as *mut _,
        Some(unref),
    );

    ret.to_glib_none().0
}

unsafe extern "C" fn stream_close<T: ObjectSubclass>(
    ptr: *mut gio_sys::GIOStream,
    cancellable: *mut gio_sys::GCancellable,
    err: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean
where
    T: IOStreamImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: IOStream = from_glib_borrow(ptr);

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

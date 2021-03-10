// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use glib::{Cast, Error};

use crate::Cancellable;
use crate::{IOStream, InputStream, OutputStream};

use std::mem;
use std::ptr;

use once_cell::sync::Lazy;

pub trait IOStreamImpl: ObjectImpl + IOStreamImplExt + Send {
    fn get_input_stream(&self, stream: &Self::Type) -> InputStream {
        self.parent_get_input_stream(stream)
    }

    fn get_output_stream(&self, stream: &Self::Type) -> OutputStream {
        self.parent_get_output_stream(stream)
    }

    fn close(&self, stream: &Self::Type, cancellable: Option<&Cancellable>) -> Result<(), Error> {
        self.parent_close(stream, cancellable)
    }
}

pub trait IOStreamImplExt: ObjectSubclass {
    fn parent_get_input_stream(&self, stream: &Self::Type) -> InputStream;

    fn parent_get_output_stream(&self, stream: &Self::Type) -> OutputStream;

    fn parent_close(
        &self,
        stream: &Self::Type,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error>;
}

impl<T: IOStreamImpl> IOStreamImplExt for T {
    fn parent_get_input_stream(&self, stream: &Self::Type) -> InputStream {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GIOStreamClass;
            let f = (*parent_class)
                .get_input_stream
                .expect("No parent class implementation for \"get_input_stream\"");
            from_glib_none(f(stream.unsafe_cast_ref::<IOStream>().to_glib_none().0))
        }
    }

    fn parent_get_output_stream(&self, stream: &Self::Type) -> OutputStream {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GIOStreamClass;
            let f = (*parent_class)
                .get_output_stream
                .expect("No parent class implementation for \"get_output_stream\"");
            from_glib_none(f(stream.unsafe_cast_ref::<IOStream>().to_glib_none().0))
        }
    }

    fn parent_close(
        &self,
        stream: &Self::Type,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GIOStreamClass;
            let mut err = ptr::null_mut();
            if let Some(f) = (*parent_class).close_fn {
                if from_glib(f(
                    stream.unsafe_cast_ref::<IOStream>().to_glib_none().0,
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

unsafe impl<T: IOStreamImpl> IsSubclassable<T> for IOStream {
    fn class_init(class: &mut ::glib::Class<Self>) {
        <glib::Object as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.get_input_stream = Some(stream_get_input_stream::<T>);
        klass.get_output_stream = Some(stream_get_output_stream::<T>);
        klass.close_fn = Some(stream_close::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <glib::Object as IsSubclassable<T>>::instance_init(instance);
    }
}

static OUTPUT_STREAM_QUARK: Lazy<glib::Quark> =
    Lazy::new(|| glib::Quark::from_string("gtk-rs-subclass-output-stream"));
static INPUT_STREAM_QUARK: Lazy<glib::Quark> =
    Lazy::new(|| glib::Quark::from_string("gtk-rs-subclass-input-stream"));

unsafe extern "C" fn stream_get_input_stream<T: IOStreamImpl>(
    ptr: *mut ffi::GIOStream,
) -> *mut ffi::GInputStream {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<IOStream> = from_glib_borrow(ptr);

    let ret = imp.get_input_stream(wrap.unsafe_cast_ref());

    // Ensure that a) the stream stays alive as long as the IO stream instance and
    // b) that the same stream is returned every time. This is a requirement by the
    // IO stream API.
    if let Some(old_stream) = wrap.get_qdata::<InputStream>(*INPUT_STREAM_QUARK) {
        assert_eq!(
            old_stream.as_ref(),
            &ret,
            "Did not return same input stream again"
        );
    }
    wrap.set_qdata(*INPUT_STREAM_QUARK, ret.clone());
    ret.to_glib_none().0
}

unsafe extern "C" fn stream_get_output_stream<T: IOStreamImpl>(
    ptr: *mut ffi::GIOStream,
) -> *mut ffi::GOutputStream {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<IOStream> = from_glib_borrow(ptr);

    let ret = imp.get_output_stream(wrap.unsafe_cast_ref());

    // Ensure that a) the stream stays alive as long as the IO stream instance and
    // b) that the same stream is returned every time. This is a requirement by the
    // IO stream API.
    if let Some(old_stream) = wrap.get_qdata::<OutputStream>(*OUTPUT_STREAM_QUARK) {
        assert_eq!(
            old_stream.as_ref(),
            &ret,
            "Did not return same output stream again"
        );
    }
    wrap.set_qdata(*OUTPUT_STREAM_QUARK, ret.clone());
    ret.to_glib_none().0
}

unsafe extern "C" fn stream_close<T: IOStreamImpl>(
    ptr: *mut ffi::GIOStream,
    cancellable: *mut ffi::GCancellable,
    err: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<IOStream> = from_glib_borrow(ptr);

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

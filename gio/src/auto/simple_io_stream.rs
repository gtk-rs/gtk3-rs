// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::IOStream;
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
use crate::InputStream;
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
use crate::OutputStream;
use glib;
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
use glib::object::Cast;
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct SimpleIOStream(Object<ffi::GSimpleIOStream>) @extends IOStream;

    match fn {
        get_type => || ffi::g_simple_io_stream_get_type(),
    }
}

impl SimpleIOStream {
    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    pub fn new<P: IsA<InputStream>, Q: IsA<OutputStream>>(
        input_stream: &P,
        output_stream: &Q,
    ) -> SimpleIOStream {
        unsafe {
            IOStream::from_glib_full(ffi::g_simple_io_stream_new(
                input_stream.as_ref().to_glib_none().0,
                output_stream.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

impl fmt::Display for SimpleIOStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SimpleIOStream")
    }
}

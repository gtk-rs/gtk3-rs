// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ConverterFlags;
use ConverterResult;
use Error;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Converter(Object<ffi::GConverter, ffi::GConverterIface>);

    match fn {
        get_type => || ffi::g_converter_get_type(),
    }
}

pub trait ConverterExt {
    fn convert(&self, inbuf: &[u8], outbuf: &mut [u8], flags: ConverterFlags) -> Result<(ConverterResult, usize, usize), Error>;

    fn reset(&self);
}

impl<O: IsA<Converter>> ConverterExt for O {
    fn convert(&self, inbuf: &[u8], outbuf: &mut [u8], flags: ConverterFlags) -> Result<(ConverterResult, usize, usize), Error> {
        let inbuf_size = inbuf.len() as usize;
        let outbuf_size = outbuf.len() as usize;
        unsafe {
            let mut bytes_read = mem::uninitialized();
            let mut bytes_written = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = ffi::g_converter_convert(self.to_glib_none().0, inbuf.to_glib_none().0, inbuf_size, outbuf.to_glib_none().0, outbuf_size, flags.to_glib(), &mut bytes_read, &mut bytes_written, &mut error);
            if error.is_null() { Ok((from_glib(ret), bytes_read, bytes_written)) } else { Err(from_glib_full(error)) }
        }
    }

    fn reset(&self) {
        unsafe {
            ffi::g_converter_reset(self.to_glib_none().0);
        }
    }
}

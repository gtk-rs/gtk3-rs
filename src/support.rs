// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::mem;
use std::marker::PhantomData;
use std::io;
use std::slice;

use ffi::{self, cairo_status_t};
use libc::{c_void, c_uchar, c_uint, c_double};
use ::enums::Status;
use surface::{Surface, SurfaceExt};


pub type Constructor = unsafe extern fn (ffi::cairo_write_func_t, *mut c_void, c_double, c_double) -> *mut ffi::cairo_surface_t;

pub trait FromRawSurface {
    unsafe fn from_raw_surface(surface: *mut ffi::cairo_surface_t) -> Self;
}

pub struct Writer<S: FromRawSurface + AsRef<Surface>, W: io::Write> {
    pub surface: S,
    writer: Box<W>,
}

impl<S: FromRawSurface + AsRef<Surface>, W: io::Write> Writer<S, W> {
    extern fn write_cb(writer: *mut c_void, data: *mut c_uchar, length: c_uint) -> cairo_status_t {
        let mut writer: Box<W> = unsafe { Box::from_raw(writer as *mut _) };
        let data = unsafe { slice::from_raw_parts(data, length as usize) };

        let result = match writer.write_all(data) {
            Ok(_) => Status::Success,
            Err(_) => Status::WriteError,
        };

        mem::forget(writer);
        result.into()
    }

    pub fn new(constructor: Constructor, width: f64, height: f64, writer: W) -> Writer<S, W> {
        let mut writer = Box::new(writer);
        let writer_ptr = unsafe { mem::transmute(&mut *writer) };
        let surface = unsafe {
            S::from_raw_surface(constructor(Some(Self::write_cb), writer_ptr, width, height))
        };

        Writer {
            surface,
            writer,
        }
    }

    pub fn finish(self) -> W {
        let surface = self.surface;
        surface.as_ref().finish();
        drop(surface);

        *self.writer
    }
}

pub struct RefWriter<'w, S: FromRawSurface, W: io::Write + 'w> {
    pub surface: S,
    _reference: PhantomData<&'w mut W>,
}

impl<'w, S: FromRawSurface, W: io::Write + 'w> RefWriter<'w, S, W> {
    extern fn write_cb(writer: *mut c_void, data: *mut c_uchar, length: c_uint) -> cairo_status_t {
        let writer: &'w mut W = unsafe { mem::transmute(writer) };
        let data = unsafe { slice::from_raw_parts(data, length as usize) };

        let result = match writer.write_all(data) {
            Ok(_) => Status::Success,
            Err(_) => Status::WriteError,
        };

        mem::forget(writer);
        result.into()
    }

    pub fn new(constructor: Constructor, width: f64, height: f64, writer: &'w mut W) -> RefWriter<'w, S, W> {
        let writer_ptr = unsafe { mem::transmute(writer) };
        let surface = unsafe {
            S::from_raw_surface(constructor(Some(Self::write_cb), writer_ptr, width, height))
        };

        RefWriter {
            surface,
            _reference: PhantomData,
        }
    }
}

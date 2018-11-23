// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::io;
use std::slice;

use ffi::{self, cairo_status_t};
use libc::{c_void, c_uchar, c_uint, c_double};
use ::enums::Status;

pub type Constructor = unsafe extern fn (ffi::cairo_write_func_t, *mut c_void, c_double, c_double) -> *mut ffi::cairo_surface_t;

pub struct Stream<'a> {
    ptr: *mut ffi::cairo_surface_t,
    func: *mut Box<'a + FnMut(&[u8]) -> Result<(), ()>>,
}

impl<'a> Stream<'a> {
    pub fn new<'b, F>(constructor: Constructor, width: f64, height: f64, func: F) -> Stream<'b>
        where F: 'b + FnMut(&[u8]) -> Result<(), ()>
    {
        unsafe {
            unsafe extern fn write_to(func: *mut c_void, data: *mut c_uchar, length: c_uint) -> cairo_status_t
            {
                // This is perfectly fine, lifetimes don't really exist.
                let mut func: Box<Box<FnMut(&[u8]) -> Result<(), ()>>> = Box::from_raw(func as *mut _);

                let data = slice::from_raw_parts(data, length as usize);
                let result = match func(data) {
                    Ok(_) => Status::Success,
                    Err(_) => Status::WriteError,
                };

                // We don't want to actually drop the closure, send it back
                // into limbo.
                Box::into_raw(func);

                result.into()
            }

            // Type inference can't into this.
            let func: *mut Box<'b + FnMut(&[u8]) -> Result<(), ()>> = Box::into_raw(Box::new(Box::new(func)));
            let surface = constructor(Some(write_to), func as *mut _, width, height);

            Stream {
                ptr: surface,
                func: func,
            }
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::cairo_surface_t {
        self.ptr
    }
}

impl<'a> Drop for Stream<'a> {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.func);
        }
    }
}

pub struct Writer<'a> {
    inner: Stream<'a>,
}

impl<'a> Writer<'a> {
    pub fn new<'b, W: 'b + io::Write>(constructor: Constructor, width: f64, height: f64, mut w: W) -> Writer<'b> {
        Writer {
            inner: Stream::new(constructor, width, height, move |data|
                match w.write_all(data) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(()),
                }),
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::cairo_surface_t {
        self.inner.as_ptr()
    }
}

pub struct Buffer {
    buffer: *mut Vec<u8>,
    inner: Stream<'static>,
}

impl Buffer {
    pub fn new(constructor: Constructor, width: f64, height: f64) -> Buffer {
        let buffer = Box::into_raw(Box::new(Vec::new()));

        Buffer {
            buffer: buffer,
            inner: Stream::new(constructor, width, height, move |data| {
                unsafe {
                    let mut out = Box::from_raw(buffer);
                    out.extend(data);
                    Box::into_raw(out);
                }

                Ok(())
            }),
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::cairo_surface_t {
        self.inner.as_ptr()
    }
}

impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            let vec = Box::from_raw(self.buffer);
            let ptr = vec.as_ptr();
            let len = vec.len();
            Box::into_raw(vec);

            slice::from_raw_parts(ptr, len)
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.buffer);
        }
    }
}

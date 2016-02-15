// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::slice;
use std::io::{Read, Write, Error};
use std::process;
use std::thread;

use libc::{c_void, c_uint};

use glib::translate::*;
use ffi;
use ffi::enums::Status;
use error::IoError;
use ImageSurface;


struct CallbackGuard;

impl Drop for CallbackGuard {
    fn drop(&mut self) {
        if thread::panicking() {
            process::exit(101);
        }
    }
}

struct ReadEnv<R: Read> {
    reader: R,
    error: Option<Error>,
}

unsafe extern "C" fn read_func<R: Read>(closure: *mut c_void, data: *mut u8, len: c_uint) -> Status {
    let _cbguard = CallbackGuard;
    let read_env: &mut ReadEnv<R> = &mut *(closure as *mut ReadEnv<R>);
    let mut buffer = slice::from_raw_parts_mut(data, len as usize);
    match read_env.reader.read_exact(buffer) {
        Ok(()) => Status::Success,
        Err(error) => {
            read_env.error = Some(error);
            Status::ReadError
        },
    }
}

struct WriteEnv<W: Write> {
    writer: W,
    error: Option<Error>,
}

unsafe extern "C" fn write_func<W: Write>(closure: *mut c_void, data: *mut u8, len: c_uint) -> Status {
    let _cbguard = CallbackGuard;
    let write_env: &mut WriteEnv<W> = &mut *(closure as *mut WriteEnv<W>);
    let buffer = slice::from_raw_parts(data, len as usize);
    match write_env.writer.write_all(buffer) {
        Ok(()) => Status::Success,
        Err(error) => {
            write_env.error = Some(error);
            Status::WriteError
        },
    }
}


impl ImageSurface {
    pub fn create_from_png<R: Read>(stream: R) -> Result<ImageSurface, IoError> {
        let mut env = ReadEnv{ reader: stream, error: None };
        let surface: ImageSurface = unsafe { from_glib_full(ffi::cairo_image_surface_create_from_png_stream(
            Some(read_func::<R>), &mut env as *mut ReadEnv<R> as *mut c_void)) };
        match env.error {
            None => match surface.as_ref().status() {   // The surface migth still be "nil" if the error occured in Cairo
                Status::Success => Ok(surface),
                st => Err(IoError::Cairo(st)),
            },
            Some(err) => Err(IoError::Io(err)),
        }
    }

    pub fn write_to_png<W: Write>(&self, stream: W) -> Result<(), IoError> {
        let mut env = WriteEnv{ writer: stream, error: None };
        let status = unsafe { ffi::cairo_surface_write_to_png_stream(self.to_glib_none().0,
            Some(write_func::<W>), &mut env as *mut WriteEnv<W> as *mut c_void) };
        match env.error {
            None => match status {
                Status::Success => Ok(()),
                st => Err(IoError::Cairo(st)),
            },
            Some(err) => Err(IoError::Io(err)),
        }
    }
}

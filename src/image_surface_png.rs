// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::slice;
use std::io::{Read, Write, Error};
use std::process;
use std::thread;

use libc::{c_void, c_uint};

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

struct ReadEnv<'a, R: 'a + Read> {
    reader: &'a mut R,
    error: Option<Error>,
}

unsafe extern "C" fn read_func<R: Read>(closure: *mut c_void, data: *mut u8, len: c_uint) -> Status {
    let _cbguard = CallbackGuard;
    let read_env: &mut ReadEnv<R> = &mut *(closure as *mut ReadEnv<R>);
    let buffer = slice::from_raw_parts_mut(data, len as usize);
    match read_env.reader.read_exact(buffer) {
        Ok(()) => Status::Success,
        Err(error) => {
            read_env.error = Some(error);
            Status::ReadError
        },
    }
}

struct WriteEnv<'a, W: 'a + Write> {
    writer: &'a mut W,
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
    pub fn create_from_png<R: Read>(stream: &mut R) -> Result<ImageSurface, IoError> {
        let mut env = ReadEnv{ reader: stream, error: None };
        unsafe {
            let raw_surface = ffi::cairo_image_surface_create_from_png_stream(
                Some(read_func::<R>),
                &mut env as *mut ReadEnv<R> as *mut c_void);

            let surface = ImageSurface::from_raw_full(raw_surface)?;

            match env.error {
                None => Ok(surface),
                Some(err) => Err(IoError::Io(err)),
            }
        }
    }

    pub fn write_to_png<W: Write>(&self, stream: &mut W) -> Result<(), IoError> {
        let mut env = WriteEnv{ writer: stream, error: None };
        let status = unsafe { ffi::cairo_surface_write_to_png_stream(self.to_raw_none(),
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

#[cfg(test)]
mod tests {
    use std::io::ErrorKind;
    use super::*;
    use ffi::enums::Format;

    struct IoErrorReader;

    // A reader that always returns an error
    impl Read for IoErrorReader {
        fn read(&mut self, _: &mut [u8]) -> Result<usize, Error> {
            Err(Error::new(ErrorKind::Other, "yikes!"))
        }
    }

    #[test]
    fn valid_png_reads_correctly() {
        // A 1x1 PNG, RGB, no alpha, with a single pixel with (42, 42, 42) values
        let png_data: Vec<u8> = vec![
            0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a,  0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,  0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53,
            0xde, 0x00, 0x00, 0x00, 0x0c, 0x49, 0x44, 0x41,  0x54, 0x08, 0xd7, 0x63, 0xd0, 0xd2, 0xd2, 0x02,
            0x00, 0x01, 0x00, 0x00, 0x7f, 0x09, 0xa9, 0x5a,  0x4d, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e,
            0x44, 0xae, 0x42, 0x60, 0x82
        ];

        let r = ImageSurface::create_from_png(&mut &png_data[..]);
        assert!(r.is_ok());

        let mut surface = r.unwrap();
        assert!(surface.get_width() == 1);
        assert!(surface.get_height() == 1);
        assert!(surface.get_format() == Format::Rgb24);

        let data = surface.get_data().unwrap();
        assert!(data.len() >= 3);

        let slice = &data[0..3];
        assert!(slice[0] == 42);
        assert!(slice[1] == 42);
        assert!(slice[2] == 42);
    }

    #[test]
    fn invalid_png_yields_error() {
        let png_data: Vec<u8> = vec![
            //      v--- this byte is modified
            0x89, 0x40, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a,  0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,  0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53,
            0xde, 0x00, 0x00, 0x00, 0x0c, 0x49, 0x44, 0x41,  0x54, 0x08, 0xd7, 0x63, 0xd0, 0xd2, 0xd2, 0x02,
            0x00, 0x01, 0x00, 0x00, 0x7f, 0x09, 0xa9, 0x5a,  0x4d, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e,
            0x44, 0xae, 0x42, 0x60, 0x82
        ];

        match ImageSurface::create_from_png(&mut &png_data[..]) {
            Err(IoError::Cairo(_)) => (),
            _ => unreachable!()
        }
    }

    #[test]
    fn io_error_yields_cairo_read_error() {
        let mut r = IoErrorReader;

        match ImageSurface::create_from_png(&mut r) {
            Err(IoError::Cairo(Status::ReadError)) => (),
            _ => unreachable!()
        }
    }
}

// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(any(all(feature = "svg", feature = "v1_16"), feature = "dox"))]
use crate::enums::SvgUnit;
use crate::enums::{SurfaceType, SvgVersion};
use crate::error::Error;
use std::convert::TryFrom;
use std::ffi::{CStr, CString};
use std::fmt;
use std::io;
use std::mem;
use std::ops::Deref;
#[cfg(not(windows))]
use std::os::unix::prelude::*;
use std::path::Path;
use std::ptr;

use crate::surface::Surface;

#[cfg(feature = "use_glib")]
use glib::translate::*;

impl SvgVersion {
    pub fn as_str(self) -> Option<&'static str> {
        unsafe {
            let res = ffi::cairo_svg_version_to_string(self.into());
            res.as_ref()
                .and_then(|cstr| CStr::from_ptr(cstr as _).to_str().ok())
        }
    }
}

declare_surface!(SvgSurface, SurfaceType::Svg);

impl SvgSurface {
    pub fn new<P: AsRef<Path>>(
        width: f64,
        height: f64,
        path: Option<P>,
    ) -> Result<SvgSurface, Error> {
        #[cfg(not(windows))]
        let path = path.map(|p| {
            CString::new(p.as_ref().as_os_str().as_bytes()).expect("Invalid path with NULL bytes")
        });
        #[cfg(windows)]
        let path = path.map(|p| {
            let path_str = p
                .as_ref()
                .to_str()
                .expect("Path can't be represented as UTF-8")
                .to_owned();
            if path_str.starts_with("\\\\?\\") {
                CString::new(path_str[4..].as_bytes())
            } else {
                CString::new(path_str.as_bytes())
            }
            .expect("Invalid path with NUL bytes")
        });

        unsafe {
            Ok(Self(Surface::from_raw_full(
                ffi::cairo_svg_surface_create(
                    path.as_ref().map(|p| p.as_ptr()).unwrap_or(ptr::null()),
                    width,
                    height,
                ),
            )?))
        }
    }

    for_stream_constructors!(cairo_svg_surface_create_for_stream);

    pub fn get_versions() -> impl Iterator<Item = SvgVersion> {
        let vers_slice = unsafe {
            let mut vers_ptr = ptr::null_mut();
            let mut num_vers = mem::MaybeUninit::uninit();
            ffi::cairo_svg_get_versions(&mut vers_ptr, num_vers.as_mut_ptr());

            std::slice::from_raw_parts(vers_ptr, num_vers.assume_init() as _)
        };

        vers_slice.iter().map(|v| SvgVersion::from(*v))
    }

    pub fn restrict(&self, version: SvgVersion) {
        unsafe {
            ffi::cairo_svg_surface_restrict_to_version(self.0.to_raw_none(), version.into());
        }
    }

    #[cfg(any(all(feature = "svg", feature = "v1_16"), feature = "dox"))]
    pub fn set_document_unit(&mut self, unit: SvgUnit) {
        unsafe {
            ffi::cairo_svg_surface_set_document_unit(self.0.to_raw_none(), unit.into());
        }
    }

    #[cfg(any(all(feature = "svg", feature = "v1_16"), feature = "dox"))]
    pub fn get_document_unit(&self) -> SvgUnit {
        unsafe {
            SvgUnit::from(ffi::cairo_svg_surface_get_document_unit(
                self.0.to_raw_none(),
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::context::*;
    use tempfile::{tempfile, NamedTempFile};

    fn draw(surface: &Surface) {
        let cr = Context::new(surface);

        cr.set_line_width(25.0);

        cr.set_source_rgba(1.0, 0.0, 0.0, 0.5);
        cr.line_to(0., 0.);
        cr.line_to(100., 100.);
        cr.stroke();

        cr.set_source_rgba(0.0, 0.0, 1.0, 0.5);
        cr.line_to(0., 100.);
        cr.line_to(100., 0.);
        cr.stroke();
    }

    fn draw_in_buffer() -> Vec<u8> {
        let buffer: Vec<u8> = vec![];

        let surface = SvgSurface::for_stream(100., 100., buffer).unwrap();
        draw(&surface);
        *surface.finish_output_stream().unwrap().downcast().unwrap()
    }

    fn assert_len_close_enough(len_a: usize, len_b: usize) {
        // It seems cairo randomizies some element IDs which might make one svg slightly
        // larger than the other. Here we make sure the difference is within ~10%.
        let len_diff = (len_a as isize - len_b as isize).abs() as usize;
        assert!(len_diff < len_b / 10);
    }

    #[test]
    fn versions() {
        assert!(SvgSurface::get_versions().any(|v| v == SvgVersion::_1_1));
    }

    #[test]
    fn version_string() {
        let ver_str = SvgVersion::_1_1.as_str().unwrap();
        assert_eq!(ver_str, "SVG 1.1");
    }

    #[test]
    fn without_file() {
        let surface = SvgSurface::new(100., 100., None::<&Path>).unwrap();
        draw(&surface);
        surface.finish();
    }

    #[test]
    fn file() {
        let file = NamedTempFile::new().expect("tempfile failed");
        let surface = SvgSurface::new(100., 100., Some(&file.path())).unwrap();
        draw(&surface);
        surface.finish();
    }

    #[test]
    fn writer() {
        let file = tempfile().expect("tempfile failed");
        let surface = SvgSurface::for_stream(100., 100., file).unwrap();

        draw(&surface);
        let stream = surface.finish_output_stream().unwrap();
        let file = stream.downcast::<std::fs::File>().unwrap();

        let buffer = draw_in_buffer();
        let file_size = file.metadata().unwrap().len();

        assert_len_close_enough(file_size as usize, buffer.len());
    }

    #[test]
    fn ref_writer() {
        let mut file = tempfile().expect("tempfile failed");
        let surface = unsafe { SvgSurface::for_raw_stream(100., 100., &mut file).unwrap() };

        draw(&surface);
        surface.finish_output_stream().unwrap();
    }

    #[test]
    fn buffer() {
        let buffer = draw_in_buffer();

        let header = b"<?xml";
        assert_eq!(&buffer[..header.len()], header);
    }

    #[test]
    fn custom_writer() {
        use std::fs;

        struct CustomWriter(usize, fs::File);

        impl io::Write for CustomWriter {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                self.1.write_all(buf)?;

                self.0 += buf.len();
                Ok(buf.len())
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let file = tempfile().expect("tempfile failed");
        let custom_writer = CustomWriter(0, file);

        let surface = SvgSurface::for_stream(100., 100., custom_writer).unwrap();
        draw(&surface);
        let stream = surface.finish_output_stream().unwrap();
        let custom_writer = stream.downcast::<CustomWriter>().unwrap();

        let buffer = draw_in_buffer();

        assert_len_close_enough(custom_writer.0, buffer.len());
    }

    fn with_panicky_stream() -> SvgSurface {
        struct PanicWriter;

        impl io::Write for PanicWriter {
            fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
                panic!("panic in writer");
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let surface = SvgSurface::for_stream(20., 20., PanicWriter).unwrap();
        surface.finish();
        surface
    }

    #[test]
    #[should_panic]
    fn finish_stream_propagates_panic() {
        let _ = with_panicky_stream().finish_output_stream();
    }
}

// Take a look at the license at the top of the repository in the LICENSE file.

use std::convert::TryFrom;
use std::ffi::{CStr, CString};
use std::fmt;
use std::io;
use std::mem;
use std::ops::Deref;
use std::path::Path;
use std::ptr;

use crate::enums::{PsLevel, SurfaceType};
use crate::error::Error;
use crate::surface::Surface;

#[cfg(feature = "use_glib")]
use glib::translate::*;

impl PsLevel {
    pub fn as_str(self) -> Option<&'static str> {
        unsafe {
            let res = ffi::cairo_ps_level_to_string(self.into());
            res.as_ref()
                .and_then(|cstr| CStr::from_ptr(cstr as _).to_str().ok())
        }
    }
}

declare_surface!(PsSurface, SurfaceType::Ps);

impl PsSurface {
    #[doc(alias = "cairo_ps_surface_create")]
    pub fn new<P: AsRef<Path>>(width: f64, height: f64, path: P) -> Result<PsSurface, Error> {
        let path = path.as_ref().to_string_lossy().into_owned();
        let path = CString::new(path).unwrap();

        unsafe { Self::from_raw_full(ffi::cairo_ps_surface_create(path.as_ptr(), width, height)) }
    }

    for_stream_constructors!(cairo_ps_surface_create_for_stream);

    #[doc(alias = "cairo_ps_get_levels")]
    pub fn get_levels() -> impl Iterator<Item = PsLevel> {
        let lvls_slice = unsafe {
            let mut vers_ptr = ptr::null_mut();
            let mut num_vers = mem::MaybeUninit::uninit();
            ffi::cairo_ps_get_levels(&mut vers_ptr, num_vers.as_mut_ptr());

            std::slice::from_raw_parts(vers_ptr, num_vers.assume_init() as _)
        };

        lvls_slice.iter().map(|v| PsLevel::from(*v))
    }

    #[doc(alias = "cairo_ps_surface_restrict_to_level")]
    pub fn restrict(&self, level: PsLevel) {
        unsafe {
            ffi::cairo_ps_surface_restrict_to_level(self.0.to_raw_none(), level.into());
        }
    }

    #[doc(alias = "cairo_ps_surface_get_eps")]
    pub fn get_eps(&self) -> bool {
        unsafe { ffi::cairo_ps_surface_get_eps(self.0.to_raw_none()).as_bool() }
    }

    #[doc(alias = "cairo_ps_surface_set_eps")]
    pub fn set_eps(&self, eps: bool) {
        unsafe {
            ffi::cairo_ps_surface_set_eps(self.0.to_raw_none(), eps.into());
        }
    }

    #[doc(alias = "cairo_ps_surface_set_size")]
    pub fn set_size(&self, width: f64, height: f64) {
        unsafe {
            ffi::cairo_ps_surface_set_size(self.0.to_raw_none(), width, height);
        }
    }

    #[doc(alias = "cairo_ps_surface_dsc_begin_setup")]
    pub fn dsc_begin_setup(&self) {
        unsafe {
            ffi::cairo_ps_surface_dsc_begin_setup(self.0.to_raw_none());
        }
    }

    #[doc(alias = "cairo_ps_surface_dsc_begin_page_setup")]
    pub fn begin_page_setup(&self) {
        unsafe {
            ffi::cairo_ps_surface_dsc_begin_page_setup(self.0.to_raw_none());
        }
    }

    #[doc(alias = "cairo_ps_surface_dsc_comment")]
    pub fn dsc_comment(&self, comment: &str) {
        let comment = CString::new(comment).unwrap();
        unsafe {
            ffi::cairo_ps_surface_dsc_comment(self.0.to_raw_none(), comment.as_ptr());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::context::*;
    use tempfile::tempfile;

    fn draw(surface: &Surface) {
        let cr = Context::new(surface);

        // Note: Not using RGBA here as PS doesn't natively support
        // semi-transparency and Cairo would then embed a rasterized bitmap

        cr.set_line_width(25.0);

        cr.set_source_rgb(1.0, 0.0, 0.0);
        cr.line_to(0., 0.);
        cr.line_to(100., 100.);
        cr.stroke();

        cr.set_source_rgb(0.0, 0.0, 1.0);
        cr.line_to(0., 100.);
        cr.line_to(100., 0.);
        cr.stroke();
    }

    fn draw_in_buffer() -> Vec<u8> {
        let buffer: Vec<u8> = vec![];

        let surface = PsSurface::for_stream(100., 100., buffer).unwrap();
        draw(&surface);
        *surface.finish_output_stream().unwrap().downcast().unwrap()
    }

    #[test]
    fn levels() {
        assert!(PsSurface::get_levels().any(|v| v == PsLevel::_2));
    }

    #[test]
    fn level_string() {
        let ver_str = PsLevel::_2.as_str().unwrap();
        assert_eq!(ver_str, "PS Level 2");
    }

    #[test]
    fn eps() {
        let buffer: Vec<u8> = vec![];
        let surface = PsSurface::for_stream(100., 100., buffer).unwrap();
        surface.set_eps(true);
        assert_eq!(surface.get_eps(), true);
    }

    #[test]
    #[cfg(unix)]
    fn file() {
        let surface = PsSurface::new(100., 100., "/dev/null").unwrap();
        draw(&surface);
        surface.finish();
    }

    #[test]
    fn writer() {
        let file = tempfile().expect("tempfile failed");
        let surface = PsSurface::for_stream(100., 100., file).unwrap();

        draw(&surface);
        let stream = surface.finish_output_stream().unwrap();
        let file = stream.downcast::<std::fs::File>().unwrap();

        let buffer = draw_in_buffer();
        let file_size = file.metadata().unwrap().len();
        assert_eq!(file_size, buffer.len() as u64);
    }

    #[test]
    fn ref_writer() {
        let mut file = tempfile().expect("tempfile failed");
        let surface = unsafe { PsSurface::for_raw_stream(100., 100., &mut file).unwrap() };

        draw(&surface);
        surface.finish_output_stream().unwrap();
    }

    #[test]
    fn buffer() {
        let buffer = draw_in_buffer();

        let header = b"%!PS-Adobe";
        assert_eq!(&buffer[..header.len()], header);
    }

    #[test]
    fn custom_writer() {
        struct CustomWriter(usize);

        impl io::Write for CustomWriter {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                self.0 += buf.len();
                Ok(buf.len())
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let custom_writer = CustomWriter(0);

        let surface = PsSurface::for_stream(20., 20., custom_writer).unwrap();
        surface.set_size(100., 100.);
        draw(&surface);
        let stream = surface.finish_output_stream().unwrap();
        let custom_writer = stream.downcast::<CustomWriter>().unwrap();

        let buffer = draw_in_buffer();

        assert_eq!(custom_writer.0, buffer.len());
    }

    fn with_panicky_stream() -> PsSurface {
        struct PanicWriter;

        impl io::Write for PanicWriter {
            fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
                panic!("panic in writer");
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let surface = PsSurface::for_stream(20., 20., PanicWriter).unwrap();
        surface.finish();
        surface
    }

    #[test]
    #[should_panic]
    fn finish_stream_propagates_panic() {
        let _ = with_panicky_stream().finish_output_stream();
    }
}

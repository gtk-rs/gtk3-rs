// Copyright 2018-2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::mem;
use std::ffi::{CStr, CString};
use std::ops::Deref;
use std::path::Path;
use std::io;
use std::fmt;

use ffi;
use ::enums::PsLevel;
use surface::Surface;

#[cfg(feature = "use_glib")]
use glib::translate::*;

impl PsLevel {
    pub fn as_str(self) -> Option<&'static str> {
        unsafe {
            let res = ffi::cairo_ps_level_to_string(self.into());
            res.as_ref().and_then(|cstr| CStr::from_ptr(cstr as _).to_str().ok())
        }
    }
}

#[derive(Debug)]
pub struct PsSurface {
    inner: Surface,
}

#[cfg(feature = "use_glib")]
impl<'a> ToGlibPtr<'a, *mut ffi::cairo_surface_t> for PsSurface {
    type Storage = &'a Surface;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::cairo_surface_t, Self> {
        let stash = self.inner.to_glib_none();
        Stash(stash.0, stash.1)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrNone<*mut ffi::cairo_surface_t> for PsSurface {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::cairo_surface_t) -> PsSurface {
        PsSurface { inner: from_glib_none(ptr) }
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrBorrow<*mut ffi::cairo_surface_t> for PsSurface {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::cairo_surface_t) -> PsSurface {
        PsSurface { inner: from_glib_borrow(ptr) }
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrFull<*mut ffi::cairo_surface_t> for PsSurface {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::cairo_surface_t) -> PsSurface {
        PsSurface { inner: Surface::from_raw_full(ptr) }
    }
}

impl PsSurface {
    pub fn new<P: AsRef<Path>>(width: f64, height: f64, path: P) -> Self {
        let path = path.as_ref().to_string_lossy().into_owned();
        let path = CString::new(path).unwrap();

        unsafe {
            Self {
                inner: Surface::from_raw_full(
                    ffi::cairo_ps_surface_create(path.as_ptr(), width, height)
                ),
            }
        }
    }

    for_stream_constructors!(cairo_ps_surface_create_for_stream);

    pub fn get_levels() -> impl Iterator<Item=PsLevel> {
        let lvls_slice = unsafe {
            let mut vers_ptr: *mut ffi::cairo_ps_level_t = mem::uninitialized();
            let mut num_vers = 0;
            ffi::cairo_ps_get_levels(&mut vers_ptr as _, &mut num_vers as _);

            std::slice::from_raw_parts(vers_ptr, num_vers as _)
        };

        lvls_slice.iter().map(|v| PsLevel::from(*v))
    }

    pub fn restrict(&self, level: PsLevel) {
        unsafe {
            ffi::cairo_ps_surface_restrict_to_level(self.inner.to_raw_none(), level.into());
        }
    }

    pub fn get_eps(&self) -> bool {
        unsafe {
            ffi::cairo_ps_surface_get_eps(self.inner.to_raw_none()).as_bool()
        }
    }

    pub fn set_eps(&self, eps: bool) {
        unsafe {
            ffi::cairo_ps_surface_set_eps(self.inner.to_raw_none(), eps.into());
        }
    }

    pub fn set_size(&self, width: f64, height: f64) {
        unsafe {
            ffi::cairo_ps_surface_set_size(self.inner.to_raw_none(), width, height);
        }
    }

    pub fn cairo_ps_surface_dsc_begin_setup(&self) {
        unsafe {
            ffi::cairo_ps_surface_dsc_begin_setup(self.inner.to_raw_none());
        }
    }

    pub fn cairo_ps_surface_dsc_begin_page_setup(&self) {
        unsafe {
            ffi::cairo_ps_surface_dsc_begin_page_setup(self.inner.to_raw_none());
        }
    }

    pub fn cairo_ps_surface_dsc_comment(&self, comment: &str) {
        let comment = CString::new(comment).unwrap();
        unsafe {
            ffi::cairo_ps_surface_dsc_comment(self.inner.to_raw_none(), comment.as_ptr());
        }
    }

}

impl Deref for PsSurface {
    type Target = Surface;

    fn deref(&self) -> &Surface {
        &self.inner
    }
}

impl fmt::Display for PsSurface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PsSurface")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use context::*;
    use tempfile::tempfile;

    fn draw(surface: &Surface) {
        let cr = Context::new(surface);

        // Note: Not using RGBA here as PS doesn't natively support
        // semi-transparency and Cairo would then embed a rasterized bitmap

        cr.set_line_width(25.0);

        cr.set_source_rgb(1.0, 0.0, 0.0);
        cr.line_to(0.,0.);
        cr.line_to(100.,100.);
        cr.stroke();

        cr.set_source_rgb(0.0, 0.0, 1.0);
        cr.line_to(0.,100.);
        cr.line_to(100.,0.);
        cr.stroke();
    }

    fn draw_in_buffer() -> Vec<u8> {
        let buffer: Vec<u8> = vec![];

        let surface = PsSurface::for_stream(100., 100., buffer);
        draw(&surface);
        surface.finish();
        *surface.take_output_stream().unwrap().downcast().unwrap()
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
        let surface = PsSurface::for_stream(100., 100., buffer);
        surface.set_eps(true);
        assert_eq!(surface.get_eps(), true);
    }

    #[test]
    #[cfg(unix)]
    fn file() {
        let surface = PsSurface::new(100., 100., "/dev/null");
        draw(&surface);
        surface.finish();
    }

    #[test]
    fn writer() {
        let file = tempfile().expect("tempfile failed");
        let surface = PsSurface::for_stream(100., 100., file);

        draw(&surface);
        surface.finish();
        let file = surface.take_output_stream().unwrap().downcast::<std::fs::File>().unwrap();

        let buffer = draw_in_buffer();
        let file_size = file.metadata().unwrap().len();
        assert_eq!(file_size, buffer.len() as u64);
    }

    #[test]
    fn ref_writer() {
        let mut file = tempfile().expect("tempfile failed");
        let surface = unsafe { PsSurface::for_raw_stream(100., 100., &mut file) };

        draw(&surface);
        surface.finish();
        surface.take_output_stream();
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

            fn flush(&mut self) -> io::Result<()> { Ok(()) }
        }

        let custom_writer = CustomWriter(0);

        let surface = PsSurface::for_stream(20., 20., custom_writer);
        surface.set_size(100., 100.);
        draw(&surface);
        surface.finish();
        let custom_writer = surface.take_output_stream().unwrap().downcast::<CustomWriter>().unwrap();

        let buffer = draw_in_buffer();

        assert_eq!(custom_writer.0, buffer.len());
    }

    fn with_panicky_stream() -> PsSurface {
        struct PanicWriter;

        impl io::Write for PanicWriter {
            fn write(&mut self, _buf: &[u8]) -> io::Result<usize> { panic!("panic in writer"); }
            fn flush(&mut self) -> io::Result<()> { Ok(()) }
        }

        let surface = PsSurface::for_stream(20., 20., PanicWriter);
        surface.finish();
        surface
    }

    #[test]
    #[should_panic]
    fn borrow_stream_propagates_panic() {
        let surface = with_panicky_stream();
        let _ = surface.borrow_output_stream();
    }

    #[test]
    #[should_panic]
    fn borrow_error_propagates_panic() {
        let surface = with_panicky_stream();
        let _ = surface.borrow_io_error();
    }

    #[test]
    #[should_panic]
    fn take_stream_propagates_panic() {
        let _ = with_panicky_stream().take_output_stream();
    }

    #[test]
    #[should_panic]
    fn take_error_propagates_panic() {
        let _ = with_panicky_stream().take_io_error();
    }
}

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
use ::enums::SvgVersion;
#[cfg(any(all(feature = "svg", feature = "v1_16"), feature = "dox"))]
use ::enums::SvgUnit;
use surface::Surface;
use support::{self, FromRawSurface};

#[cfg(feature = "use_glib")]
use glib::translate::*;


pub fn get_versions() -> Vec<SvgVersion> {
    let vers_slice = unsafe {
        let mut vers_ptr: *mut ffi::cairo_svg_version_t = mem::uninitialized();
        let mut num_vers = 0;
        ffi::cairo_svg_get_versions(&mut vers_ptr as _, &mut num_vers as _);

        std::slice::from_raw_parts(vers_ptr, num_vers as _)
    };

    vers_slice.iter().map(|v| SvgVersion::from(*v)).collect()
}

pub fn version_to_string(version: SvgVersion) -> Option<&'static str> {
    unsafe {
        let res = ffi::cairo_svg_version_to_string(version.into());
        res.as_ref().and_then(|cstr| CStr::from_ptr(cstr as _).to_str().ok())
    }
}

#[derive(Debug)]
pub struct File {
    inner: Surface,
}

impl FromRawSurface for File {
    unsafe fn from_raw_surface(surface: *mut ffi::cairo_surface_t) -> File {
        File { inner: Surface::from_raw_full(surface) }
    }
}

impl File {
    pub fn new<P: AsRef<Path>>(width: f64, height: f64, path: P) -> File {
        let path = path.as_ref().to_string_lossy().into_owned();
        let path = CString::new(path).unwrap();

        unsafe {
            Self::from_raw_surface(ffi::cairo_svg_surface_create(path.as_ptr(), width, height))
        }
    }

    pub fn restrict(&self, version: SvgVersion) {
        unsafe {
            ffi::cairo_svg_surface_restrict_to_version(self.inner.to_raw_none(), version.into());
        }
    }

    #[cfg(any(all(feature = "svg", feature = "v1_16"), feature = "dox"))]
    pub fn set_document_unit(&mut self, unit: SvgUnit) {
        unsafe {
            ffi::cairo_svg_surface_set_document_unit(self.inner.to_raw_none(), unit.into());
        }
    }

    #[cfg(any(all(feature = "svg", feature = "v1_16"), feature = "dox"))]
    pub fn get_document_unit(&self) -> SvgUnit {
        unsafe {
            SvgUnit::from(ffi::cairo_svg_surface_get_document_unit(self.inner.to_raw_none()))
        }
    }
}

impl Deref for File {
    type Target = Surface;

    fn deref(&self) -> &Surface {
        &self.inner
    }
}

#[cfg(feature = "use_glib")]
impl<'a> ToGlibPtr<'a, *mut ffi::cairo_surface_t> for File {
    type Storage = &'a Surface;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::cairo_surface_t, Self> {
        let stash = self.inner.to_glib_none();
        Stash(stash.0, stash.1)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrNone<*mut ffi::cairo_surface_t> for File {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::cairo_surface_t) -> File {
        File { inner: from_glib_borrow(ptr) }
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrBorrow<*mut ffi::cairo_surface_t> for File {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::cairo_surface_t) -> File {
        File { inner: from_glib_borrow(ptr) }
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrFull<*mut ffi::cairo_surface_t> for File {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::cairo_surface_t) -> File {
        Self::from_raw_surface(ptr)
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "svg::File")
    }
}


#[derive(Debug)]
pub struct Writer<W: io::Write> {
    writer: support::Writer<File, W>,
}

impl<W: io::Write> Writer<W> {
    pub fn new(width: f64, height: f64, writer: W) -> Writer<W> {
        let writer = support::Writer::new(ffi::cairo_svg_surface_create_for_stream,
            width, height, writer);

        Writer { writer }
    }

    pub fn writer(&self) -> &W { self.writer.writer() }
    pub fn writer_mut(&mut self) -> &mut W { self.writer.writer_mut() }

    pub fn finish(self) -> W {
        self.writer.finish()
    }
}

impl<W: io::Write> Deref for Writer<W> {
    type Target = File;

    fn deref(&self) -> &File {
        &self.writer.surface
    }
}

#[cfg(feature = "use_glib")]
impl<'a, W: io::Write> ToGlibPtr<'a, *mut ffi::cairo_surface_t> for Writer<W> {
    type Storage = &'a Surface;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::cairo_surface_t, Self> {
        let stash = self.writer.surface.to_glib_none();
        Stash(stash.0, stash.1)
    }
}

impl<W: io::Write> fmt::Display for Writer<W> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "svg::Writer")
    }
}


#[derive(Debug)]
pub struct RefWriter<'w, W: io::Write + 'w> {
    writer: support::RefWriter<'w, File, W>,
}

impl<'w, W: io::Write + 'w> RefWriter<'w, W> {
    pub fn new(width: f64, height: f64, writer: &'w mut W) -> RefWriter<'w, W> {
        let writer = support::RefWriter::new(ffi::cairo_svg_surface_create_for_stream,
            width, height, writer);

        RefWriter { writer }
    }
}

impl<'w, W: io::Write + 'w> Deref for RefWriter<'w, W> {
    type Target = File;

    fn deref(&self) -> &File {
        &self.writer.surface
    }
}

#[cfg(feature = "use_glib")]
impl<'a, 'w, W: io::Write + 'w> ToGlibPtr<'a, *mut ffi::cairo_surface_t> for RefWriter<'w, W> {
    type Storage = &'a Surface;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::cairo_surface_t, Self> {
        let stash = self.writer.surface.to_glib_none();
        Stash(stash.0, stash.1)
    }
}

impl<'w, W: io::Write + 'w> fmt::Display for RefWriter<'w, W> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "svg::RefWriter")
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use context::*;
    use tempfile::tempfile;

    fn draw(surface: &Surface) {
        let cr = Context::new(surface);

        cr.set_line_width(25.0);

        cr.set_source_rgba(1.0, 0.0, 0.0, 0.5);
        cr.line_to(0.,0.);
        cr.line_to(100.,100.);
        cr.stroke();

        cr.set_source_rgba(0.0, 0.0, 1.0, 0.5);
        cr.line_to(0.,100.);
        cr.line_to(100.,0.);
        cr.stroke();
    }

    fn draw_in_buffer() -> Vec<u8> {
        let buffer: Vec<u8> = vec![];

        let surface = Writer::new(100., 100., buffer);
        draw(&surface);
        surface.finish()
    }

    fn assert_len_close_enough(len_a: usize, len_b: usize) {
        // It seems cairo randomizies some element IDs which might make one svg slightly
        // larger than the other. Here we make sure the difference is within ~10%.
        let len_diff = (len_a as isize - len_b as isize).abs() as usize;
        assert!(len_diff < len_b / 10);
    }

    #[test]
    fn versions() {
        let vers = get_versions();
        assert!(vers.iter().any(|v| *v == SvgVersion::_1_1));
    }

    #[test]
    fn version_string() {
        let ver_str = version_to_string(SvgVersion::_1_1).unwrap();
        assert_eq!(ver_str, "SVG 1.1");
    }

    #[test]
    #[cfg(unix)]
    fn file() {
        let surface = File::new(100., 100., "/dev/null");
        draw(&surface);
        surface.finish();
    }

    #[test]
    fn writer() {
        let file = tempfile().expect("tempfile failed");
        let surface = Writer::new(100., 100., file);

        draw(&surface);
        let file = surface.finish();

        let buffer = draw_in_buffer();
        let file_size = file.metadata().unwrap().len();

        assert_len_close_enough(file_size as usize, buffer.len());
    }

    #[test]
    fn ref_writer() {
        let mut file = tempfile().expect("tempfile failed");
        let surface = RefWriter::new(100., 100., &mut file);

        draw(&surface);
        surface.finish();
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
                self.1.write(buf) ?;

                self.0 += buf.len();
                Ok(buf.len())
            }

            fn flush(&mut self) -> io::Result<()> { Ok(()) }
        }

        let file = tempfile().expect("tempfile failed");
        let custom_writer = CustomWriter(0, file);

        let surface = Writer::new(100., 100., custom_writer);
        draw(&surface);
        let custom_writer = surface.finish();

        let buffer = draw_in_buffer();

        assert_len_close_enough(custom_writer.0, buffer.len());
    }
}

// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::mem;
use std::ffi::{CStr, CString};
use std::ops::Deref;
use std::path::Path;
use std::io;

use ffi;
use ::enums::PsLevel;
use surface::Surface;
use support::{self, FromRawSurface};

#[cfg(feature = "use_glib")]
use glib::translate::*;


pub fn get_levels() -> Vec<PsLevel> {
    let lvls_slice = unsafe {
        let mut vers_ptr: *mut ffi::cairo_ps_level_t = mem::uninitialized();
        let mut num_vers = 0;
        ffi::cairo_ps_get_levels(&mut vers_ptr as _, &mut num_vers as _);

        std::slice::from_raw_parts(vers_ptr, num_vers as _)
    };

    lvls_slice.iter().map(|v| PsLevel::from(*v)).collect()
}

pub fn level_to_string(level: PsLevel) -> Option<String> {
    unsafe {
        let res = ffi::cairo_ps_level_to_string(level.into());
        res.as_ref().and_then(|cstr| CStr::from_ptr(cstr as _).to_str().ok()).map(String::from)
    }
}

pub struct File {
    inner: Surface,
}

impl FromRawSurface for File {
    unsafe fn from_raw_surface(surface: *mut ffi::cairo_surface_t) -> File {
        File { inner: Surface::from_raw_full(surface) }
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
        File { inner: from_glib_none(ptr) }
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

impl File {
    pub fn new<P: AsRef<Path>>(width: f64, height: f64, path: P) -> File {
        let path = path.as_ref().to_string_lossy().into_owned();
        let path = CString::new(path).unwrap();

        unsafe {
            Self::from_raw_surface(ffi::cairo_ps_surface_create(path.as_ptr(), width, height))
        }
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
        unsafe {
            ffi::cairo_ps_surface_dsc_comment(self.inner.to_raw_none(), comment.as_ptr() as _);
        }
    }

}

impl AsRef<Surface> for File {
    fn as_ref(&self) -> &Surface {
        &self.inner
    }
}

impl Deref for File {
    type Target = Surface;

    fn deref(&self) -> &Surface {
        &self.inner
    }
}

impl AsRef<File> for File {
    // This is included in order to be able to be generic over PS surfaces
    fn as_ref(&self) -> &File { self }
}


pub struct Writer<W: io::Write> {
    writer: support::Writer<File, W>,
}

impl<W: io::Write> Writer<W> {
    pub fn new(width: f64, height: f64, writer: W) -> Writer<W> {
        let writer = support::Writer::new(ffi::cairo_ps_surface_create_for_stream,
            width, height, writer);

        Writer { writer }
    }

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

impl<W: io::Write> AsRef<File> for Writer<W> {
    fn as_ref(&self) -> &File {
        &self.writer.surface
    }
}

impl<W: io::Write> AsRef<Surface> for Writer<W> {
    fn as_ref(&self) -> &Surface {
        &self.writer.surface.as_ref()
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


pub struct RefWriter<'w, W: io::Write + 'w> {
    writer: support::RefWriter<'w, File, W>,
}

impl<'w, W: io::Write + 'w> RefWriter<'w, W> {
    pub fn new(width: f64, height: f64, writer: &'w mut W) -> RefWriter<'w, W> {
        let writer = support::RefWriter::new(ffi::cairo_ps_surface_create_for_stream,
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

impl<'w, W: io::Write + 'w> AsRef<File> for RefWriter<'w, W> {
    fn as_ref(&self) -> &File {
        &self.writer.surface
    }
}

impl<'w, W: io::Write + 'w> AsRef<Surface> for RefWriter<'w, W> {
    fn as_ref(&self) -> &Surface {
        &self.writer.surface.as_ref()
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


#[cfg(test)]
mod test {
    use super::*;
    use surface::SurfaceExt;
    use context::*;
    use tempfile::tempfile;

    fn draw<T: AsRef<File>>(surface: &T) {
        let cr = Context::new(surface.as_ref());

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

        let surface = Writer::new(100., 100., buffer);
        draw(&surface);
        surface.finish()
    }

    #[test]
    fn levels() {
        let vers = get_levels();
        assert!(vers.iter().any(|v| *v == PsLevel::_2));
    }

    #[test]
    fn level_string() {
        let ver_str = level_to_string(PsLevel::_2).unwrap();
        assert_eq!(ver_str, "PS Level 2");
    }

    #[test]
    fn eps() {
        let buffer: Vec<u8> = vec![];
        let surface = Writer::new(100., 100., buffer);
        surface.set_eps(true);
        assert_eq!(surface.get_eps(), true);
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
        assert_eq!(file_size, buffer.len() as u64);
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

        let surface = Writer::new(20., 20., custom_writer);
        surface.set_size(100., 100.);
        draw(&surface);
        let custom_writer = surface.finish();

        let buffer = draw_in_buffer();

        assert_eq!(custom_writer.0, buffer.len());
    }
}

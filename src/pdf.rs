// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ffi::CString;
use std::ops::Deref;
use std::path::Path;
use std::io;

use ffi;
use ::enums::{SurfaceType, PdfVersion};
use surface::{Surface, SurfaceExt};
use support;

#[cfg(feature = "use_glib")]
use glib::translate::*;

macro_rules! imp {
    () => {
        pub fn restrict(&self, version: PdfVersion) {
            unsafe {
                ffi::cairo_pdf_surface_restrict_to_version(self.inner.to_raw_none(),
                                                           version.into());
            }
        }
    }
}

pub struct File {
    inner: Surface,
}

impl File {
    #[doc(hidden)]
    pub fn from(surface: Surface) -> Result<File, Surface> {
        if surface.get_type() == SurfaceType::Pdf {
            Ok(File { inner: surface })
        } else {
            Err(surface)
        }
    }

    #[doc(hidden)]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_surface_t) -> File {
        Self::from(Surface::from_raw_full(ptr)).unwrap()
    }

    pub fn new<P: AsRef<Path>>(width: f64, height: f64, path: P) -> File {
        let path = path.as_ref().to_string_lossy().into_owned();
        let path = CString::new(path).unwrap();

        unsafe {
            Self::from_raw_full(ffi::cairo_pdf_surface_create(path.as_ptr(), width, height))
        }
    }

    imp!();
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
        Self::from(from_glib_none(ptr)).unwrap()
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrBorrow<*mut ffi::cairo_surface_t> for File {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::cairo_surface_t) -> File {
        Self::from(from_glib_borrow(ptr)).unwrap()
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrFull<*mut ffi::cairo_surface_t> for File {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::cairo_surface_t) -> File {
        Self::from_raw_full(ptr)
    }
}

pub struct Buffer {
    inner: Surface,
    #[allow(unused)]
    support: support::Buffer,
}

impl Buffer {
    pub fn new(width: f64, height: f64) -> Buffer {
        let support = support::Buffer::new(ffi::cairo_pdf_surface_create_for_stream,
            width, height);

        Buffer {
            inner: unsafe { Surface::from_raw_full(support.as_ptr()) },
            support: support,
        }
    }

    imp!();
}

impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        self.support.as_ref()
    }
}

impl AsRef<Surface> for Buffer {
    fn as_ref(&self) -> &Surface {
        &self.inner
    }
}

impl Deref for Buffer {
    type Target = Surface;

    fn deref(&self) -> &Surface {
        &self.inner
    }
}

#[cfg(feature = "use_glib")]
impl<'a> ToGlibPtr<'a, *mut ffi::cairo_surface_t> for Buffer {
    type Storage = &'a Surface;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::cairo_surface_t, Self> {
        let stash = self.inner.to_glib_none();
        Stash(stash.0, stash.1)
    }
}

pub struct Writer<'a> {
    inner: Surface,
    #[allow(unused)]
    support: support::Writer<'a>,
}

impl<'a> Writer<'a> {
    pub fn new<'b, W: 'b + io::Write>(width: f64, height: f64, w: W) -> Writer<'b> {
        let support = support::Writer::new(ffi::cairo_pdf_surface_create_for_stream,
            width, height, w);

        Writer {
            inner: unsafe { Surface::from_raw_full(support.as_ptr()) },
            support: support,
        }
    }

    imp!();
}

impl<'a> AsRef<Surface> for Writer<'a> {
    fn as_ref(&self) -> &Surface {
        &self.inner
    }
}

impl<'a> Deref for Writer<'a> {
    type Target = Surface;

    fn deref(&self) -> &Surface {
        &self.inner
    }
}


#[cfg(feature = "use_glib")]
impl<'a> ToGlibPtr<'a, *mut ffi::cairo_surface_t> for Writer<'a> {
    type Storage = &'a Surface;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::cairo_surface_t, Self> {
        let stash = self.inner.to_glib_none();
        Stash(stash.0, stash.1)
    }
}

pub struct Stream<'a> {
    inner: Surface,
    #[allow(unused)]
    support: support::Stream<'a>,
}

impl<'a> Stream<'a> {
    pub fn new<'b, F>(width: f64, height: f64, func: F) -> Stream<'b>
        where F: 'b + FnMut(&[u8]) -> Result<(), ()>
    {
        let support = support::Stream::new(ffi::cairo_pdf_surface_create_for_stream,
            width, height, func);

        Stream {
            inner: unsafe { Surface::from_raw_full(support.as_ptr()) },
            support: support,
        }
    }

    imp!();
}

impl<'a> AsRef<Surface> for Stream<'a> {
    fn as_ref(&self) -> &Surface {
        &self.inner
    }
}

impl<'a> Deref for Stream<'a> {
    type Target = Surface;

    fn deref(&self) -> &Surface {
        &self.inner
    }
}

#[cfg(feature = "use_glib")]
impl<'a> ToGlibPtr<'a, *mut ffi::cairo_surface_t> for Stream<'a> {
    type Storage = &'a Surface;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::cairo_surface_t, Self> {
        let stash = self.inner.to_glib_none();
        Stash(stash.0, stash.1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use surface::Surface;
    use context::*;

    fn draw<T: AsRef<Surface>>(surface: &T) {
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

    #[test]
    fn buffer() {
        let surface = Buffer::new(100., 100.);
        draw(&surface);
        surface.finish();
    }

    #[test]
    fn writer() {
        let filename = "test_x.pdf";
        let file = ::std::fs::File::create(filename).unwrap();
        let surface = Writer::new(100., 100., file);

        draw(&surface);
        surface.finish();
        ::std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn stream() {
        let mut vec = Vec::<u8>::new();
        let surface = Stream::new(100., 100., |data| {
            vec.extend(data);
            Ok(())
        });

        draw(&surface);
        surface.finish();
    }
}

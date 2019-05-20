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
#[cfg(any(all(feature = "pdf", feature = "v1_16"), feature = "dox"))]
use ::enums::{PdfOutline, PdfMetadata};
use ::enums::PdfVersion;
use surface::Surface;

#[cfg(feature = "use_glib")]
use glib::translate::*;


impl PdfVersion {
    pub fn as_str(self) -> Option<&'static str> {
        unsafe {
            let res = ffi::cairo_pdf_version_to_string(self.into());
            res.as_ref().and_then(|cstr| CStr::from_ptr(cstr as _).to_str().ok())
        }
    }
}

#[derive(Debug)]
pub struct PdfSurface {
    inner: Surface,
}

impl PdfSurface {
    pub fn new<P: AsRef<Path>>(width: f64, height: f64, path: P) -> Self {
        let path = path.as_ref().to_string_lossy().into_owned();
        let path = CString::new(path).unwrap();

        unsafe {
            let raw = ffi::cairo_pdf_surface_create(path.as_ptr(), width, height);
            Self {
                inner: Surface::from_raw_full(raw)
            }
        }
    }

    /// Takes full ownership of the output stream,
    /// which is not allowed to borrow any lifetime shorter than `'static`.
    ///
    /// Because the underlying `cairo_surface_t` is reference-counted,
    /// a lifetime parameter in a Rust wrapper type would not be enough to track
    /// how long it can keep writing to the stream.
    pub fn for_stream<W: io::Write + 'static>(width: f64, height: f64, stream: W) -> Self {
        Self {
            inner: Surface::_for_stream(
                ffi::cairo_pdf_surface_create_for_stream,
                width,
                height,
                stream,
            ),
        }
    }

    /// Allows writing to a borrowed stream. The lifetime of the borrow is not tracked.
    ///
    /// # Safety
    ///
    /// The value that `stream` points to must live at least until the underlying `cairo_surface_t`
    /// (which maybe be longer then the Rust `PdfSurface` wrapper, because of reference-counting),
    /// or until the output stream is removed from the surface with [`Surface::take_output_stream`].
    ///
    /// Since the former is hard to track for sure, the latter is strongly recommended.
    /// The concrete type behind the `Box<dyn Any>` value returned by `take_output_stream`
    /// is private, so you won’t be able to downcast it.
    /// But removing it anyway ensures that later writes do no go through a dangling pointer.
    pub unsafe fn for_raw_stream<W: io::Write + 'static>(width: f64, height: f64, stream: *mut W) -> Self {
        Self {
            inner: Surface::_for_raw_stream(
                ffi::cairo_pdf_surface_create_for_stream,
                width,
                height,
                stream,
            ),
        }
    }

    pub fn get_versions() -> impl Iterator<Item=PdfVersion> {
        let vers_slice = unsafe {
            let mut vers_ptr: *mut ffi::cairo_pdf_version_t = mem::uninitialized();
            let mut num_vers = 0;
            ffi::cairo_pdf_get_versions(&mut vers_ptr as _, &mut num_vers as _);

            std::slice::from_raw_parts(vers_ptr, num_vers as _)
        };
        vers_slice.iter().map(|v| PdfVersion::from(*v))
    }

    pub fn restrict(&self, version: PdfVersion) {
        unsafe {
            ffi::cairo_pdf_surface_restrict_to_version(self.inner.to_raw_none(),
                                                       version.into());
        }
    }

    pub fn set_size(&self, width: f64, height: f64) {
        unsafe {
            ffi::cairo_pdf_surface_set_size(self.inner.to_raw_none(), width, height);
        }
    }

    #[cfg(any(all(feature = "pdf", feature = "v1_16"), feature = "dox"))]
    pub fn set_metadata(&self, metadata: PdfMetadata, value: &str) {
        let value = CString::new(value).unwrap();
        unsafe {
            ffi::cairo_pdf_surface_set_metadata(self.inner.to_raw_none(), metadata.into(), value.as_ptr());
        }
    }

    #[cfg(any(all(feature = "pdf", feature = "v1_16"), feature = "dox"))]
    pub fn set_page_label(&self, label: &str) {
        let label = CString::new(label).unwrap();
        unsafe {
            ffi::cairo_pdf_surface_set_page_label(self.inner.to_raw_none(), label.as_ptr());
        }
    }

    #[cfg(any(all(feature = "pdf", feature = "v1_16"), feature = "dox"))]
    pub fn set_thumbnail_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::cairo_pdf_surface_set_thumbnail_size(self.inner.to_raw_none(), width as _, height as _);
        }
    }

    #[cfg(any(all(feature = "pdf", feature = "v1_16"), feature = "dox"))]
    pub fn add_outline(&self, parent_id: i32, name: &str, link_attribs: &str, flags: PdfOutline) -> i32 {
        let name = CString::new(name).unwrap();
        let link_attribs = CString::new(link_attribs).unwrap();

        unsafe {
            ffi::cairo_pdf_surface_add_outline(
                self.inner.to_raw_none(),
                parent_id,
                name.as_ptr(),
                link_attribs.as_ptr(),
                flags.bits() as _
            ) as _
        }
    }
}

impl Deref for PdfSurface {
    type Target = Surface;

    fn deref(&self) -> &Surface {
        &self.inner
    }
}

#[cfg(feature = "use_glib")]
impl<'a> ToGlibPtr<'a, *mut ffi::cairo_surface_t> for PdfSurface {
    type Storage = &'a Surface;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::cairo_surface_t, Self> {
        let stash = self.inner.to_glib_none();
        Stash(stash.0, stash.1)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrNone<*mut ffi::cairo_surface_t> for PdfSurface {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::cairo_surface_t) -> PdfSurface {
        PdfSurface { inner: from_glib_none(ptr) }
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrBorrow<*mut ffi::cairo_surface_t> for PdfSurface {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::cairo_surface_t) -> PdfSurface {
        PdfSurface { inner: from_glib_borrow(ptr) }
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrFull<*mut ffi::cairo_surface_t> for PdfSurface {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::cairo_surface_t) -> PdfSurface {
        Self { inner: Surface::from_raw_full(ptr) }
    }
}

impl fmt::Display for PdfSurface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PdfSurface")
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

        let surface = PdfSurface::for_stream(100., 100., buffer);
        draw(&surface);
        surface.finish();
        *surface.take_output_stream().unwrap().downcast().unwrap()
    }

    #[test]
    fn versions() {
        assert!(PdfSurface::get_versions().any(|v| v == PdfVersion::_1_4));
    }

    #[test]
    fn version_string() {
        let ver_str = PdfVersion::_1_4.as_str().unwrap();
        assert_eq!(ver_str, "PDF 1.4");
    }

    #[test]
    #[cfg(unix)]
    fn file() {
        let surface = PdfSurface::new(100., 100., "/dev/null");
        draw(&surface);
        surface.finish();
    }

    #[test]
    fn writer() {
        let file = tempfile().expect("tempfile failed");
        let surface = PdfSurface::for_stream(100., 100., file);

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
        let surface = unsafe {
            PdfSurface::for_raw_stream(100., 100., &mut file)
        };

        draw(&surface);
        surface.finish();
        surface.take_output_stream();
        drop(file);
    }

    #[test]
    fn buffer() {
        let buffer = draw_in_buffer();

        let header = b"%PDF-1.5";
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

        let surface = PdfSurface::for_stream(20., 20., custom_writer);
        surface.set_size(100., 100.);
        draw(&surface);
        surface.finish();
        let custom_writer = surface.take_output_stream().unwrap().downcast::<CustomWriter>().unwrap();

        let buffer = draw_in_buffer();

        assert_eq!(custom_writer.0, buffer.len());
    }
}

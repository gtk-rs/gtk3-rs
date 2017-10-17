// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ffi::CString;
use std::ops::Deref;
use std::path::Path;

extern crate libc;
use libc::{c_char, c_double};

use ffi;
use ffi::enums::SurfaceType;

use surface::{Surface, SurfaceExt};

#[cfg(feature = "use_glib")]
use glib::translate::*;

pub struct PDFSurface(Surface);

extern "C" {
    pub fn cairo_pdf_surface_create (filename: *const c_char,
                                     width_in_points: c_double,
                                     height_in_points: c_double) -> *mut ffi::cairo_surface_t;
}

impl PDFSurface {
    pub fn from(surface: Surface) -> Result<PDFSurface, Surface> {
        if surface.get_type() == SurfaceType::Pdf {
            Ok(PDFSurface(surface))
        } else {
            Err(surface)
        }
    }

    #[doc(hidden)]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_surface_t) -> PDFSurface {
        Self::from(Surface::from_raw_full(ptr)).unwrap()
    }

    pub fn create<T: AsRef<Path>>(filename: T, width: f64, height: f64) -> PDFSurface
    {
        // Convert: AsRef<Path> -> Cow<str> -> str
        let s = filename.as_ref().to_string_lossy().into_owned();
        let file = CString::new(s).unwrap();
        unsafe { Self::from_raw_full(cairo_pdf_surface_create(file.as_ptr(), width, height)) }
    }
}

impl AsRef<Surface> for PDFSurface {
    fn as_ref(&self) -> &Surface {
        &self.0
    }
}

impl Deref for PDFSurface {
    type Target = Surface;

    fn deref(&self) -> &Surface {
        &self.0
    }
}

impl Clone for PDFSurface {
    fn clone(&self) -> PDFSurface {
        PDFSurface(self.0.clone())
    }
}

unsafe impl Send for PDFSurface {}

#[cfg(feature = "use_glib")]
impl<'a> ToGlibPtr<'a, *mut ffi::cairo_surface_t> for PDFSurface {
    type Storage = &'a Surface;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::cairo_surface_t, Self> {
        let stash = self.0.to_glib_none();
        Stash(stash.0, stash.1)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrNone<*mut ffi::cairo_surface_t> for PDFSurface {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::cairo_surface_t) -> PDFSurface {
        Self::from(from_glib_none(ptr)).unwrap()
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrBorrow<*mut ffi::cairo_surface_t> for PDFSurface {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::cairo_surface_t) -> PDFSurface {
        Self::from(from_glib_borrow(ptr)).unwrap()
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrFull<*mut ffi::cairo_surface_t> for PDFSurface {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::cairo_surface_t) -> PDFSurface {
        Self::from_raw_full(ptr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use context::*;
    use std::path::PathBuf;

    fn draw_x(cr: &Context) {
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
    fn pdf() {
        let output = PathBuf::from( env!("OUT_DIR") );

        { //Cow<str>
            let filename = output.join("test1.pdf");
            let surface = PDFSurface::create(filename, 100., 100.);
            let cr = Context::new(&surface);
            draw_x(&cr);
        }
        { // &str
            let filename = output.join("test2.pdf");
            let s = match filename.to_str() {
                None => panic!("Error converting Path to String"),
                Some(x) => x,
            };
            let surface = PDFSurface::create(s, 100., 100.);
            let cr = Context::new(&surface);
            draw_x(&cr);
        }
        { // String
            let filename = output.join("test3.pdf");
            let s = match filename.to_str() {
                None => panic!("Error converting Path to String"),
                Some(x) => String::from(x),
            };
            let surface = PDFSurface::create(s, 100., 100.);
            let cr = Context::new(&surface);
            draw_x(&cr);
        }
    }
}

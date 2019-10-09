// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;

use enums::{Content, Status, SurfaceType};
use ffi;
#[cfg(feature = "use_glib")]
use glib::translate::*;
use rectangle::Rectangle;

use surface::Surface;

#[derive(Debug)]
pub struct RecordingSurface(Surface);

impl TryFrom<Surface> for RecordingSurface {
    type Error = Surface;

    fn try_from(surface: Surface) -> Result<RecordingSurface, Surface> {
        if surface.get_type() == SurfaceType::Recording {
            Ok(RecordingSurface(surface))
        } else {
            Err(surface)
        }
    }
}

impl RecordingSurface {
    pub unsafe fn from_raw_full(
        ptr: *mut ffi::cairo_surface_t,
    ) -> Result<RecordingSurface, Status> {
        let surface = Surface::from_raw_full(ptr)?;
        Self::try_from(surface).map_err(|_| Status::SurfaceTypeMismatch)
    }

    pub fn create<T: Into<Option<Rectangle>>>(
        content: Content,
        extends: T,
    ) -> Result<RecordingSurface, Status> {
        unsafe {
            let extends = extends.into();
            let extends = match extends {
                Some(c) => c.to_raw_none(),
                None => ::std::ptr::null(),
            };

            Ok(Self::from_raw_full(ffi::cairo_recording_surface_create(
                content.into(),
                extends,
            ))?)
        }
    }

    pub fn get_extents(&self) -> Option<Rectangle> {
        unsafe {
            let rectangle: Rectangle = ::std::mem::zeroed();
            if ffi::cairo_recording_surface_get_extents(self.to_raw_none(), rectangle.to_raw_none())
                .as_bool()
            {
                Some(rectangle)
            } else {
                None
            }
        }
    }

    pub fn ink_extents(&self) -> (f64, f64, f64, f64) {
        let mut x0 = 0.;
        let mut y0 = 0.;
        let mut width = 0.;
        let mut height = 0.;

        unsafe {
            ffi::cairo_recording_surface_ink_extents(
                self.to_raw_none(),
                &mut x0,
                &mut y0,
                &mut width,
                &mut height,
            );
        }
        (x0, y0, width, height)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrNone<*mut ffi::cairo_surface_t> for RecordingSurface {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::cairo_surface_t) -> RecordingSurface {
        RecordingSurface(Surface::from_glib_none(ptr))
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrBorrow<*mut ffi::cairo_surface_t> for RecordingSurface {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::cairo_surface_t) -> RecordingSurface {
        RecordingSurface(Surface::from_glib_borrow(ptr))
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrFull<*mut ffi::cairo_surface_t> for RecordingSurface {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::cairo_surface_t) -> RecordingSurface {
        Self::from_raw_full(ptr).unwrap()
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(
    RecordingSurface,
    ffi::cairo_surface_t,
    ffi::gobject::cairo_gobject_surface_get_type
);

impl Deref for RecordingSurface {
    type Target = Surface;

    fn deref(&self) -> &Surface {
        &self.0
    }
}

impl Clone for RecordingSurface {
    fn clone(&self) -> RecordingSurface {
        RecordingSurface(self.0.clone())
    }
}

impl fmt::Display for RecordingSurface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RecordingSurface")
    }
}

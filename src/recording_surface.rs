// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ops::Deref;

#[cfg(feature = "use_glib")]
use glib::translate::*;
use ffi;
use ::enums::{
    Content,
    SurfaceType,
};
use ::rectangle::Rectangle;

use surface::{Surface, SurfaceExt};

#[derive(Debug)]
pub struct RecordingSurface(Surface);

impl RecordingSurface {
    pub fn create<T: Into<Option<Rectangle>>>(
        content: Content,
        extends: T,
    ) -> Option<RecordingSurface> {
        unsafe {
            let extends = extends.into();
            let extends = match extends {
                Some(c) => c.to_raw_none(),
                None => 0 as *const _,
            };
            let p = ffi::cairo_recording_surface_create(content.into(), extends);
            if p.is_null() {
                None
            } else {
                Some(RecordingSurface(Surface::from_raw_full(p)))
            }
        }
    }

    pub fn from(surface: Surface) -> Result<RecordingSurface, Surface> {
        if surface.get_type() == SurfaceType::Recording {
            Ok(RecordingSurface(surface))
        } else {
            Err(surface)
        }
    }

    pub fn get_extents(&self) -> Option<Rectangle> {
        unsafe {
            let rectangle: Rectangle = ::std::mem::zeroed();
            if ffi::cairo_recording_surface_get_extents(self.to_raw_none(),
                                                        rectangle.to_raw_none()).as_bool() {
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
            ffi::cairo_recording_surface_ink_extents(self.to_raw_none(),
                                                     &mut x0, &mut y0,
                                                     &mut width, &mut height);
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
        RecordingSurface(Surface::from_raw_full(ptr))
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(RecordingSurface, ffi::cairo_surface_t, ffi::gobject::cairo_gobject_surface_get_type);

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

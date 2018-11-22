// Copyright 2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ops::Deref;

#[cfg(feature = "use_glib")]
use glib::translate::*;
use ffi;
use ::enums::{Format, SurfaceType};
use surface::{Surface, SurfaceExt};
use Status;

use ffi::CGContextRef;

#[derive(Debug)]
pub struct QuartzSurface(Surface);

impl QuartzSurface {
    pub fn from(surface: Surface) -> Result<QuartzSurface, Surface> {
        if surface.get_type() == SurfaceType::Quartz {
            Ok(QuartzSurface(surface))
        } else {
            Err(surface)
        }
    }

    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_surface_t) -> Result<QuartzSurface, Status> {
        let surface = Self::from(Surface::from_raw_full(ptr)).unwrap();
        let status = surface.status();
        match status {
            Status::Success => Ok(surface),
            _ => Err(status)
        }
    }

    pub fn create(format: Format, width: u32, height: u32) -> Result<QuartzSurface, Status> {
        unsafe {
            Self::from_raw_full(ffi::cairo_quartz_surface_create(format.into(), width, height))
        }
    }

    pub fn create_for_cg_context(cg_context: CGContextRef, width: u32, height: u32) -> Result<QuartzSurface, Status> {
        unsafe { Self::from_raw_full(ffi::cairo_quartz_surface_create_for_cg_context(cg_context, width, height)) }
    }

    pub fn get_cg_context(&self) -> CGContextRef {
        unsafe { ffi::cairo_quartz_surface_get_cg_context(self.to_raw_none()) }
    }
}

#[cfg(feature = "use_glib")]
impl<'a> ToGlibPtr<'a, *mut ffi::cairo_surface_t> for QuartzSurface {
    type Storage = &'a Surface;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::cairo_surface_t, Self> {
        let stash = self.0.to_glib_none();
        Stash(stash.0, stash.1)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut ffi::cairo_surface_t {
        unsafe {
            ffi::cairo_surface_reference(self.to_glib_none().0)
        }
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrNone<*mut ffi::cairo_surface_t> for QuartzSurface {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::cairo_surface_t) -> QuartzSurface {
        Self::from(from_glib_none(ptr)).unwrap()
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrBorrow<*mut ffi::cairo_surface_t> for QuartzSurface {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::cairo_surface_t) -> QuartzSurface {
        Self::from(from_glib_borrow(ptr)).unwrap()
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrFull<*mut ffi::cairo_surface_t> for QuartzSurface {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::cairo_surface_t) -> QuartzSurface {
        Self::from(from_glib_full(ptr)).unwrap()
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(QuartzSurface, ffi::cairo_surface_t, ffi::gobject::cairo_gobject_surface_get_type);

impl AsRef<Surface> for QuartzSurface {
    fn as_ref(&self) -> &Surface {
        &self.0
    }
}

impl Deref for QuartzSurface {
    type Target = Surface;

    fn deref(&self) -> &Surface {
        &self.0
    }
}

impl Clone for QuartzSurface {
    fn clone(&self) -> QuartzSurface {
        QuartzSurface(self.0.clone())
    }
}

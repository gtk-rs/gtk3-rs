// Copyright 2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate winapi;

use std::ops::Deref;

#[cfg(feature = "glib")]
use glib::translate::*;
use ffi;
use ffi::enums::{Format, SurfaceType};
use surface::{Surface, SurfaceExt};

#[derive(Debug)]
pub struct Win32Surface(Surface);

impl Win32Surface {
    pub fn from(surface: Surface) -> Result<Win32Surface, Surface> {
        if surface.get_type() == SurfaceType::Win32 {
            Ok(Win32Surface(surface))
        } else {
            Err(surface)
        }
    }

    #[doc(hidden)]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_surface_t) -> Win32Surface {
        Self::from(Surface::from_raw_full(ptr)).unwrap()
    }

    pub fn create(hdc: winapi::HDC) -> Win32Surface {
        unsafe { Self::from_raw_full(ffi::cairo_win32_surface_create(hdc)) }
    }

    pub fn create_with_dib(format: Format, width: i32, height: i32) -> Win32Surface {
        unsafe { Self::from_raw_full(ffi::cairo_win32_surface_create_with_dib(format, width, height)) }
    }

    pub fn create_with_ddb(hdc: winapi::HDC,
                           format: Format,
                           width: i32,
                           height: i32)
                           -> Win32Surface {
        unsafe {
            Self::from_raw_full(ffi::cairo_win32_surface_create_with_ddb(hdc, format, width, height))
        }
    }

    pub fn printing_surface_create(hdc: winapi::HDC) -> Win32Surface {
        unsafe { Self::from_raw_full(ffi::cairo_win32_printing_surface_create(hdc)) }
    }
}

#[cfg(feature = "glib")]
impl<'a> ToGlibPtr<'a, *mut ffi::cairo_surface_t> for Win32Surface {
    type Storage = &'a Surface;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::cairo_surface_t, Self> {
        let stash = self.0.to_glib_none();
        Stash(stash.0, stash.1)
    }
}

#[cfg(feature = "glib")]
impl FromGlibPtrNone<*mut ffi::cairo_surface_t> for Win32Surface {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::cairo_surface_t) -> Win32Surface {
        Self::from(from_glib_none(ptr)).unwrap()
    }
}

#[cfg(feature = "glib")]
impl FromGlibPtrFull<*mut ffi::cairo_surface_t> for Win32Surface {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::cairo_surface_t) -> Win32Surface {
        Self::from(from_glib_full(ptr)).unwrap()
    }
}

impl AsRef<Surface> for Win32Surface {
    fn as_ref(&self) -> &Surface {
        &self.0
    }
}

impl Deref for Win32Surface {
    type Target = Surface;

    fn deref(&self) -> &Surface {
        &self.0
    }
}

impl Clone for Win32Surface {
    fn clone(&self) -> Win32Surface {
        Win32Surface(self.0.clone())
    }
}

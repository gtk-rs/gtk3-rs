// Copyright 2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

pub use ffi::winapi;

use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;

use enums::{Format, SurfaceType};
use ffi;
#[cfg(feature = "use_glib")]
use glib::translate::*;
use surface::Surface;
use Status;

declare_surface!(Win32Surface, SurfaceType::Win32);

impl Win32Surface {
    pub fn create(hdc: winapi::HDC) -> Result<Win32Surface, Status> {
        unsafe { Self::from_raw_full(ffi::cairo_win32_surface_create(hdc)) }
    }

    #[cfg(any(all(windows, feature = "v1_14"), feature = "dox"))]
    pub fn create_with_format(hdc: winapi::HDC, format: Format) -> Result<Win32Surface, Status> {
        unsafe {
            Self::from_raw_full(ffi::cairo_win32_surface_create_with_format(
                hdc,
                format.into(),
            ))
        }
    }

    pub fn create_with_dib(
        format: Format,
        width: i32,
        height: i32,
    ) -> Result<Win32Surface, Status> {
        unsafe {
            Self::from_raw_full(ffi::cairo_win32_surface_create_with_dib(
                format.into(),
                width,
                height,
            ))
        }
    }

    pub fn create_with_ddb(
        hdc: winapi::HDC,
        format: Format,
        width: i32,
        height: i32,
    ) -> Result<Win32Surface, Status> {
        unsafe {
            Self::from_raw_full(ffi::cairo_win32_surface_create_with_ddb(
                hdc,
                format.into(),
                width,
                height,
            ))
        }
    }

    pub fn printing_surface_create(hdc: winapi::HDC) -> Result<Win32Surface, Status> {
        unsafe { Self::from_raw_full(ffi::cairo_win32_printing_surface_create(hdc)) }
    }
}

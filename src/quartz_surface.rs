// Copyright 2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;

use enums::{Format, SurfaceType};
use ffi;
#[cfg(feature = "use_glib")]
use glib::translate::*;
use surface::Surface;
use Status;

use ffi::CGContextRef;

declare_surface!(QuartzSurface, SurfaceType::Quartz);

impl QuartzSurface {
    pub fn create(format: Format, width: u32, height: u32) -> Result<QuartzSurface, Status> {
        unsafe {
            Self::from_raw_full(ffi::cairo_quartz_surface_create(
                format.into(),
                width,
                height,
            ))
        }
    }

    pub fn create_for_cg_context(
        cg_context: CGContextRef,
        width: u32,
        height: u32,
    ) -> Result<QuartzSurface, Status> {
        unsafe {
            Self::from_raw_full(ffi::cairo_quartz_surface_create_for_cg_context(
                cg_context, width, height,
            ))
        }
    }

    pub fn get_cg_context(&self) -> CGContextRef {
        unsafe { ffi::cairo_quartz_surface_get_cg_context(self.to_raw_none()) }
    }
}

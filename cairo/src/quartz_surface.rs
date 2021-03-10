// Take a look at the license at the top of the repository in the LICENSE file.

use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;

use crate::enums::{Format, SurfaceType};
use crate::error::Error;
use crate::surface::Surface;
#[cfg(feature = "use_glib")]
use glib::translate::*;

use ffi::CGContextRef;

declare_surface!(QuartzSurface, SurfaceType::Quartz);

impl QuartzSurface {
    #[doc(alias = "cairo_quartz_surface_create")]
    pub fn create(format: Format, width: u32, height: u32) -> Result<QuartzSurface, Error> {
        unsafe {
            Self::from_raw_full(ffi::cairo_quartz_surface_create(
                format.into(),
                width,
                height,
            ))
        }
    }

    #[doc(alias = "cairo_quartz_surface_create_for_cg_context")]
    pub fn create_for_cg_context(
        cg_context: CGContextRef,
        width: u32,
        height: u32,
    ) -> Result<QuartzSurface, Error> {
        unsafe {
            Self::from_raw_full(ffi::cairo_quartz_surface_create_for_cg_context(
                cg_context, width, height,
            ))
        }
    }

    #[doc(alias = "cairo_quartz_surface_get_cg_context")]
    pub fn get_cg_context(&self) -> CGContextRef {
        unsafe { ffi::cairo_quartz_surface_get_cg_context(self.to_raw_none()) }
    }
}

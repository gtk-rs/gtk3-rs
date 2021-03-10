// Take a look at the license at the top of the repository in the LICENSE file.

use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;

use crate::enums::{Content, SurfaceType};
use crate::error::Error;
use crate::rectangle::Rectangle;
#[cfg(feature = "use_glib")]
use glib::translate::*;

use crate::surface::Surface;

declare_surface!(RecordingSurface, SurfaceType::Recording);
impl RecordingSurface {
    #[doc(alias = "cairo_recording_surface_create")]
    pub fn create<T: Into<Option<Rectangle>>>(
        content: Content,
        extends: T,
    ) -> Result<RecordingSurface, Error> {
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

    #[doc(alias = "cairo_recording_surface_get_extents")]
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

    #[doc(alias = "cairo_recording_surface_ink_extents")]
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

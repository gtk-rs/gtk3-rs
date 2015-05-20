// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Functions to support using cairo

extern crate cairo;

use std::ptr;
use glib::translate::{ToGlibPtr};
use libc::{c_double};
use ffi::{self, GdkRectangle, GdkRGBA};
use super::{Pixbuf, Window};
use self::cairo::{Context};

pub fn create(window: &Window) -> Context {
    unsafe { Context::wrap(ffi::gdk_cairo_create(window.to_glib_none().0)) }
}

//pub fn create_region_from_surface() { }
//--> WRAP: gdk_cairo_region_create_from_surface (cairo_surface_t *surface);

//pub fn create_surface_from_pixbuf() { }
//--> WRAP: gdk_cairo_surface_create_from_pixbuf (const GdkPixbuf *pixbuf, int scale, GdkWindow *for_window);

pub trait GdkCairoInteraction {
    /// This is a convenience function around `clip_extents()`. It rounds
    /// the clip extents to integer coordinates and returns a `GdkRectangle`,
    /// or `None` if no clip area exists.
    fn get_clip_rectangle(&self) -> Option<GdkRectangle>;

    /// Sets the specified `GdkRGBA` as the source color of `cr`.
    fn set_source_rgba(&self, rgba: &GdkRGBA);

    /// Sets the given pixbuf as the source pattern for `cr`. The pattern has
    /// an extend mode of `ExtendNone` and is aligned so that the origin of
    /// `pixbuf` is (`x`, `y`).
    fn set_source_pixbuf(&self, pixbuf: &Pixbuf, x: f64, y: f64);

    /// Sets the given window as the source pattern for `cr`. The pattern has
    /// an extend mode of `ExtendNone` and is aligned so that the origin of
    /// window is (`x` , `y`). The window contains all its subwindows when
    /// rendering.
    ///
    /// Note that the contents of `window` are undefined outside of the visible
    /// part of `window`, so use this function with care.
    fn set_source_window(&self, window: &Window, x: f64, y: f64);

    /// Adds the given rectangle to the current path of `cr`.
    fn rectangle(&self, rectangle: &GdkRectangle);
    
    //fn add_region(&self, region: ???);
    //--> WRAP: fn gdk_cairo_region(cr: *mut cairo_t, region: *const cairo_region_t);
}

impl GdkCairoInteraction for Context {
    fn get_clip_rectangle(&self) -> Option<GdkRectangle> {
        let rectangle = ptr::null_mut();
        unsafe {
            match ffi::gdk_cairo_get_clip_rectangle(self.get_ptr(), rectangle) {
                1 => Some(*rectangle),
                _ => None
            }
        }
    }

    fn set_source_rgba(&self, rgba: &GdkRGBA) {
        unsafe { ffi::gdk_cairo_set_source_rgba(self.get_ptr(), rgba); }
    }

    fn set_source_pixbuf(&self, pixbuf: &Pixbuf, x: f64, y: f64) {
        unsafe { ffi::gdk_cairo_set_source_pixbuf(self.get_ptr(), pixbuf.to_glib_none().0, x as c_double, y as c_double); }
    }

    fn set_source_window(&self, window: &Window, x: f64, y: f64) {
        unsafe { ffi::gdk_cairo_set_source_window(self.get_ptr(), window.to_glib_none().0, x as c_double, y as c_double); }
    }

    fn rectangle(&self, rectangle: &GdkRectangle) {
        unsafe { ffi::gdk_cairo_rectangle(self.get_ptr(), rectangle); }
    }
}


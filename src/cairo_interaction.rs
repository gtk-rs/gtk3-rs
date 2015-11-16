// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Functions to support using cairo

use std::mem;
use glib::translate::*;
use ffi::{self, GdkRGBA};
use super::{Pixbuf, Window};
use cairo::{Context, RectangleInt};

//pub fn create_region_from_surface()
//--> WRAP: gdk_cairo_region_create_from_surface (cairo_surface_t *surface);

//pub fn create_surface_from_pixbuf()
//--> WRAP: gdk_cairo_surface_create_from_pixbuf (const GdkPixbuf *pixbuf, int scale, GdkWindow *for_window);

pub trait ContextExt {
    /// Creates a Cairo context for drawing to `window`.
    ///
    /// Note that calling `reset_clip()` on the resulting `Context` will
    /// produce undefined results, so avoid it at all costs.
    fn create_from_window(window: &Window) -> Context;

    /// This is a convenience function around `clip_extents()`. It rounds
    /// the clip extents to integer coordinates and returns a `RectangleInt`,
    /// or `None` if no clip area exists.
    fn get_clip_rectangle(&self) -> Option<RectangleInt>;

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
    fn rectangle(&self, rectangle: &RectangleInt);

    //fn add_region(&self, region: ???);
    //--> WRAP: fn gdk_cairo_region(cr: *mut cairo_t, region: *const cairo_region_t);
}

impl ContextExt for Context {
    fn create_from_window(window: &Window) -> Context {
        skip_assert_initialized!();
        unsafe { from_glib_full(ffi::gdk_cairo_create(window.to_glib_none().0)) }
    }

    fn get_clip_rectangle(&self) -> Option<RectangleInt> {
        unsafe {
            let mut rectangle = mem::uninitialized();
            if from_glib(ffi::gdk_cairo_get_clip_rectangle(self.to_glib_none().0, &mut rectangle)) {
                Some(rectangle)
            } else {
                None
            }
        }
    }

    fn set_source_rgba(&self, rgba: &GdkRGBA) {
        unsafe { ffi::gdk_cairo_set_source_rgba(self.to_glib_none().0, rgba); }
    }

    fn set_source_pixbuf(&self, pixbuf: &Pixbuf, x: f64, y: f64) {
        unsafe {
            ffi::gdk_cairo_set_source_pixbuf(self.to_glib_none().0, pixbuf.to_glib_none().0, x, y);
        }
    }

    fn set_source_window(&self, window: &Window, x: f64, y: f64) {
        unsafe {
            ffi::gdk_cairo_set_source_window(self.to_glib_none().0, window.to_glib_none().0, x, y);
        }
    }

    fn rectangle(&self, rectangle: &RectangleInt) {
        unsafe { ffi::gdk_cairo_rectangle(self.to_glib_none().0, rectangle); }
    }
}


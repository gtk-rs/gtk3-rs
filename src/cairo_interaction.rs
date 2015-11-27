// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

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
    fn create_from_window(window: &Window) -> Context;

    fn get_clip_rectangle(&self) -> Option<RectangleInt>;

    fn set_source_rgba(&self, rgba: &GdkRGBA);

    fn set_source_pixbuf(&self, pixbuf: &Pixbuf, x: f64, y: f64);

    fn set_source_window(&self, window: &Window, x: f64, y: f64);

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


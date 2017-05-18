// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi;
use gdk_pixbuf::Pixbuf;
use cairo::Context;
#[cfg(feature = "v3_10")]
use cairo::Surface;
use {RGBA, Rectangle, Window};

//pub fn create_region_from_surface()
//--> WRAP: gdk_cairo_region_create_from_surface (cairo_surface_t *surface);

//pub fn create_surface_from_pixbuf()
//--> WRAP: gdk_cairo_surface_create_from_pixbuf (const GdkPixbuf *pixbuf, int scale, GdkWindow *for_window);

pub trait ContextExt {
    fn create_from_window(window: &Window) -> Context;

    #[cfg(feature = "v3_16")]
    fn cairo_draw_from_gl(cr: &Context, window: &Window, source: i32, source_type: i32, buffer_scale: i32, x: i32, y: i32, width: i32, height: i32);

    #[cfg(feature = "v3_10")]
    fn cairo_surface_create_from_pixbuf<'a, P: Into<Option<&'a Window>>>(pixbuf: &Pixbuf, scale: i32, for_window: P) -> Option<Surface>;

    fn get_clip_rectangle(&self) -> Option<Rectangle>;

    fn set_source_rgba(&self, rgba: &RGBA);

    fn set_source_pixbuf(&self, pixbuf: &Pixbuf, x: f64, y: f64);

    fn set_source_window(&self, window: &Window, x: f64, y: f64);

    fn rectangle(&self, rectangle: &Rectangle);

    //fn add_region(&self, region: ???);
    //--> WRAP: fn gdk_cairo_region(cr: *mut cairo_t, region: *const cairo_region_t);
}

impl ContextExt for Context {
    fn create_from_window(window: &Window) -> Context {
        skip_assert_initialized!();
        unsafe { from_glib_full(ffi::gdk_cairo_create(window.to_glib_none().0)) }
    }

    #[cfg(feature = "v3_16")]
    fn cairo_draw_from_gl(cr: &Context, window: &Window, source: i32, source_type: i32, buffer_scale: i32, x: i32, y: i32, width: i32, height: i32) {
        skip_assert_initialized!();
        unsafe {
            ffi::gdk_cairo_draw_from_gl(mut_override(cr.to_glib_none().0), window.to_glib_none().0, source, source_type, buffer_scale, x, y, width, height);
        }
    }

    #[cfg(feature = "v3_10")]
    fn cairo_surface_create_from_pixbuf<'a, P: Into<Option<&'a Window>>>(pixbuf: &Pixbuf, scale: i32, for_window: P) -> Option<Surface> {
        assert_initialized_main_thread!();
        let for_window = for_window.into();
        let for_window = for_window.to_glib_none();
        unsafe {
            from_glib_full(ffi::gdk_cairo_surface_create_from_pixbuf(pixbuf.to_glib_none().0, scale, for_window.0))
        }
    }

    fn get_clip_rectangle(&self) -> Option<Rectangle> {
        unsafe {
            let mut rectangle = Rectangle::uninitialized();
            if from_glib(ffi::gdk_cairo_get_clip_rectangle(self.to_glib_none().0,
                    rectangle.to_glib_none_mut().0)) {
                Some(rectangle)
            } else {
                None
            }
        }
    }

    fn set_source_rgba(&self, rgba: &RGBA) {
        unsafe { ffi::gdk_cairo_set_source_rgba(self.to_glib_none().0, rgba.to_glib_none().0); }
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

    fn rectangle(&self, rectangle: &Rectangle) {
        unsafe { ffi::gdk_cairo_rectangle(self.to_glib_none().0, rectangle.to_glib_none().0); }
    }
}


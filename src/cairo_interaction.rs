// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use cairo::Surface;
use cairo::{Context, Region};
use gdk_pixbuf::Pixbuf;
use gdk_sys;
use glib::object::IsA;
use glib::translate::*;
use {Rectangle, Window, RGBA};

pub trait SurfaceExt {
    fn create_region(&self) -> Option<Region>;
}

impl SurfaceExt for Surface {
    fn create_region(&self) -> Option<Region> {
        unsafe {
            from_glib_full(gdk_sys::gdk_cairo_region_create_from_surface(
                self.to_glib_none().0,
            ))
        }
    }
}

pub trait PixbufExt {
    fn create_surface(&self, scale: i32, for_window: &Window) -> Option<Surface>;
}

impl PixbufExt for Pixbuf {
    fn create_surface(&self, scale: i32, for_window: &Window) -> Option<Surface> {
        unsafe {
            from_glib_full(gdk_sys::gdk_cairo_surface_create_from_pixbuf(
                self.to_glib_none().0,
                scale,
                for_window.to_glib_none().0,
            ))
        }
    }
}

pub trait ContextExt {
    fn create_from_window(window: &Window) -> Context;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn cairo_draw_from_gl(
        cr: &Context,
        window: &Window,
        source: i32,
        source_type: i32,
        buffer_scale: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    );

    fn cairo_surface_create_from_pixbuf<T: IsA<Window>>(
        pixbuf: &Pixbuf,
        scale: i32,
        for_window: Option<&T>,
    ) -> Option<Surface>;

    fn get_clip_rectangle(&self) -> Option<Rectangle>;

    fn set_source_rgba(&self, rgba: &RGBA);

    fn set_source_pixbuf(&self, pixbuf: &Pixbuf, x: f64, y: f64);

    fn set_source_window(&self, window: &Window, x: f64, y: f64);

    fn rectangle(&self, rectangle: &Rectangle);

    fn add_region(&self, region: &Region);
}

impl ContextExt for Context {
    fn create_from_window(window: &Window) -> Context {
        skip_assert_initialized!();
        unsafe { from_glib_full(gdk_sys::gdk_cairo_create(window.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn cairo_draw_from_gl(
        cr: &Context,
        window: &Window,
        source: i32,
        source_type: i32,
        buffer_scale: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        skip_assert_initialized!();
        unsafe {
            gdk_sys::gdk_cairo_draw_from_gl(
                mut_override(cr.to_glib_none().0),
                window.to_glib_none().0,
                source,
                source_type,
                buffer_scale,
                x,
                y,
                width,
                height,
            );
        }
    }

    fn cairo_surface_create_from_pixbuf<T: IsA<Window>>(
        pixbuf: &Pixbuf,
        scale: i32,
        for_window: Option<&T>,
    ) -> Option<Surface> {
        assert_initialized_main_thread!();
        let for_window = for_window.map(|f| f.as_ref());
        let for_window = for_window.to_glib_none();
        unsafe {
            from_glib_full(gdk_sys::gdk_cairo_surface_create_from_pixbuf(
                pixbuf.to_glib_none().0,
                scale,
                for_window.0,
            ))
        }
    }

    fn get_clip_rectangle(&self) -> Option<Rectangle> {
        unsafe {
            let mut rectangle = Rectangle::uninitialized();
            if from_glib(gdk_sys::gdk_cairo_get_clip_rectangle(
                self.to_glib_none().0,
                rectangle.to_glib_none_mut().0,
            )) {
                Some(rectangle)
            } else {
                None
            }
        }
    }

    fn set_source_rgba(&self, rgba: &RGBA) {
        unsafe {
            gdk_sys::gdk_cairo_set_source_rgba(self.to_glib_none().0, rgba.to_glib_none().0);
        }
    }

    fn set_source_pixbuf(&self, pixbuf: &Pixbuf, x: f64, y: f64) {
        unsafe {
            gdk_sys::gdk_cairo_set_source_pixbuf(
                self.to_glib_none().0,
                pixbuf.to_glib_none().0,
                x,
                y,
            );
        }
    }

    fn set_source_window(&self, window: &Window, x: f64, y: f64) {
        unsafe {
            gdk_sys::gdk_cairo_set_source_window(
                self.to_glib_none().0,
                window.to_glib_none().0,
                x,
                y,
            );
        }
    }

    fn rectangle(&self, rectangle: &Rectangle) {
        unsafe {
            gdk_sys::gdk_cairo_rectangle(self.to_glib_none().0, rectangle.to_glib_none().0);
        }
    }

    fn add_region(&self, region: &Region) {
        unsafe {
            gdk_sys::gdk_cairo_region(self.to_glib_none().0, region.to_glib_none().0);
        }
    }
}

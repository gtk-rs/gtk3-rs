// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use super::Pixbuf;
use gdk_pixbuf_sys;
use glib::object::IsA;
use glib::translate::*;
use glib::{Error, TimeVal};
use std::path::Path;
use std::ptr;

glib_wrapper! {
    pub struct PixbufAnimationIter(Object<gdk_pixbuf_sys::GdkPixbufAnimationIter, PixbufAnimationIterClass>);

    match fn {
        get_type => || gdk_pixbuf_sys::gdk_pixbuf_animation_iter_get_type(),
    }
}

impl PixbufAnimationIter {
    pub fn advance(&self, start_time: TimeVal) -> bool {
        unsafe {
            from_glib(gdk_pixbuf_sys::gdk_pixbuf_animation_iter_advance(
                self.to_glib_none().0,
                &start_time as *const _,
            ))
        }
    }

    pub fn get_pixbuf(&self) -> Pixbuf {
        unsafe {
            from_glib_none(gdk_pixbuf_sys::gdk_pixbuf_animation_iter_get_pixbuf(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_delay_time(&self) -> i32 {
        unsafe { gdk_pixbuf_sys::gdk_pixbuf_animation_iter_get_delay_time(self.to_glib_none().0) }
    }

    pub fn on_currently_loading_frame(&self) -> bool {
        unsafe {
            from_glib(
                gdk_pixbuf_sys::gdk_pixbuf_animation_iter_on_currently_loading_frame(
                    self.to_glib_none().0,
                ),
            )
        }
    }
}

glib_wrapper! {
    pub struct PixbufAnimation(Object<gdk_pixbuf_sys::GdkPixbufAnimation, PixbufAnimationClass>);

    match fn {
        get_type => || gdk_pixbuf_sys::gdk_pixbuf_animation_get_type(),
    }
}

impl PixbufAnimation {
    pub fn new_from_file<T: AsRef<Path>>(file: T) -> Result<PixbufAnimation, Error> {
        #[cfg(not(windows))]
        use gdk_pixbuf_sys::gdk_pixbuf_animation_new_from_file;
        #[cfg(windows)]
        use gdk_pixbuf_sys::gdk_pixbuf_animation_new_from_file_utf8 as gdk_pixbuf_animation_new_from_file;

        unsafe {
            let mut error = ptr::null_mut();
            let ptr =
                gdk_pixbuf_animation_new_from_file(file.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn new_from_resource(resource_path: &str) -> Result<PixbufAnimation, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ptr = gdk_pixbuf_sys::gdk_pixbuf_animation_new_from_resource(
                resource_path.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

pub trait PixbufAnimationExt {
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;
    fn get_iter(&self, start_time: TimeVal) -> PixbufAnimationIter;
    fn is_static_image(&self) -> bool;
    fn get_static_image(&self) -> Option<Pixbuf>;
}

impl<T: IsA<PixbufAnimation>> PixbufAnimationExt for T {
    fn get_width(&self) -> i32 {
        unsafe { gdk_pixbuf_sys::gdk_pixbuf_animation_get_width(self.as_ref().to_glib_none().0) }
    }

    fn get_height(&self) -> i32 {
        unsafe { gdk_pixbuf_sys::gdk_pixbuf_animation_get_height(self.as_ref().to_glib_none().0) }
    }

    fn get_iter(&self, start_time: TimeVal) -> PixbufAnimationIter {
        unsafe {
            from_glib_full(gdk_pixbuf_sys::gdk_pixbuf_animation_get_iter(
                self.as_ref().to_glib_none().0,
                &start_time as *const _,
            ))
        }
    }

    fn is_static_image(&self) -> bool {
        unsafe {
            from_glib(gdk_pixbuf_sys::gdk_pixbuf_animation_is_static_image(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_static_image(&self) -> Option<Pixbuf> {
        unsafe {
            from_glib_none(gdk_pixbuf_sys::gdk_pixbuf_animation_get_static_image(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::mem;
use std::ptr;
use glib::{Error, TimeVal};
use glib::object::IsA;
use glib::translate::*;
use gdk_pixbuf_ffi as ffi;
use super::Pixbuf;

glib_wrapper! {
    pub struct PixbufAnimationIter(Object<ffi::GdkPixbufAnimationIter>);

    match fn {
        get_type => || ffi::gdk_pixbuf_animation_iter_get_type(),
    }
}

impl PixbufAnimationIter {
    pub fn advance(&self, start_time: &TimeVal) -> bool {
        unsafe {
            from_glib(ffi::gdk_pixbuf_animation_iter_advance(self.to_glib_none().0,
                mem::transmute(start_time)))
        }
    }

    pub fn get_pixbuf(&self) -> Pixbuf {
        unsafe { from_glib_none(ffi::gdk_pixbuf_animation_iter_get_pixbuf(self.to_glib_none().0)) }
    }

    pub fn get_delay_time(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_animation_iter_get_delay_time(self.to_glib_none().0) }
    }

    pub fn on_currently_loading_frame(&self) -> bool {
        unsafe {
            from_glib(
                ffi::gdk_pixbuf_animation_iter_on_currently_loading_frame(self.to_glib_none().0))
        }
    }
}

glib_wrapper! {
    pub struct PixbufAnimation(Object<ffi::GdkPixbufAnimation>);

    match fn {
        get_type => || ffi::gdk_pixbuf_animation_get_type(),
    }
}

impl PixbufAnimation {
    pub fn new_from_file(file: &str) -> Result<PixbufAnimation, Error> {
        #[cfg(windows)]
        use gdk_pixbuf_ffi::gdk_pixbuf_animation_new_from_file_utf8
            as gdk_pixbuf_animation_new_from_file;
        #[cfg(not(windows))]
        use gdk_pixbuf_ffi::gdk_pixbuf_animation_new_from_file;

        unsafe {
            let mut error = ptr::null_mut();
            let ptr = gdk_pixbuf_animation_new_from_file(file.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(feature = "v2_28")]
    pub fn new_from_resource(resource_path: &str) -> Result<PixbufAnimation, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ptr = ffi::gdk_pixbuf_animation_new_from_resource(resource_path.to_glib_none().0,
                                                                  &mut error);
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
    fn get_iter(&self, start_time: &TimeVal) -> PixbufAnimationIter;
    fn is_static_image(&self) -> bool;
    fn get_static_image(&self) -> Option<Pixbuf>;
}

impl<T: IsA<PixbufAnimation>> PixbufAnimationExt for T {
    fn get_width(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_animation_get_width(self.to_glib_none().0) }
    }

    fn get_height(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_animation_get_height(self.to_glib_none().0) }
    }

    fn get_iter(&self, start_time: &TimeVal) -> PixbufAnimationIter {
        unsafe {
            from_glib_full(
                ffi::gdk_pixbuf_animation_get_iter(self.to_glib_none().0,
                                                   mem::transmute(start_time)))
        }
    }

    fn is_static_image(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_pixbuf_animation_is_static_image(self.to_glib_none().0))
        }
    }

    fn get_static_image(&self) -> Option<Pixbuf> {
        unsafe {
            from_glib_none(ffi::gdk_pixbuf_animation_get_static_image(
                self.to_glib_none().0))
        }
    }
}

glib_wrapper! {
    pub struct PixbufSimpleAnim(Object<ffi::GdkPixbufSimpleAnim>): PixbufAnimation;

    match fn {
        get_type => || ffi::gdk_pixbuf_simple_anim_get_type(),
    }
}

impl PixbufSimpleAnim {
    pub fn new(width: i32, height: i32, rate: f32) -> PixbufSimpleAnim {
        unsafe { from_glib_full(ffi::gdk_pixbuf_simple_anim_new(width, height, rate)) }
    }

    pub fn add_frame(&self, pixbuf: &Pixbuf) {
        unsafe {
            ffi::gdk_pixbuf_simple_anim_add_frame(self.to_glib_none().0, pixbuf.to_glib_none().0)
        }
    }

    pub fn set_loop(&self, loop_: bool) {
        unsafe { ffi::gdk_pixbuf_simple_anim_set_loop(self.to_glib_none().0, loop_.to_glib()) }
    }

    pub fn get_loop(&self) -> bool {
        unsafe { from_glib(ffi::gdk_pixbuf_simple_anim_get_loop(self.to_glib_none().0)) }
    }
}

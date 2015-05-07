// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Animations — Animated images.

use std::{self, ptr};
use ffi;
use glib::{to_bool, GlibContainer, Error, self};
use glib::translate::ToGlibPtr;

pub struct PixbufAnimation {
    pointer: *mut ffi::C_GdkPixbufAnimation
}

impl PixbufAnimation {
    pub fn new_from_file(file: &str) -> Result<PixbufAnimation, Error> {
        let mut error = ptr::null_mut();
        let tmp = unsafe { ffi::gdk_pixbuf_animation_new_from_file(file.to_glib_none().0, &mut error) };

        if error.is_null() {
            assert!(!tmp.is_null());
            Ok(PixbufAnimation::wrap_pointer(tmp))
        } else {
            Err(Error::wrap(error))
        }
    }

    pub fn new_from_resource(resource_path: &str) -> Result<PixbufAnimation, Error> {
        let mut error = ptr::null_mut();
        let tmp = unsafe { ffi::gdk_pixbuf_animation_new_from_resource(resource_path.to_glib_none().0, &mut error) };

        if error.is_null() {
            assert!(!tmp.is_null());
            Ok(PixbufAnimation::wrap_pointer(tmp))
        } else {
            Err(Error::wrap(error))
        }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_animation_get_width(self.unwrap_pointer()) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_animation_get_height(self.unwrap_pointer()) }
    }

    pub fn get_iter(&self, start_time: &glib::TimeVal) -> Option<::PixbufAnimationIter> {
        let tmp = unsafe { ffi::gdk_pixbuf_animation_get_iter(self.unwrap_pointer(),
            std::mem::transmute(start_time)) };

        if tmp.is_null() {
            None
        } else {
            Some(::PixbufAnimationIter::wrap_pointer(tmp))
        }
    }

    pub fn is_static_image(&self) -> bool {
        unsafe { glib::to_bool(ffi::gdk_pixbuf_animation_is_static_image(self.unwrap_pointer())) }
    }

    pub fn get_static_image(&self) -> Option<::Pixbuf> {
        let tmp = unsafe { ffi::gdk_pixbuf_animation_get_static_image(self.unwrap_pointer()) };

        if tmp.is_null() {
            None
        } else {
            Some(::Pixbuf::wrap_pointer(tmp))
        }
    }
}

impl_GObjectFunctions!(PixbufAnimation, C_GdkPixbufAnimation);
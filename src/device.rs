// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use cursor::Cursor;
use display::Display;
use screen::Screen;
use window::Window;
use ffi;
use std::mem;

glib_wrapper! {
    pub struct Device(Object<ffi::GdkDevice>);

    match fn {
        get_type => || ffi::gdk_device_get_type(),
    }
}

pub type Type = ffi::GdkDeviceType;

impl Device {
    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gdk_device_get_name(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_vendor_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gdk_device_get_vendor_id(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_product_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gdk_device_get_product_id(self.to_glib_none().0))
        }
    }

    pub fn get_source(&self) -> ::InputSource {
        unsafe { ffi::gdk_device_get_source(self.to_glib_none().0) }
    }

    pub fn set_mode(&self, mode: ::InputMode) {
        unsafe { ffi::gdk_device_set_mode(self.to_glib_none().0, mode); }
    }

    pub fn get_mode(&self) -> ::InputMode {
        unsafe { ffi::gdk_device_get_mode(self.to_glib_none().0) }
    }

    pub fn set_key(&self, index_: u32, keyval: u32, modifiers: ::ModifierType) {
        unsafe { ffi::gdk_device_set_key(self.to_glib_none().0, index_, keyval, modifiers.to_glib()) }
    }

    pub fn get_key(&self, index_: u32) -> Option<(u32, ::ModifierType)> {
        unsafe {
            let mut keyval = mem::uninitialized();
            let mut modifiers = mem::uninitialized();
            let ret = from_glib(ffi::gdk_device_get_key(self.to_glib_none().0, index_,
                                                        &mut keyval, &mut modifiers));
            if ret { Some((keyval, from_glib(modifiers))) } else { None }
        }
    }

    pub fn set_axis_use(&self, index_: u32, use_: ::AxisUse) {
        unsafe { ffi::gdk_device_set_axis_use(self.to_glib_none().0, index_, use_) }
    }

    pub fn get_axis_use(&self, index_: u32) -> ::AxisUse {
        unsafe { ffi::gdk_device_get_axis_use(self.to_glib_none().0, index_) }
    }

    pub fn get_associated_device(&self) -> Option<Device> {
        unsafe { from_glib_none(ffi::gdk_device_get_associated_device(self.to_glib_none().0)) }
    }

    pub fn get_device_type(&self) -> Type {
        unsafe { ffi::gdk_device_get_device_type(self.to_glib_none().0) }
    }

    pub fn get_display(&self) -> Display {
        unsafe { from_glib_none(ffi::gdk_device_get_display(self.to_glib_none().0)) }
    }

    pub fn get_has_cursor(&self) -> bool {
        unsafe { from_glib(ffi::gdk_device_get_has_cursor(self.to_glib_none().0)) }
    }

    pub fn get_n_axes(&self) -> i32 {
        unsafe { ffi::gdk_device_get_n_axes(self.to_glib_none().0) }
    }

    pub fn get_n_keys(&self) -> i32 {
        unsafe { ffi::gdk_device_get_n_keys(self.to_glib_none().0) }
    }

    pub fn warp(&self, screen: &Screen, x: i32, y: i32) {
        unsafe { ffi::gdk_device_warp(self.to_glib_none().0, screen.to_glib_none().0, x, y) }
    }

    pub fn grab(&self, window: &Window, grab_ownership: ::GrabOwnership, owner_events: bool,
                event_mask: ::EventMask, cursor: &Cursor, time_: u32) -> ::GrabStatus {
        unsafe {
            ffi::gdk_device_grab(self.to_glib_none().0, window.to_glib_none().0, grab_ownership,
                owner_events.to_glib(), event_mask, cursor.to_glib_none().0, time_)
        }
    }

    pub fn ungrab(&self, time_: u32) {
        unsafe { ffi::gdk_device_ungrab(self.to_glib_none().0, time_) }
    }

    pub fn get_axis(&self, axes: &mut [f64], use_: ::AxisUse, value: &mut f64) -> bool {
        unsafe { from_glib(ffi::gdk_device_get_axis(self.to_glib_none().0, axes.as_mut_ptr(), use_, value)) }
    }
}

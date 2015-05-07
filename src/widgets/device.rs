// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GdkDevice — Object representing an input device

use ffi;
use libc::{c_uint};
use glib::to_bool;
use glib::translate::*;

#[repr(C)]
pub struct Device {
    pointer: *mut ffi::C_GdkDevice
}

impl Device {
    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gdk_device_get_name(self.pointer))
        }
    }

    pub fn get_source(&self) -> ::InputSource {
        unsafe { ffi::gdk_device_get_source(self.pointer) }
    }

    pub fn set_mode(&self, mode: ::InputMode) {
        unsafe { ffi::gdk_device_set_mode(self.pointer, mode) }
    }

    pub fn get_mode(&self) -> ::InputMode {
        unsafe { ffi::gdk_device_get_mode(self.pointer) }
    }

    pub fn set_key(&self, index_: u32, keyval: u32, modifiers: ::ModifierType) {
        unsafe { ffi::gdk_device_set_key(self.pointer, index_ as c_uint, keyval as c_uint, modifiers) }
    }

    pub fn get_key(&self, index_: u32, keyval: &mut u32, modifiers: &mut ::ModifierType) -> bool {
        unsafe { to_bool(ffi::gdk_device_get_key(self.pointer, index_ as c_uint, keyval as *mut c_uint, modifiers)) }
    }

    pub fn set_axis_use(&self, index_: u32, use_: ::AxisUse) {
        unsafe { ffi::gdk_device_set_axis_use(self.pointer, index_ as c_uint, use_) }
    }

    pub fn get_axis_use(&self, index_: u32) -> ::AxisUse {
        unsafe { ffi::gdk_device_get_axis_use(self.pointer, index_ as c_uint) }
    }

    pub fn get_associated_device(&self) -> Option<Device> {
        let tmp = unsafe { ffi::gdk_device_get_associated_device(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(Device {
                pointer: tmp
            })
        }
    }

    pub fn get_device_type(&self) -> ::DeviceType {
        unsafe { ffi::gdk_device_get_device_type(self.pointer) }
    }

    /*pub fn get_display(&self) -> Option<::Display> {
        let tmp = unsafe { ffi::gdk_device_get_display(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(::Display::wrap_pointer(tmp))
        }
    }*/

    pub fn get_has_cursor(&self) -> bool {
        unsafe { to_bool(ffi::gdk_device_get_has_cursor(self.pointer)) }
    }

    pub fn get_n_axes(&self) -> i32 {
        unsafe { ffi::gdk_device_get_n_axes(self.pointer) }
    }

    pub fn get_n_keys(&self) -> i32 {
        unsafe { ffi::gdk_device_get_n_keys(self.pointer) }
    }

    /*pub fn warp(&self, screen: &::Screen, x: i32, y: i32) {
        unsafe { ffi::gdk_device_warp(self.pointer, screen.unwrap_pointer(), x as c_int, y as c_int) }
    }

    pub fn grab(&self, window: &::Window, grab_ownership: ::GrabOwnership, owner_events: bool, event_mask: ::EventMask,
        cursor: &mut ::Cursor, time_: u32) -> ::GrabStatus {
        unsafe {
            ffi::gdk_device_grab(self.pointer, window.unwrap_pointer(), grab_ownership, to_gboolean(owner_events),
                event_mask, cursor.unwrap_pointer(), time_)
        }
    }*/

    pub fn ungrab(&self, time_: u32) {
        unsafe { ffi::gdk_device_ungrab(self.pointer, time_) }
    }

    /*pub fn get_state(&self, window: &::Window, axes: &mut [f64], mask: &mut gdk;:ModifierType) {
        unsafe { ffi::gdk_device_get_state(self.pointer, window.unwrap_pointer(), axes.as_mut_ptr(), mask) }
    }

    pub fn get_position(&self, x: &mut i32, y: &mut i32) -> Option<::Screen> {
        let mut ptr = ::std::ptr::null_mut();

        unsafe { ffi::gdk_device_get_position(self.pointer, &mut ptr, x as *mut c_int, y as *mut c_int) };
        if ptr.is_null() {
            None
        } else {
            Some(::Screen::wrap_pointer(ptr))
        }
    }

    pub fn get_position_double(&self, x: &mut f64, y: &mut f64) -> Option<::Screen> {
        let mut ptr = ::std::ptr::null_mut();

        unsafe { ffi::gdk_device_get_position_double(self.pointer, &mut ptr, x as *mut c_double, y as *mut c_double) };
        if ptr.is_null() {
            None
        } else {
            Some(::Screen::wrap_pointer(ptr))
        }
    }

    pub fn get_window_at_position(&self, x: &mut i32, y: &mut i32) -> Option<::Window> {
        let mut ptr = ::std::ptr::null_mut();

        unsafe { ffi::gdk_device_get_window_at_position(self.pointer, &mut ptr, x as *mut c_int, y as *mut c_int) };
        if ptr.is_null() {
            None
        } else {
            Some(::Window::wrap_pointer(ptr))
        }
    }

    pub fn get_window_at_position_double(&self, x: &mut f64, y: &mut f64) -> Option<::Window> {
        let mut ptr = ::std::ptr::null_mut();

        unsafe { ffi::gdk_device_get_window_at_position_double(self.pointer, &mut ptr, x as *mut c_double, y as *mut c_double) };
        if ptr.is_null() {
            None
        } else {
            Some(::Window::wrap_pointer(ptr))
        }
    }

    pub fn get_history(&self, window: &::Window, start: u32, stop: u32) -> Vec<::TimeCoord> {
        let mut ptr = ::std::ptr::null_mut();
        let mut n_events : c_int = 0;

        unsafe { ffi::gdk_device_get_history(self.pointer, window.unwrap_pointer(), start, stop, &mut ptr, &mut n_events) };
        
        let mut ret = Vec::with_capacity(n_events as uint);
        
        for i in range(0, n_events) {
            ret.push(::TimeCoord::wrap_pointer(::std::ptr::read(ptr.offset(i))));
        }
        ret
    }

    pub fn free_history(events: &[::TimeCoord]) {
        let mut tmp = Vec::with_capacity(events.len());

        for i in range(0, events.len()) {
            tmp.push(events[i].unwrap_pointer());
        }
        unsafe { ffi::gdk_device_free_history(events.as_mut_ptr(), events.len()) }
    }*/

    pub fn get_axis(&self, axes: &mut [f64], use_: ::AxisUse, value: &mut f64) -> bool {
        unsafe { to_bool(ffi::gdk_device_get_axis(self.pointer, axes.as_mut_ptr(), use_, value)) }
    }

    /*pub fn get_axis_value(&self, axes: &mut [f64], label: &mut ::Atom, value: &mut f64) -> bool {
        unsafe { to_bool(ffi::gdk_device_get_axis_value(self.pointer, axes.as_mut_ptr(), label.unwrap_pointer(), value)) }
    }*/

    /*pub fn get_last_event_window(&self) -> Option<::Window> {
        let ptr = unsafe { ffi::gdk_device_get_last_event_window(self.pointer) };

        if ptr.is_null() {
            None
        } else {
            Some(::Window::wrap_pointer(ptr))
        }
    }*/
}

impl_GObjectFunctions!(Device, C_GdkDevice);

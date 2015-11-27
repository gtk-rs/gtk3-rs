// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use cursor::Cursor;
use display::Display;
use screen::Screen;
use window::Window;
use ffi;

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

    #[cfg(gdk_3_16)]
    pub fn get_vendor_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gdk_device_get_vendor_id(self.to_glib_none().0))
        }
    }

    #[cfg(gdk_3_16)]
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
        unsafe { ffi::gdk_device_set_key(self.to_glib_none().0, index_, keyval, modifiers) }
    }

    pub fn get_key(&self, index_: u32, keyval: &mut u32, modifiers: &mut ::ModifierType) -> bool {
        unsafe { from_glib(ffi::gdk_device_get_key(self.to_glib_none().0, index_, keyval, modifiers)) }
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

    /*pub fn get_state(&self, window: &::Window, axes: &mut [f64], mask: &mut gdk;:ModifierType) {
        unsafe { ffi::gdk_device_get_state(self.to_glib_none().0, window.unwrap_pointer(), axes.as_mut_ptr(), mask) }
    }

    pub fn get_position(&self, x: &mut i32, y: &mut i32) -> Option<::Screen> {
        let mut ptr = ::std::ptr::null_mut();

        unsafe { ffi::gdk_device_get_position(self.to_glib_none().0, &mut ptr, x as *mut c_int, y as *mut c_int) };
        if ptr.is_null() {
            None
        } else {
            Some(::Screen::wrap_pointer(ptr))
        }
    }

    pub fn get_position_double(&self, x: &mut f64, y: &mut f64) -> Option<::Screen> {
        let mut ptr = ::std::ptr::null_mut();

        unsafe { ffi::gdk_device_get_position_double(self.to_glib_none().0, &mut ptr, x as *mut c_double, y as *mut c_double) };
        if ptr.is_null() {
            None
        } else {
            Some(::Screen::wrap_pointer(ptr))
        }
    }

    pub fn get_window_at_position(&self, x: &mut i32, y: &mut i32) -> Option<::Window> {
        let mut ptr = ::std::ptr::null_mut();

        unsafe { ffi::gdk_device_get_window_at_position(self.to_glib_none().0, &mut ptr, x as *mut c_int, y as *mut c_int) };
        if ptr.is_null() {
            None
        } else {
            Some(::Window::wrap_pointer(ptr))
        }
    }

    pub fn get_window_at_position_double(&self, x: &mut f64, y: &mut f64) -> Option<::Window> {
        let mut ptr = ::std::ptr::null_mut();

        unsafe { ffi::gdk_device_get_window_at_position_double(self.to_glib_none().0, &mut ptr, x as *mut c_double, y as *mut c_double) };
        if ptr.is_null() {
            None
        } else {
            Some(::Window::wrap_pointer(ptr))
        }
    }

    pub fn get_history(&self, window: &::Window, start: u32, stop: u32) -> Vec<::TimeCoord> {
        let mut ptr = ::std::ptr::null_mut();
        let mut n_events : c_int = 0;

        unsafe { ffi::gdk_device_get_history(self.to_glib_none().0, window.unwrap_pointer(), start, stop, &mut ptr, &mut n_events) };
        
        let mut ret = Vec::with_capacity(n_events as uint);
        
        for i in range(0, n_events) {
            ret.push(::TimeCoord::wrap_pointer(::std::ptr::read(ptr.offset(i))));
        }
        ret
    }

    pub fn free_history(events: &[::TimeCoord]) {
        assert_initialized_main_thread!();
        let mut tmp = Vec::with_capacity(events.len());

        for i in range(0, events.len()) {
            tmp.push(events[i].unwrap_pointer());
        }
        unsafe { ffi::gdk_device_free_history(events.as_mut_ptr(), events.len()) }
    }*/

    pub fn get_axis(&self, axes: &mut [f64], use_: ::AxisUse, value: &mut f64) -> bool {
        unsafe { from_glib(ffi::gdk_device_get_axis(self.to_glib_none().0, axes.as_mut_ptr(), use_, value)) }
    }

    /*pub fn get_axis_value(&self, axes: &mut [f64], label: &mut ::Atom, value: &mut f64) -> bool {
        unsafe { from_glib(ffi::gdk_device_get_axis_value(self.to_glib_none().0, axes.as_mut_ptr(), label.unwrap_pointer(), value)) }
    }*/

    /*pub fn get_last_event_window(&self) -> Option<::Window> {
        let ptr = unsafe { ffi::gdk_device_get_last_event_window(self.to_glib_none().0) };

        if ptr.is_null() {
            None
        } else {
            Some(::Window::wrap_pointer(ptr))
        }
    }*/
}

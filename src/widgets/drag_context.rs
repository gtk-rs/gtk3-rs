// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Drag And Drop — Functions for controlling drag and drop handling

use ffi;
use glib::{to_bool, to_gboolean};
use libc::c_int;

#[repr(C)]
pub struct DragContext {
    pointer: *mut ffi::C_GdkDragContext
}

impl DragContext {
    pub fn drag_get_selection(&self) -> Option<::Atom> {
        let tmp = unsafe { ffi::gdk_drag_get_selection(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(::Atom::wrap_pointer(tmp))
        }
    }

    pub fn drag_abort(&self, time_: u32) {
        unsafe { ffi::gdk_drag_abort(self.pointer, time_) }
    }

    pub fn drop_reply(&self, accepted: bool, time_: u32) {
        unsafe { ffi::gdk_drop_reply(self.pointer, to_gboolean(accepted), time_) }
    }

    pub fn drop(&self, time_: u32) {
        unsafe { ffi::gdk_drag_drop(self.pointer, time_) }
    }

    pub fn drag_find_window_for_screen(&self, drag_window: &::Window, screen: &::Screen, x_root: i32, y_root: i32,
        dest_window: &mut ::Window, protocol: &mut ::DragProtocol) {
        unsafe { ffi::gdk_drag_find_window_for_screen(self.pointer, drag_window.unwrap_pointer(), screen.unwrap_pointer(), x_root as c_int,
            y_root as c_int, &mut dest_window.unwrap_pointer(), protocol) }
    }

    pub fn drag_motion(&self, dest_window: &::Window, protocol: ::DragProtocol, x_root: i32, y_root: i32,
        suggested_action: ::DragAction, possible_actions: ::DragAction, time_: u32) -> bool {
        unsafe { to_bool(ffi::gdk_drag_motion(self.pointer, dest_window.unwrap_pointer(), protocol, x_root as c_int,
            y_root as c_int, suggested_action, possible_actions, time_)) }
    }

    pub fn drop_finish(&self, success: bool, time_: u32) {
        unsafe { ffi::gdk_drop_finish(self.pointer, to_gboolean(success), time_) }
    }

    pub fn drag_status(&self, action: ::DragAction, time_: u32) {
        unsafe { ffi::gdk_drag_status(self.pointer, action, time_) }
    }

    pub fn drag_drop_succeeded(&self) -> bool {
        unsafe { to_bool(ffi::gdk_drag_drop_succeeded(self.pointer)) }
    }

    pub fn get_actions(&self) -> ::DragAction {
        unsafe { ffi::gdk_drag_context_get_actions(self.pointer) }
    }

    pub fn get_suggested_action(&self) -> ::DragAction {
        unsafe { ffi::gdk_drag_context_get_suggested_action(self.pointer) }
    }

    pub fn get_selected_action(&self) -> ::DragAction {
        unsafe { ffi::gdk_drag_context_get_selected_action(self.pointer) }
    }

    pub fn get_device(&self) -> Option<::Device> {
        let tmp = unsafe { ffi::gdk_drag_context_get_device(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(::Device::wrap_pointer(tmp))
        }
    }

    pub fn set_device(&self, device: &::Device) {
        unsafe { ffi::gdk_drag_context_set_device(self.pointer, device.unwrap_pointer()) }
    }

    pub fn get_source_window(&self) -> Option<::Window> {
        let tmp = unsafe { ffi::gdk_drag_context_get_source_window(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(::Window::wrap_pointer(tmp))
        }
    }

    pub fn get_dest_window(&self) -> Option<::Window> {
        let tmp = unsafe { ffi::gdk_drag_context_get_dest_window(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(::Window::wrap_pointer(tmp))
        }
    }

    pub fn get_protocol(&self) -> ::DragProtocol {
        unsafe { ffi::gdk_drag_context_get_protocol(self.pointer) }
    }
}

impl_GObjectFunctions!(DragContext, C_GdkDragContext);

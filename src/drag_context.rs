// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ptr;
use glib::translate::*;
use atom::Atom;
use device::Device;
use screen::Screen;
use window::Window;
use ffi;

use {DragAction, DragProtocol};

glib_wrapper! {
    pub struct DragContext(Object<ffi::GdkDragContext>);

    match fn {
        get_type => || ffi::gdk_drag_context_get_type(),
    }
}

impl DragContext {
    pub fn drag_get_selection(&self) -> Atom {
        unsafe { from_glib_none(ffi::gdk_drag_get_selection(self.to_glib_none().0)) }
    }

    pub fn drag_abort(&self, time_: u32) {
        unsafe { ffi::gdk_drag_abort(self.to_glib_none().0, time_) }
    }

    pub fn drop_reply(&self, accepted: bool, time_: u32) {
        unsafe { ffi::gdk_drop_reply(self.to_glib_none().0, accepted.to_glib(), time_) }
    }

    pub fn drop(&self, time_: u32) {
        unsafe { ffi::gdk_drag_drop(self.to_glib_none().0, time_) }
    }

    pub fn drag_find_window_for_screen(&self, drag_window: &Window, screen: &Screen,
                                       x_root: i32, y_root: i32) -> (Option<Window>, DragProtocol) {
        unsafe {
            let mut dest_window = ptr::null_mut();
            let mut protocol = DragProtocol::None;
            ffi::gdk_drag_find_window_for_screen(self.to_glib_none().0,
                                                 drag_window.to_glib_none().0,
                                                 screen.to_glib_none().0,
                                                 x_root, y_root,
                                                 &mut dest_window, &mut protocol);
            (from_glib_full(dest_window), protocol)
        }
    }

    pub fn drag_motion(&self, dest_window: &Window, protocol: DragProtocol, x_root: i32,
                       y_root: i32, suggested_action: DragAction, possible_actions: DragAction,
                       time_: u32) -> bool {
        unsafe {
            from_glib(
                ffi::gdk_drag_motion(self.to_glib_none().0, dest_window.to_glib_none().0, protocol, 
                    x_root, y_root, suggested_action, possible_actions, time_))
        }
    }

    pub fn drop_finish(&self, success: bool, time_: u32) {
        unsafe { ffi::gdk_drop_finish(self.to_glib_none().0, success.to_glib(), time_) }
    }

    pub fn drag_status(&self, action: DragAction, time_: u32) {
        unsafe { ffi::gdk_drag_status(self.to_glib_none().0, action, time_) }
    }

    pub fn drag_drop_succeeded(&self) -> bool {
        unsafe { from_glib(ffi::gdk_drag_drop_succeeded(self.to_glib_none().0)) }
    }

    pub fn get_actions(&self) -> DragAction {
        unsafe { ffi::gdk_drag_context_get_actions(self.to_glib_none().0) }
    }

    pub fn get_suggested_action(&self) -> DragAction {
        unsafe { ffi::gdk_drag_context_get_suggested_action(self.to_glib_none().0) }
    }

    pub fn get_selected_action(&self) -> DragAction {
        unsafe { ffi::gdk_drag_context_get_selected_action(self.to_glib_none().0) }
    }

    pub fn get_device(&self) -> Device {
        unsafe { from_glib_none(ffi::gdk_drag_context_get_device(self.to_glib_none().0)) }
    }

    pub fn set_device(&self, device: &Device) {
        unsafe { ffi::gdk_drag_context_set_device(self.to_glib_none().0, device.to_glib_none().0) }
    }

    pub fn get_source_window(&self) -> Window {
        unsafe { from_glib_none(ffi::gdk_drag_context_get_source_window(self.to_glib_none().0)) }
    }

    pub fn get_dest_window(&self) -> Window {
        unsafe { from_glib_none(ffi::gdk_drag_context_get_dest_window(self.to_glib_none().0)) }
    }

    pub fn get_protocol(&self) -> DragProtocol {
        unsafe { ffi::gdk_drag_context_get_protocol(self.to_glib_none().0) }
    }
}

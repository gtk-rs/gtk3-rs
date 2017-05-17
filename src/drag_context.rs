// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ptr;
use glib::translate::*;
use glib::object::IsA;
use atom::Atom;
use ffi;
use Device;
use DragAction;
use DragContext;
use DragProtocol;
use Screen;
use Window;

pub trait DragContextExtManual {
    fn drag_get_selection(&self) -> Atom;

    fn drag_abort(&self, time_: u32);

    fn drop_reply(&self, accepted: bool, time_: u32);

    fn drop(&self, time_: u32);

    fn drag_find_window_for_screen(&self, drag_window: &Window, screen: &Screen,
                                       x_root: i32, y_root: i32) -> (Option<Window>, DragProtocol);

    fn drag_motion(&self, dest_window: &Window, protocol: DragProtocol, x_root: i32,
                       y_root: i32, suggested_action: DragAction, possible_actions: DragAction,
                       time_: u32) -> bool;

    fn drop_finish(&self, success: bool, time_: u32);

    fn drag_status(&self, action: DragAction, time_: u32);

    fn drag_drop_succeeded(&self) -> bool;

    fn drag_begin(window: &Window, targets: &[&Atom]) -> Option<DragContext>;

    fn drag_begin_for_device<P: IsA<Device>>(window: &Window, device: &P, targets: &[&Atom]) -> Option<DragContext>;

    #[cfg(feature = "v3_20")]
    fn drag_begin_from_point<P: IsA<Device>>(window: &Window, device: &P, targets: &[&Atom], x_root: i32, y_root: i32) -> Option<DragContext>;

    #[cfg(feature = "v3_20")]
    fn drag_drop_done(&self, success: bool);
}

impl<O: IsA<DragContext>> DragContextExtManual for O {
    fn drag_get_selection(&self) -> Atom {
        unsafe { from_glib_none(ffi::gdk_drag_get_selection(self.to_glib_none().0) as *mut _) }
    }

    fn drag_abort(&self, time_: u32) {
        unsafe { ffi::gdk_drag_abort(self.to_glib_none().0, time_) }
    }

    fn drop_reply(&self, accepted: bool, time_: u32) {
        unsafe { ffi::gdk_drop_reply(self.to_glib_none().0, accepted.to_glib(), time_) }
    }

    fn drop(&self, time_: u32) {
        unsafe { ffi::gdk_drag_drop(self.to_glib_none().0, time_) }
    }

    fn drag_find_window_for_screen(&self, drag_window: &Window, screen: &Screen,
                                       x_root: i32, y_root: i32) -> (Option<Window>, DragProtocol) {
        unsafe {
            let mut dest_window = ptr::null_mut();
            let mut protocol = ffi::GdkDragProtocol::None;
            ffi::gdk_drag_find_window_for_screen(self.to_glib_none().0,
                                                 drag_window.to_glib_none().0,
                                                 screen.to_glib_none().0,
                                                 x_root, y_root,
                                                 &mut dest_window, &mut protocol);
            (from_glib_full(dest_window), from_glib(protocol))
        }
    }

    fn drag_motion(&self, dest_window: &Window, protocol: DragProtocol, x_root: i32,
                       y_root: i32, suggested_action: DragAction, possible_actions: DragAction,
                       time_: u32) -> bool {
        unsafe {
            from_glib(
                ffi::gdk_drag_motion(self.to_glib_none().0, dest_window.to_glib_none().0, protocol.to_glib(), 
                    x_root, y_root, suggested_action.to_glib(), possible_actions.to_glib(), time_))
        }
    }

    fn drop_finish(&self, success: bool, time_: u32) {
        unsafe { ffi::gdk_drop_finish(self.to_glib_none().0, success.to_glib(), time_) }
    }

    fn drag_status(&self, action: DragAction, time_: u32) {
        unsafe { ffi::gdk_drag_status(self.to_glib_none().0, action.to_glib(), time_) }
    }

    fn drag_drop_succeeded(&self) -> bool {
        unsafe { from_glib(ffi::gdk_drag_drop_succeeded(self.to_glib_none().0)) }
    }
 
    fn drag_begin(window: &Window, targets: &[&Atom]) -> Option<DragContext> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gdk_drag_begin(window.to_glib_none().0, targets.to_glib_none().0))
        }
    }

    fn drag_begin_for_device<P: IsA<Device>>(window: &Window, device: &P, targets: &[&Atom]) -> Option<DragContext> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gdk_drag_begin_for_device(window.to_glib_none().0, device.to_glib_none().0, targets.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_20")]
    fn drag_begin_from_point<P: IsA<Device>>(window: &Window, device: &P, targets: &[&Atom], x_root: i32, y_root: i32) -> Option<DragContext> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gdk_drag_begin_from_point(window.to_glib_none().0, device.to_glib_none().0, targets.to_glib_none().0, x_root, y_root))
        }
    }

    #[cfg(feature = "v3_20")]
    fn drag_drop_done(&self, success: bool) {
        skip_assert_initialized!();
        unsafe {
            ffi::gdk_drag_drop_done(self.to_glib_none().0, success.to_glib());
        }
    }
}

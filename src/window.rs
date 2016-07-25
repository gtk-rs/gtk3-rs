// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::mem;
use std::ptr;
use libc::{c_char, c_int};
use glib::translate::*;
use cursor::Cursor;
use device::Device;
use display::Display;
#[cfg(feature = "v3_8")]
use frame_clock::FrameClock;
use screen::Screen;
use visual::Visual;
use ffi;
use Rectangle;

use {
    WindowEdge,
    WindowHints,
    WindowState,
    WindowType,
    WindowTypeHint,
    WindowWindowClass,
    WMDecoration,
    WMFunction,
};

pub struct WindowAttr {
    pub title: Option<String>,
    pub event_mask: i32,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: i32,
    pub height: i32,
    pub wclass: WindowWindowClass,
    pub visual: Option<Visual>,
    pub window_type: WindowType,
    pub cursor: Option<Cursor>,
    pub override_redirect: bool,
    pub type_hint: Option<WindowTypeHint>,
}

impl Default for WindowAttr {
    fn default() -> WindowAttr {
        skip_assert_initialized!();
        WindowAttr {
            title: None,
            event_mask: 0,
            x: None,
            y: None,
            width: 400,
            height: 300,
            wclass: WindowWindowClass::InputOutput,
            visual: None,
            window_type: WindowType::Toplevel,
            cursor: None,
            override_redirect: false,
            type_hint: None,
        }
    }
}

impl WindowAttr {
    fn get_mask(&self) -> u32 {
        let mut mask = ffi::GdkWindowAttributesType::empty();
        if self.title.is_some() { mask.insert(ffi::GDK_WA_TITLE); }
        if self.x.is_some() { mask.insert(ffi::GDK_WA_X); }
        if self.y.is_some() { mask.insert(ffi::GDK_WA_Y); }
        if self.cursor.is_some() { mask.insert(ffi::GDK_WA_CURSOR); }
        if self.visual.is_some() { mask.insert(ffi::GDK_WA_VISUAL); }
        if self.override_redirect { mask.insert(ffi::GDK_WA_NOREDIR); }
        if self.type_hint.is_some() { mask.insert(ffi::GDK_WA_TYPE_HINT); }
        mask.bits()
    }
}

impl<'a> ToGlibPtr<'a, *mut ffi::GdkWindowAttr> for WindowAttr {
    type Storage = (
        Box<ffi::GdkWindowAttr>,
        Stash<'a, *mut ffi::GdkVisual, Option<Visual>>,
        Stash<'a, *mut ffi::GdkCursor, Option<Cursor>>,
        Stash<'a, *const c_char, Option<String>>,
    );

    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::GdkWindowAttr, Self> {
        let title = self.title.to_glib_none();
        let visual = self.visual.to_glib_none();
        let cursor = self.cursor.to_glib_none();

        let mut attrs = Box::new(ffi::GdkWindowAttr {
            title: title.0 as *mut c_char,
            event_mask: self.event_mask,
            x: self.x.unwrap_or(0),
            y: self.y.unwrap_or(0),
            width: self.width,
            height: self.height,
            wclass: self.wclass,
            visual: visual.0,
            window_type: self.window_type,
            cursor: cursor.0,
            wmclass_name: ptr::null_mut(),
            wmclass_class: ptr::null_mut(),
            override_redirect: self.override_redirect.to_glib(),
            type_hint: self.type_hint.unwrap_or(WindowTypeHint::Normal).to_glib(),
        });

        Stash(&mut *attrs, (attrs, visual, cursor, title))
    }
}

glib_wrapper! {
    pub struct Window(Object<ffi::GdkWindow>);

    match fn {
        get_type => || ffi::gdk_window_get_type(),
    }
}

impl Window {
    pub fn new(parent: Option<&Window>, attributes: &WindowAttr) -> Window {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gdk_window_new(
                parent.to_glib_none().0,
                attributes.to_glib_none().0,
                attributes.get_mask() as c_int))
        }
    }

    pub fn get_window_type(&self) -> WindowType {
        unsafe { ffi::gdk_window_get_window_type(self.to_glib_none().0) }
    }

    pub fn get_display(&self) -> Display {
        unsafe { from_glib_none(ffi::gdk_window_get_display(self.to_glib_none().0)) }
    }

    pub fn get_screen(&self) -> Screen {
        unsafe { from_glib_none(ffi::gdk_window_get_screen(self.to_glib_none().0)) }
    }

    pub fn get_visual(&self) -> Visual {
        unsafe { from_glib_none(ffi::gdk_window_get_visual(self.to_glib_none().0)) }
    }

    pub fn show(&self) {
        unsafe { ffi::gdk_window_show(self.to_glib_none().0) }
    }

    pub fn show_unraised(&self) {
        unsafe { ffi::gdk_window_show_unraised(self.to_glib_none().0) }
    }

    pub fn hide(&self) {
        unsafe { ffi::gdk_window_hide(self.to_glib_none().0) }
    }

    pub fn is_destroyed(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_is_destroyed(self.to_glib_none().0)) }
    }

    pub fn is_visible(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_is_visible(self.to_glib_none().0)) }
    }

    pub fn is_viewable(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_is_viewable(self.to_glib_none().0)) }
    }

    pub fn is_input_only(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_is_input_only(self.to_glib_none().0)) }
    }

    pub fn is_shaped(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_is_shaped(self.to_glib_none().0)) }
    }

    pub fn get_state(&self) -> WindowState {
        unsafe { ffi::gdk_window_get_state(self.to_glib_none().0) }
    }

    pub fn withdraw(&self) {
        unsafe { ffi::gdk_window_withdraw(self.to_glib_none().0) }
    }

    pub fn iconify(&self) {
        unsafe { ffi::gdk_window_iconify(self.to_glib_none().0) }
    }

    pub fn deiconify(&self) {
        unsafe { ffi::gdk_window_deiconify(self.to_glib_none().0) }
    }

    pub fn stick(&self) {
        unsafe { ffi::gdk_window_stick(self.to_glib_none().0) }
    }

    pub fn unstick(&self) {
        unsafe { ffi::gdk_window_unstick(self.to_glib_none().0) }
    }

    pub fn maximize(&self) {
        unsafe { ffi::gdk_window_maximize(self.to_glib_none().0) }
    }

    pub fn unmaximize(&self) {
        unsafe { ffi::gdk_window_unmaximize(self.to_glib_none().0) }
    }

    pub fn fullscreen(&self) {
        unsafe { ffi::gdk_window_fullscreen(self.to_glib_none().0) }
    }

    pub fn unfullscreen(&self) {
        unsafe { ffi::gdk_window_unfullscreen(self.to_glib_none().0) }
    }

    #[cfg(feature = "v3_8")]
    pub fn get_fullscreen_mode(&self) -> ::FullscreenMode {
        unsafe { ffi::gdk_window_get_fullscreen_mode(self.to_glib_none().0) }
    }

    #[cfg(feature = "v3_8")]
    pub fn set_fullscreen_mode(&self, mode: ::FullscreenMode) {
        unsafe { ffi::gdk_window_set_fullscreen_mode(self.to_glib_none().0, mode) }
    }

    pub fn set_keep_above(&self, setting: bool) {
        unsafe { ffi::gdk_window_set_keep_above(self.to_glib_none().0, setting.to_glib()) }
    }

    pub fn set_keep_below(&self, setting: bool) {
        unsafe { ffi::gdk_window_set_keep_below(self.to_glib_none().0, setting.to_glib()) }
    }

    pub fn set_opacity(&self, opacity: f64) {
        unsafe { ffi::gdk_window_set_opacity(self.to_glib_none().0, opacity) }
    }

    pub fn set_composited(&self, composited: bool) {
        unsafe { ffi::gdk_window_set_composited(self.to_glib_none().0, composited.to_glib()) }
    }

    pub fn get_composited(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_get_composited(self.to_glib_none().0)) }
    }

    pub fn _move(&self, x: i32, y: i32) {
        unsafe { ffi::gdk_window_move(self.to_glib_none().0, x, y) }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe { ffi::gdk_window_resize(self.to_glib_none().0, width, height) }
    }

    pub fn move_resize(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { ffi::gdk_window_move_resize(self.to_glib_none().0, x, y, width, height) }
    }

    pub fn scroll(&self, dx: i32, dy: i32) {
        unsafe { ffi::gdk_window_scroll(self.to_glib_none().0, dx, dy) }
    }

    pub fn has_native(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_has_native(self.to_glib_none().0)) }
    }

    pub fn ensure_native(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_ensure_native(self.to_glib_none().0)) }
    }

    pub fn reparent(&self, new_parent: &Window, x: i32, y: i32) {
        unsafe { ffi::gdk_window_reparent(self.to_glib_none().0, new_parent.to_glib_none().0, x, y) }
    }

    pub fn raise(&self) {
        unsafe { ffi::gdk_window_raise(self.to_glib_none().0) }
    }

    pub fn lower(&self) {
        unsafe { ffi::gdk_window_lower(self.to_glib_none().0) }
    }

    pub fn restack(&self, sibling: &Window, above: bool) {
        unsafe { ffi::gdk_window_restack(self.to_glib_none().0, sibling.to_glib_none().0, above.to_glib()) }
    }

    pub fn focus(&self, timestamp: u32) {
        unsafe { ffi::gdk_window_focus(self.to_glib_none().0, timestamp) }
    }

    pub fn register_dnd(&self) {
        unsafe { ffi::gdk_window_register_dnd(self.to_glib_none().0) }
    }

    pub fn begin_resize_drag(&self, edge: WindowEdge, button: i32, root_x: i32, root_y: i32, timestamp: u32) {
        unsafe { ffi::gdk_window_begin_resize_drag(self.to_glib_none().0, edge.to_glib(), button, root_x, root_y, timestamp) }
    }

    pub fn begin_resize_drag_for_device(&self, edge: WindowEdge, device: &Device, button: i32, root_x: i32, root_y: i32,
        timestamp: u32) {
        unsafe { ffi::gdk_window_begin_resize_drag_for_device(self.to_glib_none().0, edge.to_glib(), device.to_glib_none().0, button,
            root_x, root_y, timestamp) }
    }

    pub fn begin_move_drag(&self, button: i32, root_x: i32, root_y: i32, timestamp: u32) {
        unsafe { ffi::gdk_window_begin_move_drag(self.to_glib_none().0, button, root_x, root_y, timestamp) }
    }

    pub fn begin_move_drag_for_device(&self, device: &Device, button: i32, root_x: i32, root_y: i32, timestamp: u32) {
        unsafe { ffi::gdk_window_begin_move_drag_for_device(self.to_glib_none().0, device.to_glib_none().0, button, root_x,
            root_y, timestamp) }
    }

    pub fn beep(&self) {
        unsafe { ffi::gdk_window_beep(self.to_glib_none().0) }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_scale_factor(&self) -> i32 {
        unsafe { ffi::gdk_window_get_scale_factor(self.to_glib_none().0) }
    }

    pub fn begin_paint_rect(&self, rect: &Rectangle) {
        unsafe { ffi::gdk_window_begin_paint_rect(self.to_glib_none().0, rect.to_glib_none().0) }
    }

    pub fn end_paint(&self) {
        unsafe { ffi::gdk_window_end_paint(self.to_glib_none().0) }
    }

    pub fn invalidate_rect(&self, rect: &Rectangle, invalidate_children: bool) {
        unsafe {
            ffi::gdk_window_invalidate_rect(self.to_glib_none().0, rect.to_glib_none().0,
                invalidate_children.to_glib())
        }
    }

    pub fn freeze_updates(&self) {
        unsafe { ffi::gdk_window_freeze_updates(self.to_glib_none().0) }
    }

    pub fn thaw_updates(&self) {
        unsafe { ffi::gdk_window_thaw_updates(self.to_glib_none().0) }
    }

    pub fn process_all_updates() {
        assert_initialized_main_thread!();
        unsafe { ffi::gdk_window_process_all_updates() }
    }

    pub fn process_updates(&self, update_children: bool) {
        unsafe { ffi::gdk_window_process_updates(self.to_glib_none().0, update_children.to_glib()) }
    }

    pub fn set_debug_updates(setting: bool) {
        assert_initialized_main_thread!();
        unsafe { ffi::gdk_window_set_debug_updates(setting.to_glib()) }
    }

    #[cfg(feature = "v3_8")]
    pub fn get_frame_clock(&self) -> FrameClock {
        unsafe { from_glib_none(ffi::gdk_window_get_frame_clock(self.to_glib_none().0)) }
    }

    pub unsafe fn set_user_data<T>(&self, user_data: &mut T) {
        ffi::gdk_window_set_user_data(self.to_glib_none().0, ::std::mem::transmute(user_data))
    }

    pub fn set_override_redirect(&self, override_redirect: bool) {
        unsafe {
            ffi::gdk_window_set_override_redirect(self.to_glib_none().0,
                                                  override_redirect.to_glib())
        }
    }

    pub fn set_accept_focus(&self, accept_focus: bool) {
        unsafe { ffi::gdk_window_set_accept_focus(self.to_glib_none().0, accept_focus.to_glib()) }
    }

    pub fn get_accept_focus(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_get_accept_focus(self.to_glib_none().0)) }
    }

    pub fn set_focus_on_map(&self, focus_on_map: bool) {
        unsafe { ffi::gdk_window_set_focus_on_map(self.to_glib_none().0, focus_on_map.to_glib()) }
    }

    pub fn get_focus_on_map(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_get_focus_on_map(self.to_glib_none().0)) }
    }

    pub fn set_child_shapes(&self) {
        unsafe { ffi::gdk_window_set_child_shapes(self.to_glib_none().0) }
    }

    pub fn merge_child_shapes(&self) {
        unsafe { ffi::gdk_window_merge_child_shapes(self.to_glib_none().0) }
    }

    pub fn set_child_input_shapes(&self) {
        unsafe { ffi::gdk_window_set_child_input_shapes(self.to_glib_none().0) }
    }

    pub fn merge_child_input_shapes(&self) {
        unsafe { ffi::gdk_window_merge_child_input_shapes(self.to_glib_none().0) }
    }

    pub fn set_static_gravities(&self, use_static: bool) -> bool {
        unsafe {
            from_glib(ffi::gdk_window_set_static_gravities(self.to_glib_none().0,
                                                           use_static.to_glib()))
        }
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gdk_window_set_title(self.to_glib_none().0, title.to_glib_none().0)
        }
    }

    pub fn set_background_rgba(&self, rgba: &ffi::GdkRGBA) {
        unsafe { ffi::gdk_window_set_background_rgba(self.to_glib_none().0, rgba) }
    }

    pub fn set_cursor(&self, cursor: &Cursor) {
        unsafe { ffi::gdk_window_set_cursor(self.to_glib_none().0, cursor.to_glib_none().0) }
    }

    pub fn get_cursor(&self) -> Option<Cursor> {
        unsafe { from_glib_none(ffi::gdk_window_get_cursor(self.to_glib_none().0)) }
    }

    pub unsafe fn get_user_data<'a, T>(&'a self) -> &'a mut T {
        let mut pointer = ::std::ptr::null_mut();
        ffi::gdk_window_get_user_data(self.to_glib_none().0, &mut pointer);
        ::std::mem::transmute(pointer)
    }

    pub fn get_geometry(&self, x: &mut i32, y: &mut i32, width: &mut i32, height: &mut i32) {
        unsafe { ffi::gdk_window_get_geometry(self.to_glib_none().0, x, y, width,
            height) }
    }

    pub fn set_geometry_hints(&self, geometry: &ffi::GdkGeometry, geom_mask: WindowHints) {
        unsafe { ffi::gdk_window_set_geometry_hints(self.to_glib_none().0, geometry, geom_mask) }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { ffi::gdk_window_get_width(self.to_glib_none().0) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { ffi::gdk_window_get_height(self.to_glib_none().0) }
    }

    pub fn set_modal_hint(&self, modal: bool) {
        unsafe { ffi::gdk_window_set_modal_hint(self.to_glib_none().0, modal.to_glib()) }
    }

    pub fn get_modal_hint(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_get_modal_hint(self.to_glib_none().0)) }
    }

    pub fn set_type_hint(&self, hint: WindowTypeHint) {
        unsafe { ffi::gdk_window_set_type_hint(self.to_glib_none().0, hint.to_glib()) }
    }

    pub fn get_type_hint(&self) -> WindowTypeHint {
        unsafe { from_glib(ffi::gdk_window_get_type_hint(self.to_glib_none().0)) }
    }

    #[cfg(feature = "v3_12")]
    pub fn set_shadow_width(&self, left: i32, right: i32, top: i32, bottom: i32) {
        unsafe { ffi::gdk_window_set_shadow_width(self.to_glib_none().0, left, right, top,
            bottom) }
    }

    pub fn set_skip_taskbar_hint(&self, skips_taskbar: bool) {
        unsafe {
            ffi::gdk_window_set_skip_taskbar_hint(self.to_glib_none().0, skips_taskbar.to_glib())
        }
    }

    pub fn set_skip_pager_hint(&self, skips_pager: bool) {
        unsafe { ffi::gdk_window_set_skip_pager_hint(self.to_glib_none().0, skips_pager.to_glib()) }
    }

    pub fn set_urgency_hint(&self, urgent: bool) {
        unsafe { ffi::gdk_window_set_urgency_hint(self.to_glib_none().0, urgent.to_glib()) }
    }

    pub fn get_position(&self, x: &mut i32, y: &mut i32) {
        unsafe { ffi::gdk_window_get_position(self.to_glib_none().0, x, y) }
    }

    pub fn get_root_origin(&self, x: &mut i32, y: &mut i32) {
        unsafe { ffi::gdk_window_get_root_origin(self.to_glib_none().0, x, y) }
    }

    pub fn get_frame_extents(&self) -> Rectangle {
        unsafe {
            let mut ret = Rectangle::uninitialized();
            ffi::gdk_window_get_frame_extents(self.to_glib_none().0, ret.to_glib_none_mut().0);
            ret
        }
    }

    pub fn get_origin(&self, x: &mut i32, y: &mut i32) -> i32 {
        unsafe { ffi::gdk_window_get_origin(self.to_glib_none().0, x, y) }
    }

    pub fn get_root_coords(&self, x: i32, y: i32, root_x: &mut i32, root_y: &mut i32) {
        unsafe { ffi::gdk_window_get_root_coords(self.to_glib_none().0, x, y, root_x, root_y) }
    }

    pub fn get_device_position(&self, device: &Device) -> (Option<Window>, i32, i32, ::ModifierType) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let mut mask = mem::uninitialized();
            let ret = from_glib_none(ffi::gdk_window_get_device_position(self.to_glib_none().0,
                                                                         device.to_glib_none().0,
                                                                         &mut x, &mut y, &mut mask));
            (ret, x, y, from_glib(mask))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_device_position_double(&self, device: &Device) -> (Option<Window>, f64, f64, ::ModifierType) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let mut mask = mem::uninitialized();
            let ret = from_glib_none(
                ffi::gdk_window_get_device_position_double(self.to_glib_none().0,
                                                           device.to_glib_none().0,
                                                           &mut x, &mut y, &mut mask));
            (ret, x, y, from_glib(mask))
        }
    }

    pub fn get_parent(&self) -> Window {
        unsafe { from_glib_none(ffi::gdk_window_get_parent(self.to_glib_none().0)) }
    }

    pub fn get_toplevel(&self) -> Window {
        unsafe { from_glib_none(ffi::gdk_window_get_toplevel(self.to_glib_none().0)) }
    }

    pub fn get_events(&self) -> ::EventMask {
        unsafe { ffi::gdk_window_get_events(self.to_glib_none().0) }
    }

    pub fn set_events(&self, event_mask: ::EventMask) {
        unsafe { ffi::gdk_window_set_events(self.to_glib_none().0, event_mask) }
    }

    pub fn set_icon_name(&self, name: &str) {
        unsafe {
            ffi::gdk_window_set_icon_name(self.to_glib_none().0, name.to_glib_none().0)
        }
    }

    pub fn set_transient_for(&self, parent: &Window) {
        unsafe { ffi::gdk_window_set_transient_for(self.to_glib_none().0, parent.to_glib_none().0) }
    }

    pub fn set_role(&self, role: &str) {
        unsafe {
            ffi::gdk_window_set_role(self.to_glib_none().0, role.to_glib_none().0)
        }
    }

    pub fn set_startup_id(&self, startup_id: &str) {
        unsafe {
            ffi::gdk_window_set_startup_id(self.to_glib_none().0, startup_id.to_glib_none().0)
        }
    }

    pub fn set_group(&self, leader: Option<&Window>) {
        unsafe { ffi::gdk_window_set_group(self.to_glib_none().0, leader.to_glib_none().0) }
    }

    pub fn get_group(&self) -> Window {
        unsafe { from_glib_none(ffi::gdk_window_get_group(self.to_glib_none().0)) }
    }

    pub fn set_decorations(&self, decorations: WMDecoration) {
        unsafe { ffi::gdk_window_set_decorations(self.to_glib_none().0, decorations) }
    }

    pub fn get_decorations(&self) -> Option<WMDecoration> {
        unsafe {
            let mut res = mem::uninitialized();
            match from_glib(ffi::gdk_window_get_decorations(self.to_glib_none().0, &mut res)) {
                true => Some(res),
                false => None,
            }
        }
    }

    pub fn set_functions(&self, functions: WMFunction) {
        unsafe { ffi::gdk_window_set_functions(self.to_glib_none().0, functions) }
    }

    pub fn get_default_root_window() -> Window {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gdk_get_default_root_window()) }
    }

    pub fn get_support_multidevice(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_get_support_multidevice(self.to_glib_none().0)) }
    }

    pub fn set_support_multidevice(&self, support_multidevice: bool) {
        unsafe {
            ffi::gdk_window_set_support_multidevice(self.to_glib_none().0,
                                                    support_multidevice.to_glib())
        }
    }

    pub fn get_device_cursor(&self, device: &Device) -> Option<Cursor> {
        unsafe {
            from_glib_none(
                ffi::gdk_window_get_device_cursor(self.to_glib_none().0, device.to_glib_none().0))
        }
    }

    pub fn set_device_cursor(&self, device: &Device, cursor: &Cursor) {
        unsafe {
            ffi::gdk_window_set_device_cursor(self.to_glib_none().0,
                                              device.to_glib_none().0,
                                              cursor.to_glib_none().0)
        }
    }

    pub fn get_device_events(&self, device: &Device) -> ::EventMask {
        unsafe { ffi::gdk_window_get_device_events(self.to_glib_none().0, device.to_glib_none().0) }
    }

    pub fn set_device_events(&self, device: &Device, event_mask: ::EventMask) {
        unsafe {
            ffi::gdk_window_set_device_events(self.to_glib_none().0,
                                              device.to_glib_none().0,
                                              event_mask) }
    }

    pub fn get_source_events(&self, source: ::InputSource) -> ::EventMask {
        unsafe { ffi::gdk_window_get_source_events(self.to_glib_none().0, source) }
    }

    pub fn set_source_events(&self, source: ::InputSource, event_mask: ::EventMask) {
        unsafe { ffi::gdk_window_set_source_events(self.to_glib_none().0, source, event_mask) }
    }

    #[cfg(feature = "v3_12")]
    pub fn get_event_compression(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_get_event_compression(self.to_glib_none().0)) }
    }

    #[cfg(feature = "v3_12")]
    pub fn set_event_compression(&self, event_compression: bool) {
        unsafe {
            ffi::gdk_window_set_event_compression(self.to_glib_none().0,
                                                  event_compression.to_glib())
        }
    }

    pub fn offscreen_window_set_embedder(&self, embedder: &Window) {
        unsafe {
            ffi::gdk_offscreen_window_set_embedder(self.to_glib_none().0, embedder.to_glib_none().0)
        }
    }

    pub fn offscreen_window_get_embedder(&self) -> Option<Window> {
        unsafe { from_glib_none(ffi::gdk_offscreen_window_get_embedder(self.to_glib_none().0)) }
    }

    pub fn geometry_changed(&self) {
        unsafe { ffi::gdk_window_geometry_changed(self.to_glib_none().0) }
    }

    pub fn coords_from_parent(&self, parent_x: f64, parent_y: f64, x: &mut f64, y: &mut f64) {
        unsafe { ffi::gdk_window_coords_from_parent(self.to_glib_none().0, parent_x, parent_y, x, y) }
    }

    pub fn coords_to_parent(&self, x: f64, y: f64, parent_x: &mut f64, parent_y: &mut f64) {
        unsafe { ffi::gdk_window_coords_to_parent(self.to_glib_none().0, x, y, parent_x, parent_y) }
    }

    pub fn get_effective_parent(&self) -> Window {
        unsafe { from_glib_none(ffi::gdk_window_get_effective_parent(self.to_glib_none().0)) }
    }

    pub fn get_effective_toplevel(&self) -> Window {
        unsafe { from_glib_none(ffi::gdk_window_get_effective_toplevel(self.to_glib_none().0)) }
    }

    pub fn get_drag_protocol(&self) -> (::DragProtocol, Option<Window>) {
        unsafe {
            let mut target = mem::uninitialized();
            let proto = ffi::gdk_window_get_drag_protocol(self.to_glib_none().0, &mut target);
            (proto, from_glib_full(target))
        }
    }
}

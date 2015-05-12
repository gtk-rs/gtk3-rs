// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(non_camel_case_types)]
#![allow(dead_code)]

extern crate libc;
#[macro_use] extern crate bitflags;
extern crate glib_sys as glib_ffi;

pub mod enums;

use libc::{c_int, c_char, c_double, c_void, c_uint, c_uchar, c_ulong, c_float};
pub use glib_ffi::{gboolean, gpointer, gsize, GType};

#[repr(C)]
pub struct GdkWindow;
#[repr(C)]
pub struct GdkDisplay;
#[repr(C)]
pub struct GdkDisplayManager;
#[repr(C)]
pub struct GdkScreen;
#[repr(C)]
pub struct GdkVisual;
#[repr(C)]
pub struct GdkEvent;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GdkRectangle { // FIXME should be just an alias to cairo_rectangle_int_t
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int
}
#[repr(C)]
pub struct GdkFrameClock;
/// The Color structure is used to describe a color, similar to the XColor struct used in the X11 drawing API.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub struct GdkColor {
    /// For allocated colors, the pixel value used to draw this color on the screen. Not used anymore.
    pub pixel:  u32,
    /// The red component of the color. This is a value between 0 and 65535, with 65535 indicating full intensity
    pub red:    u16,
    /// The green component of the color
    pub green:  u16,
    /// The blue component of the color
    pub blue:   u16
}
/// The GdkRGBA structure is used to represent a (possibly translucent) color, in a way that is compatible with cairos notion of color.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GdkRGBA {
    /// The intensity of the red channel from 0.0 to 1.0 inclusive
    pub red: f64,
    /// The intensity of the green channel from 0.0 to 1.0 inclusive
    pub green: f64,
    /// The intensity of the blue channel from 0.0 to 1.0 inclusive
    pub blue: f64,
    /// The opacity of the color from 0.0 for completely translucent to 1.0 for opaque
    pub alpha: f64
}
#[repr(C)]
pub struct GdkCursor;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GdkGeometry {
    /// minimum width of window (or -1 to use requisition, with GtkWindow only)
    pub min_width: c_int,
    /// minimum height of window (or -1 to use requisition, with GtkWindow only)
    pub min_height: c_int,
    /// maximum width of window (or -1 to use requisition, with GtkWindow only)
    pub max_width: c_int,
    /// maximum height of window (or -1 to use requisition, with GtkWindow only)
    pub max_height: c_int,
    /// allowed window widths are base_width + width_inc * N where N is any integer (-1 allowed with GtkWindow)
    pub base_width: c_int,
    /// allowed window widths are base_height + height_inc * N where N is any integer (-1 allowed with GtkWindow)
    pub base_height: c_int,
    /// width resize increment
    pub width_inc: c_int,
    /// height resize increment
    pub height_inc: c_int,
    /// minimum width/height ratio
    pub min_aspect: f64,
    /// maximum width/height ratio
    pub max_aspect: f64,
    /// window gravity, see gtk_window_set_gravity()
    pub win_gravity: enums::Gravity,
}
#[repr(C)]
pub struct GdkDevice;
#[repr(C)]
pub struct GdkTimeCoord;
pub type GdkAtom = *mut c_void;
#[repr(C)]
pub struct GdkDeviceManager;
#[repr(C)]
pub struct GdkAppLaunchContext;
#[repr(C)]
pub struct GdkPixbuf;
#[repr(C)]
pub struct GdkFrameTimings;
#[repr(C)]
pub struct GdkWindowAttr {
    pub title: *const c_char,
    pub event_mask: c_int,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub wclass: enums::WindowWindowClass,
    pub visual: *mut GdkVisual,
    pub window_type: enums::WindowType,
    pub cursor: *mut GdkCursor,
    pub wmclass_name: *const c_char,
    pub wmclass_class: *const c_char,
    pub override_redirect: gboolean,
    pub type_hint: enums::WindowTypeHint
}
#[repr(C)]
pub struct GdkDragContext;
#[repr(C)]
pub struct GdkPixbufLoader;
#[repr(C)]
pub struct GdkPixbufFormat;
#[repr(C)]
pub struct GdkPixbufAnimation;
#[repr(C)]
pub struct GdkPixbufAnimationIter;
#[repr(C)]
pub struct GdkPixbufSimpleAnim;

pub type GdkPixbufDestroyNotify = extern "C" fn(*mut c_uchar, gpointer);

// GdkWindowAttributesTypes
/// Honor the title field
pub const GDK_WA_TITLE: i32 = 1 << 1;
/// Honor the X coordinate field
pub const GDK_WA_X: i32 = 1 << 2;
/// Honor the Y coordinate field
pub const GDK_WA_Y: i32 = 1 << 3;
/// Honor the cursor field
pub const GDK_WA_CURSOR: i32 = 1 << 4;
/// Honor the visual field
pub const GDK_WA_VISUAL: i32 = 1 << 5;
// Deprecated
//const GDK_WA_WMCLASS: i32 = 1 << 6;
/// Honor the override_redirect field
pub const GDK_WA_NOREDIR: i32 = 1 << 7;
/// Honor the type_hint field
pub const GDK_WA_TYPE_HINT: i32 = 1 << 8;

extern "C" {
    //=========================================================================
    // General                                                           NOT OK
    //=========================================================================
    pub fn gdk_init                      (argc: *mut c_int, argv: *mut *mut *mut c_char);
    pub fn gdk_init_check                (argc: *mut c_int, argv: *mut *mut *mut c_char) -> gboolean;
    pub fn gdk_parse_args                (argc: *mut c_int, argv: *mut *mut *mut c_char);
    pub fn gdk_get_display_arg_name      () -> *const c_char;
    pub fn gdk_notify_startup_complete   ();
    pub fn gdk_notify_startup_complete_with_id(startup_id: *const c_char);
    pub fn gdk_set_allowed_backends      (backends: *const c_char);
    pub fn gdk_get_program_class         () -> *const c_char;
    pub fn gdk_set_program_class         (program_class: *const c_char);
    pub fn gdk_flush                     ();
    pub fn gdk_screen_width              () -> c_int;
    pub fn gdk_screen_height             () -> c_int;
    pub fn gdk_screen_width_mm           () -> c_int;
    pub fn gdk_screen_height_mm          () -> c_int;
    pub fn gdk_set_double_click_time     (msec: c_uint);
    pub fn gdk_beep                      ();
    pub fn gdk_error_trap_push           ();
    pub fn gdk_error_trap_pop            ();
    pub fn gdk_error_trap_pop_ignored    ();

    //=========================================================================
    // GdkWindow                                                         NOT OK
    //=========================================================================
    pub fn gdk_window_new                (parent: *mut GdkWindow, attributes: *mut GdkWindowAttr,
        attributes_mask: c_int) -> *mut GdkWindow;
    pub fn gdk_window_destroy            (window: *mut GdkWindow);
    pub fn gdk_window_get_window_type    (window: *mut GdkWindow) -> enums::WindowType;
    pub fn gdk_window_get_display        (window: *mut GdkWindow) -> *mut GdkDisplay;
    pub fn gdk_window_get_screen         (window: *mut GdkWindow) -> *mut GdkScreen;
    pub fn gdk_window_get_visual         (window: *mut GdkWindow) -> *mut GdkVisual;
    pub fn gdk_window_show               (window: *mut GdkWindow);
    pub fn gdk_window_show_unraised      (window: *mut GdkWindow);
    pub fn gdk_window_hide               (window: *mut GdkWindow);
    pub fn gdk_window_is_destroyed       (window: *mut GdkWindow) -> gboolean;
    pub fn gdk_window_is_visible         (window: *mut GdkWindow) -> gboolean;
    pub fn gdk_window_is_viewable        (window: *mut GdkWindow) -> gboolean;
    pub fn gdk_window_is_input_only      (window: *mut GdkWindow) -> gboolean;
    pub fn gdk_window_is_shaped          (window: *mut GdkWindow) -> gboolean;
    pub fn gdk_window_get_state          (window: *mut GdkWindow) -> enums::WindowState;
    pub fn gdk_window_withdraw           (window: *mut GdkWindow);
    pub fn gdk_window_iconify            (window: *mut GdkWindow);
    pub fn gdk_window_deiconify          (window: *mut GdkWindow);
    pub fn gdk_window_stick              (window: *mut GdkWindow);
    pub fn gdk_window_unstick            (window: *mut GdkWindow);
    pub fn gdk_window_maximize           (window: *mut GdkWindow);
    pub fn gdk_window_unmaximize         (window: *mut GdkWindow);
    pub fn gdk_window_fullscreen         (window: *mut GdkWindow);
    pub fn gdk_window_unfullscreen       (window: *mut GdkWindow);
    pub fn gdk_window_get_fullscreen_mode(window: *mut GdkWindow) -> enums::FullscreenMode;
    pub fn gdk_window_set_fullscreen_mode(window: *mut GdkWindow, mode: enums::FullscreenMode);
    pub fn gdk_window_set_keep_above     (window: *mut GdkWindow, setting: gboolean);
    pub fn gdk_window_set_keep_below     (window: *mut GdkWindow, setting: gboolean);
    pub fn gdk_window_set_opacity        (window: *mut GdkWindow, opacity: c_double);
    pub fn gdk_window_set_composited     (window: *mut GdkWindow, composited: gboolean);
    pub fn gdk_window_get_composited     (window: *mut GdkWindow) -> gboolean;
    pub fn gdk_window_move               (window: *mut GdkWindow, x: c_int, y: c_int);
    pub fn gdk_window_resize             (window: *mut GdkWindow, width: c_int, height: c_int);
    pub fn gdk_window_move_resize        (window: *mut GdkWindow, x: c_int, y: c_int, width: c_int, height: c_int);
    pub fn gdk_window_scroll             (window: *mut GdkWindow, dx: c_int, dy: c_int);
    //pub fn gdk_window_move_region        (window: *mut GdkWindow, region: *mut cairo_region_t, dx: c_int, dy: c_int);
    pub fn gdk_window_has_native         (window: *mut GdkWindow) -> gboolean;
    pub fn gdk_window_ensure_native      (window: *mut GdkWindow) -> gboolean;
    pub fn gdk_window_reparent           (window: *mut GdkWindow, new_parent: *mut GdkWindow, x: c_int, y: c_int);
    pub fn gdk_window_raise              (window: *mut GdkWindow);
    pub fn gdk_window_lower              (window: *mut GdkWindow);
    pub fn gdk_window_restack            (window: *mut GdkWindow, sibling: *mut GdkWindow, above: gboolean);
    pub fn gdk_window_focus              (window: *mut GdkWindow, timestamp: u32);
    pub fn gdk_window_register_dnd       (window: *mut GdkWindow);
    pub fn gdk_window_begin_resize_drag  (window: *mut GdkWindow, edge: enums::WindowEdge, button: c_int, root_x: c_int, root_y: c_int,
        timestamp: u32);
    pub fn gdk_window_begin_resize_drag_for_device(window: *mut GdkWindow, edge: enums::WindowEdge, device: *mut GdkDevice,
        button: c_int, root_x: c_int, root_y: c_int, timestamp: u32);
    pub fn gdk_window_begin_move_drag    (window: *mut GdkWindow, button: c_int, root_x: c_int, root_y: c_int, timestamp: u32);
    pub fn gdk_window_begin_move_drag_for_device(window: *mut GdkWindow, device: *mut GdkDevice, button: c_int, root_x: c_int,
        root_y: c_int, timestamp: u32);
    pub fn gdk_window_show_window_menu   (window: *mut GdkWindow, event: *mut GdkEvent);
    pub fn gdk_window_constrain_size     (window: *mut GdkWindow, flags: enums::WindowHints, width: c_int, height: c_int,
        new_width: *mut c_int, new_height: *mut c_int);
    pub fn gdk_window_beep               (window: *mut GdkWindow);
    pub fn gdk_window_get_scale_factor   (window: *mut GdkWindow) -> c_int;
    //pub fn gdk_window_set_opaque_region  (window: *mut GdkWindow, region: *mut cairo_region_t);
    //pub fn gdk_window_get_clip_region    (window: *mut GdkWindow) -> *mut cairo_region_t;
    pub fn gdk_window_begin_paint_rect   (window: *mut GdkWindow, rectangle: *const GdkRectangle);
    //pub fn gdk_window_begin_paint_region (window: *mut GdkWindow, region: *const cairo_region_t);
    pub fn gdk_window_end_paint          (window: *mut GdkWindow);
    //pub fn gdk_window_get_visible_region (window: *mut GdkWindow) -> *mut cairo_region_t;
    //pub fn gdk_window_set_invalidate_handler(window: *mut GdkWindow, handler: GdkWindowInvalidateHandlerFunc);
    pub fn gdk_window_invalidate_rect    (window: *mut GdkWindow, rectangle: *const GdkRectangle, invalidate_children: gboolean);
    //pub fn gdk_window_invalidate_region  (window: *mut GdkWindow, region: *const cairo_region_t, invalidate_children: gboolean);
    //pub fn gdk_window_invalidate_maybe_recurse(window: *mut GdkWindow, region: *const cairo_region_t, child_func: GdkWindowChildFunc,
    //    user_data: *mut c_void);
    //pub fn gdk_window_get_update_area    (window: *mut GdkWindow) -> *mut cairo_region_t;
    pub fn gdk_window_freeze_updates     (window: *mut GdkWindow);
    pub fn gdk_window_thaw_updates       (window: *mut GdkWindow);
    pub fn gdk_window_process_all_updates();
    pub fn gdk_window_process_updates    (window: *mut GdkWindow, update_children: gboolean);
    pub fn gdk_window_set_debug_updates  (setting: gboolean);
    pub fn gdk_window_get_frame_clock    (window: *mut GdkWindow) -> *mut GdkFrameClock;
    pub fn gdk_window_set_user_data      (window: *mut GdkWindow, user_data: *mut c_void);
    pub fn gdk_window_set_override_redirect(window: *mut GdkWindow, override_redirect: gboolean);
    pub fn gdk_window_set_accept_focus   (window: *mut GdkWindow, accept_focus: gboolean);
    pub fn gdk_window_get_accept_focus   (window: *mut GdkWindow) -> gboolean;
    pub fn gdk_window_set_focus_on_map   (window: *mut GdkWindow, focus_on_map: gboolean);
    pub fn gdk_window_get_focus_on_map   (window: *mut GdkWindow) -> gboolean;
    //pub fn gdk_window_add_filter         (window: *mut GdkWindow, function: GdkFilterFunc, data: *mut c_void);
    //pub fn gdk_window_remove_filter      (window: *mut GdkWindow, function: GdkFilterFunc, data: *mut c_void);
    //pub fn gdk_window_shape_combine_region(window: *mut GdkWindow, shape_region: *const cairo_region_t, offset_x: c_int,
    //    offset_y: c_int);
    pub fn gdk_window_set_child_shapes   (window: *mut GdkWindow);
    pub fn gdk_window_merge_child_shapes (window: *mut GdkWindow);
    //pub fn gdk_window_input_shape_combine_region(window: *mut GdkWindow, shape_region: *const cairo_region_t, offset_x: c_int,
    //    offset_y: c_int);
    pub fn gdk_window_set_child_input_shapes(window: *mut GdkWindow);
    pub fn gdk_window_merge_child_input_shapes(window: *mut GdkWindow);
    pub fn gdk_window_set_static_gravities(window: *mut GdkWindow, use_static: gboolean) -> gboolean;
    pub fn gdk_window_set_title          (window: *mut GdkWindow, title: *const c_char);
    pub fn gdk_window_set_background_rgba(window: *mut GdkWindow, rgba: *const GdkRGBA);
    //pub fn gdk_window_set_background_pattern(window: *mut GdkWindow, pattern: *const cairo_pattern_t);
    //pub fn gdk_window_get_background_pattern(window: *mut GdkWindow) -> *const cairo_pattern_t;
    pub fn gdk_window_set_cursor         (window: *mut GdkWindow, cursor: *mut GdkCursor);
    pub fn gdk_window_get_cursor         (window: *mut GdkWindow) -> *mut GdkCursor;
    pub fn gdk_window_get_user_data      (window: *mut GdkWindow, data: *mut *mut c_void);
    pub fn gdk_window_get_geometry       (window: *mut GdkWindow, x: *mut c_int, y: *mut c_int, width: *mut c_int, height: *mut c_int);
    pub fn gdk_window_set_geometry_hints (window: *mut GdkWindow, geometry: *const GdkGeometry, geom_mask: enums::WindowHints);
    pub fn gdk_window_get_width          (window: *mut GdkWindow) -> c_int;
    pub fn gdk_window_get_height         (window: *mut GdkWindow) -> c_int;
    //pub fn gdk_window_set_icon_list      (window: *mut GdkWindow, pixbufs: *mut GList);
    pub fn gdk_window_set_modal_hint     (window: *mut GdkWindow, modal: gboolean);
    pub fn gdk_window_get_modal_hint     (window: *mut GdkWindow) -> gboolean;
    pub fn gdk_window_set_type_hint      (window: *mut GdkWindow, hint: enums::WindowTypeHint);
    pub fn gdk_window_get_type_hint      (window: *mut GdkWindow) -> enums::WindowTypeHint;
    pub fn gdk_window_set_shadow_width   (window: *mut GdkWindow, left: c_int, right: c_int, top: c_int, bottom: c_int);
    pub fn gdk_window_set_skip_taskbar_hint(window: *mut GdkWindow, skips_taskbar: gboolean);
    pub fn gdk_window_set_skip_pager_hint(window: *mut GdkWindow, skips_pager: gboolean);
    pub fn gdk_window_set_urgency_hint   (window: *mut GdkWindow, urgent: gboolean);
    pub fn gdk_window_get_position       (window: *mut GdkWindow, x: *mut c_int, y: *mut c_int);
    pub fn gdk_window_get_root_origin    (window: *mut GdkWindow, x: *mut c_int, y: *mut c_int);
    pub fn gdk_window_get_frame_extents  (window: *mut GdkWindow, rect: *mut GdkRectangle);
    pub fn gdk_window_get_origin         (window: *mut GdkWindow, x: *mut c_int, y: *mut c_int);
    pub fn gdk_window_get_root_coords    (window: *mut GdkWindow, x: c_int, y: c_int, root_x: *mut c_int, root_y: *mut c_int);
    pub fn gdk_window_get_device_position(window: *mut GdkWindow, device: *mut GdkDevice, x: *mut c_int, y: *mut c_int,
        mask: *mut enums::modifier_type::ModifierType) -> *mut GdkWindow;
    pub fn gdk_window_get_device_position_double(window: *mut GdkWindow, device: *mut GdkDevice, x: *mut c_double, y: *mut c_double,
        mask: *mut enums::modifier_type::ModifierType) -> *mut GdkWindow;
    pub fn gdk_window_get_parent         (window: *mut GdkWindow) -> *mut GdkWindow;
    pub fn gdk_window_get_toplevel       (window: *mut GdkWindow) -> *mut GdkWindow;
    //pub fn gdk_window_get_children       (window: *mut GdkWindow) -> *mut GList;
    //pub fn gdk_window_get_children_with_user_data(window: *mut GdkWindow, user_data: *mut c_void) -> *mut GList;
    //pub fn gdk_window_peek_children      (window: *mut GdkWindow) -> *mut GList;
    pub fn gdk_window_get_events         (window: *mut GdkWindow) -> enums::EventMask;
    pub fn gdk_window_set_events         (window: *mut GdkWindow, event_mask: enums::EventMask);
    pub fn gdk_window_set_icon_name      (window: *mut GdkWindow, name: *const c_char);
    pub fn gdk_window_set_transient_for  (window: *mut GdkWindow, parent: *mut GdkWindow);
    pub fn gdk_window_set_role           (window: *mut GdkWindow, role: *const c_char);
    pub fn gdk_window_set_startup_id     (window: *mut GdkWindow, startup_id: *const c_char);
    pub fn gdk_window_set_group          (window: *mut GdkWindow, leader: *mut GdkWindow);
    pub fn gdk_window_get_group          (window: *mut GdkWindow) -> *mut GdkWindow;
    pub fn gdk_window_set_decorations    (window: *mut GdkWindow, decorations: enums::WMDecoration);
    pub fn gdk_window_get_decorations    (window: *mut GdkWindow, decorations: *mut enums::WMDecoration) -> gboolean;
    pub fn gdk_window_set_functions      (window: *mut GdkWindow, functions: enums::WMFunction);
    pub fn gdk_get_default_root_window   () -> *mut GdkWindow;
    pub fn gdk_window_get_support_multidevice(window: *mut GdkWindow) -> gboolean;
    pub fn gdk_window_set_support_multidevice(window: *mut GdkWindow, support_multidevice: gboolean);
    pub fn gdk_window_get_device_cursor  (window: *mut GdkWindow, device: *mut GdkDevice) -> *mut GdkCursor;
    pub fn gdk_window_set_device_cursor  (window: *mut GdkWindow, device: *mut GdkDevice, cursor: *mut GdkCursor);
    pub fn gdk_window_get_device_events  (window: *mut GdkWindow, device: *mut GdkDevice) -> enums::EventMask;
    pub fn gdk_window_set_device_events  (window: *mut GdkWindow, device: *mut GdkDevice, event_mask: enums::EventMask);
    pub fn gdk_window_get_source_events  (window: *mut GdkWindow, source: enums::InputSource) -> enums::EventMask;
    pub fn gdk_window_set_source_events  (window: *mut GdkWindow, source: enums::InputSource, event_mask: enums::EventMask);
    pub fn gdk_window_get_event_compression(window: *mut GdkWindow) -> gboolean;
    pub fn gdk_window_set_event_compression(window: *mut GdkWindow, event_compression: gboolean);
    //pub fn gdk_offscreen_window_get_surface(window: *mut GdkWindow) -> *mut cairo_surface_t;
    pub fn gdk_offscreen_window_set_embedder(window: *mut GdkWindow, embedder: *mut GdkWindow);
    pub fn gdk_offscreen_window_get_embedder(window: *mut GdkWindow) -> *mut GdkWindow;
    pub fn gdk_window_geometry_changed   (window: *mut GdkWindow);
    pub fn gdk_window_coords_from_parent (window: *mut GdkWindow, parent_x: c_double, parent_y: c_double, x: *mut c_double,
        y: *mut c_double);
    pub fn gdk_window_coords_to_parent   (window: *mut GdkWindow, x: c_double, y: c_double, parent_x: *mut c_double,
        parent_y: *mut c_double);
    pub fn gdk_window_get_effective_parent(window: *mut GdkWindow) -> *mut GdkWindow;
    pub fn gdk_window_get_effective_toplevel(window: *mut GdkWindow) -> *mut GdkWindow;
    pub fn gdk_window_get_type           () -> GType;

    // Callback
    //let GdkWindowInvalidateHandlerFunc = fn(window: *mut GdkWindow, region: *const cairo_region_t);
    //let GdkWindowChildFunc = fn(window: *mut GdkWindow, user_data: *mut c_void);
    //let GdkFilterFunc = fn(xevent: *mut GdkXEvent, event: *mut GdkEvent, data: *mut c_void) -> GdkFilterReturn;

    //=========================================================================
    // GdkDevice                                                         NOT OK
    //=========================================================================
    pub fn gdk_device_get_name             (device: *mut GdkDevice) -> *const c_char;
    pub fn gdk_device_get_source           (device: *mut GdkDevice) -> enums::InputSource;
    pub fn gdk_device_set_mode             (device: *mut GdkDevice, mode: enums::InputMode);
    pub fn gdk_device_get_mode             (device: *mut GdkDevice) -> enums::InputMode;
    pub fn gdk_device_set_key              (device: *mut GdkDevice, index_: c_uint, keyval: c_uint, modifiers: enums::modifier_type::ModifierType);
    pub fn gdk_device_get_key              (device: *mut GdkDevice, index_: c_uint, keyval: *mut c_uint,
        modifiers: *mut enums::modifier_type::ModifierType) -> gboolean;
    pub fn gdk_device_set_axis_use         (device: *mut GdkDevice, index_: c_uint, use_: enums::AxisUse);
    pub fn gdk_device_get_axis_use         (device: *mut GdkDevice, index_: c_uint) -> enums::AxisUse;
    pub fn gdk_device_get_associated_device(device: *mut GdkDevice) -> *mut GdkDevice;
    //pub fn gdk_device_list_slave_devices   (device: *mut GdkDevice) -> *mut GList;
    pub fn gdk_device_get_device_type      (device: *mut GdkDevice) -> enums::DeviceType;
    pub fn gdk_device_get_display          (device: *mut GdkDevice) -> *mut GdkDisplay;
    pub fn gdk_device_get_has_cursor       (device: *mut GdkDevice) -> gboolean;
    pub fn gdk_device_get_n_axes           (device: *mut GdkDevice) -> c_int;
    pub fn gdk_device_get_n_keys           (device: *mut GdkDevice) -> c_int;
    pub fn gdk_device_warp                 (device: *mut GdkDevice, screen: *mut GdkScreen, x: c_int, y: c_int);
    pub fn gdk_device_grab                 (device: *mut GdkDevice, window: *mut GdkWindow, grab_ownership: enums::GrabOwnership,
        owner_events: gboolean, event_mask: enums::EventMask, cursor: *mut GdkCursor, time_: u32) -> enums::GrabStatus;
    pub fn gdk_device_ungrab               (device: *mut GdkDevice, time_: u32);
    pub fn gdk_device_get_state            (device: *mut GdkDevice, window: *mut GdkWindow, axes: *mut c_double,
        mask: *mut enums::modifier_type::ModifierType);
    pub fn gdk_device_get_position         (device: *mut GdkDevice, screen: *mut *mut GdkScreen, x: *mut c_int, y: *mut c_int);
    pub fn gdk_device_get_position_double  (device: *mut GdkDevice, screen: *mut *mut GdkScreen, x: *mut c_double, y: *mut c_double);
    pub fn gdk_device_get_window_at_position(device: *mut GdkDevice, win_x: *mut c_int, win_y: *mut c_int) -> *mut GdkWindow;
    pub fn gdk_device_get_window_at_position_double(device: *mut GdkDevice, win_x: *mut c_double,
        win_y: *mut c_double) -> *mut GdkWindow;
    pub fn gdk_device_get_history          (device: *mut GdkDevice, window: *mut GdkWindow, start: u32, stop: u32,
        events: *mut *mut *mut GdkTimeCoord, n_events: *mut c_int);
    pub fn gdk_device_free_history         (events: *mut *mut GdkTimeCoord, n_events: c_int);
    pub fn gdk_device_get_axis             (device: *mut GdkDevice, axes: *mut c_double, use_: enums::AxisUse,
        value: *mut c_double) -> gboolean;
    //pub fn gdk_device_list_axes            (device: *mut GdkDevice) -> *mut GList;
    pub fn gdk_device_get_axis_value       (device: *mut GdkDevice, axes: *mut c_double, use_: GdkAtom,
        value: *mut c_double) -> gboolean;
    pub fn gdk_device_get_last_event_window(device: *mut GdkDevice) -> *mut GdkWindow;
    pub fn gdk_device_get_type             () -> GType;

    //=========================================================================
    // GdkDeviceManager                                                  NOT OK
    //=========================================================================
    pub fn gdk_disable_multidevice         ();
    pub fn gdk_device_manager_get_display  (device_manager: *mut GdkDeviceManager) -> *mut GdkDisplay;
    //pub fn gdk_device_manager_list_devices (device_manager: *mut GdkDeviceManager, type_: enums::DeviceType) -> *mut GList;
    pub fn gdk_device_manager_get_client_pointer(device_manager: *mut GdkDeviceManager) -> *mut GdkDevice;
    pub fn gdk_device_manager_get_type     () -> GType;

    //=========================================================================
    // GdkDisplay                                                        NOT OK
    //=========================================================================
    pub fn gdk_display_open                (display_name: *const c_char) -> *mut GdkDisplay;
    pub fn gdk_display_get_default         () -> *mut GdkDisplay;
    pub fn gdk_display_get_name            (display: *mut GdkDisplay) -> *const c_char;
    pub fn gdk_display_get_screen          (display: *mut GdkDisplay, screen_num: c_int) -> *mut GdkScreen;
    pub fn gdk_display_get_default_screen  (display: *mut GdkDisplay) -> *mut GdkScreen;
    pub fn gdk_display_get_device_manager  (display: *mut GdkDisplay) -> *mut GdkDeviceManager;
    pub fn gdk_display_device_is_grabbed   (display: *mut GdkDisplay, device: *mut GdkDevice) -> gboolean;
    pub fn gdk_display_beep                (display: *mut GdkDisplay);
    pub fn gdk_display_sync                (display: *mut GdkDisplay);
    pub fn gdk_display_flush               (display: *mut GdkDisplay);
    pub fn gdk_display_close               (display: *mut GdkDisplay);
    pub fn gdk_display_is_closed           (display: *mut GdkDisplay) -> gboolean;
    pub fn gdk_display_get_event           (display: *mut GdkDisplay) -> *mut GdkEvent;
    pub fn gdk_display_peek_event          (display: *mut GdkDisplay) -> *mut GdkEvent;
    pub fn gdk_display_put_event           (display: *mut GdkDisplay, event: *const GdkEvent);
    pub fn gdk_display_has_pending         (display: *mut GdkDisplay) -> gboolean;
    pub fn gdk_display_set_double_click_time(display: *mut GdkDisplay, msec: c_uint);
    pub fn gdk_display_set_double_click_distance(display: *mut GdkDisplay, distance: c_uint);
    pub fn gdk_display_supports_cursor_color(display: *mut GdkDisplay) -> gboolean;
    pub fn gdk_display_supports_cursor_alpha(display: *mut GdkDisplay) -> gboolean;
    pub fn gdk_display_get_default_cursor_size(display: *mut GdkDisplay) -> c_uint;
    pub fn gdk_display_get_maximal_cursor_size(display: *mut GdkDisplay, width: *mut c_uint, height: *mut c_uint);
    pub fn gdk_display_get_default_group   (display: *mut GdkDisplay) -> *mut GdkWindow;
    pub fn gdk_display_supports_selection_notification(display: *mut GdkDisplay) -> gboolean;
    pub fn gdk_display_request_selection_notification(display: *mut GdkDisplay, selection: GdkAtom) -> gboolean;
    pub fn gdk_display_supports_clipboard_persistence(display: *mut GdkDisplay) -> gboolean;
    pub fn gdk_display_store_clipboard     (display: *mut GdkDisplay, clipboard_window: *mut GdkWindow, time_: u32,
        targets: *const GdkAtom, n_targets: c_int) -> gboolean;
    pub fn gdk_display_supports_shapes     (display: *mut GdkDisplay) -> gboolean;
    pub fn gdk_display_supports_input_shapes(display: *mut GdkDisplay) -> gboolean;
    pub fn gdk_display_supports_composite  (display: *mut GdkDisplay) -> gboolean;
    pub fn gdk_display_get_app_launch_context(display: *mut GdkDisplay) -> *mut GdkAppLaunchContext;
    pub fn gdk_display_notify_startup_complete(display: *mut GdkDisplay, startup_id: *const c_char);
    pub fn gdk_display_get_type            () -> GType;

    //=========================================================================
    // GdkDisplayManager                                                 NOT OK
    //=========================================================================
    pub fn gdk_display_manager_get                () -> *mut GdkDisplayManager;
    pub fn gdk_display_manager_get_default_display(manager: *mut GdkDisplayManager) -> *mut GdkDisplay;
    pub fn gdk_display_manager_set_default_display(manager: *mut GdkDisplayManager, display: *mut GdkDisplay);
    //pub fn gdk_display_manager_list_displays      (manager: *mut GdkDisplayManager) -> *mut GSList;
    pub fn gdk_display_manager_open_display       (manager: *mut GdkDisplayManager, name: *const c_char) -> *mut GdkDisplay;
    pub fn gdk_display_manager_get_type           () -> GType;

    //=========================================================================
    // GdkScreen                                                         NOT OK
    //=========================================================================
    pub fn gdk_screen_get_default             () -> *mut GdkScreen;
    pub fn gdk_screen_get_system_visual       (screen: *mut GdkScreen) -> *mut GdkVisual;
    pub fn gdk_screen_get_rgba_visual         (screen: *mut GdkScreen) -> *mut GdkVisual;
    pub fn gdk_screen_is_composited           (screen: *mut GdkScreen) -> gboolean;
    pub fn gdk_screen_get_root_window         (screen: *mut GdkScreen) -> *mut GdkWindow;
    pub fn gdk_screen_get_display             (screen: *mut GdkScreen) -> *mut GdkDisplay;
    pub fn gdk_screen_get_number              (screen: *mut GdkScreen) -> c_int;
    pub fn gdk_screen_get_width               (screen: *mut GdkScreen) -> c_int;
    pub fn gdk_screen_get_height              (screen: *mut GdkScreen) -> c_int;
    pub fn gdk_screen_get_width_mm            (screen: *mut GdkScreen) -> c_int;
    pub fn gdk_screen_get_height_mm           (screen: *mut GdkScreen) -> c_int;
    //pub fn gdk_screen_list_visuals            (screen: *mut GdkScreen) -> *mut GList;
    //pub fn gdk_screen_get_toplevel_windows    (screen: *mut GdkScreen) -> *mut GList;
    pub fn gdk_screen_make_display_name       (screen: *mut GdkScreen) -> *mut c_char;
    pub fn gdk_screen_get_n_monitors          (screen: *mut GdkScreen) -> c_int;
    pub fn gdk_screen_get_primary_monitor     (screen: *mut GdkScreen) -> c_int;
    pub fn gdk_screen_get_monitor_geometry    (screen: *mut GdkScreen, monitor_num: c_int, dest: *mut GdkRectangle);
    pub fn gdk_screen_get_monitor_workarea    (screen: *mut GdkScreen, monitor_num: c_int, dest: *mut GdkRectangle);
    pub fn gdk_screen_get_monitor_at_point    (screen: *mut GdkScreen, x: c_int, y: c_int) -> c_int;
    pub fn gdk_screen_get_monitor_at_window   (screen: *mut GdkScreen, window: *mut GdkWindow) -> c_int;
    pub fn gdk_screen_get_monitor_height_mm   (screen: *mut GdkScreen, monitor_num: c_int) -> c_int;
    pub fn gdk_screen_get_monitor_width_mm    (screen: *mut GdkScreen, monitor_num: c_int) -> c_int;
    pub fn gdk_screen_get_monitor_plug_name   (screen: *mut GdkScreen, monitor_num: c_int) -> *mut c_char;
    pub fn gdk_screen_get_monitor_scale_factor(screen: *mut GdkScreen, monitor_num: c_int) -> c_int;
    //pub fn gdk_screen_get_setting             (screen: *mut GdkScreen, name: *const c_char, value: *mut GValue) -> gboolean;
    //pub fn gdk_screen_get_font_options        (screen: *mut GdkScreen) -> *const cairo_font_options_t;
    //pub fn gdk_screen_set_font_options        (screen: *mut GdkScreen, options: *const cairo_font_options_t);
    pub fn gdk_screen_get_resolution          (screen: *mut GdkScreen) -> c_double;
    pub fn gdk_screen_set_resolution          (screen: *mut GdkScreen, dpi: c_double);
    pub fn gdk_screen_get_active_window       (screen: *mut GdkScreen) -> *mut GdkWindow;
    //pub fn gdk_screen_get_window_stack        (screen: *mut GdkScreen) -> *mut GList;
    pub fn gdk_screen_get_type                () -> GType;

    //=========================================================================
    // GdkVisual                                                         NOT OK
    //=========================================================================
    pub fn gdk_query_depths                   (depths: *mut *mut c_int, count: *mut c_int);
    //pub fn gdk_query_visual_types             (visual_types: *mut *mut GdkVisualType, count: *mut c_int);
    //pub fn gdk_list_visuals                   () -> *mut GList;
    pub fn gdk_visual_get_bits_per_rgb        (visual: *mut GdkVisual) -> c_int;
    pub fn gdk_visual_get_blue_pixel_details  (visual: *mut GdkVisual, mask: *mut u32, shift: *mut c_int, precision: *mut c_int);
    //pub fn gdk_visual_get_byte_order          (visual: *mut GdkVisual) -> GdkByteOrder;
    pub fn gdk_visual_get_colormap_size       (visual: *mut GdkVisual) -> c_int;
    pub fn gdk_visual_get_depth               (visual: *mut GdkVisual) -> c_int;
    pub fn gdk_visual_get_green_pixel_details (visual: *mut GdkVisual, mask: *mut u32, shift: *mut c_int, precision: *mut c_int);
    pub fn gdk_visual_get_red_pixel_details   (visual: *mut GdkVisual, mask: *mut u32, shift: *mut c_int, precision: *mut c_int);
    //pub fn gdk_visual_get_visual_type         (visual: *mut GdkVisual) -> GdkVisualType;
    pub fn gdk_visual_get_best_depth          () -> c_int;
    //pub fn gdk_visual_get_best_type           () -> GdkVisualType;
    pub fn gdk_visual_get_system              () -> *mut GdkVisual;
    pub fn gdk_visual_get_best                () -> *mut GdkVisual;
    pub fn gdk_visual_get_best_with_depth     (depth: c_int) -> *mut GdkVisual;
    //pub fn gdk_visual_get_best_with_type      (visual_type: GdkVisualType) -> *mut GdkVisual;
    //pub fn gdk_visual_get_best_with_both      (depth: c_int, visual_type: GdkVisualType) -> *mut GdkVisual;
    pub fn gdk_visual_get_screen              (visual: *mut GdkVisual) -> *mut GdkScreen;
    pub fn gdk_visual_get_type                () -> GType;

    //=========================================================================
    // GdkCursor                                                         NOT OK
    //=========================================================================
    pub fn gdk_cursor_new                     (cursor_type: enums::CursorType) -> *mut GdkCursor;
    pub fn gdk_cursor_new_from_pixbuf         (display: *mut GdkDisplay, pixbuf: *mut GdkPixbuf, x: c_int, y: c_int) -> *mut GdkCursor;
    //pub fn gdk_cursor_new_from_surface        (display: *mut GdkDisplay, surface: *mut cairo_surface_t, x: c_double,
    //    y: c_double) -> *mut GdkCursor;
    pub fn gdk_cursor_new_from_name           (display: *mut GdkDisplay, name: *const c_char) -> *mut GdkCursor;
    pub fn gdk_cursor_new_for_display         (display: *mut GdkDisplay, cursor_type: enums::CursorType) -> *mut GdkCursor;
    pub fn gdk_cursor_get_display             (cursor: *mut GdkCursor) -> *mut GdkDisplay;
    pub fn gdk_cursor_get_image               (cursor: *mut GdkCursor) -> *mut GdkPixbuf;
    //pub fn gdk_cursor_get_surface             (cursor: *mut GdkCursor, x_hot: *mut c_double, y_hot: *mut c_double) -> *mut cairo_surface_t;
    pub fn gdk_cursor_get_cursor_type         (cursor: *mut GdkCursor) -> enums::CursorType;
    pub fn gdk_cursor_get_type                () -> GType;

    //=========================================================================
    // GdkPixbuf                                                         NOT OK
    //=========================================================================
    pub fn gdk_pixbuf_new(colorspace: enums::ColorSpace, has_alpha: gboolean,
        bits_per_sample: c_int, width: c_int, height: c_int) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_data(data: *mut c_uchar, colorspace: enums::ColorSpace,
        has_alpha: gboolean, bits_per_sample: c_int, width: c_int, height: c_int, row_stride: c_int,
        destroy_fn: GdkPixbufDestroyNotify, destroy_fn_data: gpointer) -> *mut GdkPixbuf;
    
    #[cfg(not(target_os = "windows"))]
    pub fn gdk_pixbuf_new_from_file(filename: *const c_char, error: *mut *mut glib_ffi::GError)
        -> *mut GdkPixbuf;
    #[cfg(not(target_os = "windows"))]
    pub fn gdk_pixbuf_new_from_file_at_size(filename: *const c_char, width: c_int, height: c_int,
        error: *mut *mut glib_ffi::GError) -> *mut GdkPixbuf;
    #[cfg(not(target_os = "windows"))]
    pub fn gdk_pixbuf_new_from_file_at_scale(filename: *const c_char, width: c_int, height: c_int,
        preserve_aspect_ratio: gboolean, error: *mut *mut glib_ffi::GError) -> *mut GdkPixbuf;

    #[cfg(target_os = "windows")]
    pub fn gdk_pixbuf_new_from_file_utf8(filename: *const c_char,
        error: *mut *mut glib_ffi::GError) -> *mut GdkPixbuf;
    #[cfg(target_os = "windows")]
    pub fn gdk_pixbuf_new_from_file_at_size_utf8(filename: *const c_char, width: c_int,
        height: c_int, error: *mut *mut glib_ffi::GError) -> *mut GdkPixbuf;
    #[cfg(target_os = "windows")]
    pub fn gdk_pixbuf_new_from_file_at_scale_utf8(filename: *const c_char, width: c_int,
        height: c_int, preserve_aspect_ratio: gboolean, error: *mut *mut glib_ffi::GError)
        -> *mut GdkPixbuf;

    pub fn gdk_pixbuf_get_file_info(filename: *const c_char, width: *mut c_int, height: *mut c_int) -> *mut GdkPixbufFormat;
    pub fn gdk_pixbuf_new_from_resource(resource_path: *const c_char, error: *mut *mut glib_ffi::GError) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_resource_at_scale(resource_path: *const c_char, width: c_int, height: c_int,
        preserve_aspect_ratio: gboolean, error: *mut *mut glib_ffi::GError) -> *mut GdkPixbuf;
    /*pub fn gdk_pixbuf_new_from_stream(stream: *mut GInputStream, cancellable: *mut GCancellable,
        error: *mut *mut glib_ffi::GError) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_stream_async(stream: *mut GInputStream, cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback, error: *mut *mut glib_ffi::GError) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_stream_finish(async_result: *mut GAsyncResult, error: *mut *mut glib_ffi::GError) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_stream_at_scale(stream: *mut GInputStream, width: c_int, height: c_int,
        preserve_aspect_ratio: gboolean, cancellable: *mut GCancellable, error: *mut *mut glib_ffi::GError) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_stream_at_scale_async(stream: *mut GInputStream, width: c_int, height: c_int,
        preserve_aspect_ratio: gboolean, cancellable: *mut GCancellable, callback: GAsyncReadyCallback,
        error: *mut *mut glib_ffi::GError) -> *mut GdkPixbuf;*/

    pub fn gdk_pixbuf_new_subpixbuf(src_pixbuf: *mut GdkPixbuf, src_x: c_int, src_y: c_int,
        width: c_int, height: c_int) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_get_colorspace(pixbuf: *const GdkPixbuf) -> enums::ColorSpace;
    pub fn gdk_pixbuf_get_n_channels(pixbuf: *const GdkPixbuf) -> c_int;
    pub fn gdk_pixbuf_get_has_alpha(pixbuf: *const GdkPixbuf) -> gboolean;
    pub fn gdk_pixbuf_get_bits_per_sample(pixbuf: *const GdkPixbuf) -> c_int;
    //pub fn gdk_pixbuf_get_pixels(pixbuf: *const GdkPixbuf) -> *mut c_uchar;
    pub fn gdk_pixbuf_get_pixels_with_length(pixbuf: *const GdkPixbuf, length: *mut c_uint)
        -> *mut c_uchar;
    pub fn gdk_pixbuf_get_width(pixbuf: *const GdkPixbuf) -> c_int;
    pub fn gdk_pixbuf_get_height(pixbuf: *const GdkPixbuf) -> c_int;
    pub fn gdk_pixbuf_get_rowstride(pixbuf: *const GdkPixbuf) -> c_int;
    pub fn gdk_pixbuf_get_byte_length(pixbuf: *const GdkPixbuf) -> c_ulong;
    pub fn gdk_pixbuf_get_option(pixbuf: *const GdkPixbuf, key: *const c_char) -> *const c_char;
    pub fn gdk_pixbuf_get_type() -> GType;

    //=========================================================================
    // GdkRectangle                                                      NOT OK
    //=========================================================================
    pub fn gdk_rectangle_intersect            (src1: *const GdkRectangle, src2: *const GdkRectangle,
        dest: *mut GdkRectangle) -> gboolean;
    pub fn gdk_rectangle_union                (src1: *const GdkRectangle, src2: *const GdkRectangle, dest: *mut GdkRectangle);

    //=========================================================================
    // GdkRGBA                                                           NOT OK
    //=========================================================================
    //pub fn gdk_rgba_copy                      (rgba: *const GdkRGBA) -> *mut GdkRGBA;
    //pub fn gdk_rgba_free                      (rgba: *mut GdkRGBA);
    pub fn gdk_rgba_parse                       (rgba: *mut GdkRGBA, spec: *const c_char) -> gboolean;
    pub fn gdk_rgba_equal                       (p1: *const GdkRGBA, p2: *const GdkRGBA) -> gboolean;
    pub fn gdk_rgba_hash                        (p: *const GdkRGBA) -> c_uint;
    pub fn gdk_rgba_to_string                   (rgba: *const GdkRGBA) -> *mut c_char;

    //=========================================================================
    // GdkFrameClock                                                     NOT OK
    //=========================================================================
    pub fn gdk_frame_clock_get_frame_time       (frame_clock: *mut GdkFrameClock) -> i64;
    pub fn gdk_frame_clock_request_phase        (frame_clock: *mut GdkFrameClock, phase: enums::FrameClockPhase);
    pub fn gdk_frame_clock_begin_updating       (frame_clock: *mut GdkFrameClock);
    pub fn gdk_frame_clock_end_updating         (frame_clock: *mut GdkFrameClock);
    pub fn gdk_frame_clock_get_frame_counter    (frame_clock: *mut GdkFrameClock) -> i64;
    pub fn gdk_frame_clock_get_history_start    (frame_clock: *mut GdkFrameClock) -> i64;
    pub fn gdk_frame_clock_get_timings          (frame_clock: *mut GdkFrameClock, frame_counter: i64) -> *mut GdkFrameTimings;
    pub fn gdk_frame_clock_get_current_timings  (frame_clock: *mut GdkFrameClock) -> *mut GdkFrameTimings;
    pub fn gdk_frame_clock_get_refresh_info     (frame_clock: *mut GdkFrameClock, base_time: i64, refresh_interval_return: *mut i64,
        presentation_time_return: *mut i64);
    pub fn gdk_frame_clock_get_type             () -> GType;

    //=========================================================================
    // GdkFrameTimings                                                   NOT OK
    //=========================================================================
    // Since 3.8
    pub fn gdk_frame_timings_ref                  (timings: *mut GdkFrameTimings) -> *mut GdkFrameTimings;
    // Since 3.8
    pub fn gdk_frame_timings_unref                (timings: *mut GdkFrameTimings);
    // Since 3.8
    pub fn gdk_frame_timings_get_frame_counter    (timings: *mut GdkFrameTimings) -> i64;
    // Since 3.8
    pub fn gdk_frame_timings_get_complete         (timings: *mut GdkFrameTimings) -> gboolean;
    pub fn gdk_frame_timings_get_frame_time       (timings: *mut GdkFrameTimings) -> i64;
    // Since 3.8
    pub fn gdk_frame_timings_get_presentation_time(timings: *mut GdkFrameTimings) -> i64;
    // Since 3.8
    pub fn gdk_frame_timings_get_refresh_interval (timings: *mut GdkFrameTimings) -> i64;
    // Since 3.8
    pub fn gdk_frame_timings_get_predicted_presentation_time(timings: *mut GdkFrameTimings) -> i64;
    pub fn gdk_frame_timings_get_type             () -> GType;

    //=========================================================================
    // GdkAtom                                                           NOT OK
    //=========================================================================
    //pub fn gdk_text_property_to_utf8_list_for_display(display: *mut GdkDisplay, encoding: GdkAtom, format: c_int,
    //    text: *const c_uchar, length: c_int, list: *mut *mut *mut c_char) -> c_int;
    pub fn gdk_utf8_to_string_target               (str: *const c_char) -> *mut c_char;
    pub fn gdk_atom_intern                         (atom_name: *const c_char, only_if_exists: gboolean) -> GdkAtom;
    pub fn gdk_atom_intern_static_string           (atom_name: *const c_char) -> GdkAtom;
    pub fn gdk_atom_name                           (atom: GdkAtom) -> *mut c_char;
    //pub fn gdk_property_get                        (window: *mut GdkWindow, atom: GdkAtom, type_: GdkAtom, offset: c_ulong,
    //    length: c_ulong, pdelete: c_int, actual_property_type: *mut GdkAtom, actual_format: *mut c_int, actual_length: *mut c_int,
    //    data: *mut *mut c_uchar) -> gboolean;
    //pub fn gdk_property_change                     (window: *mut GdkWindow, property: GdkAtom, type_: GdkAtom, format: c_int,
    //    mode: enums::PropMode, data: *const c_uchar, nelements: c_int);
    //pub fn gdk_property_delete                     (window: *mut GdkWindow, property: GdkAtom);

    //=========================================================================
    // GdkDragContext                                                    NOT OK
    //=========================================================================
    pub fn gdk_drag_get_selection                  (context: *mut GdkDragContext) -> GdkAtom;
    pub fn gdk_drag_abort                          (context: *mut GdkDragContext, time_: u32);
    pub fn gdk_drop_reply                          (context: *mut GdkDragContext, accepted: gboolean, time_: u32);
    pub fn gdk_drag_drop                           (context: *mut GdkDragContext, time_: u32);
    pub fn gdk_drag_find_window_for_screen         (context: *mut GdkDragContext, drag_window: *mut GdkWindow, screen: *mut GdkScreen,
        x_root: c_int, y_root: c_int, dest_window: *mut *mut GdkWindow, protocol: *mut enums::DragProtocol);
    //pub fn gdk_drag_begin                          (window: *mut GdkWindow, targets: *mut GList) -> *mut GdkDragContext;
    //pub fn gdk_drag_begin_for_device               (window: *mut GdkWindow, device: *mut GdkDevice,
    //    targets: *mut GList) -> *mut GdkDragContext;
    pub fn gdk_drag_motion                         (context: *mut GdkDragContext, dest_window: *mut GdkWindow, protocol: enums::DragProtocol,
        x_root: c_int, y_root: c_int, suggested_action: enums::DragAction, possible_actions: enums::DragAction,
        time_: u32) -> gboolean;
    pub fn gdk_drop_finish                         (context: *mut GdkDragContext, success: gboolean, time_: u32);
    pub fn gdk_drag_status                         (context: *mut GdkDragContext, action: enums::DragAction, time_: u32);
    pub fn gdk_drag_drop_succeeded                 (context: *mut GdkDragContext) -> gboolean;
    pub fn gdk_window_get_drag_protocol            (window: *mut GdkWindow, target: *mut *mut GdkWindow) -> enums::DragProtocol;
    pub fn gdk_drag_context_get_actions            (context: *mut GdkDragContext) -> enums::DragAction;
    pub fn gdk_drag_context_get_suggested_action   (context: *mut GdkDragContext) -> enums::DragAction;
    pub fn gdk_drag_context_get_selected_action    (context: *mut GdkDragContext) -> enums::DragAction;
    //pub fn gdk_drag_context_list_targets           (context: *mut GdkDragContext) -> *mut GList;
    pub fn gdk_drag_context_get_device             (context: *mut GdkDragContext) -> *mut GdkDevice;
    pub fn gdk_drag_context_set_device             (context: *mut GdkDragContext, device: *mut GdkDevice);
    pub fn gdk_drag_context_get_source_window      (context: *mut GdkDragContext) -> *mut GdkWindow;
    pub fn gdk_drag_context_get_dest_window        (context: *mut GdkDragContext) -> *mut GdkWindow;
    pub fn gdk_drag_context_get_protocol           (context: *mut GdkDragContext) -> enums::DragProtocol;
    pub fn gdk_drag_context_get_type               () -> GType;

    //=========================================================================
    // GdkAppLaunchContext                                               NOT OK
    //=========================================================================
    pub fn gdk_app_launch_context_set_screen       (context: *mut GdkAppLaunchContext, screen: *mut GdkScreen);
    pub fn gdk_app_launch_context_set_desktop      (context: *mut GdkAppLaunchContext, desktop: c_int);
    pub fn gdk_app_launch_context_set_timestamp    (context: *mut GdkAppLaunchContext, timestamp: u32);
    //pub fn gdk_app_launch_context_set_icon         (context: *mut GdkAppLaunchContext, icon: *mut GIcon);
    pub fn gdk_app_launch_context_set_icon_name    (context: *mut GdkAppLaunchContext, icon_name: *const c_char);
    pub fn gdk_app_launch_context_get_type         () -> GType;

    //=========================================================================
    // Gdk Key Handling                                                  NOT OK
    //=========================================================================
    pub fn gdk_keyval_name                         (keyval:c_uint) -> *mut c_char;

    //=========================================================================
    // GdkPixbufLoader                                                   NOT OK
    //=========================================================================
    pub fn gdk_pixbuf_loader_new                    () -> *mut GdkPixbufLoader;
    pub fn gdk_pixbuf_loader_new_with_type          (image_type: *const c_char, error: *mut *mut glib_ffi::GError) -> *mut GdkPixbufLoader;
    pub fn gdk_pixbuf_loader_new_with_mime_type     (mime_type: *const c_char, error: *mut *mut glib_ffi::GError) -> *mut GdkPixbufLoader;
    pub fn gdk_pixbuf_loader_get_format             (loader: *mut GdkPixbufLoader) -> *mut GdkPixbufFormat;
    pub fn gdk_pixbuf_loader_write                  (loader: *mut GdkPixbufLoader, buf: *const u8, count: gsize,
        error: *mut *mut glib_ffi::GError) -> gboolean;
    //pub fn gdk_pixbuf_loader_write_bytes            (loader: *mut GdkPixbufLoader, buffer: glib_ffi::GBytes,
    //    error: *mut *mut glib_ffi::GError) -> gboolean;
    pub fn gdk_pixbuf_loader_set_size               (loader: *mut GdkPixbufLoader, width: c_int, height: c_int);
    pub fn gdk_pixbuf_loader_get_pixbuf             (loader: *mut GdkPixbufLoader) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_loader_get_animation          (loader: *mut GdkPixbufLoader) -> *mut GdkPixbufAnimation;
    pub fn gdk_pixbuf_loader_close                  (loader: *mut GdkPixbufLoader, error: *mut *mut glib_ffi::GError) -> gboolean;
    pub fn gdk_pixbuf_loader_get_type               () -> GType;

    //=========================================================================
    // GdkPixbufFormat                                                   NOT OK
    //=========================================================================
    pub fn gdk_pixbuf_format_copy                   (format: *const GdkPixbufFormat) -> *mut GdkPixbufFormat;
    pub fn gdk_pixbuf_format_free                   (format: *mut GdkPixbufFormat);
    pub fn gdk_pixbuf_format_get_name               (format: *mut GdkPixbufFormat) -> *mut c_char;
    pub fn gdk_pixbuf_format_get_description        (format: *mut GdkPixbufFormat) -> *mut c_char;
    pub fn gdk_pixbuf_format_get_mime_types         (format: *mut GdkPixbufFormat) -> *mut *mut c_char;
    pub fn gdk_pixbuf_format_get_extensions         (format: *mut GdkPixbufFormat) -> *mut *mut c_char;
    pub fn gdk_pixbuf_format_is_writable            (format: *mut GdkPixbufFormat) -> gboolean;
    pub fn gdk_pixbuf_format_is_scalable            (format: *mut GdkPixbufFormat) -> gboolean;
    pub fn gdk_pixbuf_format_is_disabled            (format: *mut GdkPixbufFormat) -> gboolean;
    pub fn gdk_pixbuf_format_set_disabled           (format: *mut GdkPixbufFormat, disabled: gboolean);
    pub fn gdk_pixbuf_format_get_license            (format: *mut GdkPixbufFormat) -> *mut c_char;

    //=========================================================================
    // GdkPixbufAnimation                                                NOT OK
    //=========================================================================
    #[cfg(not(target_os = "windows"))]
    pub fn gdk_pixbuf_animation_new_from_file       (file: *const c_char, error: *mut *mut glib_ffi::GError) -> *mut GdkPixbufAnimation;
    #[cfg(target_os = "windows")]
    pub fn gdk_pixbuf_animation_new_from_file_utf8  (file: *const c_char, error: *mut *mut glib_ffi::GError) -> *mut GdkPixbufAnimation;

    pub fn gdk_pixbuf_animation_new_from_resource   (resource_path: *const c_char, error: *mut *mut glib_ffi::GError) -> *mut GdkPixbufAnimation;
    //pub fn gdk_pixbuf_animation_new_from_stream     (stream: *mut GInputStream, cancellable: GCancellable,
    //    error: *mut *mut glib_ffi::GError) -> *mut GdkPixbufAnimation;
    //pub fn gdk_pixbuf_animation_new_from_stream_async(stream: *mut GInputStream, cancellable: *mut GCancellable, callback, user_data: *mut c_void);
    //pub fn gdk_pixbuf_animation_new_from_stream_finish(async_result: *mut GAsyncResult,
    //    error: *mut *mut glib_ffi::GError) -> *mut GdkPixbufAnimation;
    pub fn gdk_pixbuf_animation_get_width           (animation: *mut GdkPixbufAnimation) -> c_int;
    pub fn gdk_pixbuf_animation_get_height          (animation: *mut GdkPixbufAnimation) -> c_int;
    pub fn gdk_pixbuf_animation_get_iter            (animation: *mut GdkPixbufAnimation,
        start_time: *const /*glib::TimeVal*/c_void) -> *mut GdkPixbufAnimationIter;
    pub fn gdk_pixbuf_animation_is_static_image     (animation: *mut GdkPixbufAnimation) -> gboolean;
    pub fn gdk_pixbuf_animation_get_static_image    (animation: *mut GdkPixbufAnimation) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_animation_get_type            () -> GType;

    //=========================================================================
    // GdkPixbufIter                                                     NOT OK
    //=========================================================================
    pub fn gdk_pixbuf_animation_iter_advance        (iter: *mut GdkPixbufAnimationIter,
        start_time: *const /*glib_ffi::GTimeVal*/c_void) -> gboolean;
    pub fn gdk_pixbuf_animation_iter_get_delay_time  (iter: *mut GdkPixbufAnimationIter) -> c_int;
    pub fn gdk_pixbuf_animation_iter_on_currently_loading_frame(iter: *mut GdkPixbufAnimationIter) -> gboolean;
    pub fn gdk_pixbuf_animation_iter_get_pixbuf      (iter: *mut GdkPixbufAnimationIter) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_animation_iter_get_type        () -> GType;

    //=========================================================================
    // GdkPixbufSimpleAnim                                               NOT OK
    //=========================================================================
    pub fn gdk_pixbuf_simple_anim_new                (width: c_int, height: c_int, rate: c_float) -> *mut GdkPixbufSimpleAnim;
    pub fn gdk_pixbuf_simple_anim_add_frame          (animation: *mut GdkPixbufSimpleAnim, pixbuf: *mut GdkPixbuf);
    pub fn gdk_pixbuf_simple_anim_set_loop           (animation: *mut GdkPixbufSimpleAnim, loop_: gboolean);
    pub fn gdk_pixbuf_simple_anim_get_loop           (animation: *mut GdkPixbufSimpleAnim) -> gboolean;
    pub fn gdk_pixbuf_simple_anim_get_type           () -> GType;
}

#[cfg(target_os = "windows")]
pub unsafe fn gdk_pixbuf_new_from_file(filename: *const c_char,
        error: *mut *mut glib_ffi::GError) -> *mut GdkPixbuf {
    gdk_pixbuf_new_from_file_utf8(filename, error)
}

#[cfg(target_os = "windows")]
pub unsafe fn gdk_pixbuf_new_from_file_at_size(filename: *const c_char, width: c_int,
        height: c_int, error: *mut *mut glib_ffi::GError) -> *mut GdkPixbuf {
    gdk_pixbuf_new_from_file_at_size_utf8(filename, width, height, error)
}

#[cfg(target_os = "windows")]
pub unsafe fn gdk_pixbuf_new_from_file_at_scale(filename: *const c_char, width: c_int,
        height: c_int, preserve_aspect_ratio: gboolean, error: *mut *mut glib_ffi::GError)
        -> *mut GdkPixbuf {
    gdk_pixbuf_new_from_file_at_scale_utf8(filename, width, height, preserve_aspect_ratio, error)
}

#[cfg(target_os = "windows")]
pub unsafe fn gdk_pixbuf_animation_new_from_file(file: *const c_char,
        error: *mut *mut glib_ffi::GError) -> *mut GdkPixbufAnimation {
    gdk_pixbuf_animation_new_from_file_utf8(file, error)
}

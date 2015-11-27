// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate gdk_pixbuf_sys as gdk_pixbuf_ffi;
extern crate gdk_sys as gdk_ffi;
#[macro_use]
extern crate glib as glib_main;
extern crate cairo;
extern crate libc;

pub use gdk_ffi as ffi;
pub use glib_main as glib;

#[macro_use]
mod rt;

mod events;
mod keys;
mod rectangle;

pub mod prelude;
pub mod enums;

pub mod app_launch_context;
pub mod atom;
pub mod cursor;
pub mod device;
pub mod device_manager;
pub mod display;
pub mod display_manager;
pub mod drag_context;
#[cfg(gdk_3_8)]
pub mod frame_clock;
#[cfg(gdk_3_8)]
pub mod frame_timings;
pub mod pixbuf;
pub mod rgba;
pub mod screen;
pub mod visual;
pub mod window;
pub mod cairo_interaction;
#[cfg(gdk_3_16)]
pub mod gl_context;

pub use gdk_ffi::GdkColor as Color;

pub use self::rt::{
    init,
    set_initialized,
    get_display_arg_name,
    notify_startup_complete,
    notify_startup_complete_with_id,
    get_program_class,
    set_program_class,
    flush,
    screen_width,
    screen_height,
    screen_width_mm,
    screen_height_mm,
    beep,
    error_trap_push,
    error_trap_pop,
    error_trap_pop_ignored
};
#[cfg(gdk_3_10)]
pub use self::rt::set_allowed_backends;

pub use app_launch_context::AppLaunchContext;
pub use atom::Atom;
pub use cursor::Cursor;
pub use device::Device;
pub use device_manager::DeviceManager;
pub use display::Display;
pub use display_manager::DisplayManager;
pub use drag_context::DragContext;
#[cfg(gdk_3_8)]
pub use frame_clock::FrameClock;
#[cfg(gdk_3_8)]
pub use frame_timings::FrameTimings;
pub use pixbuf::Pixbuf;
pub use pixbuf::animation::PixbufAnimation;
pub use pixbuf::animation::PixbufSimpleAnim;
pub use pixbuf::format::PixbufFormat;
pub use pixbuf::loader::PixbufLoader;
pub use screen::Screen;
pub use visual::Visual;
pub use window::Window;
#[cfg(gdk_3_16)]
pub use gl_context::GLContext;

pub use self::events::{
    EventType,
    Event,
    EventAny,
    EventExpose,
    EventVisibility,
    EventMotion,
    EventButton,
    EventTouch,
    EventScroll,
    EventKey,
    EventCrossing,
    EventFocus,
    EventConfigure,
    EventProperty,
    EventSelection,
    EventOwnerChange,
    EventProximity,
    EventSetting,
    EventWindowState,
    EventGrabBroken,
    EventDND,
    VisibilityState,
    ScrollDirection,
    NotifyType,
    CrossingMode,
    PropertyState,
    SettingAction,
    OwnerChange
};


pub use gdk_ffi::GdkAxisUse as AxisUse;
pub use gdk_ffi::GdkDragAction as DragAction;
pub use gdk_ffi::GdkDragProtocol as DragProtocol;
pub use gdk_ffi::GdkEventMask as EventMask;
pub use gdk_ffi::GdkFullscreenMode as FullscreenMode;
pub use gdk_ffi::GdkGrabOwnership as GrabOwnership;
pub use gdk_ffi::GdkGrabStatus as GrabStatus;
pub use gdk_ffi::GdkGravity as Gravity;
pub use gdk_ffi::GdkInputMode as InputMode;
pub use gdk_ffi::GdkInputSource as InputSource;
pub use gdk_ffi::GdkModifierIntent as ModifierIntent;
pub use gdk_ffi::GdkModifierType as ModifierType;
pub use gdk_ffi::GdkWMDecoration as WMDecoration;
pub use gdk_ffi::GdkWMFunction as WMFunction;
pub use gdk_ffi::GdkWindowEdge as WindowEdge;
pub use gdk_ffi::GdkWindowHints as WindowHints;
pub use gdk_ffi::GdkWindowState as WindowState;
pub use gdk_ffi::GdkWindowType as WindowType;
pub use gdk_ffi::GdkWindowTypeHint as WindowTypeHint;
pub use gdk_ffi::GdkWindowWindowClass as WindowWindowClass;
pub use gdk_pixbuf_ffi::GdkColorspace as Colorspace;
pub use gdk_pixbuf_ffi::GdkInterpType as InterpType;
pub use gdk_pixbuf_ffi::GdkPixbufAlphaMode as PixbufAlphaMode;
pub use gdk_pixbuf_ffi::GdkPixbufError as PixbufError;

#[allow(non_camel_case_types)]
pub type key = i32;

pub use self::keys::{
    keyval_name,
    keyval_to_unicode
};

// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate gdk_sys as gdk_ffi;
extern crate gdk_pixbuf;
#[macro_use]
extern crate glib;
extern crate cairo;
extern crate libc;

pub use gdk_pixbuf as pixbuf;
pub use gdk_ffi as ffi;

#[macro_use]
mod rt;

#[macro_use]
pub mod event;
mod event_button;
mod event_configure;
mod event_crossing;
mod event_dnd;
mod event_expose;
mod event_focus;
mod event_grab_broken;
mod event_key;
mod event_motion;
mod event_owner_change;
mod event_property;
mod event_proximity;
mod event_scroll;
mod event_selection;
mod event_setting;
mod event_touch;
mod event_visibility;
mod event_window_state;

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
pub mod rgba;
pub mod screen;
pub mod visual;
pub mod window;
pub mod cairo_interaction;
#[cfg(gdk_3_16)]
pub mod gl_context;

pub use gdk_ffi::GdkColor as Color;
pub use gdk_ffi::GdkRGBA as RGBA;

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
pub use event::Event;
pub use event_button::EventButton;
pub use event_configure::EventConfigure;
pub use event_crossing::EventCrossing;
pub use event_dnd::EventDND;
pub use event_expose::EventExpose;
pub use event_focus::EventFocus;
pub use event_grab_broken::EventGrabBroken;
pub use event_key::EventKey;
pub use event_motion::EventMotion;
pub use event_owner_change::EventOwnerChange;
pub use event_property::EventProperty;
pub use event_proximity::EventProximity;
pub use event_scroll::EventScroll;
pub use event_selection::EventSelection;
pub use event_setting::EventSetting;
pub use event_touch::EventTouch;
pub use event_visibility::EventVisibility;
pub use event_window_state::EventWindowState;
#[cfg(gdk_3_8)]
pub use frame_clock::FrameClock;
#[cfg(gdk_3_8)]
pub use frame_timings::FrameTimings;
pub use screen::Screen;
pub use visual::Visual;
pub use window::Window;
#[cfg(gdk_3_16)]
pub use gl_context::GLContext;

pub use gdk_ffi::GdkAxisUse as AxisUse;
pub use gdk_ffi::GdkCrossingMode as CrossingMode;
pub use gdk_ffi::GdkDragAction as DragAction;
pub use gdk_ffi::GdkDragProtocol as DragProtocol;
pub use gdk_ffi::GdkEventMask as EventMask;
pub use gdk_ffi::GdkEventType as EventType;
pub use gdk_ffi::GdkFullscreenMode as FullscreenMode;
pub use gdk_ffi::GdkGrabOwnership as GrabOwnership;
pub use gdk_ffi::GdkGrabStatus as GrabStatus;
pub use gdk_ffi::GdkGravity as Gravity;
pub use gdk_ffi::GdkInputMode as InputMode;
pub use gdk_ffi::GdkInputSource as InputSource;
pub use gdk_ffi::GdkModifierIntent as ModifierIntent;
pub use gdk_ffi::GdkModifierType as ModifierType;
pub use gdk_ffi::GdkNotifyType as NotifyType;
pub use gdk_ffi::GdkOwnerChange as OwnerChange;
pub use gdk_ffi::GdkPropertyState as PropertyState;
pub use gdk_ffi::GdkScrollDirection as ScrollDirection;
pub use gdk_ffi::GdkSettingAction as SettingAction;
pub use gdk_ffi::GdkVisibilityState as VisibilityState;
pub use gdk_ffi::GdkWMDecoration as WMDecoration;
pub use gdk_ffi::GdkWMFunction as WMFunction;
pub use gdk_ffi::GdkWindowEdge as WindowEdge;
pub use gdk_ffi::GdkWindowHints as WindowHints;
pub use gdk_ffi::GdkWindowState as WindowState;
pub use gdk_ffi::GdkWindowType as WindowType;
pub use gdk_ffi::GdkWindowTypeHint as WindowTypeHint;
pub use gdk_ffi::GdkWindowWindowClass as WindowWindowClass;

#[allow(non_camel_case_types)]
pub type key = i32;

pub use self::keys::{
    keyval_name,
    keyval_to_unicode
};

// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

/*!
Bindings and wrappers for __GDK__
*/

extern crate gdk_sys as gdk_ffi;
extern crate glib_sys as glib_ffi;
extern crate glib as glib_main;
extern crate cairo;
extern crate libc;

pub use gdk_ffi as ffi;
pub use glib_main as glib;

mod events;
mod keys;
mod object;
mod rectangle;
mod rt;

pub mod prelude;

pub mod app_launch_context;
pub mod atom;
pub mod cursor;
pub mod device;
pub mod device_manager;
pub mod display;
pub mod display_manager;
pub mod drag_context;
#[cfg(feature = "gdk_3_8")]
pub mod frame_clock;
#[cfg(feature = "gdk_3_8")]
pub mod frame_timings;
pub mod pixbuf;
pub mod rgba;
pub mod screen;
pub mod visual;
pub mod window;
pub mod cairo_interaction;

pub use gdk_ffi::GdkColor as Color;

pub use self::rt::{
    init,
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
#[cfg(feature = "gdk_3_10")]
pub use self::rt::set_allowed_backends;

pub use app_launch_context::AppLaunchContext;
pub use atom::Atom;
pub use cursor::Cursor;
pub use device::Device;
pub use device_manager::DeviceManager;
pub use display::Display;
pub use display_manager::DisplayManager;
pub use drag_context::DragContext;
#[cfg(feature = "gdk_3_8")]
pub use frame_clock::FrameClock;
#[cfg(feature = "gdk_3_8")]
pub use frame_timings::FrameTimings;
pub use pixbuf::Pixbuf;
pub use pixbuf::animation::PixbufAnimation;
pub use pixbuf::animation::PixbufSimpleAnim;
pub use pixbuf::format::PixbufFormat;
pub use pixbuf::loader::PixbufLoader;
pub use screen::Screen;
pub use visual::Visual;
pub use window::Window;

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

pub use gdk_ffi::enums::modifier_intent::ModifierIntent;
pub use gdk_ffi::enums::modifier_type::ModifierType;
pub use gdk_ffi::enums::{
    self,
    WindowType,
    WindowState,
    WindowEdge,
    WindowHints,
    WindowTypeHint,
    FullscreenMode,
    EventMask,
    InputSource,
    InputMode,
    AxisUse,
    GrabOwnership,
    GrabStatus,
    key,
    PixbufAlphaMode,
    PixbufError,
    ColorSpace,
    WindowWindowClass,
    Gravity,
    DragAction,
    DragProtocol
};

pub use self::keys::{
    keyval_name
};

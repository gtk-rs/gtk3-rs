// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

/*!
Bindings and wrappers for __GDK__
*/

#![feature(unique)]

extern crate gdk_sys as gdk_ffi;
extern crate glib_sys as glib_ffi;
extern crate glib as glib_main;
extern crate libc;
extern crate c_vec;

pub use gdk_ffi as ffi;
pub use glib_main as glib;

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
    WMDecoration,
    EventMask,
    InputSource,
    InputMode,
    AxisUse,
    DeviceType,
    GrabOwnership,
    GrabStatus,
    key,
    CursorType,
    PixbufAlphaMode,
    PixbufError,
    ColorSpace,
    FrameClockPhase,
    WindowWindowClass,
    Gravity,
    WMFunction,
    DragAction,
    DragProtocol
};

pub use self::widgets::{
    Color,
    RGBA,
    Device,
    Display,
    Atom,
    Screen,
    Rectangle,
    Window,
    Visual,
    DeviceManager,
    Cursor,
    Pixbuf,
    Point,
    DisplayManager,
    WindowAttr,
    DragContext,
    AppLaunchContext
};
#[cfg(feature = "gdk_3_8")]
pub use self::widgets::{
    FrameClock,
    FrameTimings,
};

pub use self::keys::{
    keyval_name
};

mod events;
mod rt;
mod keys;
mod macros;
pub mod widgets;

// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate glib_sys as glib_ffi;
extern crate gdk_sys as ffi;
extern crate gdk_pixbuf;
#[macro_use]
extern crate glib;
extern crate gobject_sys as gobject_ffi;
extern crate cairo;
extern crate cairo_sys as cairo_ffi;
extern crate libc;
#[macro_use]
extern crate bitflags;

#[macro_use]
mod rt;
#[macro_use]
mod event;

mod auto;

pub mod prelude;

pub use prelude::*;
pub use auto::*;

pub mod enums;

mod atom;
mod cairo_interaction;
mod device;
mod device_manager;
mod drag_context;
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
#[cfg(feature = "v3_8")]
mod frame_clock;
mod keys;
mod rectangle;
mod rgba;
mod visual;
mod window;

pub use ffi::GdkColor as Color;
pub use glib::Error;

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
#[cfg(feature = "v3_10")]
pub use self::rt::set_allowed_backends;

pub use atom::Atom;
pub use atom::NONE as ATOM_NONE;
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
pub use rectangle::Rectangle;
pub use rgba::{RGBA, RgbaParseError};
pub use window::WindowAttr;

#[allow(non_camel_case_types)]
pub type key = i32;

pub use self::keys::{
    keyval_name,
    keyval_to_unicode
};

// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

#![allow(deprecated)]
#![cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
#![cfg_attr(feature = "cargo-clippy", allow(trivially_copy_pass_by_ref))]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;

#[macro_use]
mod rt;
#[macro_use]
mod event;

#[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
#[allow(unused_imports)]
mod auto;

pub mod prelude;

pub use self::auto::functions::*;
pub use self::auto::*;

mod atom;
mod cairo_interaction;
mod change_data;
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
#[cfg(any(feature = "v3_22", feature = "dox"))]
mod event_pad_axis;
#[cfg(any(feature = "v3_22", feature = "dox"))]
mod event_pad_button;
#[cfg(any(feature = "v3_22", feature = "dox"))]
mod event_pad_group_mode;
mod event_property;
mod event_proximity;
mod event_scroll;
mod event_selection;
mod event_setting;
mod event_touch;
#[cfg(any(feature = "v3_18", feature = "dox"))]
mod event_touchpad_pinch;
#[cfg(any(feature = "v3_18", feature = "dox"))]
mod event_touchpad_swipe;
mod event_visibility;
mod event_window_state;
mod frame_clock;
mod frame_timings;
mod functions;
mod geometry;
mod keymap;
mod keymap_key;
pub mod keys;
mod rectangle;
mod rgba;
mod screen;
mod time_coord;
mod visual;
mod window;

pub use ffi::GdkColor as Color;

pub use self::rt::{init, set_initialized};

pub use self::atom::Atom;
pub use self::atom::NONE as ATOM_NONE;
pub use self::atom::SELECTION_CLIPBOARD;
pub use self::atom::SELECTION_PRIMARY;
pub use self::atom::SELECTION_SECONDARY;
pub use self::atom::SELECTION_TYPE_ATOM;
pub use self::atom::SELECTION_TYPE_BITMAP;
pub use self::atom::SELECTION_TYPE_COLORMAP;
pub use self::atom::SELECTION_TYPE_DRAWABLE;
pub use self::atom::SELECTION_TYPE_INTEGER;
pub use self::atom::SELECTION_TYPE_PIXMAP;
pub use self::atom::SELECTION_TYPE_STRING;
pub use self::atom::SELECTION_TYPE_WINDOW;
pub use self::atom::TARGET_BITMAP;
pub use self::atom::TARGET_COLORMAP;
pub use self::atom::TARGET_DRAWABLE;
pub use self::atom::TARGET_PIXMAP;
pub use self::atom::TARGET_STRING;
pub use self::change_data::ChangeData;
pub use self::event::Event;
pub use self::event_button::EventButton;
pub use self::event_configure::EventConfigure;
pub use self::event_crossing::EventCrossing;
pub use self::event_dnd::EventDND;
pub use self::event_expose::EventExpose;
pub use self::event_focus::EventFocus;
pub use self::event_grab_broken::EventGrabBroken;
pub use self::event_key::EventKey;
pub use self::event_motion::EventMotion;
pub use self::event_owner_change::EventOwnerChange;
#[cfg(any(feature = "v3_22", feature = "dox"))]
pub use self::event_pad_axis::EventPadAxis;
#[cfg(any(feature = "v3_22", feature = "dox"))]
pub use self::event_pad_button::EventPadButton;
#[cfg(any(feature = "v3_22", feature = "dox"))]
pub use self::event_pad_group_mode::EventPadGroupMode;
pub use self::event_property::EventProperty;
pub use self::event_proximity::EventProximity;
pub use self::event_scroll::EventScroll;
pub use self::event_selection::EventSelection;
pub use self::event_setting::EventSetting;
pub use self::event_touch::EventTouch;
#[cfg(any(feature = "v3_18", feature = "dox"))]
pub use self::event_touchpad_pinch::EventTouchpadPinch;
#[cfg(any(feature = "v3_18", feature = "dox"))]
pub use self::event_touchpad_swipe::EventTouchpadSwipe;
pub use self::event_visibility::EventVisibility;
pub use self::event_window_state::EventWindowState;
pub use self::functions::*;
pub use self::geometry::Geometry;
pub use self::keymap_key::KeymapKey;
pub use self::rectangle::Rectangle;
pub use self::rgba::{RgbaParseError, RGBA};
pub use self::time_coord::TimeCoord;
pub use self::window::WindowAttr;

#[allow(non_camel_case_types)]
pub type key = i32;

/// The primary button. This is typically the left mouse button, or the right button in a left-handed setup.
pub const BUTTON_PRIMARY: u32 = ffi::GDK_BUTTON_PRIMARY as u32;

/// The middle button.
pub const BUTTON_MIDDLE: u32 = ffi::GDK_BUTTON_MIDDLE as u32;

/// The secondary button. This is typically the right mouse button, or the left button in a left-handed setup.
pub const BUTTON_SECONDARY: u32 = ffi::GDK_BUTTON_SECONDARY as u32;

// Used as the return value for stopping the propagation of an event handler.
pub const EVENT_STOP: u32 = ffi::GDK_EVENT_STOP as u32;

// Used as the return value for continuing the propagation of an event handler.
pub const EVENT_PROPAGATE: u32 = ffi::GDK_EVENT_PROPAGATE as u32;

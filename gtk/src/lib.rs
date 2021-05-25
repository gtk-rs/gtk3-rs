// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::needless_doctest_main)]
#![allow(clippy::upper_case_acronyms)]

//! # Rust GTK 3 bindings
//!
//! This library contains safe Rust bindings for [GTK 3](http://www.gtk.org), a
//! multi-platform GUI toolkit. It is a part of [Gtk-rs](http://gtk-rs.org/).
//!
//! Most of this documentation is generated from the C API.
//! Until all parts of the documentation have been reviewed there will be incongruities
//! with the actual Rust API.
//!
//! See also
//!
//! - [Gtk-rs documentation overview](https://gtk-rs.org/docs-src/)
//!
//! - [General `GLib` family types and object system overview](mod@glib)
//!
//! - [GTK documentation](https://www.gtk.org/docs/)
//!
//! # "Hello, World!" example program
//!
//! GTK needs to be initialized before use by calling [`fn@init`]. Creating an
//! [`struct@Application`] will call [`fn@init`] for you.
//!
//! ```no_run
//! use gtk::prelude::*;
//! use gtk::{ApplicationBuilder, ApplicationWindowBuilder};
//!
//! fn main() {
//!     let app = ApplicationBuilder::new()
//!         .application_id("org.example.HelloWorld")
//!         .build();
//!
//!     app.connect_activate(|app| {
//!         // We create the main window.
//!         let win = ApplicationWindowBuilder::new()
//!             .application(app)
//!             .default_width(320)
//!             .default_height(200)
//!             .title("Hello, World!")
//!             .build();
//!
//!         // Don't forget to make all widgets visible.
//!         win.show_all();
//!     });
//!
//!     app.run();
//! }
//! ```
//!
//! # The main loop
//!
//! In a typical GTK application you set up the UI, assign signal handlers
//! and run the main event loop.
//!
//! ```no_run

//! use gtk::prelude::*;
//! use gtk::{ApplicationBuilder, ApplicationWindowBuilder, Button};
//!
//! fn main() {
//!     let application = ApplicationBuilder::new()
//!         .application_id("com.example.FirstGtkApp")
//!         .build();
//!
//!     application.connect_activate(|app| {
//!         let window = ApplicationWindowBuilder::new()
//!             .application(app)
//!             .title("First GTK Program")
//!             .default_width(350)
//!             .default_height(70)
//!             .build();
//!
//!         let button = Button::with_label("Click me!");
//!         button.connect_clicked(|_| {
//!             eprintln!("Clicked!");
//!         });
//!         window.add(&button);
//!
//!         window.show_all();
//!     });
//!
//!     application.run();
//! }
//! ```
//!
//! # Threads
//!
//! GTK is not thread-safe. Accordingly, none of this crate's structs implement
//! [`Send`] or [`Sync`].
//!
//! The thread where [`fn@init`] was called is considered the main thread. OS X has
//! its own notion of the main thread and [`fn@init`] must be called on that thread.
//! After successful initialization, calling any [`gtk`](mod@crate) or [`mod@gdk`] functions
//! (including [`fn@init`]) from other threads will `panic`.
//!
//! Any thread can schedule a closure to be run by the main loop on the main
//! thread via [`fn@glib::idle_add`] or [`fn@glib::timeout_add`]. While
//! working with GTK you might need the [`fn@glib::idle_add_local`]
//! or [`fn@glib::timeout_add_local`] version without the
//! [`Send`] bound. Those may only be called from the main thread.
//!
//! # Panics
//!
//! The [`gtk`](mod@crate) and [`mod@gdk`] crates have some run-time safety and contract checks.
//!
//! - Any constructor or free function will panic if called before [`fn@init`] or on
//! a non-main thread.
//!
//! - Any [`&str`] or [`&Path`](std::path::Path) parameter with an interior null (`\0`) character will
//! cause a panic.
//!
//! - Some functions will panic if supplied out-of-range integer parameters. All
//! such cases will be documented individually but they are not yet.
//!
//! - A panic in a closure that handles signals or in any other closure passed
//! to a [`gtk`](mod@crate) function will abort the process.
//!
//! # Features
//!
//! ## Library versions
//!
//! By default this crate provides only GTK 3.18 APIs. You can access additional
//! functionality by selecting one of the `v3_20`, `v3_24`, etc. features.
//!
//! `Cargo.toml` example:
//!
//! ```toml
//! [dependencies.gtk]
//! version = "0.x.y"
//! features = ["v3_20"]
//! ```
//!
//! Take care when choosing the version to target: some of your users might
//! not have easy access to the latest ones. The higher the version, the fewer
//! users will have it installed.

#![allow(clippy::type_complexity)]
#![allow(clippy::derive_hash_xor_eq)]
#![allow(clippy::too_many_arguments)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;
// Re-export gtk dependencies
pub use atk;
pub use cairo;
pub use gdk;
pub use gdk_pixbuf;
pub use gio;
pub use glib;
pub use pango;

#[doc(hidden)]
pub use field_offset::*;
#[doc(hidden)]
pub use gtk3_macros::*;

pub mod xlib;

/// The priority used for default style information
/// that is used in the absence of themes.
///
/// Note that this is not very useful for providing default
/// styling for custom style classes - themes are likely to
/// override styling provided at this priority with
/// catch-all `* {...}` rules.
pub const STYLE_PROVIDER_PRIORITY_FALLBACK: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_FALLBACK as u32;
/// The priority used for style information provided
/// by themes.
pub const STYLE_PROVIDER_PRIORITY_THEME: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_THEME as u32;
/// The priority used for style information provided
/// via [Settings](crate::Settings).
///
/// This priority is higher than GTK_STYLE_PROVIDER_PRIORITY_THEME
/// to let settings override themes.
pub const STYLE_PROVIDER_PRIORITY_SETTINGS: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_SETTINGS as u32;
/// A priority that can be used when adding a [StyleProvider](crate::StyleProvider)
/// for application-specific style information.
pub const STYLE_PROVIDER_PRIORITY_APPLICATION: u32 =
    ffi::GTK_STYLE_PROVIDER_PRIORITY_APPLICATION as u32;
/// The priority used for the style information from
/// `XDG_CONFIG_HOME/gtk-3.0/gtk.css`.
///
/// You should not use priorities higher than this, to
/// give the user the last word.
pub const STYLE_PROVIDER_PRIORITY_USER: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_USER as u32;

#[macro_use]
mod rt;

#[allow(clippy::let_and_return)]
#[allow(clippy::many_single_char_names)]
#[allow(clippy::wrong_self_convention)]
#[allow(clippy::clone_on_copy)]
#[allow(unused_imports)]
mod auto;

mod accel_group;
mod app_chooser;
mod application;
mod application_window;
mod border;
mod buildable;
mod builder;
mod cell_renderer_pixbuf;
mod clipboard;
mod color_button;
mod color_chooser;
mod combo_box;
mod dialog;
mod drag_context;
mod entry;
mod entry_buffer;
mod entry_completion;
mod enums;
mod file_chooser_dialog;
mod fixed;
mod flow_box;
#[cfg(any(feature = "v3_24", feature = "dox"))]
mod gesture_stylus;
mod im_context_simple;
mod invisible;
mod list_box;
mod list_store;
mod menu;
mod message_dialog;
#[cfg(any(feature = "v3_20", feature = "dox"))]
mod native_dialog;
mod notebook;
#[cfg(any(feature = "v3_22", feature = "dox"))]
mod pad_action_entry;
#[cfg(any(feature = "v3_22", feature = "dox"))]
mod pad_controller;
mod page_range;
mod print_settings;
mod radio_button;
mod radio_menu_item;
mod radio_tool_button;
mod recent_chooser_dialog;
mod recent_data;
mod requisition;
mod response_type;
mod selection_data;
mod signal;
mod style_context;
mod switch;
mod target_entry;
mod target_list;
mod text_buffer;
mod text_iter;
mod tree_model_filter;
mod tree_path;
mod tree_row_reference;
mod tree_sortable;
mod tree_store;
mod widget;
mod window;

#[macro_use]
pub mod subclass;

pub mod prelude;

pub use crate::auto::functions::*;
pub use crate::auto::*;
pub use crate::rt::*;
pub use crate::signal::*;

pub use gdk::Rectangle as Allocation;
pub use gdk::Rectangle;

pub use crate::app_chooser::AppChooser;
pub use crate::border::Border;
pub use crate::entry_buffer::EntryBuffer;
pub use crate::page_range::PageRange;
pub use crate::recent_data::RecentData;
pub use crate::requisition::Requisition;
pub use crate::response_type::ResponseType;
pub use crate::target_entry::TargetEntry;
pub use crate::tree_sortable::SortColumn;
pub use crate::widget::TickCallbackId;
#[cfg(any(feature = "v3_22", feature = "dox"))]
pub use pad_action_entry::PadActionEntry;

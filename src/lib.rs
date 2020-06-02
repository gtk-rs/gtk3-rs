// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(deprecated)]
#![allow(clippy::let_and_return)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

#[macro_use]
extern crate bitflags;
extern crate once_cell;
#[macro_use]
extern crate glib;
#[cfg_attr(test, macro_use)]
#[cfg(test)]
extern crate serial_test_derive;

extern crate gio_sys;
extern crate glib_sys;
extern crate gobject_sys;
extern crate libc;

extern crate futures_channel;
extern crate futures_core;
extern crate futures_io;
extern crate futures_util;

mod app_info;
mod application;
#[cfg(test)]
mod cancellable;
mod converter;
mod data_input_stream;
mod dbus;
pub use dbus::*;
mod dbus_connection;
pub use dbus_connection::{
    ActionGroupExportId, FilterId, MenuModelExportId, RegistrationId, SignalSubscriptionId,
    WatcherId,
};
mod dbus_message;
mod dbus_method_invocation;
#[cfg(any(all(not(windows), not(target_os = "macos")), feature = "dox"))]
mod desktop_app_info;
mod error;
mod file;
mod file_attribute_matcher;
pub use file_attribute_matcher::FileAttributematcherIter;
mod file_enumerator;
mod flags;
mod inet_address;
mod io_stream;
pub use io_stream::IOStreamAsyncReadWrite;
mod input_stream;
pub use input_stream::{InputStreamAsyncBufRead, InputStreamRead};
#[cfg(any(feature = "v2_44", feature = "dox"))]
mod list_store;
mod memory_input_stream;
mod memory_output_stream;
mod output_stream;
pub use output_stream::OutputStreamWrite;
mod pollable_input_stream;
pub use pollable_input_stream::InputStreamAsyncRead;
mod pollable_output_stream;
pub use pollable_output_stream::OutputStreamAsyncWrite;
mod resource;
mod settings;
mod socket;
mod socket_listener;
mod subprocess;
mod subprocess_launcher;
mod threaded_socket_service;
#[cfg(any(unix, feature = "dox"))]
mod unix_fd_list;
#[cfg(any(unix, feature = "dox"))]
mod unix_input_stream;
#[cfg(any(unix, feature = "dox"))]
#[cfg(any(feature = "v2_54", feature = "dox"))]
mod unix_mount_entry;
#[cfg(any(unix, feature = "dox"))]
#[cfg(any(feature = "v2_54", feature = "dox"))]
mod unix_mount_point;
#[cfg(any(unix, feature = "dox"))]
mod unix_output_stream;
#[cfg(any(unix, feature = "dox"))]
mod unix_socket_address;
pub use inet_address::InetAddressBytes;

#[cfg(test)]
mod test_util;

pub use auto::functions::*;
pub use auto::*;

pub mod prelude;

#[allow(clippy::cast_ptr_alignment)]
#[allow(clippy::wrong_self_convention)]
mod auto;

mod gio_future;
pub use gio_future::*;

#[macro_use]
pub mod subclass;
mod read_input_stream;
pub use read_input_stream::ReadInputStream;
mod write_output_stream;
pub use write_output_stream::WriteOutputStream;

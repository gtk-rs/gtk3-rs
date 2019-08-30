// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(deprecated)]
#![cfg_attr(feature = "cargo-clippy", allow(let_and_return))]
#![cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#![cfg_attr(feature = "cargo-clippy", allow(type_complexity))]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate glib;
#[cfg_attr(test, macro_use)]
#[cfg(test)]
extern crate serial_test_derive;

extern crate fragile;
extern crate gio_sys;
extern crate glib_sys;
extern crate gobject_sys;
extern crate libc;

#[cfg(feature = "futures")]
extern crate futures;

mod application;
mod converter;
#[cfg(any(not(windows), feature = "dox"))]
mod desktop_app_info;
mod error;
mod file;
mod file_attribute_matcher;
pub use file_attribute_matcher::FileAttributematcherIter;
mod file_enumerator;
mod flags;
mod inet_address;
mod input_stream;
#[cfg(any(feature = "v2_44", feature = "dox"))]
mod list_store;
mod memory_input_stream;
mod memory_output_stream;
mod output_stream;
mod pollable_input_stream;
mod pollable_output_stream;
mod resource;
mod settings;
mod socket;
mod socket_listener;
mod subprocess;
mod subprocess_launcher;
#[cfg(any(unix, feature = "dox"))]
mod unix_input_stream;
#[cfg(any(unix, feature = "dox"))]
mod unix_mount_entry;
#[cfg(any(unix, feature = "dox"))]
mod unix_mount_point;
#[cfg(any(unix, feature = "dox"))]
mod unix_output_stream;
#[cfg(any(unix, feature = "dox"))]
mod unix_socket_address;
pub use inet_address::InetAddressBytes;

#[cfg(test)]
mod test_util;

pub use glib::{Error, Object};

pub use auto::functions::*;
pub use auto::*;

pub mod signal {
    pub use glib::signal::Inhibit;
}

pub mod prelude;
pub use prelude::*;

#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
mod auto;

#[cfg(feature = "futures")]
mod gio_future;
#[cfg(feature = "futures")]
pub use gio_future::*;

#[macro_use]
#[cfg(feature = "subclassing")]
pub mod subclass;
#[cfg(feature = "subclassing")]
mod read_input_stream;
#[cfg(feature = "subclassing")]
pub use read_input_stream::ReadInputStream;
#[cfg(feature = "subclassing")]
mod write_output_stream;
#[cfg(feature = "subclassing")]
pub use write_output_stream::WriteOutputStream;

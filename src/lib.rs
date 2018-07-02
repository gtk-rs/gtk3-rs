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

extern crate gio_sys as ffi;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate libc;
extern crate send_cell;

#[cfg(feature = "futures")]
extern crate futures_core;
#[cfg(feature = "futures")]
extern crate futures_channel;
#[cfg(feature = "futures")]
extern crate futures_util;

mod application;
#[cfg(any(not(windows), feature = "dox"))]
mod desktop_app_info;
mod converter;
mod input_stream;
mod memory_input_stream;
mod memory_output_stream;
mod output_stream;
mod resource;
mod socket;
mod socket_listener;
#[cfg(any(unix, feature = "dox"))]
mod unix_socket_address;
mod file;
mod pollable_input_stream;
mod pollable_output_stream;
mod subprocess;
mod subprocess_launcher;
#[cfg(any(feature = "v2_44", feature = "dox"))]
mod list_store;

#[cfg(test)]
mod test_util;

pub use glib::{
    Error,
    Object,
};

pub use auto::*;
pub use auto::functions::*;

pub mod signal {
    pub use glib::signal::Inhibit;
}

pub mod prelude {
    pub use auto::traits::*;
    pub use application::*;
    pub use converter::*;
    #[cfg(any(not(windows), feature = "dox"))]
    pub use desktop_app_info::*;
    pub use input_stream::InputStreamExtManual;
    pub use output_stream::OutputStreamExtManual;
    pub use socket::*;
    pub use socket_listener::SocketListenerExtManual;
    #[cfg(any(unix, feature = "dox"))]
    pub use unix_socket_address::{UnixSocketAddressPath, UnixSocketAddressExtManual};
    pub use file::FileExtManual;
    pub use pollable_input_stream::PollableInputStreamExtManual;
    pub use pollable_output_stream::PollableOutputStreamExtManual;
    pub use subprocess::SubprocessExtManual;
    pub use subprocess_launcher::SubprocessLauncherExtManual;
    #[cfg(any(feature = "v2_44", feature = "dox"))]
    pub use list_store::ListStoreExtManual;
}

pub use prelude::*;

#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
mod auto;

#[cfg(feature = "futures")]
mod gio_future;
#[cfg(feature = "futures")]
pub use gio_future::*;

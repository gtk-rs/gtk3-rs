// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Traits and essential types intended for blanket imports.

pub use glib::prelude::*;

pub use auto::traits::*;

pub use application::*;
pub use converter::*;
#[cfg(any(
    all(not(windows), feature = "v2_58"),
    all(not(windows), feature = "dox")
))]
pub use desktop_app_info::DesktopAppInfoExtManual;
pub use file::FileExtManual;
pub use input_stream::InputStreamExtManual;
#[cfg(any(feature = "v2_44", feature = "dox"))]
pub use list_store::ListStoreExtManual;
pub use output_stream::OutputStreamExtManual;
pub use pollable_input_stream::PollableInputStreamExtManual;
pub use pollable_output_stream::PollableOutputStreamExtManual;
pub use settings::SettingsExtManual;
pub use socket::*;
pub use socket_listener::SocketListenerExtManual;
#[cfg(any(unix, feature = "dox"))]
pub use unix_input_stream::UnixInputStreamExtManual;
#[cfg(any(unix, feature = "dox"))]
pub use unix_output_stream::UnixOutputStreamExtManual;
#[cfg(any(unix, feature = "dox"))]
pub use unix_socket_address::{UnixSocketAddressExtManual, UnixSocketAddressPath};

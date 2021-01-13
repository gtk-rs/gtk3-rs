// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::missing_safety_doc)]
#![allow(deprecated)]

pub use ffi;
pub use glib;

mod app_info;
mod application;
#[cfg(test)]
mod cancellable;
mod converter;
mod data_input_stream;
mod dbus;
pub use self::dbus::*;
mod dbus_connection;
pub use self::dbus_connection::{
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
pub use crate::file_attribute_matcher::FileAttributematcherIter;
mod file_enumerator;
mod file_info;
mod flags;
mod inet_address;
mod inet_socket_address;
mod io_stream;
pub use crate::io_stream::IOStreamAsyncReadWrite;
mod input_stream;
pub use crate::input_stream::{InputStreamAsyncBufRead, InputStreamRead};
#[cfg(any(feature = "v2_44", feature = "dox"))]
mod list_store;
mod memory_input_stream;
mod memory_output_stream;
mod output_stream;
pub use crate::output_stream::OutputStreamWrite;
mod pollable_input_stream;
pub use crate::pollable_input_stream::InputStreamAsyncRead;
mod pollable_output_stream;
pub use crate::pollable_output_stream::OutputStreamAsyncWrite;
mod resource;
pub use crate::resource::{compile_resources, resources_register_include_impl};
mod settings;
mod socket;
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
pub use crate::inet_address::InetAddressBytes;

#[cfg(test)]
mod test_util;

pub use crate::auto::functions::*;
pub use crate::auto::*;

pub mod prelude;

#[allow(clippy::cast_ptr_alignment)]
#[allow(clippy::wrong_self_convention)]
#[allow(clippy::new_ret_no_self)]
#[allow(clippy::let_and_return)]
#[allow(unused_doc_comments)]
#[allow(unused_imports)]
mod auto;

mod gio_future;
pub use crate::gio_future::*;

#[macro_use]
pub mod subclass;
mod read_input_stream;
pub use crate::read_input_stream::ReadInputStream;
mod write_output_stream;
pub use crate::write_output_stream::WriteOutputStream;
mod tls_connection;
pub use crate::tls_connection::TlsConnectionManualExt;

pub mod task;

#[cfg(target_family = "windows")]
mod win32_input_stream;
#[cfg(target_family = "windows")]
pub use self::win32_input_stream::{Win32InputStream, NONE_WIN32_INPUT_STREAM};

#[cfg(target_family = "windows")]
mod win32_output_stream;
#[cfg(target_family = "windows")]
pub use self::win32_output_stream::{Win32OutputStream, NONE_WIN32_OUTPUT_STREAM};

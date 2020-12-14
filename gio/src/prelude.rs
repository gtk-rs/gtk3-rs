// Take a look at the license at the top of the repository in the LICENSE file.

//! Traits and essential types intended for blanket imports.

#[doc(hidden)]
pub use glib::prelude::*;

pub use crate::auto::traits::*;

#[cfg(any(feature = "v2_60", feature = "dox"))]
pub use crate::app_info::AppInfoExtManual;
pub use crate::application::*;
pub use crate::converter::*;
pub use crate::data_input_stream::DataInputStreamExtManual;
#[cfg(any(feature = "v2_58", feature = "dox"))]
#[cfg(any(all(not(windows), not(target_os = "macos")), feature = "dox"))]
pub use crate::desktop_app_info::DesktopAppInfoExtManual;
pub use crate::file::FileExtManual;
pub use crate::inet_address::InetAddressExtManual;
pub use crate::input_stream::InputStreamExtManual;
pub use crate::io_stream::IOStreamExtManual;
#[cfg(any(feature = "v2_44", feature = "dox"))]
pub use crate::list_store::ListStoreExtManual;
pub use crate::output_stream::OutputStreamExtManual;
pub use crate::pollable_input_stream::PollableInputStreamExtManual;
pub use crate::pollable_output_stream::PollableOutputStreamExtManual;
pub use crate::settings::SettingsExtManual;
pub use crate::socket::*;
#[cfg(any(unix, feature = "dox"))]
pub use crate::unix_input_stream::UnixInputStreamExtManual;
#[cfg(any(unix, feature = "dox"))]
pub use crate::unix_output_stream::UnixOutputStreamExtManual;
#[cfg(any(unix, feature = "dox"))]
pub use crate::unix_socket_address::{UnixSocketAddressExtManual, UnixSocketAddressPath};

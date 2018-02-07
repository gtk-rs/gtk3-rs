// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate glib;

extern crate gio_sys as ffi;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate libc;

macro_rules! callback_guard {
    () => (
        let _guard = ::glib::CallbackGuard::new();
    )
}

mod application;
mod buffered_input_stream;
mod input_stream;
mod file;
mod file_i_o_stream;
mod file_input_stream;
mod file_output_stream;
mod memory_input_stream;
mod memory_output_stream;
mod output_stream;
mod resource;
mod socket;
mod socket_address_enumerator;
mod resolver;

#[cfg(test)]
mod test_util;

pub use glib::{
    Error,
    Object,
};

pub use auto::*;
pub use auto::functions::*;

use file_i_o_stream::FileIOStream;
use file_input_stream::FileInputStream;
use file_output_stream::FileOutputStream;

pub mod signal {
    pub use glib::signal::Inhibit;
}

pub mod prelude {
    pub use auto::traits::*;
    pub use application::*;
    pub use buffered_input_stream::BufferedInputStreamExtManual;
    pub use file::FileExtManual;
    pub use file_i_o_stream::FileIOStreamExt;
    pub use file_input_stream::FileInputStreamExt;
    pub use file_output_stream::FileOutputStreamExt;
    pub use input_stream::InputStreamExtManual;
    pub use output_stream::OutputStreamExtManual;
    pub use socket::SocketExtManual;
    pub use socket_address_enumerator::SocketAddressEnumeratorExtManual;
    pub use resolver::ResolverExtManual;
}

pub use prelude::*;

#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
mod auto;

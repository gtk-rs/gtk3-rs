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
}

pub use prelude::*;

mod auto;

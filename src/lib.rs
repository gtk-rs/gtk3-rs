#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate glib;

extern crate gio_sys as ffi;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate libc;

mod auto;

pub use glib::Error;
pub use auto::*;

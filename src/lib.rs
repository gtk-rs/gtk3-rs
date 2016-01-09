// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate libc;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gio_sys as gio_ffi;

use libc::c_char;

pub use self::app_info::AppInfo;
pub use self::glib_container::GlibContainer;
pub use self::error::{Error};
pub use self::object::Object;
pub use self::permission::Permission;
pub use self::source::{Continue, idle_add, timeout_add, timeout_add_seconds};
pub use self::traits::FFIGObject;
pub use self::value::{Value, ValuePublic};
pub use types::Type;
pub use self::date::{TimeVal, Time, Date, Year, Month, Weekday, Day};

#[macro_use]
pub mod wrapper;
#[macro_use]
pub mod boxed;
#[macro_use]
pub mod refcounted;
#[macro_use]
pub mod object;

mod app_info;
pub mod glib_container;
pub mod error;
mod permission;
pub mod signal;
pub mod source;
pub mod traits;
pub mod translate;
mod value;

pub mod types;
pub mod date;

pub fn to_gboolean(b: bool) -> glib_ffi::gboolean {
    match b {
        true => glib_ffi::GTRUE,
        false => glib_ffi::GFALSE
    }
}

pub fn to_bool(b: glib_ffi::gboolean) -> bool {
    b != glib_ffi::GFALSE
}

// An opaque structure used as the base of all interface types.
pub struct TypeInterface;

// An opaque structure used as the base of all type instances.
pub struct TypeInstance;

// An opaque structure used as the base of all classes.
pub struct TypeClass;

//FIXME: Check if this is actually correct (maybe not since ParamFlags is deprecated)
#[derive(Clone, Copy)]
pub enum ParamFlags{
    Readable,
    Writable,
    ReadWrite,
    Construct,
    ConstructOnly,
    LaxValidation,
    StaticName,
    Private,
    StaticNick,
    StaticBlurb,
    Deprecated
}

#[repr(C)]
pub struct ParamSpec {
    g_type_instance: TypeInstance,
    name: *mut c_char,
    flags: ParamFlags,
    value_type: glib_ffi::GType,
    owner_type: glib_ffi::GType,
}

// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

/*!
Bindings and wrappers for __GLib__
*/

extern crate libc;
extern crate glib_sys as glib_ffi;

pub use glib_ffi as ffi;

use libc::c_char;

pub use self::list::{List, Elem, RevElem};
pub use self::slist::{SList, SElem};
pub use self::glib_container::GlibContainer;
pub use self::error::{Error};
pub use self::permission::Permission;
pub use self::timeout_func::timeout;
pub use self::traits::{FFIGObject, Connect};
pub use self::value::{Value, ValuePublic};
pub use type_::Type;
pub use self::date::{Time, Date, Year, Month, Weekday, Day};

mod list;
mod slist;
pub mod glib_container;
mod error;
mod permission;
pub mod signal;
pub mod timeout_func;
pub mod traits;
pub mod translate;
mod value;
pub mod type_;
pub mod date;

pub fn to_gboolean(b: bool) -> ffi::Gboolean {
    match b {
        true => ffi::GTRUE,
        false => ffi::GFALSE
    }
}

pub fn to_bool(b: ffi::Gboolean) -> bool {
    b != ffi::GFALSE
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
    value_type: ffi::GType,
    owner_type: ffi::GType,
}
// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

/*!
Bindings and wrappers for __GLib__
*/

extern crate libc;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gio_sys as gio_ffi;

use libc::c_char;

pub use self::app_info::AppInfo;
pub use self::list::{List, Elem, RevElem};
pub use self::slist::{SList, SElem};
pub use self::glib_container::GlibContainer;
pub use self::error::{Error};
pub use self::permission::Permission;
pub use self::source::{Continue, idle_add, timeout_add, timeout_add_seconds};
pub use self::traits::FFIGObject;
pub use self::value::{Value, ValuePublic};
pub use types::Type;
pub use self::date::{TimeVal, Time, Date, Year, Month, Weekday, Day};

/// Defines a wrapper type and implements the appropriate traits.
///
/// The basic syntax is
///
/// ```ignore
/// glib_wrapper! {
///     /// Documentation
///     pub struct $name($kind<$foreign>);
///
///     match fn {
///         $fn_name => /* a closure-like expression */,
///         ...
///     }
/// }
/// ```
///
/// This creates a wrapper named `$name` around the foreign type `$foreign`
/// of $kind (one of `Boxed`, `Object`) using expressions from the `match fn`
/// block to implement type-specific low-level operations. The expression
/// will be evaluated in `unsafe` context.
///
/// ### Boxed
///
/// ```ignore
/// glib_wrapper! {
///     /// Text buffer iterator
///     pub struct TextIter(Boxed<ffi::GtkTextIter>);
///
///     match fn {
///         copy => |ptr| ffi::gtk_text_iter_copy(ptr),
///         free => |ptr| ffi::gtk_text_iter_free(ptr),
///     }
/// }
/// ```
///
/// `copy`: `|*const $foreign| -> *mut $foreign` creates a copy of the value.
///
/// `free`: `|*mut $foreign|` frees the value.
///
/// ### Refcounted
///
/// ```ignore
/// glib_wrapper! {
///     /// Object holding timing information for a single frame.
///     pub struct FrameTimings(Refcounted<ffi::GdkFrameTimings>);
///
///     match fn {
///         ref => |ptr| ffi::gdk_frame_timings_ref(ptr),
///         unref => |ptr| ffi::gdk_frame_timings_unref(ptr),
///     }
/// }
/// ```
///
/// `ref`: `|*mut $foreign|` increases the refcount.
///
/// `unref`: `|*mut $foreign|` decreases the refcount.
///
/// ### Object
///
/// ```
/// glib_wrapper! {
///     /// Object representing an input device.
///     pub struct Device(Object<ffi::GdkDevice>);
///
///     match fn {
///         get_type => || ffi::gdk_device_get_type(),
///     }
/// }
/// ```
///
/// ```
/// glib_wrapper! {
///     /// A container with just one child.
///     pub struct Bin(Object<ffi::GtkBin>): Container, Widget, Buildable;
///
///     match fn {
///         get_type => || ffi::gtk_bin_get_type(),
///     }
/// }
/// ```
///
/// `get_type: || -> GType` returns the type identifier of the class.
#[macro_export]
macro_rules! glib_wrapper {
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Boxed<$ffi_name:path>);

        match fn {
            copy => |$copy_arg:ident| $copy_expr:expr,
            free => |$free_arg:ident| $free_expr:expr,
        }
    ) => {
        glib_boxed_wrapper!([$($attr)*] $name, $ffi_name, @copy $copy_arg $copy_expr,
            @free $free_arg $free_expr);
    };

    (
        $(#[$attr:meta])*
        pub struct $name:ident(Refcounted<$ffi_name:path>);

        match fn {
            ref => |$ref_arg:ident| $ref_expr:expr,
            unref => |$unref_arg:ident| $unref_expr:expr,
        }
    ) => {
        glib_refcounted_wrapper!([$($attr)*] $name, $ffi_name, @ref $ref_arg $ref_expr,
            @unref $unref_arg $unref_expr);
    };

    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:path>);

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        glib_object_wrapper!([$($attr)*] $name, $ffi_name, @get_type $get_type_expr, []);
    };

    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:path>): $($implements:path),+;

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        glib_object_wrapper!([$($attr)*] $name, $ffi_name, @get_type $get_type_expr,
            [$($implements),+]);
    };
}

#[macro_use]
pub mod boxed;
#[macro_use]
pub mod refcounted;
#[macro_use]
pub mod object;

mod app_info;
mod list;
mod slist;
pub mod glib_container;
mod error;
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

// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! # **glib**, **gobject** and **gio** bindings for Rust
//!
//! This library contains
//!
//! - bindings to some essential GLib, GObject, GIO types and APIs,
//!
//! - common building blocks used in both handmade and machine generated
//! bindings to GTK+ and other GLib-based libraries.
//!
//! It is the foundation for higher level libraries with uniform Rusty (safe and
//! strongly typed) APIs. It avoids exposing GLib-specific data types where
//! possible and is not meant to provide comprehensive GLib bindings, which
//! would often amount to duplicating the Rust Standard Library or other utility
//! crates.
//!
//! The library is a work in progress: expect missing functionality and breaking
//! changes.
//!
//! # Dynamic typing
//!
//! Most types in the GLib family have type identifiers
//! ([`Type`](types/enum.Type.html)). Their corresponding Rust types implement
//! the [`StaticType`](types/trait.StaticType.html) trait.
//!
//! Dynamically typed [`Value`](value/index.html) can carry values of any `T:
//! StaticType`.
//!
//! [`Variant`](variant/index.html) can carry values of `T: StaticVariantType`.
//!
//! # Errors
//!
//! Errors are represented by [`Error`](error/struct.Error.html), which can
//! carry values from various [error
//! domains](error/trait.ErrorDomain.html#implementors) (such as
//! [`FileError`](enum.FileError.html)).
//!
//! # Objects
//!
//! Each class and interface has a corresponding smart pointer struct
//! representing an instance of that type (e.g. `Object` for `GObject`,
//! `gtk::Widget` for `GtkWidget`). They are reference counted and feature
//! interior mutability similarly to Rust's `Rc<RefCell<T>>` idiom.
//! Consequently, cloning objects is cheap and their methods never require
//! mutable borrows. Two smart pointers are equal iff they point to the same
//! object.
//!
//! The root of the object hierarchy is [`Object`](object/struct.Object.html).
//! Inheritance and subtyping is denoted with the [`IsA`](object/trait.IsA.html)
//! marker trait. The [`Cast`](object/trait.Cast.html) trait enables upcasting
//! and downcasting.
//!
//! Interfaces and non-leaf classes also have corresponding traits (e.g.
//! `ObjectExt` and `gtk::WidgetExt`), which are blanketly implemented for all
//! their subtypes.
//!
//! You can create new subclasses of `Object` or other object types. Look at
//! the module's documentation for further details and a code example.
//!
//! # Under the hood
//!
//! GLib-based libraries largely operate on pointers to various boxed or
//! reference counted structures so the bindings have to implement corresponding
//! smart pointers (wrappers), which encapsulate resource management and safety
//! checks. Such wrappers are defined via the
//! [`glib_wrapper!`](macro.glib_wrapper!.html) macro, which uses abstractions
//! defined in the [`wrapper`](wrapper/index.html), [`boxed`](boxed/index.html),
//! [`shared`](shared/index.html) and [`object`](object/index.html) modules.
//!
//! The [`translate`](translate/index.html) module defines and partly implements
//! conversions between high level Rust types (including the aforementioned
//! wrappers) and their FFI counterparts.

#![allow(clippy::doc_markdown)]
#![allow(clippy::unreadable_literal)]

#[doc(hidden)]
#[macro_use]
pub extern crate bitflags;
extern crate libc;
extern crate once_cell;

#[doc(hidden)]
pub extern crate glib_sys;
#[doc(hidden)]
pub extern crate gobject_sys;

extern crate glib_macros;
pub use glib_macros::{gflags, GBoxed, GEnum};

extern crate futures_channel;
extern crate futures_core;
extern crate futures_executor;
extern crate futures_task;
extern crate futures_util;

pub use byte_array::ByteArray;
pub use bytes::Bytes;
pub use closure::Closure;
pub use error::{BoolError, Error};
pub use file_error::FileError;
pub use object::{
    Cast, InitiallyUnowned, InitiallyUnownedClass, IsA, IsClassFor, Object, ObjectClass, ObjectExt,
    ObjectType, SendWeakRef, WeakRef,
};
pub use signal::{
    signal_handler_block, signal_handler_disconnect, signal_handler_unblock,
    signal_stop_emission_by_name, SignalHandlerId,
};
use std::ffi::CStr;
pub use string::String;

pub use enums::{EnumClass, EnumValue, FlagsBuilder, FlagsClass, FlagsValue, UserDirectory};
pub use time_val::{get_current_time, TimeVal};
pub use types::{StaticType, Type};
pub use value::{SendValue, ToSendValue, ToValue, TypedValue, Value};
pub use variant::{FromVariant, StaticVariantType, ToVariant, Variant};
pub use variant_dict::VariantDict;
pub use variant_iter::VariantIter;
pub use variant_type::{VariantTy, VariantType};

#[macro_use]
pub mod clone;
#[macro_use]
pub mod wrapper;
#[macro_use]
pub mod boxed;
#[macro_use]
pub mod shared;
#[macro_use]
pub mod error;
#[macro_use]
pub mod object;

pub use auto::functions::*;
pub use auto::*;
#[allow(clippy::let_and_return)]
#[allow(clippy::let_unit_value)]
#[allow(clippy::too_many_arguments)]
#[allow(non_upper_case_globals)]
mod auto;

pub use gobject::*;
mod gobject;

mod byte_array;
mod bytes;
pub mod char;
mod string;
pub use char::*;
mod checksum;
pub mod closure;
mod enums;
mod file_error;
mod functions;
pub use functions::*;
mod key_file;
pub mod prelude;
pub mod signal;
pub mod source;
pub use source::*;
mod time_val;
#[macro_use]
pub mod translate;
mod gstring;
pub use gstring::GString;
pub mod types;
mod utils;
pub use utils::*;
mod main_context;
mod main_context_channel;
pub mod value;
pub mod variant;
mod variant_dict;
mod variant_iter;
mod variant_type;
pub use main_context_channel::{Receiver, Sender, SyncSender};
mod date;
pub use date::Date;
mod value_array;
pub use value_array::ValueArray;
mod param_spec;
pub use param_spec::*;
mod quark;
pub use quark::Quark;
#[macro_use]
mod log;
#[cfg(any(feature = "v2_46", feature = "dox"))]
pub use log::log_set_handler;

// #[cfg(any(feature = "v2_50", feature = "dox"))]
// pub use log::log_variant;
pub use log::{
    log_default_handler, log_remove_handler, log_set_always_fatal, log_set_default_handler,
    log_set_fatal_mask, log_unset_default_handler, set_print_handler, set_printerr_handler,
    unset_print_handler, unset_printerr_handler, LogHandlerId, LogLevel, LogLevels,
};

#[cfg(any(feature = "log", feature = "dox"))]
#[macro_use]
mod bridged_logging;
#[cfg(any(feature = "log", feature = "dox"))]
pub use bridged_logging::{GlibLogger, GlibLoggerDomain, GlibLoggerFormat};

pub mod send_unique;
pub use send_unique::{SendUnique, SendUniqueCell};

#[macro_use]
pub mod subclass;

mod main_context_futures;
mod source_futures;
pub use source_futures::*;

mod thread_pool;
pub use thread_pool::ThreadPool;

/// This is the log domain used by the [`clone!`][crate::clone] macro. If you want to use a custom
/// logger (it prints to stdout by default), you can set your own logger using the corresponding
/// `log` functions.
pub const CLONE_MACRO_LOG_DOMAIN: &str = "glib-rs-clone";

// Actual thread IDs can be reused by the OS once the old thread finished.
// This works around it by using our own counter for threads.
//
// Taken from the fragile crate
use std::sync::atomic::{AtomicUsize, Ordering};
fn next_thread_id() -> usize {
    static mut COUNTER: AtomicUsize = AtomicUsize::new(0);
    unsafe { COUNTER.fetch_add(1, Ordering::SeqCst) }
}

pub(crate) fn get_thread_id() -> usize {
    thread_local!(static THREAD_ID: usize = next_thread_id());
    THREAD_ID.with(|&x| x)
}

pub(crate) struct ThreadGuard<T> {
    thread_id: usize,
    value: T,
}

impl<T> ThreadGuard<T> {
    pub(crate) fn new(value: T) -> Self {
        Self {
            thread_id: get_thread_id(),
            value,
        }
    }

    pub(crate) fn get_ref(&self) -> &T {
        if self.thread_id != get_thread_id() {
            panic!("Value accessed from different thread than where it was created");
        }

        &self.value
    }

    pub(crate) fn get_mut(&mut self) -> &mut T {
        if self.thread_id != get_thread_id() {
            panic!("Value accessed from different thread than where it was created");
        }

        &mut self.value
    }
}

impl<T> Drop for ThreadGuard<T> {
    fn drop(&mut self) {
        if self.thread_id != get_thread_id() {
            panic!("Value dropped on a different thread than where it was created");
        }
    }
}

unsafe impl<T> Send for ThreadGuard<T> {}

// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate proc_macro;

mod gboxed_derive;
mod genum_derive;
mod gflags_attribute;
mod utils;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput, LitStr};

#[proc_macro_derive(GEnum, attributes(genum))]
#[proc_macro_error]
pub fn genum_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let gen = genum_derive::impl_genum(&input);
    gen.into()
}

/// Derive macro for defining a [`BoxedType`]`::get_type` function and
/// the [`glib::Value`] traits.
///
/// # Example
///
/// ```
/// #[macro_use] extern crate glib;
/// use glib::prelude::*;
/// use glib::subclass::prelude::*;
///
/// #[derive(Clone, Debug, PartialEq, Eq, GBoxed)]
/// #[gboxed(type_name = "MyBoxed")]
/// struct MyBoxed(String);
/// ```
///
/// [`BoxedType`]: subclass/boxed/trait.BoxedType.html
/// [`glib::Value`]: value/struct.Value.html
#[proc_macro_derive(GBoxed, attributes(gboxed))]
#[proc_macro_error]
pub fn gboxed_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let gen = gboxed_derive::impl_gboxed(&input);
    gen.into()
}

/// Attribute macro for defining flags using the `bitflags` crate.
/// This macro will also define a `GFlags::get_type` function and
/// the [`glib::Value`] traits.
///
/// The expected `GType` name has to be passed as macro attribute.
/// The name and nick of each flag can also be optionally defined.
/// Default name is the flag identifier in CamelCase and default nick
/// is the identifier in kebab-case.
/// Combined flags should not be registered with the `GType` system
/// and so needs to be tagged with the `#[gflags(skip)]` attribute.
///
/// # Example
///
/// ```
/// #[macro_use] extern crate glib;
/// use glib::prelude::*;
/// use glib::subclass::prelude::*;
///
/// #[gflags("MyFlags")]
/// enum MyFlags {
///     #[gflags(name = "Flag A", nick = "nick-a")]
///     A = 0b00000001,
///     #[gflags(name = "Flag B")]
///     B = 0b00000010,
///     #[gflags(skip)]
///     AB = Self::A.bits() | Self::B.bits(),
///     C = 0b00000100,
/// }
/// ```
///
/// [`glib::Value`]: value/struct.Value.html
#[proc_macro_attribute]
#[proc_macro_error]
pub fn gflags(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let gtype_name = parse_macro_input!(attr as LitStr);
    let gen = gflags_attribute::impl_gflags(&input, &gtype_name);
    gen.into()
}

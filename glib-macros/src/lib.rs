// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate proc_macro;

mod gboxed_derive;
mod genum_derive;
mod utils;

use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(GEnum, attributes(genum))]
#[proc_macro_error]
pub fn genum_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
pub fn gboxed_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let gen = gboxed_derive::impl_gboxed(&input);
    gen.into()
}

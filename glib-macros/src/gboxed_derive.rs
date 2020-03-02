// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;

use crate::utils::parse_type_name;

pub fn impl_gboxed(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;

    let gtype_name = match parse_type_name(&input, "gboxed") {
        Ok(v) => v,
        Err(e) => abort_call_site!(
            "{}: derive(GBoxed) requires #[gboxed(type_name = \"BoxedTypeName\")]",
            e
        ),
    };

    quote! {
        impl BoxedType for #name {
            const NAME: &'static str = #gtype_name;
            glib_boxed_type!();
        }

        glib_boxed_derive_traits!(#name);
    }
}

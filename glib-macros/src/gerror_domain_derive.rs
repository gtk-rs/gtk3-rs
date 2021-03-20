// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::Data;

use crate::utils::{crate_ident_new, gen_enum_from_glib, parse_name};

pub fn impl_gerror_domain(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;

    let crate_ident = crate_ident_new();

    let enum_variants = match input.data {
        Data::Enum(ref e) => &e.variants,
        _ => abort_call_site!("GErrorDomain only supports enums"),
    };

    let domain_name = match parse_name(&input, "gerror_domain") {
        Ok(v) => v,
        Err(e) => abort_call_site!(
            "{}: derive(GErrorDomain) requires #[gerror_domain(name = \"DomainName\")]",
            e
        ),
    };
    let from_glib = gen_enum_from_glib(name, enum_variants);

    quote! {
        impl #crate_ident::error::ErrorDomain for #name {
            fn domain() -> #crate_ident::Quark {
                use #crate_ident::translate::from_glib;

                static QUARK: #crate_ident::once_cell::sync::Lazy<#crate_ident::Quark> =
                    #crate_ident::once_cell::sync::Lazy::new(|| unsafe {
                        from_glib(#crate_ident::ffi::g_quark_from_static_string(concat!(#domain_name, "\0") as *const str as *const _))
                    });
                *QUARK
            }

            fn code(self) -> i32 {
                self as i32
            }

            fn from(value: i32) -> Option<Self>
            where
                Self: Sized
            {
                #from_glib
            }
        }
    }
}

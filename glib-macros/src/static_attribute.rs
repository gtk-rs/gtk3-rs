// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{ItemFn, ReturnType, Type, TypePath, TypeReference, TypeSlice};

pub fn impl_lazy_static(input: ItemFn) -> TokenStream {
    let vis = &input.vis;
    let block = &input.block;
    let name = &input.sig.ident;
    let output = &input.sig.output;

    // Extract the function return_type
    let return_type = match output {
        ReturnType::Type(_, ret_type) => ret_type,
        syn::ReturnType::Default => {
            abort_call_site!("#[lazy_static] expects a return type")
        }
    };
    let return_type_path = match &**return_type {
        Type::Reference(TypeReference { elem, .. }) => match &**elem {
            // In case we have &'static [T]
            Type::Slice(TypeSlice { elem, .. }) => {
                if let Type::Path(TypePath { path, .. }) = &**elem {
                    quote! { Vec<#path> }
                } else {
                    abort_call_site!("#[lazy_static] expects a [T] return type")
                }
            }
            // In case we have &'static T
            Type::Path(TypePath { path, .. }) => {
                // Special case str -> String
                let string_path: syn::Path = syn::parse((quote! { str }).into()).unwrap();
                let std_path_path: syn::Path =
                    syn::parse((quote! { std::path::Path }).into()).unwrap();
                if &string_path == path {
                    quote! { String }
                } else if &std_path_path == path {
                    quote! { std::path::PathBuf }
                } else {
                    quote! { #path }
                }
            }
            _ => abort_call_site!("#[lazy_static] expects a return type"),
        },
        _ => abort_call_site!("#[lazy_static] expects a &'static lifetime"),
    };

    (quote! {
        #vis fn #name() #output {
            static STATIC: once_cell::sync::Lazy<#return_type_path> = once_cell::sync::Lazy::new(|| {
                #block
            });
            STATIC.as_ref()
        }
    }).into()
}

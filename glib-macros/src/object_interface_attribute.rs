// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;

pub const WRONG_PLACE_MSG: &str =
    "This macro should be used on `impl` block for `glib::ObjectInterface` trait";

pub fn impl_object_interface(input: &syn::ItemImpl) -> TokenStream {
    let mut has_prerequisites = false;
    for item in &input.items {
        if let syn::ImplItem::Type(type_) = item {
            let name = type_.ident.to_string();
            if name == "Prerequisites" {
                has_prerequisites = true;
            }
        }
    }

    let syn::ItemImpl {
        attrs,
        generics,
        trait_,
        self_ty,
        items,
        ..
    } = &input;

    let prerequisites_opt = if has_prerequisites {
        None
    } else {
        Some(quote!(
            type Prerequisites = ();
        ))
    };

    let crate_ident = crate::utils::crate_ident_new();

    let trait_path = match &trait_ {
        Some(path) => &path.1,
        None => abort_call_site!(WRONG_PLACE_MSG),
    };

    quote! {
        #(#attrs)*
        impl#generics #trait_path for #self_ty {
            #prerequisites_opt
            #crate_ident::object_interface_internal!();
            #(#items)*
        }
    }
}

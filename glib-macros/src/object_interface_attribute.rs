// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;

pub const WRONG_PLACE_MSG: &str =
    "This macro should be used on `impl` block for `glib::ObjectInterface` trait";

pub fn impl_object_interface(input: &syn::ItemImpl) -> TokenStream {
    let syn::ItemImpl {
        attrs,
        generics,
        trait_,
        self_ty,
        items,
        ..
    } = &input;

    let crate_ident = crate::utils::crate_ident_new();

    let trait_path = match &trait_ {
        Some(path) => &path.1,
        None => abort_call_site!(WRONG_PLACE_MSG),
    };

    quote! {
        #(#attrs)*
        impl#generics #trait_path for #self_ty {
            #crate_ident::object_interface_internal!();
            #(#items)*
        }
    }
}

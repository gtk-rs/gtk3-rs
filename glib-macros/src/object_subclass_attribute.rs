// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;

pub const WRONG_PLACE_MSG: &str =
    "This macro should be used on `impl` block for `glib::ObjectSubclass` trait";

pub fn impl_object_subclass(input: &syn::ItemImpl) -> TokenStream {
    let mut has_new = false;
    let mut has_interfaces = false;
    let mut has_instance = false;
    let mut has_class = false;
    for item in &input.items {
        match item {
            syn::ImplItem::Method(method) => {
                let name = method.sig.ident.to_string();
                if name == "new" || name == "with_class" {
                    has_new = true;
                }
            }
            syn::ImplItem::Type(type_) => {
                let name = type_.ident.to_string();
                if name == "Interfaces" {
                    has_interfaces = true;
                } else if name == "Instance" {
                    has_instance = true;
                } else if name == "Class" {
                    has_class = true;
                }
            }
            _ => {}
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

    let interfaces_opt = if has_interfaces {
        None
    } else {
        Some(quote!(
            type Interfaces = ();
        ))
    };

    let new_opt = if has_new {
        None
    } else {
        Some(quote! {
            fn new() -> Self {
                std::default::Default::default()
            }
        })
    };

    let crate_ident = crate::utils::crate_ident_new();

    let class_opt = if has_class {
        None
    } else {
        Some(quote!(type Class = #crate_ident::subclass::basic::ClassStruct<Self>;))
    };

    let instance_opt = if has_instance {
        None
    } else {
        Some(quote!(type Instance = #crate_ident::subclass::basic::InstanceStruct<Self>;))
    };

    let trait_path = match &trait_ {
        Some(path) => &path.1,
        None => abort_call_site!(WRONG_PLACE_MSG),
    };

    quote! {
        #(#attrs)*
        impl#generics #trait_path for #self_ty {
            #interfaces_opt
            #class_opt
            #instance_opt
            #new_opt
            #(#items)*
        }

        unsafe impl #crate_ident::subclass::types::ObjectSubclassType for #self_ty {
            fn type_data() -> std::ptr::NonNull<#crate_ident::subclass::TypeData> {
                static mut DATA: #crate_ident::subclass::TypeData = #crate_ident::subclass::TypeData {
                    type_: #crate_ident::Type::INVALID,
                    parent_class: std::ptr::null_mut(),
                    parent_ifaces: None,
                    class_data: None,
                    private_offset: 0,
                    private_imp_offset: 0,
                };

                unsafe { std::ptr::NonNull::new_unchecked(&mut DATA) }
            }

            fn get_type() -> #crate_ident::Type {
                static ONCE: std::sync::Once = std::sync::Once::new();

                ONCE.call_once(|| {
                    #crate_ident::subclass::register_type::<Self>();
                });

                unsafe {
                    let data = Self::type_data();
                    let type_ = data.as_ref().get_type();
                    assert!(type_.is_valid());

                    type_
                }
            }
        }
    }
}

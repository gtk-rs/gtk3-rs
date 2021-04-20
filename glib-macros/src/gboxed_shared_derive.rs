// Take a look at the license at the top of the repository in the LICENSE file.

use crate::utils::{crate_ident_new, find_attribute_meta, find_nested_meta, parse_type_name};
use proc_macro2::{Ident, TokenStream};
use proc_macro_error::abort_call_site;
use quote::quote;

fn gen_impl_set_value_optional(name: &Ident, crate_ident: &TokenStream) -> TokenStream {
    let refcounted_type_prefix = refcounted_type_prefix(name, crate_ident);

    quote! {
        impl #crate_ident::value::SetValueOptional for #name {
            unsafe fn set_value_optional(value: &mut #crate_ident::value::Value, this: Option<&Self>) {
                let ptr = match this {
                    Some(this) => #refcounted_type_prefix::into_raw(this.0.clone()),
                    None => std::ptr::null(),
                };

                #crate_ident::gobject_ffi::g_value_take_boxed(
                    #crate_ident::translate::ToGlibPtrMut::to_glib_none_mut(value).0,
                    ptr as *mut _,
                );
            }
        }
    }
}

fn gen_impl_from_value(name: &Ident, crate_ident: &TokenStream) -> TokenStream {
    let refcounted_type_prefix = refcounted_type_prefix(name, crate_ident);

    quote! {
        impl<'a> #crate_ident::value::FromValue<'a> for #name {
            unsafe fn from_value(value: &'a #crate_ident::value::Value) -> Self {
                let ptr = #crate_ident::gobject_ffi::g_value_dup_boxed(
                    #crate_ident::translate::ToGlibPtr::to_glib_none(value).0,
                );
                assert!(!ptr.is_null());
                #name(#refcounted_type_prefix::from_raw(ptr as *mut _))
            }
        }
    }
}

fn gen_ptr_to_option(name: &Ident, nullable: bool, crate_ident: &TokenStream) -> TokenStream {
    let refcounted_type_prefix = refcounted_type_prefix(name, crate_ident);

    if nullable {
        quote! {
            if ptr.is_null() {
                None
            } else {
                Some(#name(#refcounted_type_prefix::from_raw(ptr as *mut _)))
            }
        }
    } else {
        quote! {
            assert!(!ptr.is_null());
            Some(#name(#refcounted_type_prefix::from_raw(ptr as *mut _)))
        }
    }
}

fn refcounted_type(input: &syn::DeriveInput) -> Option<&syn::TypePath> {
    let fields = match &input.data {
        syn::Data::Struct(s) => &s.fields,
        _ => return None,
    };

    let unnamed = match fields {
        syn::Fields::Unnamed(u) if u.unnamed.len() == 1 => &u.unnamed[0],
        _ => return None,
    };

    let refcounted = match &unnamed.ty {
        syn::Type::Path(p) => p,
        _ => return None,
    };

    Some(refcounted)
}

fn refcounted_type_prefix(name: &Ident, crate_ident: &TokenStream) -> proc_macro2::TokenStream {
    quote! {
        <<#name as #crate_ident::subclass::shared::SharedType>::RefCountedType as #crate_ident::subclass::shared::RefCounted>
    }
}

pub fn impl_gshared_boxed(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let refcounted_type = match refcounted_type(input) {
        Some(p) => p,
        _ => abort_call_site!("derive(GSharedBoxed) requires struct MyStruct(T: RefCounted)"),
    };

    let name = &input.ident;
    let gtype_name = match parse_type_name(&input, "gshared_boxed") {
        Ok(v) => v,
        Err(e) => abort_call_site!(
            "{}: derive(GSharedBoxed) requires #[gshared_boxed(type_name = \"SharedTypeName\")]",
            e
        ),
    };

    let meta = find_attribute_meta(&input.attrs, "gshared_boxed")
        .unwrap()
        .unwrap();
    let nullable = find_nested_meta(&meta, "nullable").is_some();
    let crate_ident = crate_ident_new();
    let refcounted_type_prefix = refcounted_type_prefix(name, &crate_ident);
    let ptr_to_option = gen_ptr_to_option(name, nullable, &crate_ident);

    let impl_from_value = if !nullable {
        gen_impl_from_value(name, &crate_ident)
    } else {
        quote! {}
    };

    let impl_set_value_optional = if nullable {
        gen_impl_set_value_optional(name, &crate_ident)
    } else {
        quote! {}
    };

    quote! {
        impl #crate_ident::subclass::shared::SharedType for #name {
            const NAME: &'static str = #gtype_name;

            type RefCountedType = #refcounted_type;

            fn type_() -> #crate_ident::Type {
                static mut TYPE_: #crate_ident::Type = #crate_ident::Type::INVALID;
                static ONCE: ::std::sync::Once = ::std::sync::Once::new();

                ONCE.call_once(|| {
                    let type_ = #crate_ident::subclass::shared::register_shared_type::<Self>();
                    unsafe {
                        TYPE_ = type_;
                    }
                });

                unsafe { TYPE_ }
            }

            fn from_refcounted(this: Self::RefCountedType) -> Self {
                Self(this)
            }

            fn into_refcounted(self) -> Self::RefCountedType {
                self.0
            }
        }

        impl #crate_ident::StaticType for #name {
            fn static_type() -> #crate_ident::Type {
                <#name as #crate_ident::subclass::shared::SharedType>::get_type()
            }
        }

        impl #crate_ident::value::SetValue for #name {
            unsafe fn set_value(value: &mut #crate_ident::value::Value, this: &Self) {
                let ptr = #refcounted_type_prefix::into_raw(this.0.clone());
                #crate_ident::gobject_ffi::g_value_take_boxed(
                    #crate_ident::translate::ToGlibPtrMut::to_glib_none_mut(value).0,
                    ptr as *mut _,
                );
            }
        }

        #impl_set_value_optional

        impl<'a> #crate_ident::value::FromValueOptional<'a> for #name {
            unsafe fn from_value_optional(value: &'a #crate_ident::value::Value) -> Option<Self> {
                let ptr = #crate_ident::gobject_ffi::g_value_dup_boxed(
                    #crate_ident::translate::ToGlibPtr::to_glib_none(value).0,
                );
                #ptr_to_option
            }
        }

        #impl_from_value
    }
}

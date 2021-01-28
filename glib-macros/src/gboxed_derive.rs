// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::{Ident, TokenStream};
use proc_macro_error::abort_call_site;
use quote::quote;

use crate::utils::{crate_ident_new, find_attribute_meta, find_nested_meta, parse_type_name};

fn gen_option_to_ptr() -> TokenStream {
    quote! {
        match this {
            Some(this) => Box::into_raw(Box::new(this.clone())),
            None => std::ptr::null_mut(),
        };
    }
}

fn gen_ptr_to_option(name: &Ident, nullable: bool) -> TokenStream {
    if nullable {
        quote! {
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *mut #name))
            }
        }
    } else {
        quote! {
            assert!(!ptr.is_null());
            Some(&*(ptr as *mut #name))
        }
    }
}

fn gen_impl_from_value(name: &Ident, crate_ident: &Ident) -> TokenStream {
    quote! {
        impl<'a> #crate_ident::value::FromValue<'a> for &'a #name {
            unsafe fn from_value(value: &'a #crate_ident::value::Value) -> Self {
                let ptr = #crate_ident::gobject_ffi::g_value_get_boxed(
                    #crate_ident::translate::ToGlibPtr::to_glib_none(value).0,
                );
                assert!(!ptr.is_null());
                &*(ptr as *mut #name)
            }
        }
    }
}

fn gen_impl_set_value_optional(name: &Ident, crate_ident: &Ident) -> TokenStream {
    let option_to_ptr = gen_option_to_ptr();

    quote! {
        impl #crate_ident::value::SetValueOptional for #name {
            unsafe fn set_value_optional(value: &mut #crate_ident::value::Value, this: Option<&Self>) {
                let ptr: *mut #name = #option_to_ptr;
                #crate_ident::gobject_ffi::g_value_take_boxed(
                    #crate_ident::translate::ToGlibPtrMut::to_glib_none_mut(value).0,
                    ptr as *mut _,
                );
            }
        }
    }
}

pub fn impl_gboxed(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;

    let gtype_name = match parse_type_name(&input, "gboxed") {
        Ok(v) => v,
        Err(e) => abort_call_site!(
            "{}: derive(GBoxed) requires #[gboxed(type_name = \"BoxedTypeName\")]",
            e
        ),
    };

    let crate_ident = crate_ident_new();

    let meta = find_attribute_meta(&input.attrs, "gboxed")
        .unwrap()
        .unwrap();
    let nullable = find_nested_meta(&meta, "nullable").is_some();

    let ptr_to_option = gen_ptr_to_option(name, nullable);
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
        impl #crate_ident::subclass::boxed::BoxedType for #name {
            const NAME: &'static str = #gtype_name;

            fn get_type() -> #crate_ident::Type {
                static mut TYPE_: #crate_ident::Type = #crate_ident::Type::Invalid;
                static ONCE: ::std::sync::Once = ::std::sync::Once::new();

                ONCE.call_once(|| {
                    let type_ = #crate_ident::subclass::register_boxed_type::<Self>();
                    unsafe {
                        TYPE_ = type_;
                    }
                });

                unsafe { TYPE_ }
            }
        }

        impl #crate_ident::StaticType for #name {
            fn static_type() -> #crate_ident::Type {
                <#name as #crate_ident::subclass::boxed::BoxedType>::get_type()
            }
        }

        impl #crate_ident::value::SetValue for #name {
            unsafe fn set_value(value: &mut #crate_ident::value::Value, this: &Self) {
                let ptr: *mut #name = Box::into_raw(Box::new(this.clone()));
                #crate_ident::gobject_ffi::g_value_take_boxed(
                    #crate_ident::translate::ToGlibPtrMut::to_glib_none_mut(value).0,
                    ptr as *mut _,
                );
            }
        }

        #impl_set_value_optional

        impl<'a> #crate_ident::value::FromValueOptional<'a> for &'a #name {
            unsafe fn from_value_optional(value: &'a #crate_ident::value::Value) -> Option<Self> {
                let ptr = #crate_ident::gobject_ffi::g_value_get_boxed(
                    #crate_ident::translate::ToGlibPtr::to_glib_none(value).0,
                );
                #ptr_to_option
            }
        }

        #impl_from_value
    }
}

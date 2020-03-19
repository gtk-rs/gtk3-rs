// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::abort_call_site;
use proc_macro_crate::crate_name;
use quote::quote;

use crate::utils::{find_attribute_meta, find_nested_meta, parse_type_name};

fn gen_option_to_ptr(nullable: bool) -> TokenStream {
    if nullable {
        quote! {
            match this {
                Some(this) => Box::into_raw(Box::new(this.clone())),
                None => std::ptr::null_mut(),
            };
        }
    } else {
        quote! {
            Box::into_raw(Box::new(this.expect("None not allowed").clone()));
        }
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
                let ptr = #crate_ident::gobject_sys::g_value_get_boxed(
                    #crate_ident::translate::ToGlibPtr::to_glib_none(value).0,
                );
                assert!(!ptr.is_null());
                &*(ptr as *mut #name)
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

    let crate_name = match crate_name("glib") {
        Ok(x) => x,
	Err(_) => {
            // In case we use it directly from glib itself (it cannot find glib as a dependency
	    // in this case)
            "glib".to_owned()
	}
    };
    let crate_ident = Ident::new(&crate_name, Span::call_site());

    let meta = find_attribute_meta(&input.attrs, "gboxed")
        .unwrap()
        .unwrap();
    let nullable = find_nested_meta(&meta, "nullable").is_some();

    let option_to_ptr = gen_option_to_ptr(nullable);
    let ptr_to_option = gen_ptr_to_option(name, nullable);
    let impl_from_value = if !nullable {
        gen_impl_from_value(name, &crate_ident)
    } else {
        quote! {}
    };

    quote! {
        impl BoxedType for #name {
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
                #crate_ident::gobject_sys::g_value_take_boxed(
                    #crate_ident::translate::ToGlibPtrMut::to_glib_none_mut(value).0,
                    ptr as *mut _,
                );
            }
        }

        impl #crate_ident::value::SetValueOptional for #name {
            unsafe fn set_value_optional(value: &mut #crate_ident::value::Value, this: Option<&Self>) {
                let ptr: *mut #name = #option_to_ptr;
                #crate_ident::gobject_sys::g_value_take_boxed(
                    #crate_ident::translate::ToGlibPtrMut::to_glib_none_mut(value).0,
                    ptr as *mut _,
                );
            }
        }

        impl<'a> #crate_ident::value::FromValueOptional<'a> for &'a #name {
            unsafe fn from_value_optional(value: &'a #crate_ident::value::Value) -> Option<Self> {
                let ptr = #crate_ident::gobject_sys::g_value_get_boxed(
                    #crate_ident::translate::ToGlibPtr::to_glib_none(value).0,
                );
                #ptr_to_option
            }
        }

        #impl_from_value
    }
}

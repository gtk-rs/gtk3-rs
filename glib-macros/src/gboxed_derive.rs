// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use proc_macro2::{Ident, TokenStream};
use proc_macro_error::abort_call_site;
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

fn gen_impl_from_value(name: &Ident) -> TokenStream {
    quote! {
        impl<'a> glib::value::FromValue<'a> for &'a #name {
            unsafe fn from_value(value: &'a glib::value::Value) -> Self {
                let ptr = glib::gobject_sys::g_value_get_boxed(
                    glib::translate::ToGlibPtr::to_glib_none(value).0,
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

    let meta = find_attribute_meta(&input.attrs, "gboxed")
        .unwrap()
        .unwrap();
    let nullable = find_nested_meta(&meta, "nullable").is_some();

    let option_to_ptr = gen_option_to_ptr(nullable);
    let ptr_to_option = gen_ptr_to_option(name, nullable);
    let impl_from_value = if !nullable {
        gen_impl_from_value(name)
    } else {
        quote! {}
    };

    quote! {
        impl BoxedType for #name {
            const NAME: &'static str = #gtype_name;

            fn get_type() -> glib::Type {
                static mut TYPE_: glib::Type = glib::Type::Invalid;
                static ONCE: ::std::sync::Once = ::std::sync::Once::new();

                ONCE.call_once(|| {
                    let type_ = glib::subclass::register_boxed_type::<Self>();
                    unsafe {
                        TYPE_ = type_;
                    }
                });

                unsafe { TYPE_ }
            }
        }

        impl glib::StaticType for #name {
            fn static_type() -> glib::Type {
                <#name as glib::subclass::boxed::BoxedType>::get_type()
            }
        }

        impl glib::value::SetValue for #name {
            unsafe fn set_value(value: &mut glib::value::Value, this: &Self) {
                let ptr: *mut #name = Box::into_raw(Box::new(this.clone()));
                glib::gobject_sys::g_value_take_boxed(
                    glib::translate::ToGlibPtrMut::to_glib_none_mut(value).0,
                    ptr as *mut _,
                );
            }
        }

        impl glib::value::SetValueOptional for #name {
            unsafe fn set_value_optional(value: &mut glib::value::Value, this: Option<&Self>) {
                let ptr: *mut #name = #option_to_ptr;
                glib::gobject_sys::g_value_take_boxed(
                    glib::translate::ToGlibPtrMut::to_glib_none_mut(value).0,
                    ptr as *mut _,
                );
            }
        }

        impl<'a> glib::value::FromValueOptional<'a> for &'a #name {
            unsafe fn from_value_optional(value: &'a glib::value::Value) -> Option<Self> {
                let ptr = glib::gobject_sys::g_value_get_boxed(
                    glib::translate::ToGlibPtr::to_glib_none(value).0,
                );
                #ptr_to_option
            }
        }

        #impl_from_value
    }
}

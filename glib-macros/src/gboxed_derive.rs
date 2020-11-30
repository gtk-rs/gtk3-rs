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

fn gen_impl_from_value(name: &Ident, crate_ident: &Ident) -> TokenStream {
    quote! {
        impl<'a> #crate_ident::value::FromValue<'a> for #name {
            type Error = #crate_ident::value::WrongValueTypeOrNoneError;

            fn check(value: &'a #crate_ident::Value) -> Result<(), Self::Error> {
                #crate_ident::value::WrongValueTypeError::check::<#name>(value)?;

                unsafe {
                    let ptr = #crate_ident::gobject_ffi::g_value_get_boxed(#crate_ident::translate::ToGlibPtr::to_glib_none(value).0);
                    if ptr.is_null() {
                        return Err(#crate_ident::value::WrongValueTypeOrNoneError::UnexpectedNone);
                    }
                }

                Ok(())
            }

            fn from_value(value: &'a #crate_ident::Value) -> Result<Self, Self::Error> {
                Self::check(value)?;

                unsafe {
                    let ptr = #crate_ident::gobject_ffi::g_value_dup_boxed(#crate_ident::translate::ToGlibPtr::to_glib_none(value).0);
                    assert!(!ptr.is_null());
                    Ok(Box::from_raw(ptr as *mut #name))
                }
            }
        }

        impl<'a> #crate_ident::value::FromValue<'a> for &'a #name {
            type Error = #crate_ident::value::WrongValueTypeOrNoneError;

            fn check(value: &'a #crate_ident::Value) -> Result<(), Self::Error> {
                <Self as #crate_ident::value::FromValue>::check(value)
            }

            fn from_value(value: &'a #crate_ident::Value) -> Result<Self, Self::Error> {
                Self::check(value)?;

                unsafe {
                    let ptr = #crate_ident::gobject_ffi::g_value_get_boxed(#crate_ident::translate::ToGlibPtr::to_glib_none(value).0);
                    assert!(!ptr.is_null());
                    Ok(&*(ptr as *mut #name))
                }
            }
        }
    }
}

fn gen_impl_from_value_optional(name: &Ident, crate_ident: &Ident) -> TokenStream {
    quote! {
        impl<'a> #crate_ident::value::FromValue<'a> for #name {
            type Error = #crate_ident::value::WrongValueTypeError;

            fn check(value: &'a #crate_ident::Value) -> Result<(), Self::Error> {
                #crate_ident::value::WrongValueTypeError::check::<#name>(value)
            }

            fn from_value(value: &'a #crate_ident::Value) -> Result<Self, Self::Error> {
                Self::check(value)?;

                unsafe {
                    let ptr = #crate_ident::gobject_ffi::g_value_dup_boxed(#crate_ident::translate::ToGlibPtr::to_glib_none(value).0);
                    assert!(!ptr.is_null());
                    Ok(Box::from_raw(ptr as *mut #name))
                }
            }
        }

        impl<'a> #crate_ident::value::FromValue<'a> for &'a #name {
            type Error = #crate_ident::value::WrongValueTypeError;

            fn check(value: &'a #crate_ident::Value) -> Result<(), Self::Error> {
                <Self as #crate_ident::value::FromValue>::check(value)
            }

            fn from_value(value: &'a #crate_ident::Value) -> Result<Self, Self::Error> {
                Self::check(value)?;

                unsafe {
                    let ptr = #crate_ident::gobject_ffi::g_value_get_boxed(#crate_ident::translate::ToGlibPtr::to_glib_none(value).0);
                    assert!(!ptr.is_null());
                    Ok(&*(ptr as *mut #name))
                }
            }
        }
    }
}

fn gen_impl_to_value_optional(name: &Ident, crate_ident: &Ident) -> TokenStream {
    let option_to_ptr = gen_option_to_ptr();

    quote! {
        impl #crate_ident::value::ToValueOptional for #name {
            fn to_value_optional(s: &Option<Self>) -> #crate_ident::Value {
                let mut value = #crate_ident::Value::for_value_type::<#name>();
                unsafe {
                    let ptr: *mut #name = #option_to_ptr;
                    #crate_ident::gobject_ffi::g_value_take_boxed(#crate_ident::translate::ToGlibPtrMut::to_glib_none_mut(value).0, ptr as *mut _);
                }

                value
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

    let impl_from_value = if !nullable {
        gen_impl_from_value(name, &crate_ident)
    } else {
        gen_impl_from_value_optional(name, &crate_ident)
    };
    let impl_to_value_optional = if nullable {
        gen_impl_to_value_optional(name, &crate_ident)
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

        impl #crate_ident::value::ValueType for #name {
            type Type = #name;
        }

        impl #crate_ident::value::ToValue for #name {
            fn to_value(&self) -> #crate_ident::Value {
                unsafe {
                    let mut value = #crate_ident::Value::from_type(<#name as #crate_ident::StaticType>::static_type());
                    #crate_ident::gobject_ffi::g_value_take_boxed(#crate_ident::translate::ToGlibPtrMut::to_glib_none_mut(value).0, #crate_ident::translate::ToGlibPtr::to_glib_full(self) as *mut _);
                    value
                }
            }

            fn to_value_type(&self) -> #crate_ident::Type {
                <#name as #crate_ident::StaticType>::static_type()
            }
        }

        #impl_to_value_optional

        #impl_from_value
    }
}

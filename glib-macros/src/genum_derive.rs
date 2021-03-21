// Take a look at the license at the top of the repository in the LICENSE file.

use heck::{CamelCase, KebabCase, SnakeCase};
use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::{format_ident, quote, quote_spanned};
use syn::{punctuated::Punctuated, spanned::Spanned, token::Comma, Data, Ident, Variant};

use crate::utils::{
    crate_ident_new, gen_enum_from_glib, parse_item_attributes, parse_type_name, ItemAttribute,
};

// Generate glib::gobject_ffi::GEnumValue structs mapping the enum such as:
//     glib::gobject_ffi::GEnumValue {
//         value: Animal::Goat as i32,
//         value_name: "Goat\0" as *const _ as *const _,
//         value_nick: "goat\0" as *const _ as *const _,
//     },
fn gen_genum_values(
    enum_name: &Ident,
    enum_variants: &Punctuated<Variant, Comma>,
) -> (TokenStream, usize) {
    let crate_ident = crate_ident_new();

    // start at one as GEnumValue array is null-terminated
    let mut n = 1;
    let recurse = enum_variants.iter().map(|v| {
        let name = &v.ident;
        let mut value_name = name.to_string().to_camel_case();
        let mut value_nick = name.to_string().to_kebab_case();

        let attrs = parse_item_attributes("genum", &v.attrs);
        let attrs = match attrs {
            Ok(attrs) => attrs,
            Err(e) => abort_call_site!(
                "{}: GEnum enum supports only the following optional attributes: #[genum(name = \"The Cat\", nick = \"chat\")]",
                e
            ),
        };

        attrs.into_iter().for_each(|attr|
            match attr {
                ItemAttribute::Name(n) => value_name = n,
                ItemAttribute::Nick(n) => value_nick = n,
            }
        );

        let value_name = format!("{}\0", value_name);
        let value_nick = format!("{}\0", value_nick);

        n += 1;
        quote_spanned! {v.span()=>
            #crate_ident::gobject_ffi::GEnumValue {
                value: #enum_name::#name as i32,
                value_name: #value_name as *const _ as *const _,
                value_nick: #value_nick as *const _ as *const _,
            },
        }
    });
    (
        quote! {
            #(#recurse)*
        },
        n,
    )
}

pub fn impl_genum(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;

    let crate_ident = crate_ident_new();

    let enum_variants = match input.data {
        Data::Enum(ref e) => &e.variants,
        _ => abort_call_site!("GEnum only supports enums"),
    };

    let gtype_name = match parse_type_name(&input, "genum") {
        Ok(v) => v,
        Err(e) => abort_call_site!(
            "{}: derive(GEnum) requires #[genum(type_name = \"EnumTypeName\")]",
            e
        ),
    };
    let get_type = format_ident!("{}_get_type", name.to_string().to_snake_case());
    let from_glib = gen_enum_from_glib(name, enum_variants);
    let (genum_values, nb_genum_values) = gen_genum_values(name, enum_variants);

    quote! {
        impl #crate_ident::translate::ToGlib for #name {
            type GlibType = i32;

            fn to_glib(&self) -> i32 {
                *self as i32
            }
        }

        impl #crate_ident::translate::TryFromGlib<i32> for #name {
            type Error = i32;

            fn try_from_glib(value: i32) -> Result<Self, i32> {
                let from_glib = || {
                    #from_glib
                };

                from_glib().ok_or(value)
            }
        }

        impl #crate_ident::translate::FromGlib<i32> for #name {
            unsafe fn from_glib(value: i32) -> Self {
                use #crate_ident::translate::TryFromGlib;

                Self::try_from_glib(value).unwrap()
            }
        }

        impl<'a> #crate_ident::value::FromValueOptional<'a> for #name {
            unsafe fn from_value_optional(value: &#crate_ident::Value) -> Option<Self> {
                Some(#crate_ident::value::FromValue::from_value(value))
            }
        }

        impl<'a> #crate_ident::value::FromValue<'a> for #name {
            unsafe fn from_value(value: &#crate_ident::Value) -> Self {
                #crate_ident::translate::from_glib(
                    #crate_ident::gobject_ffi::g_value_get_enum(
                        #crate_ident::translate::ToGlibPtr::to_glib_none(value).0))
            }
        }

        impl #crate_ident::value::SetValue for #name {
            unsafe fn set_value(value: &mut #crate_ident::Value, this: &Self) {
                #crate_ident::gobject_ffi::g_value_set_enum(
                    #crate_ident::translate::ToGlibPtrMut::to_glib_none_mut(value).0,
                    #crate_ident::translate::ToGlib::to_glib(this))
            }
        }

        impl #crate_ident::StaticType for #name {
            fn static_type() -> #crate_ident::Type {
                #get_type()
            }
        }

        fn #get_type() -> #crate_ident::Type {
            static ONCE: std::sync::Once = std::sync::Once::new();
            static mut TYPE: #crate_ident::Type = #crate_ident::Type::INVALID;

            ONCE.call_once(|| {
                static mut VALUES: [#crate_ident::gobject_ffi::GEnumValue; #nb_genum_values] = [
                    #genum_values
                    #crate_ident::gobject_ffi::GEnumValue {
                        value: 0,
                        value_name: std::ptr::null(),
                        value_nick: std::ptr::null(),
                    },
                ];

                let name = std::ffi::CString::new(#gtype_name).expect("CString::new failed");
                unsafe {
                    let type_ = #crate_ident::gobject_ffi::g_enum_register_static(name.as_ptr(), VALUES.as_ptr());
                    TYPE = #crate_ident::translate::from_glib(type_);
                }
            });

            unsafe {
                assert!(TYPE.is_valid());
                TYPE
            }
        }
    }
}

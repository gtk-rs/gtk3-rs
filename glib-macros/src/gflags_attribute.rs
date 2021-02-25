// Take a look at the license at the top of the repository in the LICENSE file.

use heck::{CamelCase, KebabCase, SnakeCase};
use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::{format_ident, quote, quote_spanned};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, Data, DeriveInput, Ident,
    LitStr, Variant, Visibility,
};

use crate::utils::{
    crate_ident_new, find_attribute_meta, find_nested_meta, parse_item_attributes, ItemAttribute,
};

// Flag is not registered if it has the #[gflags(skip)] meta
fn attribute_has_skip(attrs: &[Attribute]) -> bool {
    let meta = find_attribute_meta(attrs, "gflags").unwrap();

    match meta {
        None => false,
        Some(meta) => find_nested_meta(&meta, "skip").is_some(),
    }
}

// Generate glib::gobject_ffi::GFlagsValue structs mapping the enum such as:
//     glib::gobject_ffi::GFlagsValue {
//         value: MyFlags::A.bits(),
//         value_name: "The Name\0" as *const _ as *const _,
//         value_nick: "nick\0" as *const _ as *const _,
//     },
fn gen_gflags_values(
    enum_name: &Ident,
    enum_variants: &Punctuated<Variant, Comma>,
) -> (TokenStream, usize) {
    let crate_ident = crate_ident_new();

    // start at one as GFlagsValue array is null-terminated
    let mut n = 1;
    let recurse = enum_variants.iter().filter(|v| { !attribute_has_skip(&v.attrs) } ).map(|v| {
        let name = &v.ident;
        let mut value_name = name.to_string().to_camel_case();
        let mut value_nick = name.to_string().to_kebab_case();

        let attrs = parse_item_attributes("gflags", &v.attrs);
        let attrs = match attrs {
            Ok(attrs) => attrs,
            Err(e) => abort_call_site!(
                "{}: gflags enum supports only the following optional attributes: #[gflags(name = \"The Name\", nick = \"the-nick\")] or #[gflags(skip)]",
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
            #crate_ident::gobject_ffi::GFlagsValue {
                value: #enum_name::#name.bits(),
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

fn gen_bitflags(
    enum_name: &Ident,
    visibility: &Visibility,
    enum_variants: &Punctuated<Variant, Comma>,
    crate_ident: &Ident,
) -> TokenStream {
    let recurse = enum_variants.iter().map(|v| {
        let name = &v.ident;
        let disc = v.discriminant.as_ref().expect("missing discriminant");
        let value = &disc.1;

        quote_spanned! {v.span()=>
            const #name = #value;
        }
    });

    quote! {
        #crate_ident::bitflags::bitflags! {
            #visibility struct #enum_name: u32 {
                #(#recurse)*
            }
        }
    }
}

pub fn impl_gflags(input: &DeriveInput, gtype_name: &LitStr) -> TokenStream {
    let visibility = &input.vis;
    let name = &input.ident;
    let crate_ident = crate_ident_new();

    let enum_variants = match input.data {
        Data::Enum(ref e) => &e.variants,
        _ => abort_call_site!("gflags only supports enums"),
    };

    let bitflags = gen_bitflags(name, visibility, enum_variants, &crate_ident);

    let get_type = format_ident!("{}_get_type", name.to_string().to_snake_case());
    let (gflags_values, nb_gflags_values) = gen_gflags_values(name, enum_variants);

    quote! {
        #bitflags

        impl #crate_ident::translate::ToGlib for #name {
            type GlibType = u32;

            fn to_glib(&self) -> u32 {
                self.bits()
            }
        }

        impl #crate_ident::translate::FromGlib<u32> for #name {
            unsafe fn from_glib(value: u32) -> Self {
                #name::from_bits_truncate(value)
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
                    #crate_ident::gobject_ffi::g_value_get_flags(
                        #crate_ident::translate::ToGlibPtr::to_glib_none(value).0))
            }
        }

        impl #crate_ident::value::SetValue for #name {
            unsafe fn set_value(value: &mut #crate_ident::Value, this: &Self) {
                #crate_ident::gobject_ffi::g_value_set_flags(
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
                static mut VALUES: [#crate_ident::gobject_ffi::GFlagsValue; #nb_gflags_values] = [
                    #gflags_values
                    #crate_ident::gobject_ffi::GFlagsValue {
                        value: 0,
                        value_name: std::ptr::null(),
                        value_nick: std::ptr::null(),
                    },
                ];

                let name = std::ffi::CString::new(#gtype_name).expect("CString::new failed");
                unsafe {
                    let type_ = #crate_ident::gobject_ffi::g_flags_register_static(name.as_ptr(), VALUES.as_ptr());
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

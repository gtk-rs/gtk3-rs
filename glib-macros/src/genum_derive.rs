// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use anyhow::{bail, Result};
use heck::{CamelCase, KebabCase, SnakeCase};
use itertools::Itertools;
use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::{format_ident, quote, quote_spanned};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, Data, Ident, NestedMeta,
    Variant,
};

use crate::utils::{find_attribute_meta, parse_attribute, parse_type_name};

// Generate i32 to enum mapping, used to implement glib::translate::FromGlib<i32>, such as:
//   if value == Animal::Goat as i32 {
//       return Animal::Goat;
//   }
fn gen_from_glib(enum_name: &Ident, enum_variants: &Punctuated<Variant, Comma>) -> TokenStream {
    // FIXME: can we express this with a match()?
    let recurse = enum_variants.iter().map(|v| {
        let name = &v.ident;
        quote_spanned! {v.span()=>
            if value == #enum_name::#name as i32 {
                return #enum_name::#name;
            }
        }
    });
    quote! {
        #(#recurse)*
    }
}

#[derive(Debug)]
enum ItemAttribute {
    Name(String),
    Nick(String),
}

fn parse_item_attribute(meta: &NestedMeta) -> Result<ItemAttribute> {
    let (ident, v) = parse_attribute(meta)?;

    match ident.as_ref() {
        "name" => Ok(ItemAttribute::Name(v)),
        "nick" => Ok(ItemAttribute::Nick(v)),
        s => bail!("Unknown item meta {}", s),
    }
}

// Parse optional enum item attributes such as:
// #[genum(name = "My Name", nick = "my-nick")]
fn parse_item_attributes(attrs: &[Attribute]) -> Result<Vec<ItemAttribute>> {
    let meta = find_attribute_meta(attrs, "genum")?;

    let v = match meta {
        Some(meta) => meta
            .nested
            .iter()
            .map(|m| parse_item_attribute(&m))
            .fold_results(Vec::new(), |mut v, a| {
                v.push(a);
                v
            })?,
        None => Vec::new(),
    };

    Ok(v)
}

// Generate gobject_sys::GEnumValue structs mapping the enum such as:
//     gobject_sys::GEnumValue {
//         value: Animal::Goat as i32,
//         value_name: "Goat\0" as *const _ as *const _,
//         value_nick: "goat\0" as *const _ as *const _,
//     },
fn gen_genum_values(
    enum_name: &Ident,
    enum_variants: &Punctuated<Variant, Comma>,
) -> (TokenStream, usize) {
    // start at one as GEnumValue array is null-terminated
    let mut n = 1;
    let recurse = enum_variants.iter().map(|v| {
        let name = &v.ident;
        let mut value_name = name.to_string().to_camel_case();
        let mut value_nick = name.to_string().to_kebab_case();

        let attrs = parse_item_attributes(&v.attrs);
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
            gobject_sys::GEnumValue {
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
    let from_glib = gen_from_glib(name, enum_variants);
    let (genum_values, nb_genum_values) = gen_genum_values(name, enum_variants);

    quote! {
        impl glib::translate::ToGlib for #name {
            type GlibType = i32;

            fn to_glib(&self) -> i32 {
                *self as i32
            }
        }

        impl ::glib::translate::FromGlib<i32> for #name {
            fn from_glib(value: i32) -> Self {
                #from_glib
                unreachable!();
            }
        }

        impl<'a> ::glib::value::FromValueOptional<'a> for #name {
            unsafe fn from_value_optional(value: &::glib::Value) -> Option<Self> {
                Some(::glib::value::FromValue::from_value(value))
            }
        }

        impl<'a> ::glib::value::FromValue<'a> for #name {
            unsafe fn from_value(value: &::glib::Value) -> Self {
                ::glib::translate::from_glib(
                    gobject_sys::g_value_get_enum(
                        ::glib::translate::ToGlibPtr::to_glib_none(value).0))
            }
        }

        impl ::glib::value::SetValue for #name {
            unsafe fn set_value(value: &mut ::glib::Value, this: &Self) {
                gobject_sys::g_value_set_enum(
                    ::glib::translate::ToGlibPtrMut::to_glib_none_mut(value).0,
                    ::glib::translate::ToGlib::to_glib(this))
            }
        }

        impl StaticType for #name {
            fn static_type() -> ::glib::Type {
                #get_type()
            }
        }

        fn #get_type() -> ::glib::Type {
            static ONCE: std::sync::Once = std::sync::Once::new();
            static mut TYPE: ::glib::Type = ::glib::Type::Invalid;

            ONCE.call_once(|| {
                static mut VALUES: [gobject_sys::GEnumValue; #nb_genum_values] = [
                    #genum_values
                    gobject_sys::GEnumValue {
                        value: 0,
                        value_name: std::ptr::null(),
                        value_nick: std::ptr::null(),
                    },
                ];

                let name = std::ffi::CString::new(#gtype_name).expect("CString::new failed");
                unsafe {
                    let type_ = gobject_sys::g_enum_register_static(name.as_ptr(), VALUES.as_ptr());
                    TYPE = ::glib::translate::from_glib(type_);
                }
            });

            unsafe {
                assert_ne!(TYPE, ::glib::Type::Invalid);
                TYPE
            }
        }
    }
}

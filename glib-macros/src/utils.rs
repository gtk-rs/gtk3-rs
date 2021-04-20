// Take a look at the license at the top of the repository in the LICENSE file.

use anyhow::{bail, Result};
use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_crate::crate_name;
use quote::{quote, quote_spanned};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, DeriveInput, Lit, Meta,
    MetaList, NestedMeta, Variant,
};

// find the #[@attr_name] attribute in @attrs
pub fn find_attribute_meta(attrs: &[Attribute], attr_name: &str) -> Result<Option<MetaList>> {
    let meta = match attrs.iter().find(|a| a.path.is_ident(attr_name)) {
        Some(a) => a.parse_meta(),
        _ => return Ok(None),
    };
    match meta? {
        Meta::List(n) => Ok(Some(n)),
        _ => bail!("wrong meta type"),
    }
}

// parse a single meta like: ident = "value"
fn parse_attribute(meta: &NestedMeta) -> Result<(String, String)> {
    let meta = match &meta {
        NestedMeta::Meta(m) => m,
        _ => bail!("wrong meta type"),
    };
    let meta = match meta {
        Meta::NameValue(n) => n,
        _ => bail!("wrong meta type"),
    };
    let value = match &meta.lit {
        Lit::Str(s) => s.value(),
        _ => bail!("wrong meta type"),
    };

    let ident = match meta.path.get_ident() {
        None => bail!("missing ident"),
        Some(ident) => ident,
    };

    Ok((ident.to_string(), value))
}

#[derive(Debug)]
pub enum EnumAttribute {
    TypeName(String),
}

pub fn parse_enum_attribute(meta: &NestedMeta) -> Result<EnumAttribute> {
    let (ident, v) = parse_attribute(meta)?;

    match ident.as_ref() {
        "type_name" => Ok(EnumAttribute::TypeName(v)),
        s => bail!("Unknown enum meta {}", s),
    }
}

pub fn find_nested_meta<'a>(meta: &'a MetaList, name: &str) -> Option<&'a NestedMeta> {
    meta.nested.iter().find(|n| match n {
        NestedMeta::Meta(m) => m.path().is_ident(name),
        _ => false,
    })
}

// Parse attribute such as:
// #[genum(type_name = "TestAnimalType")]
pub fn parse_type_name(input: &DeriveInput, attr_name: &str) -> Result<String> {
    let meta = match find_attribute_meta(&input.attrs, attr_name)? {
        Some(meta) => meta,
        _ => bail!("Missing '{}' attribute", attr_name),
    };

    let meta = match find_nested_meta(&meta, "type_name") {
        Some(meta) => meta,
        _ => bail!("Missing meta 'type_name'"),
    };

    match parse_enum_attribute(&meta)? {
        EnumAttribute::TypeName(n) => Ok(n),
    }
}

#[derive(Debug)]
pub enum ErrorDomainAttribute {
    Name(String),
}

pub fn parse_error_attribute(meta: &NestedMeta) -> Result<ErrorDomainAttribute> {
    let (ident, v) = parse_attribute(meta)?;

    match ident.as_ref() {
        "name" => Ok(ErrorDomainAttribute::Name(v)),
        s => bail!("Unknown enum meta {}", s),
    }
}

// Parse attribute such as:
// #[gerror_domain(name = "MyError")]
pub fn parse_name(input: &DeriveInput, attr_name: &str) -> Result<String> {
    let meta = match find_attribute_meta(&input.attrs, attr_name)? {
        Some(meta) => meta,
        _ => bail!("Missing '{}' attribute", attr_name),
    };

    let meta = match find_nested_meta(&meta, "name") {
        Some(meta) => meta,
        _ => bail!("Missing meta 'name'"),
    };

    match parse_error_attribute(&meta)? {
        ErrorDomainAttribute::Name(n) => Ok(n),
    }
}

#[derive(Debug)]
pub enum ItemAttribute {
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
pub fn parse_item_attributes(attr_name: &str, attrs: &[Attribute]) -> Result<Vec<ItemAttribute>> {
    let meta = find_attribute_meta(attrs, attr_name)?;

    let v = match meta {
        Some(meta) => meta
            .nested
            .iter()
            .map(|m| parse_item_attribute(&m))
            .collect::<Result<Vec<_>, _>>()?,
        None => Vec::new(),
    };

    Ok(v)
}

const KNOWN_GLIB_EXPORTS: [&str; 10] = [
    // Current Re-exports from gtk-rs
    "gtk",
    "gio",
    "gdk",
    "pango",
    "graphene-rs",
    // Current Re-exports from gtk4-rs
    "gtk4",
    "gsk4",
    "gdk4",
    // Special Request
    "gstreamer",
    // Re-export may fail
    "cairo-rs",
];

pub fn crate_ident_new() -> TokenStream {
    use proc_macro_crate::FoundCrate;

    let crate_path = match crate_name("glib") {
        Ok(FoundCrate::Name(name)) => Some(name),
        Ok(FoundCrate::Itself) => Some("glib".to_string()),
        Err(_) => None,
    }
    .map(|s| {
        let glib = Ident::new(&s, Span::call_site());
        quote!(#glib)
    });

    let crate_path = crate_path.or_else(|| {
        KNOWN_GLIB_EXPORTS.iter().find_map(|c| {
            if let Ok(f) = crate_name(c) {
                let crate_name = match f {
                    FoundCrate::Name(name) => name,
                    FoundCrate::Itself => c.to_string(),
                };
                let crate_root = Ident::new(&crate_name, Span::call_site());
                Some(quote::quote! {
                    #crate_root::glib
                })
            } else {
                None
            }
        })
    });

    crate_path.unwrap_or_else(|| {
        proc_macro_error::emit_call_site_warning!(
            "Can't find glib crate. Please ensure you have a glib in scope"
        );
        let glib = Ident::new("glib", Span::call_site());
        quote!(#glib)
    })
}

// Generate i32 to enum mapping, used to implement
// glib::translate::TryFromGlib<i32>, such as:
//
//   if value == Animal::Goat as i32 {
//       return Some(Animal::Goat);
//   }
pub fn gen_enum_from_glib(
    enum_name: &Ident,
    enum_variants: &Punctuated<Variant, Comma>,
) -> TokenStream {
    // FIXME: can we express this with a match()?
    let recurse = enum_variants.iter().map(|v| {
        let name = &v.ident;
        quote_spanned! { v.span() =>
            if value == #enum_name::#name as i32 {
                return Some(#enum_name::#name);
            }
        }
    });
    quote! {
        #(#recurse)*
        None
    }
}

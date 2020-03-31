// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use anyhow::{bail, Result};
use itertools::Itertools;
use syn::{Attribute, DeriveInput, Lit, Meta, MetaList, NestedMeta};

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
            .fold_results(Vec::new(), |mut v, a| {
                v.push(a);
                v
            })?,
        None => Vec::new(),
    };

    Ok(v)
}

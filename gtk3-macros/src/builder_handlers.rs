// Take a look at the license at the top of the repository in the LICENSE file.

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, quote_spanned};
use syn::{
    parse, spanned::Spanned, Attribute, Error, FnArg, ImplItem, ImplItemMethod, ItemImpl, Meta,
    MetaList, NestedMeta, PatType, Signature, Type,
};

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
pub struct HandlersImplAttributes {
    get_handler_fn: Option<String>,
}

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
struct HandlerAttributes {
    name: Option<String>,
}

#[derive(Debug)]
struct HandlerInfo {
    name: String,
    sig: Signature,
}

fn generate_handler(info: &HandlerInfo) -> Result<TokenStream2, Error> {
    let handler_name = &info.name;
    let arguments: Vec<TokenStream2> = info.sig.inputs
        .iter()
        .skip(1)
        .enumerate()
        .map(|(index, arg)| {
            let arg_type = argument_type(arg)?;
            Ok(quote_spanned! { arg.span() =>
                match values[#index].get() {
                    Ok(value) => value,
                    Err(error) => {
                        glib::g_critical!("builder handler", "Handler {} expects an argument of type {} but received `{:?}`: {}.", #handler_name, stringify!(#arg_type), values[#index], error);
                        return None;
                    },
                }
            })
        })
        .collect::<Result<_, Error>>()?;

    let signal = &info.name;
    let method = &info.sig.ident;
    let is_unit = matches!(info.sig.output, syn::ReturnType::Default);
    let handler = if is_unit {
        quote_spanned! { info.sig.span() =>
            #signal => Some({
                Box::new(glib::clone!(@weak self as this => @default-return None, move |values: &[glib::Value]| {
                    this.#method(#(#arguments),*);
                    None
                }))
            }),
        }
    } else {
        quote_spanned! { info.sig.span() =>
            #signal => Some({
                Box::new(glib::clone!(@weak self as this => @default-return None, move |values: &[glib::Value]| {
                    let result = this.#method(#(#arguments),*);
                    Some(glib::value::ToValue::to_value(&result))
                }))
            }),
        }
    };
    Ok(handler)
}

fn combine_errors(error_acc: &mut Option<Error>, error: Error) {
    match error_acc {
        Some(ref mut error_acc) => {
            error_acc.combine(error);
        }
        None => {
            error_acc.replace(error);
        }
    }
}

fn attributes_to_metas(attributes: Vec<Attribute>) -> Result<Vec<NestedMeta>, Error> {
    let mut metas = Vec::new();
    let mut error = None;
    for attr in attributes {
        let meta = attr.parse_meta()?;
        match meta {
            Meta::List(MetaList { nested, .. }) => metas.extend(nested),
            _ => combine_errors(&mut error, Error::new(attr.span(), "Unexpected attribute")),
        }
    }
    if let Some(error) = error {
        Err(error)
    } else {
        Ok(metas)
    }
}

fn is_assoc(sig: &Signature) -> bool {
    sig.inputs
        .first()
        .map_or(false, |arg| matches!(arg, FnArg::Receiver(..)))
}

fn argument_type(arg: &FnArg) -> Result<&Type, Error> {
    match arg {
        FnArg::Typed(PatType { ty, .. }) => Ok(&*ty),
        _ => Err(Error::new(
            arg.span(),
            "Cannot extract type of an argument.",
        )),
    }
}

fn generate_connect_method(
    attrs: &HandlersImplAttributes,
    actions: &[TokenStream2],
) -> ImplItemMethod {
    let get_handler_fn = format_ident!(
        "{}",
        attrs.get_handler_fn.as_deref().unwrap_or("get_handler")
    );
    let builder_connect_method = quote! {
        #[allow(clippy)]
        #[allow(unused_variables, unused_braces)]
        fn #get_handler_fn(&self, builder: &gtk::Builder, signal: &str) -> Option<Box<dyn Fn(&[glib::Value]) -> Option<glib::Value> + 'static>> {
            match signal {
                #(
                    #actions
                )*
                _ => None,
            }
        }
    };
    parse(builder_connect_method.into()).unwrap()
}

pub fn handlers(
    attrs: HandlersImplAttributes,
    mut input: ItemImpl,
) -> Result<TokenStream, TokenStream> {
    let mut handlers: Vec<HandlerInfo> = Vec::new();
    for item in input.items.iter_mut() {
        if let ImplItem::Method(method) = item {
            if !is_assoc(&method.sig) {
                return Err(Error::new(
                    method.sig.span(),
                    "Unsupported signature of method. Only associated methods are supported.",
                )
                .to_compile_error()
                .into());
            }

            let attributes =
                extract_from_vec(&mut method.attrs, |attr| attr.path.is_ident("handler"));
            let metas = attributes_to_metas(attributes).map_err(|err| err.to_compile_error())?;
            let attrs = HandlerAttributes::from_list(&metas)
                .map_err(|err| TokenStream::from(err.write_errors()))?;

            let info = HandlerInfo {
                name: attrs.name.unwrap_or_else(|| method.sig.ident.to_string()),
                sig: method.sig.clone(),
            };
            handlers.push(info);
        }
    }

    let connects: Vec<TokenStream2> = handlers
        .iter()
        .map(generate_handler)
        .collect::<Result<_, _>>()
        .map_err(|err| err.to_compile_error())?;

    let connect_method = generate_connect_method(&attrs, &connects);
    input.items.push(ImplItem::Method(connect_method));

    let s = quote!(#input);
    // println!("{}", s);
    Ok(s.into())
}

// TODO: Replace this by Vec::drain_filter as soon as it is stabilized.
fn extract_from_vec<T>(vec: &mut Vec<T>, predicate: impl Fn(&T) -> bool) -> Vec<T> {
    let mut i = 0;
    let mut result: Vec<T> = Vec::new();
    while i != vec.len() {
        if (predicate)(&vec[i]) {
            let item = vec.remove(i);
            result.push(item);
        } else {
            i += 1;
        }
    }
    result
}

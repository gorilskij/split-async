extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use syn::{Error, ImplItem, TraitItem};

use quote::quote;
use util::{TwoIdents, split_idents};
use visit::IntoAsync;

use crate::{parse::Item, visit::IntoSync};

mod parse;
mod util;
mod visit;

fn into_sync(input: &mut Item) -> TokenStream2 {
    match input {
        Item::Impl(item) => {
            for inner in &mut item.items {
                if let ImplItem::Fn(method) = inner {
                    method.sig.asyncness = None;
                }
            }
            IntoSync.into_sync(quote!(#item))
        }
        Item::Trait(item) => {
            for inner in &mut item.items {
                if let TraitItem::Fn(method) = inner {
                    method.sig.asyncness = None;
                }
            }
            IntoSync.into_sync(quote!(#item))
        }
        Item::Fn(item) => {
            item.sig.asyncness = None;
            IntoSync.into_sync(quote!(#item))
        } // Item::Static(item) => IntoSync.into_sync(quote!(#item)),
    }
}

fn into_async(input: &Item) -> TokenStream2 {
    match input {
        Item::Impl(item) => IntoAsync.into_async(quote!(#item)),
        Item::Trait(item) => IntoAsync.into_async(quote!(#item)),
        Item::Fn(item) => IntoAsync.into_async(quote!(#item)),
        // Item::Static(item) => IntoAsync.into_async(quote!(#item)),
    }
}

fn split_(args: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    let idents: Option<TwoIdents> = (!args.is_empty()).then(|| syn::parse(args)).transpose()?;

    let item: Item = syn::parse(input)?;

    let (mut sync_item, async_item) = match item {
        Item::Fn(item_fn) => {
            if item_fn.sig.asyncness.is_none() {
                return Err(Error::new_spanned(&item_fn.sig, "function must be async"));
            }
            let idents = idents.unwrap_or_else(|| split_idents(&item_fn.sig.ident));
            let mut sync_item = item_fn.clone();
            sync_item.sig.ident = idents.sync_ident;
            let mut async_item = item_fn;
            async_item.sig.ident = idents.async_ident;
            (Item::Fn(sync_item), Item::Fn(async_item))
        }
        _ => unimplemented!(),
    };

    let mut sync_ts = into_sync(&mut sync_item);
    let async_ts = into_async(&async_item);
    sync_ts.extend(async_ts);
    Ok(sync_ts.into())
}

/// Can be applied to trait item, trait impl, functions and struct impls.
#[proc_macro_attribute]
pub fn split(args: TokenStream, input: TokenStream) -> TokenStream {
    split_(args, input).unwrap_or_else(|err| err.to_compile_error().into())
}

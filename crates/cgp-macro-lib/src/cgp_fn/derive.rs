use core::mem;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemFn, Visibility};

use crate::cgp_fn::fn_body::inject_implicit_args;
use crate::cgp_fn::item_impl::derive_item_impl;
use crate::cgp_fn::item_trait::derive_item_trait;
use crate::cgp_fn::{extract_and_parse_implicit_args, parse_function_attributes};

pub fn derive_cgp_fn(trait_ident: &Ident, mut item_fn: ItemFn) -> syn::Result<TokenStream> {
    let visibility = item_fn.vis.clone();
    item_fn.vis = Visibility::Inherited;

    let implicit_args = extract_and_parse_implicit_args(&mut item_fn.sig.inputs)?;

    let attributes = parse_function_attributes(&mut item_fn.attrs)?;

    inject_implicit_args(&implicit_args, &mut item_fn.block)?;

    let generics = mem::take(&mut item_fn.sig.generics);

    let mut item_trait = derive_item_trait(trait_ident, &item_fn, &generics, &attributes)?;

    let item_impl = derive_item_impl(
        trait_ident,
        &item_fn,
        &implicit_args,
        &generics,
        &attributes,
    )?;

    item_trait.vis = visibility.clone();

    let output = quote! {
        #item_trait

        #item_impl
    };

    Ok(output)
}

use core::mem;

use cgp_macro_core::functions::extract_and_parse_implicit_args;
use cgp_macro_core::types::attributes::FunctionAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemFn, Visibility};

use crate::cgp_fn::item_impl::derive_item_impl;
use crate::cgp_fn::item_trait::derive_item_trait;

pub fn derive_cgp_fn(trait_ident: &Ident, mut item_fn: ItemFn) -> syn::Result<TokenStream> {
    let visibility = item_fn.vis.clone();
    item_fn.vis = Visibility::Inherited;

    let implicit_args = extract_and_parse_implicit_args(&mut item_fn.sig.inputs)?;
    implicit_args.prepend_to_block(&mut item_fn.block)?;

    let attributes = FunctionAttributes::parse(core::mem::take(&mut item_fn.attrs))?;

    let generics = mem::take(&mut item_fn.sig.generics);

    let mut item_trait = derive_item_trait(trait_ident, &item_fn, &generics, &attributes)?;
    item_trait.attrs.extend(attributes.raw_attributes.clone());

    let mut item_impl = derive_item_impl(
        trait_ident,
        &item_fn,
        &implicit_args,
        &generics,
        &attributes,
    )?;

    item_impl.attrs.extend(attributes.raw_attributes);

    item_trait.vis = visibility.clone();

    let output = quote! {
        #item_trait

        #item_impl
    };

    Ok(output)
}

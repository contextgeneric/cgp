use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, ItemTrait, parse2};

use crate::extra_macro_core::types::cgp_auto_dispatch::ItemCgpAutoDispatch;
use crate::extra_macro_lib::handler_fn::lower_handler_fn;

pub fn cgp_auto_dispatch(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    if !attr.is_empty() {
        return Err(Error::new_spanned(
            attr,
            "`#[cgp_auto_dispatch]` takes no arguments",
        ));
    }

    let item_trait: ItemTrait = parse2(body)?;

    let evaluated = ItemCgpAutoDispatch { item_trait }.preprocess()?.eval()?;

    let item_trait = &evaluated.item_trait;
    let blanket_impl = &evaluated.blanket_impl;

    let mut output = quote! {
        #item_trait
        #blanket_impl
    };

    for computer in evaluated.computers.iter() {
        output.extend(lower_handler_fn(&computer.preprocess()?.eval()?)?);
    }

    Ok(output)
}

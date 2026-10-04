use cgp_macro_extra_core::types::cgp_auto_dispatch::ItemCgpAutoDispatch;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemTrait, parse2};

use crate::handler_fn::lower_handler_fn;

pub fn cgp_auto_dispatch(_attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
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

use alloc::vec::Vec;

use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{ImplItemFn, Signature, Visibility, parse2};

use crate::derive_component::signature_args::signature_to_args;

pub fn derive_delegated_fn_impl(
    sig: &Signature,
    delegator: &TokenStream,
) -> syn::Result<ImplItemFn> {
    let fn_name = &sig.ident;

    let args: Punctuated<_, Comma> = signature_to_args(sig).collect();

    let await_expr: TokenStream = if sig.asyncness.is_some() {
        quote!( .await )
    } else {
        TokenStream::new()
    };

    let body = parse2(quote!({
        #delegator :: #fn_name (
            #args
        ) #await_expr
    }))?;

    let item = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig: sig.clone(),
        block: body,
    };

    Ok(item)
}

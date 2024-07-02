use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, ImplItemFn, Signature, TypePath, Visibility};

use crate::derive_component::signature_args::signature_to_args;

pub fn derive_delegated_fn_impl(sig: &Signature, delegator: &TypePath) -> ImplItemFn {
    let fn_name = &sig.ident;

    let args = signature_to_args(sig);

    let await_expr: TokenStream = if sig.asyncness.is_some() {
        quote!( .await )
    } else {
        TokenStream::new()
    };

    let body = parse_quote!({
        #delegator :: #fn_name (
            #args
        ) #await_expr
    });

    ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig: sig.clone(),
        block: body,
    }
}

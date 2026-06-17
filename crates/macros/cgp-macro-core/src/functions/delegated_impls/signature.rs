use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{FnArg, Ident, ImplItemFn, Signature, Type, Visibility};

use crate::functions::parse_internal;

pub fn signature_to_delegated_impl_item_fn(
    signature: &Signature,
    delegate_type: &Type,
) -> syn::Result<ImplItemFn> {
    let fn_name = &signature.ident;

    let args: Punctuated<_, Comma> = signature_to_idents(signature)?;

    let await_expr: TokenStream = if signature.asyncness.is_some() {
        quote!( .await )
    } else {
        TokenStream::new()
    };

    let body = parse_internal!({
        #delegate_type :: #fn_name (
            #args
        ) #await_expr
    });

    let item = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig: signature.clone(),
        block: body,
    };

    Ok(item)
}

fn signature_to_idents(sig: &Signature) -> syn::Result<Punctuated<Ident, Comma>> {
    sig.inputs.iter().map(arg_to_ident).collect()
}

fn arg_to_ident(arg: &FnArg) -> syn::Result<Ident> {
    let ident = match arg {
        FnArg::Receiver(_) => Ident::new("self", Span::call_site()),
        FnArg::Typed(pat) => parse_internal(pat.pat.to_token_stream())?,
    };

    Ok(ident)
}

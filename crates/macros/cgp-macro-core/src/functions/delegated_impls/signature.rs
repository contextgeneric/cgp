use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Error, FnArg, Ident, ImplItemFn, Pat, Signature, Type, Visibility};

use crate::functions::parse_internal;

/// Build an impl method that keeps `signature` and forwards its arguments to the
/// same method on `delegate_type`, adding `.await` when the signature is `async`.
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

/// The name an argument is forwarded by. A parameter that is not a plain name (`_`, `mut value`,
/// a destructuring pattern) has none, so it is rejected with a message saying what to write.
fn arg_to_ident(arg: &FnArg) -> syn::Result<Ident> {
    match arg {
        FnArg::Receiver(_) => Ok(Ident::new("self", Span::call_site())),
        FnArg::Typed(arg) => match arg.pat.as_ref() {
            Pat::Ident(pat)
                if pat.by_ref.is_none() && pat.mutability.is_none() && pat.subpat.is_none() =>
            {
                Ok(pat.ident.clone())
            }
            pat => Err(Error::new_spanned(
                pat,
                "a component method parameter needs a plain name, since the generated impls forward each argument by name; write a name such as `_value`, and destructure inside a default body",
            )),
        },
    }
}

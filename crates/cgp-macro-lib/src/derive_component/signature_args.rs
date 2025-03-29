use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_quote, FnArg, Ident, Signature};

pub fn signature_to_args(sig: &Signature) -> Punctuated<Ident, Comma> {
    let args = sig
        .inputs
        .iter()
        .map(|arg| -> Ident {
            match arg {
                FnArg::Receiver(_) => Ident::new("self", Span::call_site()),
                FnArg::Typed(pat) => {
                    let ident_pat = &pat.pat;
                    parse_quote!( #ident_pat )
                }
            }
        })
        .collect();

    args
}

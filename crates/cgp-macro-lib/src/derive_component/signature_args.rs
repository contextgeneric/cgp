use proc_macro2::Span;
use syn::{parse_quote, FnArg, Ident, Signature};

pub fn signature_to_args(sig: &Signature) -> impl Iterator<Item = Ident> + '_ {
    sig.inputs.iter().map(|arg| -> Ident {
        match arg {
            FnArg::Receiver(_) => Ident::new("self", Span::call_site()),
            FnArg::Typed(pat) => {
                let ident_pat = &pat.pat;
                parse_quote!( #ident_pat )
            }
        }
    })
}

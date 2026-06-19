use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemFn, parse2};

use crate::keywords::CgpFn;
use crate::types::AttributeMacroSnapshot;

pub fn snapshot_cgp_fn(body: TokenStream) -> syn::Result<TokenStream> {
    let item: AttributeMacroSnapshot<CgpFn, ItemFn> = parse2(body)?;

    let output = cgp_macro_lib::cgp_fn(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}

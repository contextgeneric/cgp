use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemFn, parse2};

use crate::test_util_lib::keywords::CgpFn;
use crate::test_util_lib::types::AttributeMacroSnapshot;

pub fn snapshot_cgp_fn(body: TokenStream) -> syn::Result<TokenStream> {
    let item: AttributeMacroSnapshot<CgpFn, ItemFn> = parse2(body)?;

    let output = crate::macro_lib::cgp_fn(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}

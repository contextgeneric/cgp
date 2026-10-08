use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemImpl, parse2};

use crate::test_util_lib::keywords::CgpImpl;
use crate::test_util_lib::types::AttributeMacroSnapshot;

pub fn snapshot_cgp_impl(body: TokenStream) -> syn::Result<TokenStream> {
    let item: AttributeMacroSnapshot<CgpImpl, ItemImpl> = parse2(body)?;

    let output = crate::macro_lib::cgp_impl(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}

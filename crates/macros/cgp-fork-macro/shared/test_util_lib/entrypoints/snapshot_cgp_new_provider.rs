use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemImpl, parse2};

use crate::test_util_lib::keywords::CgpNewProvider;
use crate::test_util_lib::types::AttributeMacroSnapshot;

pub fn snapshot_cgp_new_provider(body: TokenStream) -> syn::Result<TokenStream> {
    let item: AttributeMacroSnapshot<CgpNewProvider, ItemImpl> = parse2(body)?;

    let output = crate::macro_lib::cgp_new_provider(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}

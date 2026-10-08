use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemImpl, parse2};

use crate::test_util_lib::keywords::CgpAutoError;
use crate::test_util_lib::types::AttributeMacroSnapshot;

pub fn snapshot_cgp_auto_error(body: TokenStream) -> syn::Result<TokenStream> {
    let item: AttributeMacroSnapshot<CgpAutoError, ItemImpl> = parse2(body)?;

    let output = crate::macro_lib::cgp_auto_error(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}

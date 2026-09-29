use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemImpl, parse2};

use crate::keywords::CgpAutoError;
use crate::types::AttributeMacroSnapshot;

pub fn snapshot_cgp_auto_error(body: TokenStream) -> syn::Result<TokenStream> {
    let item: AttributeMacroSnapshot<CgpAutoError, ItemImpl> = parse2(body)?;

    let output = cgp_fork_macro_lib::cgp_auto_error(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}

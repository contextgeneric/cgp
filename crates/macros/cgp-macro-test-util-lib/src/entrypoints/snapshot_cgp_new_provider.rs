use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemImpl, parse2};

use crate::keywords::CgpNewProvider;
use crate::types::AttributeMacroSnapshot;

pub fn snapshot_cgp_new_provider(body: TokenStream) -> syn::Result<TokenStream> {
    let item: AttributeMacroSnapshot<CgpNewProvider, ItemImpl> = parse2(body)?;

    let output = cgp_macro_lib::cgp_new_provider(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}

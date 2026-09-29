use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemTrait, parse2};

use crate::keywords::CgpAutoImpl;
use crate::types::AttributeMacroSnapshot;

pub fn snapshot_cgp_auto_impl(body: TokenStream) -> syn::Result<TokenStream> {
    let item: AttributeMacroSnapshot<CgpAutoImpl, ItemTrait> = parse2(body)?;

    let output = cgp_macro_lib::cgp_auto_impl(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemTrait, parse2};

use crate::keywords::CgpGetter;
use crate::types::AttributeMacroSnapshot;

pub fn snapshot_cgp_getter(body: TokenStream) -> syn::Result<TokenStream> {
    let item: AttributeMacroSnapshot<CgpGetter, ItemTrait> = parse2(body)?;

    let output = cgp_macro_lib::cgp_getter(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}

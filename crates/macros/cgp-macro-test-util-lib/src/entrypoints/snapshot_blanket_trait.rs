use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemTrait, parse2};

use crate::keywords::BlanketTrait;
use crate::types::AttributeMacroSnapshot;

pub fn snapshot_blanket_trait(body: TokenStream) -> syn::Result<TokenStream> {
    let item: AttributeMacroSnapshot<BlanketTrait, ItemTrait> = parse2(body)?;

    let output = cgp_macro_lib::blanket_trait(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}

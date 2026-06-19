use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemTrait, parse2};

use crate::keywords::CgpComponent;
use crate::types::AttributeMacroSnapshot;

pub fn snapshot_cgp_component(body: TokenStream) -> syn::Result<TokenStream> {
    let item: AttributeMacroSnapshot<CgpComponent, ItemTrait> = parse2(body)?;

    let output = cgp_macro_lib::cgp_component(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}

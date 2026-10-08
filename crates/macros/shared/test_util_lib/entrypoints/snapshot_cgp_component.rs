use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemTrait, parse2};

use crate::test_util_lib::keywords::CgpComponent;
use crate::test_util_lib::types::AttributeMacroSnapshot;

pub fn snapshot_cgp_component(body: TokenStream) -> syn::Result<TokenStream> {
    let item: AttributeMacroSnapshot<CgpComponent, ItemTrait> = parse2(body)?;

    let output = crate::macro_lib::cgp_component(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}

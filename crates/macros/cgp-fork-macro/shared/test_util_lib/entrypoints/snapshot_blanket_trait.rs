use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemTrait, parse2};

use crate::test_util_lib::keywords::BlanketTrait;
use crate::test_util_lib::types::AttributeMacroSnapshot;

pub fn snapshot_blanket_trait(body: TokenStream) -> syn::Result<TokenStream> {
    let item: AttributeMacroSnapshot<BlanketTrait, ItemTrait> = parse2(body)?;

    let output = crate::macro_lib::blanket_trait(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}

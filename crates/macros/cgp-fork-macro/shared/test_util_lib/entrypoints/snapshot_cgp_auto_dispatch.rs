use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemTrait, parse2};

use crate::test_util_lib::keywords::CgpAutoDispatch;
use crate::test_util_lib::types::AttributeMacroSnapshot;

pub fn snapshot_cgp_auto_dispatch(body: TokenStream) -> syn::Result<TokenStream> {
    let item: AttributeMacroSnapshot<CgpAutoDispatch, ItemTrait> = parse2(body)?;

    let output = crate::extra_macro_lib::cgp_auto_dispatch(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}

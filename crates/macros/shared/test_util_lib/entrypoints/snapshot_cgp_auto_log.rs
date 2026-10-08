use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemTrait, parse2};

use crate::test_util_lib::keywords::CgpAutoLog;
use crate::test_util_lib::types::AttributeMacroSnapshot;

pub fn snapshot_cgp_auto_log(body: TokenStream) -> syn::Result<TokenStream> {
    let item: AttributeMacroSnapshot<CgpAutoLog, ItemTrait> = parse2(body)?;

    let output = crate::extra_macro_lib::cgp_auto_log(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}

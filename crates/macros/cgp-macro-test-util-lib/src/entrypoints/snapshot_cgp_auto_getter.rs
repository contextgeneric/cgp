use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::types::SnapshotCgpAutoGetter;

pub fn snapshot_cgp_auto_getter(body: TokenStream) -> syn::Result<TokenStream> {
    let item: SnapshotCgpAutoGetter = parse2(body)?;

    let output = cgp_macro_lib::cgp_auto_getter(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}

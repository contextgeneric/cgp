use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::types::SnapshotCgpGetter;

pub fn snapshot_cgp_getter(body: TokenStream) -> syn::Result<TokenStream> {
    let item: SnapshotCgpGetter = parse2(body)?;

    let output = cgp_macro_lib::cgp_getter(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::types::SnapshotCgpType;

pub fn snapshot_cgp_type(body: TokenStream) -> syn::Result<TokenStream> {
    let item: SnapshotCgpType = parse2(body)?;

    let output = cgp_macro_lib::cgp_type(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}

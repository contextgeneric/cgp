use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::types::SnapshotCgpImpl;

pub fn snapshot_cgp_impl(body: TokenStream) -> syn::Result<TokenStream> {
    let item: SnapshotCgpImpl = parse2(body)?;

    let output = cgp_macro_lib::cgp_impl(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}

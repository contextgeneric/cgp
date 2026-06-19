use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::types::SnapshotCgpComponent;

pub fn snapshot_cgp_component(body: TokenStream) -> syn::Result<TokenStream> {
    let item: SnapshotCgpComponent = parse2(body)?;

    let output = cgp_macro_lib::cgp_component(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}

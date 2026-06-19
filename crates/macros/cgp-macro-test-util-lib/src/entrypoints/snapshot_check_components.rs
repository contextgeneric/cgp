use proc_macro2::TokenStream;
use syn::parse2;

use crate::types::AssertCheckComponents;

pub fn snapshot_check_components(body: TokenStream) -> syn::Result<TokenStream> {
    let item: AssertCheckComponents = parse2(body)?;

    let output = cgp_macro_lib::check_components(item.body.clone())?;

    item.snapshot.wrap_output(output)
}

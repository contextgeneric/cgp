use proc_macro2::TokenStream;
use syn::parse2;

use crate::types::AssertCgpNamespace;

pub fn snapshot_cgp_namespace(body: TokenStream) -> syn::Result<TokenStream> {
    let item: AssertCgpNamespace = parse2(body)?;

    let output = cgp_macro_lib::cgp_namespace(item.body.clone())?;

    item.snapshot.wrap_output(output)
}

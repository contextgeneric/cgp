use proc_macro2::TokenStream;
use syn::parse2;

use crate::types::AssertDelegateAndCheckComponents;

pub fn snapshot_delegate_and_check_components(body: TokenStream) -> syn::Result<TokenStream> {
    let item: AssertDelegateAndCheckComponents = parse2(body)?;

    let output = cgp_macro_lib::delegate_and_check_components(item.body.clone())?;

    item.snapshot.wrap_output(output)
}

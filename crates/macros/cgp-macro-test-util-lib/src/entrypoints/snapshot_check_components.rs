use proc_macro2::TokenStream;
use syn::parse2;

use crate::keywords::CheckComponents;
use crate::types::StatementMacroSnapshot;

pub fn snapshot_check_components(body: TokenStream) -> syn::Result<TokenStream> {
    let item: StatementMacroSnapshot<CheckComponents> = parse2(body)?;

    let output = cgp_macro_lib::check_components(item.body.clone())?;

    item.snapshot.wrap_output(output)
}

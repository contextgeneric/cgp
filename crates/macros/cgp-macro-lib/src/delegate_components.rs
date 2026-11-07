use cgp_macro_core::types::delegate_component::{DelegateTable, ValidateAttributes};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

/// Build a context's wiring table, naming the provider that implements each component, plus the operators, path keys and statements its body accepts.
///
/// See [`delegate_components`](https://contextgeneric.dev/docs/reference/macros/delegate_components).
pub fn delegate_components(body: TokenStream) -> syn::Result<TokenStream> {
    let table: DelegateTable = parse2(body.clone())?;

    // `delegate_components!` does not support any attributes on the table or its
    // keys, so reject them instead of silently parsing and discarding them.
    table.validate_attributes()?;

    let evaluated_table = table.eval()?;

    Ok(evaluated_table.to_token_stream())
}

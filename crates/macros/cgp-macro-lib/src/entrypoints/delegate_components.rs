use cgp_macro_core::types::delegate_component::{DelegateTable, ValidateAttributes};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

pub fn delegate_components(body: TokenStream) -> syn::Result<TokenStream> {
    let table: DelegateTable = parse2(body.clone())?;

    // `delegate_components!` does not support any attributes on the table or its
    // keys, so reject them instead of silently parsing and discarding them.
    table.validate_attributes()?;

    let evaluated_table = table.eval()?;

    Ok(evaluated_table.to_token_stream())
}

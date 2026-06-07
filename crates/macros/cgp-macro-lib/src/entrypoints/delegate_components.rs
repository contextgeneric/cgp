use cgp_macro_core::types::delegate_component::DelegateTable;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

pub fn delegate_components(body: TokenStream) -> syn::Result<TokenStream> {
    let table: DelegateTable = parse2(body.clone())?;

    let evaluated_table = table.eval()?;

    Ok(evaluated_table.to_token_stream())
}

use cgp_macro_core::types::namespace::NamespaceTable;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

pub fn cgp_namespace(body: TokenStream) -> syn::Result<TokenStream> {
    let namespace_table: NamespaceTable = parse2(body)?;
    Ok(namespace_table.eval()?.to_token_stream())
}

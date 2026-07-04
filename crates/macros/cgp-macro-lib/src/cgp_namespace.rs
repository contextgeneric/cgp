use cgp_macro_core::types::delegate_component::ValidateAttributes;
use cgp_macro_core::types::namespace::NamespaceTable;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

pub fn cgp_namespace(body: TokenStream) -> syn::Result<TokenStream> {
    let namespace_table: NamespaceTable = parse2(body)?;

    // A namespace body reuses the `delegate_components!` entry grammar, which
    // supports no attributes, so reject any on its entries instead of silently
    // parsing and discarding them — matching `delegate_components!`.
    namespace_table.entries.validate_attributes()?;

    Ok(namespace_table.eval()?.to_token_stream())
}

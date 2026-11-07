use cgp_macro_core::types::delegate_component::ValidateAttributes;
use cgp_macro_core::types::namespace::NamespaceTable;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

/// Define a reusable, inheritable wiring table that many contexts can join, so a configuration stays short as the component count grows.
///
/// See [`cgp_namespace`](https://contextgeneric.dev/docs/reference/macros/cgp_namespace).
pub fn cgp_namespace(body: TokenStream) -> syn::Result<TokenStream> {
    let namespace_table: NamespaceTable = parse2(body)?;

    // A namespace body reuses the `delegate_components!` entry grammar, which
    // supports no attributes, so reject any on its entries instead of silently
    // parsing and discarding them — matching `delegate_components!`.
    namespace_table.entries.validate_attributes()?;

    Ok(namespace_table.eval()?.to_token_stream())
}

use cgp_macro_core::types::delegate_component::NamespaceTable;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::cgp_namespace::{NamespaceSpec, derive_namespace};

pub fn cgp_namespace(body: TokenStream) -> syn::Result<TokenStream> {
    let namespace_table: NamespaceTable = parse2(body)?;
    Ok(namespace_table.eval()?.to_token_stream())

    // let spec: NamespaceSpec = parse2(body)?;

    // derive_namespace(spec)
}

use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, parse2};

use crate::keywords::ExtractField;
use crate::types::DeriveMacroSnapshot;

pub fn snapshot_derive_extract_field(body: TokenStream) -> syn::Result<TokenStream> {
    let item: DeriveMacroSnapshot<ExtractField, ItemEnum> = parse2(body)?;

    let body = &item.body;

    let output = cgp_macro_lib::derive_extract_field(quote! {
        #[derive(ExtractField)]
        #body
    })?;

    let wrapped = item.snapshot.wrap_output(output)?;

    Ok(quote! {
        #body

        #wrapped
    })
}

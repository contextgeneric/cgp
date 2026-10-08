use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, parse2};

use crate::test_util_lib::keywords::ExtractField;
use crate::test_util_lib::types::DeriveMacroSnapshot;

pub fn snapshot_derive_extract_field(body: TokenStream) -> syn::Result<TokenStream> {
    let item: DeriveMacroSnapshot<ExtractField, ItemEnum> = parse2(body)?;

    let body = &item.body;

    let output = crate::macro_lib::derive_extract_field(quote! {
        #[derive(ExtractField)]
        #body
    })?;

    let wrapped = item.snapshot.wrap_output(output)?;

    Ok(quote! {
        #body

        #wrapped
    })
}

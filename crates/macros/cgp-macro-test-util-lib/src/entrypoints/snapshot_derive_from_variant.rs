use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, parse2};

use crate::keywords::FromVariant;
use crate::types::DeriveMacroSnapshot;

pub fn snapshot_derive_from_variant(body: TokenStream) -> syn::Result<TokenStream> {
    let item: DeriveMacroSnapshot<FromVariant, ItemEnum> = parse2(body)?;

    let body = &item.body;

    let output = cgp_macro_lib::derive_from_variant(quote! {
        #[derive(FromVariant)]
        #body
    })?;

    let wrapped = item.snapshot.wrap_output(output)?;

    Ok(quote! {
        #body

        #wrapped
    })
}

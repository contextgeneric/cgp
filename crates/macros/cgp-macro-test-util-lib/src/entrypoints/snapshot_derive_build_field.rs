use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse2};

use crate::keywords::BuildField;
use crate::types::DeriveMacroSnapshot;

pub fn snapshot_derive_build_field(body: TokenStream) -> syn::Result<TokenStream> {
    let item: DeriveMacroSnapshot<BuildField, ItemStruct> = parse2(body)?;

    let body = &item.body;

    let output = cgp_macro_lib::derive_build_field(quote! {
        #[derive(BuildField)]
        #body
    })?;

    let wrapped = item.snapshot.wrap_output(output)?;

    Ok(quote! {
        #body

        #wrapped
    })
}

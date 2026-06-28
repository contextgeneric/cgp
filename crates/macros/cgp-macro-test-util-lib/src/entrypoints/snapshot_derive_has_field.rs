use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse2};

use crate::keywords::HasField;
use crate::types::DeriveMacroSnapshot;

pub fn snapshot_derive_has_field(body: TokenStream) -> syn::Result<TokenStream> {
    let item: DeriveMacroSnapshot<HasField, ItemStruct> = parse2(body)?;

    let body = &item.body;

    let output = cgp_macro_lib::derive_has_field(quote! {
        #[derive(HasField)]
        #body
    })?;

    let wrapped = item.snapshot.wrap_output(output)?;

    Ok(quote! {
        #body

        #wrapped
    })
}

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Item, parse2};

use crate::keywords::HasFields;
use crate::types::DeriveMacroSnapshot;

pub fn snapshot_derive_has_fields(body: TokenStream) -> syn::Result<TokenStream> {
    let item: DeriveMacroSnapshot<HasFields, Item> = parse2(body)?;

    let body = &item.body;

    let output = cgp_macro_lib::derive_has_fields(quote! {
        #[derive(HasFields)]
        #body
    })?;

    let wrapped = item.snapshot.wrap_output(output)?;

    Ok(quote! {
        #body

        #wrapped
    })
}

use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse2};

use crate::test_util_lib::keywords::HasField;
use crate::test_util_lib::types::DeriveMacroSnapshot;

pub fn snapshot_derive_has_field(body: TokenStream) -> syn::Result<TokenStream> {
    let item: DeriveMacroSnapshot<HasField, ItemStruct> = parse2(body)?;

    let body = &item.body;

    let output = crate::macro_lib::derive_has_field(quote! {
        #[derive(HasField)]
        #body
    })?;

    let wrapped = item.snapshot.wrap_output(output)?;

    Ok(quote! {
        #body

        #wrapped
    })
}

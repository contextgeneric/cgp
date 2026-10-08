use proc_macro2::TokenStream;
use quote::quote;
use syn::{Item, parse2};

use crate::test_util_lib::keywords::CgpData;
use crate::test_util_lib::types::DeriveMacroSnapshot;

pub fn snapshot_derive_cgp_data(body: TokenStream) -> syn::Result<TokenStream> {
    let item: DeriveMacroSnapshot<CgpData, Item> = parse2(body)?;

    let body = &item.body;

    let output = crate::macro_lib::derive_cgp_data(quote! {
        #[derive(CgpData)]
        #body
    })?;

    let wrapped = item.snapshot.wrap_output(output)?;

    Ok(quote! {
        #body

        #wrapped
    })
}
